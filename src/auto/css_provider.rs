// This file was generated by gir (e48471c) from gir-files (71d73f0)
// DO NOT EDIT

use CssSection;
use Error;
use StyleProvider;
use ffi;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct CssProvider(Object<ffi::GtkCssProvider>): StyleProvider;

    match fn {
        get_type => || ffi::gtk_css_provider_get_type(),
    }
}

impl CssProvider {
    pub fn new() -> CssProvider {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_css_provider_new())
        }
    }

    //pub fn load_from_data(&self, data: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 3 }, length: /*Unimplemented*/Fundamental: SSize) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::gtk_css_provider_load_from_data() }
    //}

    //pub fn load_from_file<T: IsA</*Ignored*/gio::File>>(&self, file: &T) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::gtk_css_provider_load_from_file() }
    //}

    pub fn load_from_path(&self, path: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_css_provider_load_from_path(self.to_glib_none().0, path.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn load_from_resource(&self, resource_path: &str) {
        unsafe {
            ffi::gtk_css_provider_load_from_resource(self.to_glib_none().0, resource_path.to_glib_none().0);
        }
    }

    fn to_string(&self) -> String {
        unsafe {
            from_glib_full(ffi::gtk_css_provider_to_string(self.to_glib_none().0))
        }
    }

    pub fn get_default() -> Option<CssProvider> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_css_provider_get_default())
        }
    }

    pub fn get_named(name: &str, variant: Option<&str>) -> Option<CssProvider> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_css_provider_get_named(name.to_glib_none().0, variant.to_glib_none().0))
        }
    }

    pub fn connect_parsing_error<F: Fn(&CssProvider, &CssSection, &Error) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&CssProvider, &CssSection, &Error) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "parsing-error",
                transmute(parsing_error_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

impl fmt::Display for CssProvider {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

unsafe extern "C" fn parsing_error_trampoline(this: *mut ffi::GtkCssProvider, section: *mut ffi::GtkCssSection, error: *mut glib_ffi::GError, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&CssProvider, &CssSection, &Error) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(section), &from_glib_none(error))
}
