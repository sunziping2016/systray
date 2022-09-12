use std::mem::transmute;

use glib::{
    signal::connect_raw,
    translate::{
        from_glib, from_glib_borrow, FromGlib, FromGlibPtrArrayContainerAsVec, FromGlibPtrBorrow,
        FromGlibPtrNone, ToGlibPtr,
    },
    Cast, IsA, SignalHandlerId,
};

use crate::{Client, Menuitem};

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TextDirection {
    None,
    LTR,
    RTL,
}

impl FromGlib<ffi::DbusmenuTextDirection> for TextDirection {
    unsafe fn from_glib(val: ffi::DbusmenuTextDirection) -> Self {
        match val {
            ffi::DBUSMENU_TEXT_DIRECTION_NONE => Self::None,
            ffi::DBUSMENU_TEXT_DIRECTION_LTR => Self::LTR,
            ffi::DBUSMENU_TEXT_DIRECTION_RTL => Self::RTL,
            _ => panic!("illegal DbusmenuStatus"),
        }
    }
}

pub trait ClientExtManual: 'static {
    #[doc(alias = "dbusmenu_client_get_status")]
    #[doc(alias = "get_status")]
    fn status(&self) -> Status;

    #[doc(alias = "dbusmenu_client_get_text_direction")]
    #[doc(alias = "get_text_direction")]
    fn text_direction(&self) -> TextDirection;

    #[doc(alias = "event-result")]
    fn connect_event_result<
        F: Fn(&Self, &Menuitem, &str, &glib::Variant, u32, &glib::error::Error) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "icon-theme-dirs-changed")]
    fn connect_icon_theme_dirs_changed<F: Fn(&Self, Vec<String>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Client>> ClientExtManual for O {
    fn status(&self) -> Status {
        unsafe {
            from_glib(ffi::dbusmenu_client_get_status(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn text_direction(&self) -> TextDirection {
        unsafe {
            from_glib(ffi::dbusmenu_client_get_text_direction(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_event_result<
        F: Fn(&Self, &Menuitem, &str, &glib::Variant, u32, &glib::error::Error) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn event_result_trampoline<
            P: IsA<Client>,
            F: Fn(&P, &Menuitem, &str, &glib::Variant, u32, &glib::error::Error) + 'static,
        >(
            this: *mut ffi::DbusmenuClient,
            arg1: *mut ffi::DbusmenuMenuitem,
            arg2: *mut libc::c_char,
            arg3: *mut glib::ffi::GVariant,
            arg4: libc::c_uint,
            arg5: *mut glib::ffi::GError,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Client::from_glib_borrow(this).unsafe_cast_ref(),
                &Menuitem::from_glib_borrow(arg1),
                &String::from_glib_none(arg2),
                &from_glib_borrow(arg3),
                arg4,
                &from_glib_borrow(arg5),
            );
        }
        let f = Box::new(f);
        unsafe {
            connect_raw(
                self.as_ptr() as *mut _,
                ffi::DBUSMENU_CLIENT_SIGNAL_EVENT_RESULT,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    event_result_trampoline::<Self, F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    fn connect_icon_theme_dirs_changed<F: Fn(&Self, Vec<String>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn icon_theme_dirs_changed_trampoline<
            P: IsA<Client>,
            F: Fn(&P, Vec<String>) + 'static,
        >(
            this: *mut ffi::DbusmenuClient,
            arg1: glib::ffi::GStrv,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Client::from_glib_borrow(this).unsafe_cast_ref(),
                String::from_glib_none_as_vec(arg1),
            );
        }
        let f = Box::new(f);
        unsafe {
            connect_raw(
                self.as_ptr() as *mut _,
                ffi::DBUSMENU_CLIENT_SIGNAL_ICON_THEME_DIRS_CHANGED,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    icon_theme_dirs_changed_trampoline::<Self, F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }
}
