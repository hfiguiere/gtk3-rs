// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Widget;
use glib::object::IsA;
use glib::translate::*;

pub trait DragContextExtManual: 'static {
    #[doc(alias = "gtk_drag_finish")]
    fn drag_finish(&self, success: bool, del: bool, time_: u32);

    #[doc(alias = "gtk_drag_cancel")]
    fn drag_cancel(&self);

    #[doc(alias = "gtk_drag_get_source_widget")]
    fn drag_get_source_widget(&self) -> Option<Widget>;

    #[doc(alias = "gtk_drag_set_icon_default")]
    fn drag_set_icon_default(&self);

    #[doc(alias = "gtk_drag_set_icon_gicon")]
    fn drag_set_icon_gicon<P: IsA<gio::Icon>>(&self, icon: &P, hot_x: i32, hot_y: i32);

    #[doc(alias = "gtk_drag_set_icon_name")]
    fn drag_set_icon_name(&self, icon_name: &str, hot_x: i32, hot_y: i32);

    #[doc(alias = "gtk_drag_set_icon_pixubf")]
    fn drag_set_icon_pixbuf(&self, pixbuf: &gdk_pixbuf::Pixbuf, hot_x: i32, hot_y: i32);

    #[doc(alias = "gtk_drag_set_icon_stock")]
    fn drag_set_icon_stock(&self, stock_id: &str, hot_x: i32, hot_y: i32);

    #[doc(alias = "gtk_drag_set_icon_surface")]
    fn drag_set_icon_surface(&self, surface: &cairo::Surface);

    #[doc(alias = "gtk_drag_set_icon_widget")]
    fn drag_set_icon_widget<P: IsA<Widget>>(&self, widget: &P, hot_x: i32, hot_y: i32);
}

impl<O: IsA<gdk::DragContext>> DragContextExtManual for O {
    fn drag_finish(&self, success: bool, del: bool, time_: u32) {
        unsafe {
            ffi::gtk_drag_finish(
                self.as_ref().to_glib_none().0,
                success as i32,
                del as i32,
                time_,
            )
        };
    }

    fn drag_cancel(&self) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_drag_cancel(self.as_ref().to_glib_none().0);
        }
    }

    fn drag_get_source_widget(&self) -> Option<Widget> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_drag_get_source_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn drag_set_icon_default(&self) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_drag_set_icon_default(self.as_ref().to_glib_none().0);
        }
    }

    fn drag_set_icon_gicon<P: IsA<gio::Icon>>(&self, icon: &P, hot_x: i32, hot_y: i32) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_drag_set_icon_gicon(
                self.as_ref().to_glib_none().0,
                icon.as_ref().to_glib_none().0,
                hot_x,
                hot_y,
            );
        }
    }

    fn drag_set_icon_name(&self, icon_name: &str, hot_x: i32, hot_y: i32) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_drag_set_icon_name(
                self.as_ref().to_glib_none().0,
                icon_name.to_glib_none().0,
                hot_x,
                hot_y,
            );
        }
    }

    fn drag_set_icon_pixbuf(&self, pixbuf: &gdk_pixbuf::Pixbuf, hot_x: i32, hot_y: i32) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_drag_set_icon_pixbuf(
                self.as_ref().to_glib_none().0,
                pixbuf.to_glib_none().0,
                hot_x,
                hot_y,
            );
        }
    }

    fn drag_set_icon_stock(&self, stock_id: &str, hot_x: i32, hot_y: i32) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_drag_set_icon_stock(
                self.as_ref().to_glib_none().0,
                stock_id.to_glib_none().0,
                hot_x,
                hot_y,
            );
        }
    }

    fn drag_set_icon_surface(&self, surface: &cairo::Surface) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_drag_set_icon_surface(
                self.as_ref().to_glib_none().0,
                mut_override(surface.to_glib_none().0),
            );
        }
    }

    fn drag_set_icon_widget<P: IsA<Widget>>(&self, widget: &P, hot_x: i32, hot_y: i32) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_drag_set_icon_widget(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                hot_x,
                hot_y,
            );
        }
    }
}
