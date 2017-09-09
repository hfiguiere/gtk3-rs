// This file was generated by gir (6a48033) from gir-files (1069259)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FrameTimings(Shared<ffi::GdkFrameTimings>);

    match fn {
        ref => |ptr| ffi::gdk_frame_timings_ref(ptr),
        unref => |ptr| ffi::gdk_frame_timings_unref(ptr),
        get_type => || ffi::gdk_frame_timings_get_type(),
    }
}

impl FrameTimings {
    #[cfg(feature = "v3_8")]
    pub fn get_complete(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_frame_timings_get_complete(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn get_frame_counter(&self) -> i64 {
        unsafe {
            ffi::gdk_frame_timings_get_frame_counter(self.to_glib_none().0)
        }
    }

    pub fn get_frame_time(&self) -> i64 {
        unsafe {
            ffi::gdk_frame_timings_get_frame_time(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn get_predicted_presentation_time(&self) -> i64 {
        unsafe {
            ffi::gdk_frame_timings_get_predicted_presentation_time(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn get_presentation_time(&self) -> i64 {
        unsafe {
            ffi::gdk_frame_timings_get_presentation_time(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn get_refresh_interval(&self) -> i64 {
        unsafe {
            ffi::gdk_frame_timings_get_refresh_interval(self.to_glib_none().0)
        }
    }
}
