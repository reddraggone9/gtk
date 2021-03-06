// This file was generated by gir (1644ef1) from gir-files (71d73f0)
// DO NOT EDIT

use Widget;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Actionable(Object<ffi::GtkActionable>): Widget;

    match fn {
        get_type => || ffi::gtk_actionable_get_type(),
    }
}

pub trait ActionableExt {
    fn get_action_name(&self) -> Option<String>;

    fn get_action_target_value(&self) -> Option<glib::Variant>;

    fn set_action_name(&self, action_name: Option<&str>);

    //fn set_action_target(&self, format_string: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn set_action_target_value(&self, target_value: &glib::Variant);

    fn set_detailed_action_name(&self, detailed_action_name: &str);
}

impl<O: IsA<Actionable>> ActionableExt for O {
    fn get_action_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_actionable_get_action_name(self.to_glib_none().0))
        }
    }

    fn get_action_target_value(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::gtk_actionable_get_action_target_value(self.to_glib_none().0))
        }
    }

    fn set_action_name(&self, action_name: Option<&str>) {
        unsafe {
            ffi::gtk_actionable_set_action_name(self.to_glib_none().0, action_name.to_glib_none().0);
        }
    }

    //fn set_action_target(&self, format_string: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_actionable_set_action_target() }
    //}

    fn set_action_target_value(&self, target_value: &glib::Variant) {
        unsafe {
            ffi::gtk_actionable_set_action_target_value(self.to_glib_none().0, target_value.to_glib_none().0);
        }
    }

    fn set_detailed_action_name(&self, detailed_action_name: &str) {
        unsafe {
            ffi::gtk_actionable_set_detailed_action_name(self.to_glib_none().0, detailed_action_name.to_glib_none().0);
        }
    }
}
