use std::{collections::HashMap, mem::transmute};

use glib::{
    signal::connect_raw,
    translate::{
        from_glib, from_glib_borrow, from_glib_full, from_glib_none, FromGlibPtrBorrow,
        FromGlibPtrNone, IntoGlib, ToGlibPtr,
    },
    Cast, IsA, SignalHandlerId,
};

use crate::Menuitem;

pub trait MenuitemExtManual: 'static {
    #[doc(alias = "dbusmenu_menuitem_property_set_byte_array")]
    fn property_set_byte_array(&self, property: &str, value: &[u8]) -> bool;

    #[doc(alias = "dbusmenu_menuitem_property_get_variant")]
    fn property_get_variant(&self, property: &str) -> Option<glib::Variant>;

    #[doc(alias = "dbusmenu_menuitem_property_set_variant")]
    fn property_set_variant(&self, property: &str, value: &glib::Variant) -> bool;

    #[doc(alias = "dbusmenu_menuitem_properties_copy")]
    fn properties_copy(&self) -> HashMap<String, Option<glib::Variant>>;

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

    #[doc(alias = "dbusmenu_menuitem_handle_event")]
    fn handle_event(&self, name: &str, variant: &glib::Variant, timestamp: u32);
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

    fn property_get_variant(&self, property: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::dbusmenu_menuitem_property_get_variant(
                self.as_ref().to_glib_none().0,
                property.to_glib_none().0,
            ))
        }
    }

    fn property_set_variant(&self, property: &str, value: &glib::Variant) -> bool {
        unsafe {
            from_glib(ffi::dbusmenu_menuitem_property_set_variant(
                self.as_ref().to_glib_none().0,
                property.to_glib_none().0,
                value as *const _ as *mut _,
            ))
        }
    }

    fn properties_copy(&self) -> HashMap<String, Option<glib::Variant>> {
        unsafe extern "C" fn copy_trampoline(
            key: glib::ffi::gpointer,
            value: glib::ffi::gpointer,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let hash_map = &mut *(user_data as *mut HashMap<String, Option<glib::Variant>>);
            let key = from_glib_full(key as *const libc::c_char);
            let value = from_glib_full(value as *mut glib::ffi::GVariant);
            hash_map.insert(key, value);
            glib::ffi::GTRUE
        }
        unsafe {
            let hash_table = ffi::dbusmenu_menuitem_properties_copy(self.as_ref().to_glib_none().0);
            let mut map = HashMap::new();
            glib::ffi::g_hash_table_foreach_steal(
                hash_table,
                Some(copy_trampoline),
                &mut map as *mut _ as *mut _,
            );
            map
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
                ffi::DBUSMENU_MENUITEM_SIGNAL_PROPERTY_CHANGED,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    property_changed_trampoline::<Self, F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    fn handle_event(&self, name: &str, variant: &glib::Variant, timestamp: u32) {
        unsafe {
            ffi::dbusmenu_menuitem_handle_event(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                variant.to_glib_none().0,
                timestamp,
            )
        }
    }
}
