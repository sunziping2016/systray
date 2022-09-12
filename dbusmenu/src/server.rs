use std::mem::transmute;

use glib::{
    signal::connect_raw,
    translate::{
        from_glib, from_glib_borrow, FromGlibPtrBorrow, FromGlibPtrNone, IntoGlib, ToGlibPtr,
    },
    Cast, IsA, SignalHandlerId,
};

use crate::{
    prelude::{Status, TextDirection},
    Server,
};

pub trait ServerExtManual: 'static {
    #[doc(alias = "dbusmenu_server_get_status")]
    #[doc(alias = "get_status")]
    fn status(&self) -> Status;

    #[doc(alias = "dbusmenu_server_get_text_direction")]
    #[doc(alias = "get_text_direction")]
    fn text_direction(&self) -> TextDirection;

    #[doc(alias = "dbusmenu_server_set_status")]
    fn set_status(&self, status: Status);

    #[doc(alias = "dbusmenu_server_set_text_direction")]
    fn set_text_direction(&self, dir: TextDirection);

    #[doc(alias = "item-property-updated")]
    fn connect_item_property_updated<F: Fn(&Self, i32, &str, &glib::Variant) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Server>> ServerExtManual for O {
    fn status(&self) -> Status {
        unsafe {
            from_glib(ffi::dbusmenu_server_get_status(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn text_direction(&self) -> TextDirection {
        unsafe {
            from_glib(ffi::dbusmenu_server_get_text_direction(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_status(&self, status: Status) {
        unsafe {
            ffi::dbusmenu_server_set_status(self.as_ref().to_glib_none().0, status.into_glib())
        }
    }

    fn set_text_direction(&self, dir: TextDirection) {
        unsafe {
            ffi::dbusmenu_server_set_text_direction(self.as_ref().to_glib_none().0, dir.into_glib())
        }
    }

    fn connect_item_property_updated<F: Fn(&Self, i32, &str, &glib::Variant) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn item_property_updated_trampoline<
            P: IsA<Server>,
            F: Fn(&P, i32, &str, &glib::Variant) + 'static,
        >(
            this: *mut ffi::DbusmenuServer,
            arg1: libc::c_int,
            arg2: *mut libc::c_char,
            arg3: *mut glib::ffi::GVariant,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Server::from_glib_borrow(this).unsafe_cast_ref(),
                arg1,
                &String::from_glib_none(arg2),
                &from_glib_borrow(arg3),
            )
        }
        let f = Box::new(f);
        unsafe {
            connect_raw(
                self.as_ptr() as *mut _,
                ffi::DBUSMENU_SERVER_SIGNAL_ID_PROP_UPDATE,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    item_property_updated_trampoline::<Self, F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }
}
