// This file was generated by gir (b85b267) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
#[cfg(gtk_3_10)]
use PlacesOpenFlags;
use ScrolledWindow;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct PlacesSidebar(Object<ffi::GtkPlacesSidebar>): Widget, Container, Bin, ScrolledWindow, Buildable;

    match fn {
        get_type => || ffi::gtk_places_sidebar_get_type(),
    }
}

impl PlacesSidebar {
    #[cfg(gtk_3_10)]
    pub fn new() -> PlacesSidebar {
        unsafe {
            Widget::from_glib_none(ffi::gtk_places_sidebar_new()).downcast_unchecked()
        }
    }

    //#[cfg(gtk_3_10)]
    //pub fn add_shortcut(&self, location: Unknown rust type: "File") {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_add_shortcut() }
    //}

    #[cfg(gtk_3_12)]
    pub fn get_local_only(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_local_only(self.to_glib_none().0))
        }
    }

    //#[cfg(gtk_3_10)]
    //pub fn get_location(&self) -> Unknown rust type: "File" {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_get_location() }
    //}

    //#[cfg(gtk_3_10)]
    //pub fn get_nth_bookmark(&self, n: i32) -> Unknown rust type: "File" {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_get_nth_bookmark() }
    //}

    #[cfg(gtk_3_10)]
    pub fn get_open_flags(&self) -> PlacesOpenFlags {
        unsafe {
            ffi::gtk_places_sidebar_get_open_flags(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_show_connect_to_server(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_connect_to_server(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_show_desktop(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_desktop(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_14)]
    pub fn get_show_enter_location(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_enter_location(self.to_glib_none().0))
        }
    }

    //#[cfg(gtk_3_10)]
    //pub fn list_shortcuts(&self) -> /*Unknown conversion*/Unknown rust type: "SList TypeId { ns_id: 6, id: 149 }" {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_list_shortcuts() }
    //}

    //#[cfg(gtk_3_10)]
    //pub fn remove_shortcut(&self, location: Unknown rust type: "File") {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_remove_shortcut() }
    //}

    #[cfg(gtk_3_12)]
    pub fn set_local_only(&self, local_only: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_local_only(self.to_glib_none().0, local_only.to_glib());
        }
    }

    //#[cfg(gtk_3_10)]
    //pub fn set_location(&self, location: Unknown rust type: "File") {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_set_location() }
    //}

    #[cfg(gtk_3_10)]
    pub fn set_open_flags(&self, flags: PlacesOpenFlags) {
        unsafe {
            ffi::gtk_places_sidebar_set_open_flags(self.to_glib_none().0, flags);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_show_connect_to_server(&self, show_connect_to_server: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_connect_to_server(self.to_glib_none().0, show_connect_to_server.to_glib());
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_show_desktop(&self, show_desktop: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_desktop(self.to_glib_none().0, show_desktop.to_glib());
        }
    }

    #[cfg(gtk_3_14)]
    pub fn set_show_enter_location(&self, show_enter_location: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_enter_location(self.to_glib_none().0, show_enter_location.to_glib());
        }
    }

}
