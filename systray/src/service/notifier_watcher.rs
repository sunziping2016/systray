use futures::{future::OptionFuture, Future, StreamExt};
use std::collections::{HashMap, HashSet};
use tokio_util::sync::CancellationToken;
use tracing::info;
use zbus::{
    dbus_interface,
    fdo::{DBusProxy, Error},
    names::{BusName, WellKnownName},
    zvariant::ObjectPath,
    ConnectionBuilder, MessageHeader, Result, SignalContext,
};

pub const NOTIFIER_WATCHER_SERVICE: WellKnownName =
    WellKnownName::from_static_str_unchecked("org.kde.StatusNotifierWatcher");
pub const NOTIFIER_WATCHER_PATH: ObjectPath =
    ObjectPath::from_static_str_unchecked("/StatusNotifierWatcher");

#[derive(Debug, Default)]
pub struct NotifierWatcher {
    items: HashMap<BusName<'static>, Vec<ObjectPath<'static>>>,
    hosts: HashSet<BusName<'static>>,
    cancel: Option<CancellationToken>,
}

impl NotifierWatcher {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn cancellable(self, cancel: CancellationToken) -> Self {
        Self {
            cancel: Some(cancel),
            ..self
        }
    }

    pub async fn run(mut self) -> Result<impl Future<Output = ()>> {
        let mut cancel = self.cancel.take();
        let connection = ConnectionBuilder::session()?
            .internal_executor(false)
            .build()
            .await?;
        let dbus_proxy = DBusProxy::new(&connection).await?;
        let mut name_owner_changed = dbus_proxy.receive_name_owner_changed().await?;
        let object_server = connection.object_server();
        object_server.at(&NOTIFIER_WATCHER_PATH, self).await?;
        connection.request_name(&NOTIFIER_WATCHER_SERVICE).await?;
        let watch_ref = object_server
            .interface::<_, NotifierWatcher>(&NOTIFIER_WATCHER_PATH)
            .await?;
        Ok(async move {
            loop {
                tokio::select! {
                    Some(signal) = name_owner_changed.next() => {
                        let args = signal.args().expect("illegal NameOwnerChanged signal");
                        if args.new_owner().is_none() {
                            watch_ref
                                .get_mut()
                                .await
                                .handle_service_unregistered(
                                    &args.name().to_owned(),
                                    watch_ref.signal_context(),
                                )
                                .await;
                        }
                    },
                    _ = OptionFuture::from(cancel.as_mut().map(|x| x.cancelled())) => break,
                }
            }
        })
    }

    pub async fn handle_service_unregistered(
        &mut self,
        service: &BusName<'static>,
        ctxt: &SignalContext<'_>,
    ) {
        if let Some(paths) = self.items.remove(service) {
            for path in paths.into_iter() {
                info!(service = %service, path = %path, "item unregistered");
                Self::status_notifier_item_unregistered(&ctxt, &format!("{}{}", service, path))
                    .await
                    .unwrap();
            }
            // `paths` must be non-empty.
            self.registered_status_notifier_items_changed(&ctxt)
                .await
                .unwrap();
        }
        if self.hosts.remove(service) {
            info!(service = %service, "host unregistered");
            Self::status_notifier_host_unregistered(&ctxt)
                .await
                .unwrap();
            if self.hosts.is_empty() {
                self.is_status_notifier_host_registered_changed(&ctxt)
                    .await
                    .unwrap();
            }
        }
    }
}

#[dbus_interface(name = "org.kde.StatusNotifierWatcher")]
impl NotifierWatcher {
    async fn register_status_notifier_item(
        &mut self,
        service: &str,
        #[zbus(header)] hdr: MessageHeader<'_>,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
    ) -> zbus::fdo::Result<()> {
        let (service, path) = if service.starts_with('/') {
            (
                BusName::from(
                    hdr.sender()?
                        .ok_or_else(|| Error::Failed("missing service".into()))?
                        .to_owned(),
                ),
                ObjectPath::try_from(service)
                    .map_err(|_| Error::Failed("illegal path".into()))?
                    .to_owned(),
            )
        } else {
            (
                BusName::try_from(service)
                    .map_err(|_| Error::Failed("illegal service".into()))?
                    .to_owned(),
                ObjectPath::from_static_str("/StatusNotifierItem").unwrap(),
            )
        };
        if let Some(true) = self.items.get(&service).map(|x| x.contains(&path)) {
            return Ok(());
        }
        self.items
            .entry(service.clone())
            .or_default()
            .push(path.clone());
        info!(service = %service, path = %path, "item registered");
        self.registered_status_notifier_items_changed(&ctxt)
            .await
            .unwrap();
        Self::status_notifier_item_registered(&ctxt, &format!("{}{}", service, path))
            .await
            .unwrap();
        Ok(())
    }

    async fn register_status_notifier_host(
        &mut self,
        service: &str,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
    ) -> zbus::fdo::Result<()> {
        let service = BusName::try_from(service)
            .map_err(|_| Error::Failed("illegal service".into()))?
            .to_owned();
        info!(service = %service, "host registered");
        Self::status_notifier_host_registered(&ctxt).await.unwrap();
        if self.hosts.len() == 1 {
            self.is_status_notifier_host_registered_changed(&ctxt)
                .await
                .unwrap();
        }
        Ok(())
    }

    #[dbus_interface(property)]
    fn registered_status_notifier_items(&self) -> Vec<String> {
        self.items
            .iter()
            .flat_map(|(service, paths)| {
                paths.iter().map(move |path| format!("{}{}", service, path))
            })
            .collect()
    }

    #[dbus_interface(property)]
    async fn is_status_notifier_host_registered(&self) -> bool {
        !self.hosts.is_empty()
    }

    #[dbus_interface(property)]
    async fn protocol_version(&self) -> i32 {
        0
    }

    #[dbus_interface(signal)]
    async fn status_notifier_host_registered(signal_ctxt: &SignalContext<'_>) -> Result<()>;

    #[dbus_interface(signal)]
    async fn status_notifier_host_unregistered(signal_ctxt: &SignalContext<'_>) -> Result<()>;

    #[dbus_interface(signal)]
    async fn status_notifier_item_registered(
        signal_ctxt: &SignalContext<'_>,
        service: &str,
    ) -> Result<()>;

    #[dbus_interface(signal)]
    async fn status_notifier_item_unregistered(
        signal_ctxt: &SignalContext<'_>,
        service: &str,
    ) -> Result<()>;
}
