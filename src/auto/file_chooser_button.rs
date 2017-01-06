// This file was generated by gir (e48471c) from gir-files (71d73f0)
// DO NOT EDIT

use Box;
use Container;
use Dialog;
use FileChooser;
use FileChooserAction;
use Orientable;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct FileChooserButton(Object<ffi::GtkFileChooserButton>): Box, Container, Widget, Orientable, FileChooser;

    match fn {
        get_type => || ffi::gtk_file_chooser_button_get_type(),
    }
}

impl FileChooserButton {
    pub fn new(title: &str, action: FileChooserAction) -> FileChooserButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_file_chooser_button_new(title.to_glib_none().0, action.to_glib())).downcast_unchecked()
        }
    }

    pub fn new_with_dialog<T: IsA<Dialog>>(dialog: &T) -> FileChooserButton {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_file_chooser_button_new_with_dialog(dialog.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_focus_on_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_button_get_focus_on_click(self.to_glib_none().0))
        }
    }

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_file_chooser_button_get_title(self.to_glib_none().0))
        }
    }

    pub fn get_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_file_chooser_button_get_width_chars(self.to_glib_none().0)
        }
    }

    pub fn set_focus_on_click(&self, focus_on_click: bool) {
        unsafe {
            ffi::gtk_file_chooser_button_set_focus_on_click(self.to_glib_none().0, focus_on_click.to_glib());
        }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_file_chooser_button_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    pub fn set_width_chars(&self, n_chars: i32) {
        unsafe {
            ffi::gtk_file_chooser_button_set_width_chars(self.to_glib_none().0, n_chars);
        }
    }

    pub fn connect_file_set<F: Fn(&FileChooserButton) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FileChooserButton) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "file-set",
                transmute(file_set_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn file_set_trampoline(this: *mut ffi::GtkFileChooserButton, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FileChooserButton) + 'static> = transmute(f);
    f(&from_glib_none(this))
}
