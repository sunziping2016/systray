use std::{collections::HashSet, mem, process};

use derive_more::{Display, Error, From};
use futures::{future::OptionFuture, StreamExt};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use zbus::{
    fdo::DBusProxy,
    names::{BusName, WellKnownName},
    zvariant::ObjectPath,
    Connection, Result,
};

use crate::{
    proxy::notifier_watcher::{
        StatusNotifierItemRegisteredStream, StatusNotifierItemUnregisteredStream,
        StatusNotifierWatcherProxy,
    },
    service::STATUS_NOTIFIER_WATCHER_SERVICE,
};

#[derive(Debug, Clone)]
pub enum ItemMessage {
    Added(BusName<'static>, ObjectPath<'static>),
    Removed(BusName<'static>, ObjectPath<'static>),
}

#[derive(Debug)]
pub struct StatusNotifierHost {
    name: WellKnownName<'static>,
}

#[derive(Error, Debug, Display, From)]
enum ContextError {
    ZBus(zbus::Error),
    Send(mpsc::error::SendError<ItemMessage>),
}

type ContextResult<T> = std::result::Result<T, ContextError>;

struct StatusNotifierHostContext {
    host: StatusNotifierHost,
    services: HashSet<(BusName<'static>, ObjectPath<'static>)>,
    tx: mpsc::Sender<ItemMessage>,
}

fn try_into_service_path(service: &str) -> zbus::names::Result<(BusName<'_>, ObjectPath<'_>)> {
    if let Some(pos) = service.find('/') {
        Ok((
            BusName::try_from(&service[..pos])?,
            ObjectPath::try_from(&service[pos..])?,
        ))
    } else {
        Err(zbus::names::Error::InvalidBusName("".into(), "".into()))
    }
}

impl<'c> StatusNotifierHostContext {
    fn new(host: StatusNotifierHost, tx: mpsc::Sender<ItemMessage>) -> Self {
        Self {
            host,
            services: HashSet::new(),
            tx,
        }
    }

    async fn add_watcher(
        &mut self,
        connection: &Connection,
    ) -> ContextResult<(
        StatusNotifierItemRegisteredStream<'c>,
        StatusNotifierItemUnregisteredStream<'c>,
    )> {
        let result = async {
            let proxy = StatusNotifierWatcherProxy::new(connection).await?;
            proxy
                .register_status_notifier_host(self.host.name().as_str())
                .await?;
            let item_registered = proxy.receive_status_notifier_item_registered().await?;
            let item_unregistered = proxy.receive_status_notifier_item_unregistered().await?;
            for item in proxy.registered_status_notifier_items().await? {
                self.add_item(&item).await?;
            }
            Ok((item_registered, item_unregistered))
        }
        .await;
        match result.as_ref() {
            Err(ContextError::ZBus(_)) => {
                self.remove_watcher().await?;
            }
            _ => (),
        }
        result
    }

    async fn remove_watcher(
        &mut self,
    ) -> std::result::Result<(), mpsc::error::SendError<ItemMessage>> {
        for (service, path) in mem::take(&mut self.services) {
            self.tx.send(ItemMessage::Removed(service, path)).await?;
        }
        Ok(())
    }

    async fn add_item(
        &mut self,
        item: &str,
    ) -> std::result::Result<(), mpsc::error::SendError<ItemMessage>> {
        if let Ok((service, path)) = try_into_service_path(&item) {
            let service = service.to_owned();
            let path = path.to_owned();
            if self.services.insert((service.clone(), path.clone())) {
                self.tx.send(ItemMessage::Added(service, path)).await?;
            }
        }
        Ok(())
    }

    async fn remove_item(
        &mut self,
        item: &str,
    ) -> std::result::Result<(), mpsc::error::SendError<ItemMessage>> {
        if let Ok((service, path)) = try_into_service_path(&item) {
            let key = (service.to_owned(), path.to_owned());
            if self.services.remove(&key) {
                self.tx.send(ItemMessage::Removed(key.0, key.1)).await?;
            }
        }
        Ok(())
    }
}

impl StatusNotifierHost {
    pub fn new() -> Self {
        let name = WellKnownName::try_from(
            format!("org.kde.StatusNotifierHost-{}", process::id()).as_str(),
        )
        .unwrap()
        .into_owned();
        Self { name }
    }

    pub fn name(&self) -> &WellKnownName<'static> {
        &self.name
    }

    pub async fn run(self, cancel: CancellationToken, tx: mpsc::Sender<ItemMessage>) -> Result<()> {
        let connection = Connection::session().await?;
        connection.request_name(self.name()).await?;
        let dbus = DBusProxy::new(&connection).await?;
        let mut name_owner_changed = dbus.receive_name_owner_changed().await?;
        let mut item_added = None;
        let mut item_removed = None;

        let mut context = StatusNotifierHostContext::new(self, tx);
        match context.add_watcher(&connection).await {
            Ok((x, y)) => {
                item_added = Some(x);
                item_removed = Some(y);
            }
            Err(ContextError::ZBus(_)) => (),
            Err(ContextError::Send(_)) => return Ok(()),
        }
        loop {
            tokio::select! {
                Some(signal) = name_owner_changed.next() => {
                    let args = signal.args()?;
                    if args.name == STATUS_NOTIFIER_WATCHER_SERVICE {
                        if args.new_owner.is_none() && item_added.is_some() {
                            match context.remove_watcher().await {
                                Ok(_) => {
                                    item_added = None;
                                    item_removed = None;
                                }
                                Err(_) => break,
                            }
                        } else if args.old_owner.is_none() && item_added.is_none() {
                            match context.add_watcher(&connection).await {
                                Ok((x, y)) => {
                                    item_added = Some(x);
                                    item_removed = Some(y);
                                }
                                Err(ContextError::ZBus(_)) => (),
                                Err(ContextError::Send(_)) => return Ok(()),
                            }
                        }
                    }
                },
                Some(Some(signal)) =
                    OptionFuture::from(item_added.as_mut().map(|x| x.next())) =>
                {
                    if context.add_item(&signal.args()?.service).await.is_err() {
                        break;
                    }
                },
                Some(Some(signal)) =
                    OptionFuture::from(item_removed.as_mut().map(|x| x.next())) =>
                {
                    if context.remove_item(&signal.args()?.service).await.is_err() {
                        break;
                    }
                },
                _ = cancel.cancelled() => break,
            }
        }
        Ok(())
    }
}
