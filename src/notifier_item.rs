//! # DBus interface proxy for: `org.kde.StatusNotifierItem`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `Interface '/StatusNotifierItem' from service 'org.fcitx.Fcitx5.StatusNotifierItem-1479-29' on session bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::PeerProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PropertiesProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.
//!
//! See <https://www.freedesktop.org/wiki/Specifications/StatusNotifierItem/StatusNotifierItem/>,
//! <https://github.com/KDE/knotifications/blob/master/src/org.kde.StatusNotifierItem.xml>,
//! and <https://bazaar.launchpad.net/~indicator-applet-developers/libappindicator/trunk/view/head:/src/notification-item.xml>.

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.kde.StatusNotifierItem")]
trait StatusNotifierItem {
    /// Category property
    #[dbus_proxy(property)]
    fn category(&self) -> zbus::Result<String>;

    /// Id property
    #[dbus_proxy(property)]
    fn id(&self) -> zbus::Result<String>;

    /// Title property
    #[dbus_proxy(property)]
    fn title(&self) -> zbus::Result<String>;

    /// Status property
    #[dbus_proxy(property)]
    fn status(&self) -> zbus::Result<String>;

    /// WindowId property
    ///
    /// Note: In KDE specification, it's i32, while in freedesktop, its' u32.
    #[dbus_proxy(property)]
    fn window_id(&self) -> zbus::Result<u32>;

    /// IconThemePath property
    ///
    /// Note: KDE&Ayatana-specific.
    #[dbus_proxy(property)]
    fn icon_theme_path(&self) -> zbus::Result<String>;

    /// Menu property
    #[dbus_proxy(property)]
    fn menu(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// XAyatanaLabel property
    ///
    /// Note: Ayatana-specific.
    #[dbus_proxy(property)]
    fn x_ayatana_label(&self) -> zbus::Result<String>;

    /// XAyatanaLabelGuide property
    ///
    /// Note: Ayatana-specific.
    #[dbus_proxy(property)]
    fn x_ayatana_label_guide(&self) -> zbus::Result<String>;

    /// XAyatanaOrderingIndex property
    ///
    /// Note: Ayatana-specific.
    #[dbus_proxy(property)]
    fn x_ayatana_ordering_index(&self) -> zbus::Result<u32>;

    /// ItemIsMenu property
    #[dbus_proxy(property)]
    fn item_is_menu(&self) -> zbus::Result<bool>;

    /// IconName property
    #[dbus_proxy(property)]
    fn icon_name(&self) -> zbus::Result<String>;

    /// IconAccessibleDesc property
    ///
    /// Note: Ayatana-specific.
    #[dbus_proxy(property)]
    fn icon_accessible_desc(&self) -> zbus::Result<String>;

    /// IconPixmap property
    #[dbus_proxy(property)]
    fn icon_pixmap(&self) -> zbus::Result<Vec<(i32, i32, Vec<u8>)>>;

    /// OverlayIconName property
    #[dbus_proxy(property)]
    fn overlay_icon_name(&self) -> zbus::Result<String>;

    /// OverlayIconPixmap property
    #[dbus_proxy(property)]
    fn overlay_icon_pixmap(&self) -> zbus::Result<Vec<(i32, i32, Vec<u8>)>>;

    /// AttentionIconName property
    #[dbus_proxy(property)]
    fn attention_icon_name(&self) -> zbus::Result<String>;

    /// AttentionAccessibleDesc property
    ///
    /// Note: Ayatana-specific.
    #[dbus_proxy(property)]
    fn attention_accessible_desc(&self) -> zbus::Result<String>;

    /// AttentionIconPixmap property
    #[dbus_proxy(property)]
    fn attention_icon_pixmap(&self) -> zbus::Result<Vec<(i32, i32, Vec<u8>)>>;

    /// AttentionMovieName property
    ///
    /// Note: KDE-specific.
    #[dbus_proxy(property)]
    fn attention_movie_name(&self) -> zbus::Result<String>;

    /// ToolTip property
    #[dbus_proxy(property)]
    fn tool_tip(&self) -> zbus::Result<(String, Vec<(i32, i32, Vec<u8>)>, String, String)>;

    /// ProvideXdgActivationToken method
    fn provide_xdg_activation_token(&self, token: &str) -> zbus::Result<()>;

    /// ContextMenu property
    fn context_menu(&self, x: i32, y: i32) -> zbus::Result<()>;

    /// Activate method
    fn activate(&self, x: i32, y: i32) -> zbus::Result<()>;

    /// SecondaryActivate method
    fn secondary_activate(&self, x: i32, y: i32) -> zbus::Result<()>;

    /// XAyatanaSecondaryActivate method
    ///
    /// Note: Ayatana-specific.
    fn x_ayatana_secondary_activate(&self, timestamp: u32) -> zbus::Result<()>;

    /// Scroll method
    fn scroll(&self, delta: i32, orientation: &str) -> zbus::Result<()>;

    /// NewTitle signal
    #[dbus_proxy(signal)]
    fn new_title(&self) -> zbus::Result<()>;

    /// NewIcon signal
    #[dbus_proxy(signal)]
    fn new_icon(&self) -> zbus::Result<()>;

    /// NewAttentionIcon signal
    #[dbus_proxy(signal)]
    fn new_attention_icon(&self) -> zbus::Result<()>;

    /// NewOverlayIcon signal
    #[dbus_proxy(signal)]
    fn new_overlay_icon(&self) -> zbus::Result<()>;

    /// NewMenu signal
    ///
    /// Note: KDE-specific.
    #[dbus_proxy(signal)]
    fn new_menu(&self) -> zbus::Result<()>;

    /// NewToolTip signal
    #[dbus_proxy(signal)]
    fn new_tool_tip(&self) -> zbus::Result<()>;

    /// NewStatus signal
    #[dbus_proxy(signal)]
    fn new_status(&self, status: &str) -> zbus::Result<()>;

    /// NewIconThemePath signal
    ///
    /// Note: KDE&Ayatana-specific.
    #[dbus_proxy(signal)]
    fn new_icon_theme_path(&self, icon_theme_path: &str) -> zbus::Result<()>;

    /// XAyatanaNewLabel signal
    #[dbus_proxy(signal)]
    fn x_ayatana_new_label(&self, label: &str, guide: &str) -> zbus::Result<()>;
}