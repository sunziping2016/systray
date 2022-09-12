use glib::translate::{FromGlib, IntoGlib};

pub use crate::auto::traits::*;
pub use crate::client::ClientExtManual;
pub use crate::menuitem::MenuitemExtManual;
pub use crate::server::ServerExtManual;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Normal,
    Notice,
}

impl FromGlib<ffi::DbusmenuStatus> for Status {
    unsafe fn from_glib(val: ffi::DbusmenuStatus) -> Self {
        match val {
            ffi::DBUSMENU_STATUS_NORMAL => Self::Normal,
            ffi::DBUSMENU_STATUS_NOTICE => Self::Notice,
            _ => panic!("illegal DbusmenuStatus"),
        }
    }
}

impl IntoGlib for Status {
    type GlibType = ffi::DbusmenuStatus;

    fn into_glib(self) -> Self::GlibType {
        match self {
            Self::Normal => ffi::DBUSMENU_STATUS_NORMAL,
            Self::Notice => ffi::DBUSMENU_STATUS_NOTICE,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TextDirection {
    None,
    Ltr,
    Rtl,
}

impl FromGlib<ffi::DbusmenuTextDirection> for TextDirection {
    unsafe fn from_glib(val: ffi::DbusmenuTextDirection) -> Self {
        match val {
            ffi::DBUSMENU_TEXT_DIRECTION_NONE => Self::None,
            ffi::DBUSMENU_TEXT_DIRECTION_LTR => Self::Ltr,
            ffi::DBUSMENU_TEXT_DIRECTION_RTL => Self::Rtl,
            _ => panic!("illegal DbusmenuStatus"),
        }
    }
}

impl IntoGlib for TextDirection {
    type GlibType = ffi::DbusmenuTextDirection;

    fn into_glib(self) -> Self::GlibType {
        match self {
            Self::None => ffi::DBUSMENU_TEXT_DIRECTION_NONE,
            Self::Ltr => ffi::DBUSMENU_TEXT_DIRECTION_LTR,
            Self::Rtl => ffi::DBUSMENU_TEXT_DIRECTION_RTL,
        }
    }
}
