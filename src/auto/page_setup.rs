// This file was generated by gir (25bba39) from gir-files (71d73f0)
// DO NOT EDIT

use Error;
use PageOrientation;
use PaperSize;
use Unit;
use ffi;
#[cfg(feature = "v3_22")]
use glib;
use glib::translate::*;
use std;
use std::ptr;

glib_wrapper! {
    pub struct PageSetup(Object<ffi::GtkPageSetup>);

    match fn {
        get_type => || ffi::gtk_page_setup_get_type(),
    }
}

impl PageSetup {
    pub fn new() -> PageSetup {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_page_setup_new())
        }
    }

    pub fn new_from_file<T: AsRef<std::path::Path>>(file_name: T) -> Result<PageSetup, Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_page_setup_new_from_file(file_name.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(feature = "v3_22")]
    pub fn new_from_gvariant(variant: &glib::Variant) -> PageSetup {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_page_setup_new_from_gvariant(variant.to_glib_none().0))
        }
    }

    //pub fn new_from_key_file(key_file: /*Ignored*/&glib::KeyFile, group_name: Option<&str>) -> Result<PageSetup, Error> {
    //    unsafe { TODO: call ffi::gtk_page_setup_new_from_key_file() }
    //}

    pub fn copy(&self) -> Option<PageSetup> {
        unsafe {
            from_glib_full(ffi::gtk_page_setup_copy(self.to_glib_none().0))
        }
    }

    pub fn get_bottom_margin(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_page_setup_get_bottom_margin(self.to_glib_none().0, unit.to_glib())
        }
    }

    pub fn get_left_margin(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_page_setup_get_left_margin(self.to_glib_none().0, unit.to_glib())
        }
    }

    pub fn get_orientation(&self) -> PageOrientation {
        unsafe {
            from_glib(ffi::gtk_page_setup_get_orientation(self.to_glib_none().0))
        }
    }

    pub fn get_page_height(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_page_setup_get_page_height(self.to_glib_none().0, unit.to_glib())
        }
    }

    pub fn get_page_width(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_page_setup_get_page_width(self.to_glib_none().0, unit.to_glib())
        }
    }

    pub fn get_paper_height(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_page_setup_get_paper_height(self.to_glib_none().0, unit.to_glib())
        }
    }

    pub fn get_paper_size(&self) -> PaperSize {
        unsafe {
            from_glib_none(ffi::gtk_page_setup_get_paper_size(self.to_glib_none().0))
        }
    }

    pub fn get_paper_width(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_page_setup_get_paper_width(self.to_glib_none().0, unit.to_glib())
        }
    }

    pub fn get_right_margin(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_page_setup_get_right_margin(self.to_glib_none().0, unit.to_glib())
        }
    }

    pub fn get_top_margin(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_page_setup_get_top_margin(self.to_glib_none().0, unit.to_glib())
        }
    }

    pub fn load_file<T: AsRef<std::path::Path>>(&self, file_name: T) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_page_setup_load_file(self.to_glib_none().0, file_name.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //pub fn load_key_file(&self, key_file: /*Ignored*/&glib::KeyFile, group_name: Option<&str>) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::gtk_page_setup_load_key_file() }
    //}

    pub fn set_bottom_margin(&self, margin: f64, unit: Unit) {
        unsafe {
            ffi::gtk_page_setup_set_bottom_margin(self.to_glib_none().0, margin, unit.to_glib());
        }
    }

    pub fn set_left_margin(&self, margin: f64, unit: Unit) {
        unsafe {
            ffi::gtk_page_setup_set_left_margin(self.to_glib_none().0, margin, unit.to_glib());
        }
    }

    pub fn set_orientation(&self, orientation: PageOrientation) {
        unsafe {
            ffi::gtk_page_setup_set_orientation(self.to_glib_none().0, orientation.to_glib());
        }
    }

    pub fn set_paper_size(&self, size: &PaperSize) {
        unsafe {
            ffi::gtk_page_setup_set_paper_size(self.to_glib_none().0, mut_override(size.to_glib_none().0));
        }
    }

    pub fn set_paper_size_and_default_margins(&self, size: &PaperSize) {
        unsafe {
            ffi::gtk_page_setup_set_paper_size_and_default_margins(self.to_glib_none().0, mut_override(size.to_glib_none().0));
        }
    }

    pub fn set_right_margin(&self, margin: f64, unit: Unit) {
        unsafe {
            ffi::gtk_page_setup_set_right_margin(self.to_glib_none().0, margin, unit.to_glib());
        }
    }

    pub fn set_top_margin(&self, margin: f64, unit: Unit) {
        unsafe {
            ffi::gtk_page_setup_set_top_margin(self.to_glib_none().0, margin, unit.to_glib());
        }
    }

    pub fn to_file<T: AsRef<std::path::Path>>(&self, file_name: T) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_page_setup_to_file(self.to_glib_none().0, file_name.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(feature = "v3_22")]
    pub fn to_gvariant(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::gtk_page_setup_to_gvariant(self.to_glib_none().0))
        }
    }

    //pub fn to_key_file(&self, key_file: /*Ignored*/&glib::KeyFile, group_name: &str) {
    //    unsafe { TODO: call ffi::gtk_page_setup_to_key_file() }
    //}
}
