// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Display;
use crate::Screen;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    pub struct AppLaunchContext(Object<ffi::GdkAppLaunchContext>) @extends gio::AppLaunchContext;

    match fn {
        type_ => || ffi::gdk_app_launch_context_get_type(),
    }
}

impl AppLaunchContext {
    #[doc(alias = "gdk_app_launch_context_set_desktop")]
    pub fn set_desktop(&self, desktop: i32) {
        unsafe {
            ffi::gdk_app_launch_context_set_desktop(self.to_glib_none().0, desktop);
        }
    }

    #[doc(alias = "gdk_app_launch_context_set_icon")]
    pub fn set_icon<P: IsA<gio::Icon>>(&self, icon: Option<&P>) {
        unsafe {
            ffi::gdk_app_launch_context_set_icon(
                self.to_glib_none().0,
                icon.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_app_launch_context_set_icon_name")]
    pub fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::gdk_app_launch_context_set_icon_name(
                self.to_glib_none().0,
                icon_name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_app_launch_context_set_screen")]
    pub fn set_screen(&self, screen: &Screen) {
        unsafe {
            ffi::gdk_app_launch_context_set_screen(self.to_glib_none().0, screen.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_app_launch_context_set_timestamp")]
    pub fn set_timestamp(&self, timestamp: u32) {
        unsafe {
            ffi::gdk_app_launch_context_set_timestamp(self.to_glib_none().0, timestamp);
        }
    }

    pub fn display(&self) -> Option<Display> {
        unsafe {
            let mut value = glib::Value::from_type(<Display as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"display\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `display` getter")
        }
    }
}

impl fmt::Display for AppLaunchContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AppLaunchContext")
    }
}
