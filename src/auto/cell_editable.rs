// This file was generated by gir (15fe1aa) from gir-files (11e0e6d)
// DO NOT EDIT

use Widget;
use ffi;
use glib::object::Upcast;
use glib::translate::*;

glib_wrapper! {
    pub struct CellEditable(Object<ffi::GtkCellEditable>): Widget;

    match fn {
        get_type => || ffi::gtk_cell_editable_get_type(),
    }
}

pub trait CellEditableExt {
    fn editing_done(&self);
    fn remove_widget(&self);
    //fn start_editing(&self, event: /*Unknown conversion*/Unknown rust type: "Event");
}

impl<O: Upcast<CellEditable>> CellEditableExt for O {
    fn editing_done(&self) {
        unsafe {
            ffi::gtk_cell_editable_editing_done(self.to_glib_none().0);
        }
    }

    fn remove_widget(&self) {
        unsafe {
            ffi::gtk_cell_editable_remove_widget(self.to_glib_none().0);
        }
    }

    //fn start_editing(&self, event: /*Unknown conversion*/Unknown rust type: "Event") {
    //    unsafe { TODO: call ffi::gtk_cell_editable_start_editing() }
    //}

}
