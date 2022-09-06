use std::{collections::HashSet, mem, process};

use async_stream::stream;
use futures::{future::OptionFuture, Stream, StreamExt};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
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
    service::NOTIFIER_WATCHER_SERVICE,
};

pub type RemoteObject<'a> = (BusName<'a>, ObjectPath<'a>);
pub type RemoteObjectOwned = RemoteObject<'static>;

#[derive(Debug, Clone)]
pub enum NotifierHostMessage {
    Added(RemoteObjectOwned),
    Removed(RemoteObjectOwned),
}

#[derive(Debug)]
pub struct NotifierHost {
    name: WellKnownName<'static>,
    cancel: Option<CancellationToken>,
}

#[derive(Default)]
struct NotifierHostContext {
    services: HashSet<RemoteObjectOwned>,
}

fn try_into_service_path(service: &str) -> zbus::names::Result<RemoteObject<'_>> {
    if let Some(pos) = service.find('/') {
        Ok((
            BusName::try_from(&service[..pos])?,
            ObjectPath::try_from(&service[pos..])?,
        ))
    } else {
        Err(zbus::names::Error::InvalidBusName("".into(), "".into()))
    }
}

impl NotifierHostContext {
    fn new() -> Self {
        Self::default()
    }

    async fn add_watcher<'c>(
        &mut self,
        connection: &Connection,
        name: &str,
    ) -> Result<(
        Vec<RemoteObjectOwned>,
        StatusNotifierItemRegisteredStream<'c>,
        StatusNotifierItemUnregisteredStream<'c>,
    )> {
        let proxy = StatusNotifierWatcherProxy::new(connection).await?;
        proxy.register_status_notifier_host(name).await?;
        let item_registered = proxy.receive_status_notifier_item_registered().await?;
        let item_unregistered = proxy.receive_status_notifier_item_unregistered().await?;
        let items = proxy
            .registered_status_notifier_items()
            .await?
            .into_iter()
            .filter_map(|x| self.add_item(&x))
            .collect::<Vec<_>>();
        Ok((items, item_registered, item_unregistered))
    }

    fn remove_watcher(&mut self) -> HashSet<RemoteObjectOwned> {
        mem::take(&mut self.services)
    }

    fn add_item(&mut self, item: &str) -> Option<RemoteObjectOwned> {
        if let Ok((service, path)) = try_into_service_path(&item) {
            let service = service.to_owned();
            let path = path.to_owned();
            if self.services.insert((service.clone(), path.clone())) {
                return Some((service, path));
            }
        }
        None
    }

    fn remove_item(&mut self, item: &str) -> Option<RemoteObjectOwned> {
        if let Ok((service, path)) = try_into_service_path(&item) {
            let key = (service.to_owned(), path.to_owned());
            if self.services.remove(&key) {
                return Some(key);
            }
        }
        None
    }
}

impl NotifierHost {
    pub fn new() -> Self {
        let name = WellKnownName::try_from(
            format!("org.kde.StatusNotifierHost-{}", process::id()).as_str(),
        )
        .unwrap()
        .into_owned();
        Self { name, cancel: None }
    }

    pub fn cancellable(self, cancel: CancellationToken) -> Self {
        Self {
            cancel: Some(cancel),
            ..self
        }
    }

    pub fn name(&self) -> &WellKnownName<'static> {
        &self.name
    }

    pub async fn run(self) -> Result<impl Stream<Item = NotifierHostMessage>> {
        let Self { name, mut cancel } = self;
        let connection = Connection::session().await?;
        connection.request_name(&name).await?;
        let dbus = DBusProxy::new(&connection).await?;
        let mut name_owner_changed = dbus.receive_name_owner_changed().await?;
        Ok(stream! {
            let mut context = NotifierHostContext::new();

            let mut item_added = None;
            let mut item_removed = None;


            if let Ok((items, added, removed)) = context.add_watcher(&connection, &name).await {
                info!("watcher connected");
                for item in items {
                    yield NotifierHostMessage::Added(item);
                }
                item_added = Some(added);
                item_removed = Some(removed);
            }
            loop {
                match tokio::select! {
                    Some(signal) = name_owner_changed.next() => async {
                        let args =  signal.args().expect("illegal NameOwnerChanged signal");
                        if args.name == NOTIFIER_WATCHER_SERVICE {
                            if args.new_owner.is_none() && item_added.is_some() {
                                info!("watcher disconnected");
                                for item in context.remove_watcher() {
                                    yield NotifierHostMessage::Removed(item);
                                }
                                item_added = None;
                                item_removed = None;
                            } else if args.old_owner.is_none() && item_added.is_none() {
                                let (items, added, removed) = context.add_watcher(&connection, &name).await?;
                                info!("watcher connected");
                                for item in items {
                                    yield NotifierHostMessage::Added(item);
                                }
                                item_added = Some(added);
                                item_removed = Some(removed);
                            }
                        }
                        Ok::<_, zbus::Error>(())
                    }.await,
                    Some(Some(signal)) =
                        OptionFuture::from(item_added.as_mut().map(|x| x.next())) =>
                    async {
                        if let Some(item) = context.add_item(&signal.args()?.service) {
                            yield NotifierHostMessage::Added(item);
                        }
                        Ok(())
                    }.await,
                    Some(Some(signal)) =
                        OptionFuture::from(item_removed.as_mut().map(|x| x.next())) =>
                    async {
                        if let Some(item) = context.remove_item(&signal.args()?.service) {
                            yield NotifierHostMessage::Removed(item);
                        }
                        Ok(())
                    }.await,
                    _ = OptionFuture::from(cancel.as_mut().map(|x| x.cancelled())) => break,
                } {
                    Ok(()) => (),
                    Err(e) => error!(error=%e),
                }
            }
        })
    }
}
