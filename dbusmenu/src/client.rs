// use std::mem::transmute;

// use glib::{
//     signal::connect_raw,
//     translate::{from_glib_borrow, FromGlibPtrBorrow, FromGlibPtrNone},
//     Cast, IsA, SignalHandlerId,
// };

// use crate::Client;

// pub trait ClientExtManual: 'static {
//     #[doc(alias = "event-result")]
//     fn connect_event_result<
//         F: Fn(&Self, &glib::Object, &str, &glib::Variant, u32, glib::ffi::gpointer) + 'static,
//     >(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;
// }

// impl<O: IsA<Client>> ClientExtManual for O {
//     fn connect_event_result<
//         F: Fn(&Self, &glib::Object, &str, &glib::Variant, u32, glib::ffi::gpointer) + 'static,
//     >(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn event_result_trampoline<
//             P: IsA<Client>,
//             F: Fn(&P, &glib::Object, &str, &glib::Variant, u32, glib::ffi::gpointer) + 'static,
//         >(
//             this: *mut ffi::DbusmenuClient,
//             object: *mut glib::object::GObject,
//             p0: *mut libc::c_char,
//             p1: *mut glib::ffi::GVariant,
//             p2: libc::c_uint,
//             p3: glib::ffi::gpointer,
//             f: glib::ffi::gpointer,
//         ) {
//             let f: &F = &*(f as *const F);
//             f(
//                 Client::from_glib_borrow(this).unsafe_cast_ref(),
//                 &from_glib_borrow(object),
//                 &String::from_glib_none(p0),
//                 &from_glib_borrow(p1),
//                 p2,
//                 p3,
//             )
//         }
//         let f = Box::new(f);
//         unsafe {
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"event-result\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     event_result_trampoline::<Self, F> as *const (),
//                 )),
//                 Box::into_raw(f),
//             )
//         }
//     }
// }
