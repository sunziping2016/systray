use std::mem::transmute;

use glib::{
    signal::connect_raw,
    translate::{
        from_glib, from_glib_borrow, FromGlibPtrBorrow, FromGlibPtrNone, IntoGlib, ToGlibPtr,
    },
    Cast, IsA, SignalHandlerId,
};

use crate::Menuitem;

pub trait MenuitemExtManual: 'static {
    #[doc(alias = "dbusmenu_menuitem_property_set_byte_array")]
    fn property_set_byte_array(&self, property: &str, value: &[u8]) -> bool;
    #[doc(alias = "child-removed")]
    fn connect_child_added<F: Fn(&Self, &glib::Object, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
    #[doc(alias = "child-moved")]
    fn connect_child_moved<F: Fn(&Self, &glib::Object, u32, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
    #[doc(alias = "child-removed")]
    fn connect_child_removed<F: Fn(&Self, &glib::Object) + 'static>(&self, f: F)
        -> SignalHandlerId;

    #[doc(alias = "event")]
    fn connect_event<F: Fn(&Self, &str, &glib::Variant, u32) -> bool + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "property-changed")]
    fn connect_property_changed<F: Fn(&Self, &str, &glib::Variant) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Menuitem>> MenuitemExtManual for O {
    fn property_set_byte_array(&self, property: &str, value: &[u8]) -> bool {
        unsafe {
            from_glib(ffi::dbusmenu_menuitem_property_set_byte_array(
                self.as_ref().to_glib_none().0,
                property.to_glib_none().0,
                value.as_ptr(),
                value.len(),
            ))
        }
    }

    fn connect_child_added<F: Fn(&Self, &glib::Object, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn child_added_trampoline<
            P: IsA<Menuitem>,
            F: Fn(&P, &glib::Object, u32) + 'static,
        >(
            this: *mut ffi::DbusmenuMenuitem,
            arg1: *mut glib::object::GObject,
            arg2: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Menuitem::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(arg1),
                arg2,
            )
        }
        let f = Box::new(f);
        unsafe {
            connect_raw(
                self.as_ptr() as *mut _,
                b"child-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    child_added_trampoline::<Self, F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    fn connect_child_moved<F: Fn(&Self, &glib::Object, u32, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn child_moved_trampoline<
            P: IsA<Menuitem>,
            F: Fn(&P, &glib::Object, u32, u32) + 'static,
        >(
            this: *mut ffi::DbusmenuMenuitem,
            arg1: *mut glib::object::GObject,
            arg2: libc::c_uint,
            arg3: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Menuitem::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(arg1),
                arg2,
                arg3,
            )
        }
        let f = Box::new(f);
        unsafe {
            connect_raw(
                self.as_ptr() as *mut _,
                b"child-moved\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    child_moved_trampoline::<Self, F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    fn connect_child_removed<F: Fn(&Self, &glib::Object) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn child_added_trampoline<
            P: IsA<Menuitem>,
            F: Fn(&P, &glib::Object) + 'static,
        >(
            this: *mut ffi::DbusmenuMenuitem,
            arg1: *mut glib::object::GObject,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Menuitem::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(arg1),
            )
        }
        let f = Box::new(f);
        unsafe {
            connect_raw(
                self.as_ptr() as *mut _,
                b"child-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    child_added_trampoline::<Self, F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    fn connect_event<F: Fn(&Self, &str, &glib::Variant, u32) -> bool + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn event_trampoline<
            P: IsA<Menuitem>,
            F: Fn(&P, &str, &glib::Variant, u32) -> bool + 'static,
        >(
            this: *mut ffi::DbusmenuMenuitem,
            arg1: *mut libc::c_char,
            arg2: *mut glib::ffi::GVariant,
            arg3: libc::c_uint,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                Menuitem::from_glib_borrow(this).unsafe_cast_ref(),
                &String::from_glib_none(arg1),
                &from_glib_borrow(arg2),
                arg3,
            )
            .into_glib()
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            let detailed_signal_name = detail.map(|name| format!("event::{}\0", name));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"event\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    event_trampoline::<Self, F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    fn connect_property_changed<F: Fn(&Self, &str, &glib::Variant) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn property_changed_trampoline<
            P: IsA<Menuitem>,
            F: Fn(&P, &str, &glib::Variant) + 'static,
        >(
            this: *mut ffi::DbusmenuMenuitem,
            arg1: *mut libc::c_char,
            arg2: *mut glib::ffi::GVariant,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Menuitem::from_glib_borrow(this).unsafe_cast_ref(),
                &String::from_glib_none(arg1),
                &from_glib_borrow(arg2),
            )
        }
        let f = Box::new(f);
        unsafe {
            connect_raw(
                self.as_ptr() as *mut _,
                b"property-changed".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    property_changed_trampoline::<Self, F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }
}
