use std::mem::transmute;

use glib::{
    signal::connect_raw,
    translate::{
        from_glib, from_glib_borrow, FromGlibPtrArrayContainerAsVec, FromGlibPtrBorrow,
        FromGlibPtrNone, ToGlibPtr,
    },
    Cast, IsA, SignalHandlerId,
};

use crate::{
    prelude::{Status, TextDirection},
    Client, Menuitem,
};

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
