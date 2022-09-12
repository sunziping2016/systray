// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::Menuitem;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "DbusmenuMenuitemProxy")]
    pub struct MenuitemProxy(Object<ffi::DbusmenuMenuitemProxy, ffi::DbusmenuMenuitemProxyClass>) @extends Menuitem;

    match fn {
        type_ => || ffi::dbusmenu_menuitem_proxy_get_type(),
    }
}

impl MenuitemProxy {
        pub const NONE: Option<&'static MenuitemProxy> = None;
    

    #[doc(alias = "dbusmenu_menuitem_proxy_new")]
    pub fn new(mi: &impl IsA<Menuitem>) -> MenuitemProxy {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::dbusmenu_menuitem_proxy_new(mi.as_ref().to_glib_none().0))
        }
    }
}

pub trait MenuitemProxyExt: 'static {
    #[doc(alias = "dbusmenu_menuitem_proxy_get_wrapped")]
    #[doc(alias = "get_wrapped")]
    fn wrapped(&self) -> Option<Menuitem>;

    #[doc(alias = "menu-item")]
    fn menu_item(&self) -> Option<Menuitem>;
}

impl<O: IsA<MenuitemProxy>> MenuitemProxyExt for O {
    fn wrapped(&self) -> Option<Menuitem> {
        unsafe {
            from_glib_none(ffi::dbusmenu_menuitem_proxy_get_wrapped(self.as_ref().to_glib_none().0))
        }
    }

    fn menu_item(&self) -> Option<Menuitem> {
        glib::ObjectExt::property(self.as_ref(), "menu-item")
    }
}

impl fmt::Display for MenuitemProxy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MenuitemProxy")
    }
}