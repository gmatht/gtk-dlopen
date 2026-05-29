use crate::error::Error;
use libloading::os::unix::Library;
use std::collections::HashMap;
use std::ffi::c_void;

// Minimal subset of function pointer types we need
pub type GMainLoopNew = unsafe extern "C" fn(context: *mut c_void, is_running: i32) -> *mut c_void;
pub type GMainLoopRun = unsafe extern "C" fn(loop_: *mut c_void);
pub type GMainLoopQuit = unsafe extern "C" fn(loop_: *mut c_void);

pub type GObjectRef = unsafe extern "C" fn(obj: *mut c_void) -> *mut c_void;
pub type GObjectUnref = unsafe extern "C" fn(obj: *mut c_void);
pub type GObjectRefSink = unsafe extern "C" fn(obj: *mut c_void) -> *mut c_void;

pub type GSignalConnectData = unsafe extern "C" fn(instance: *mut c_void, detailed_signal: *const i8, c_handler: *mut c_void, data: *mut c_void, destroy_data: Option<unsafe extern "C" fn(data: *mut c_void, closure: *mut c_void)>, connect_flags: u32) -> u64;
pub type GSignalConnect = unsafe extern "C" fn(instance: *mut c_void, detailed_signal: *const i8, c_handler: *mut c_void, data: *mut c_void) -> u64;
pub type GSignalEmitByName = unsafe extern "C" fn(instance: *mut c_void, detailed_signal: *const i8) -> u64;

pub type GtkWindowNew = unsafe extern "C" fn(window_type: i32) -> *mut c_void;
pub type GtkWindowSetTitle = unsafe extern "C" fn(window: *mut c_void, title: *const i8);
pub type GtkButtonNewWithLabel = unsafe extern "C" fn(label: *const i8) -> *mut c_void;
pub type GtkLabelNew = unsafe extern "C" fn(str: *const i8) -> *mut c_void;
pub type GtkLabelSetText = unsafe extern "C" fn(label: *mut c_void, str: *const i8);
pub type GtkLabelGetText = unsafe extern "C" fn(label: *mut c_void) -> *const i8;
pub type GtkBoxNew = unsafe extern "C" fn(orientation: i32, spacing: i32) -> *mut c_void;

pub type GtkBoxAppend = unsafe extern "C" fn(box_: *mut c_void, child: *mut c_void);
pub type GtkBoxPackStart = unsafe extern "C" fn(box_: *mut c_void, child: *mut c_void, expand: i32, fill: i32, padding: u32);
pub type GtkContainerAdd = unsafe extern "C" fn(container: *mut c_void, widget: *mut c_void);
pub type GtkWindowSetChild = unsafe extern "C" fn(window: *mut c_void, child: *mut c_void);
pub type GtkWidgetShowAll = unsafe extern "C" fn(widget: *mut c_void);
pub type GtkWindowPresent = unsafe extern "C" fn(window: *mut c_void);
pub type GtkInit = unsafe extern "C" fn(argc: *mut libc::c_int, argv: *mut *mut *mut libc::c_char);

pub struct Symbols {
    // glib
    pub g_main_loop_new: Option<GMainLoopNew>,
    pub g_main_loop_run: Option<GMainLoopRun>,
    pub g_main_loop_quit: Option<GMainLoopQuit>,

    // gobject
    pub g_object_ref: Option<GObjectRef>,
    pub g_object_unref: Option<GObjectUnref>,
    pub g_object_ref_sink: Option<GObjectRefSink>,
    pub g_signal_connect_data: Option<GSignalConnectData>,
    pub g_signal_connect: Option<GSignalConnect>,

    // gtk
    pub gtk_window_new: Option<GtkWindowNew>,
    pub gtk_window_set_title: Option<GtkWindowSetTitle>,
    pub gtk_button_new_with_label: Option<GtkButtonNewWithLabel>,
    pub gtk_label_new: Option<GtkLabelNew>,
    pub gtk_label_set_text: Option<GtkLabelSetText>,
    pub gtk_label_get_text: Option<GtkLabelGetText>,
    pub gtk_box_new: Option<GtkBoxNew>,
    pub gtk_box_append: Option<GtkBoxAppend>,
    pub gtk_box_pack_start: Option<GtkBoxPackStart>,
    pub gtk_container_add: Option<GtkContainerAdd>,
    pub gtk_window_set_child: Option<GtkWindowSetChild>,
    pub gtk_widget_show_all: Option<GtkWidgetShowAll>,
    pub gtk_window_present: Option<GtkWindowPresent>,
    pub gtk_init: Option<GtkInit>,
    pub g_signal_emit_by_name: Option<GSignalEmitByName>,
}

impl Symbols {
    pub fn load(libs: &std::collections::HashMap<String, std::sync::Arc<Library>>) -> Result<Self, Error> {
        // helper to lookup in libgtk first then glib/gobject as appropriate
        let gtk = libs.get("libgtk").expect("libgtk missing");
        let glib = libs.get("libglib").expect("libglib missing");
        let gobject = libs.get("libgobject").expect("libgobject missing");

        unsafe fn sym<T: Copy>(lib: &Library, name: &str) -> Option<T> {
            match lib.get::<T>(name.as_bytes()) {
                Ok(s) => Some(*s),
                Err(_) => None,
            }
        }

        // glib
        let g_main_loop_new = unsafe { sym::<GMainLoopNew>(glib, "g_main_loop_new") };
        let g_main_loop_run = unsafe { sym::<GMainLoopRun>(glib, "g_main_loop_run") };
        let g_main_loop_quit = unsafe { sym::<GMainLoopQuit>(glib, "g_main_loop_quit") };

        // gobject
        let g_object_ref = unsafe { sym::<GObjectRef>(gobject, "g_object_ref") };
        let g_object_unref = unsafe { sym::<GObjectUnref>(gobject, "g_object_unref") };
        let g_object_ref_sink = unsafe { sym::<GObjectRefSink>(gobject, "g_object_ref_sink") };
        let g_signal_connect_data = unsafe { sym::<GSignalConnectData>(gobject, "g_signal_connect_data") };
        let g_signal_connect = unsafe { sym::<GSignalConnect>(gobject, "g_signal_connect") };

        // gtk symbols
        let gtk_window_new = unsafe { sym::<GtkWindowNew>(gtk, "gtk_window_new") };
        let gtk_window_set_title = unsafe { sym::<GtkWindowSetTitle>(gtk, "gtk_window_set_title") };
        let gtk_button_new_with_label = unsafe { sym::<GtkButtonNewWithLabel>(gtk, "gtk_button_new_with_label") };
        let gtk_label_new = unsafe { sym::<GtkLabelNew>(gtk, "gtk_label_new") };
        let gtk_label_set_text = unsafe { sym::<GtkLabelSetText>(gtk, "gtk_label_set_text") };
        let gtk_label_get_text = unsafe { sym::<GtkLabelGetText>(gtk, "gtk_label_get_text") };
        let gtk_box_new = unsafe { sym::<GtkBoxNew>(gtk, "gtk_box_new") };
        let gtk_box_append = unsafe { sym::<GtkBoxAppend>(gtk, "gtk_box_append") };
        let gtk_box_pack_start = unsafe { sym::<GtkBoxPackStart>(gtk, "gtk_box_pack_start") };
        let gtk_container_add = unsafe { sym::<GtkContainerAdd>(gtk, "gtk_container_add") };
        let gtk_window_set_child = unsafe { sym::<GtkWindowSetChild>(gtk, "gtk_window_set_child") };
        let gtk_widget_show_all = unsafe { sym::<GtkWidgetShowAll>(gtk, "gtk_widget_show_all") };
            let gtk_window_present = unsafe { sym::<GtkWindowPresent>(gtk, "gtk_window_present") };
        let gtk_init = unsafe { sym::<GtkInit>(gtk, "gtk_init") };
        let g_signal_emit_by_name = unsafe { sym::<GSignalEmitByName>(gobject, "g_signal_emit_by_name") };

        Ok(Symbols {
            g_main_loop_new, g_main_loop_run, g_main_loop_quit,
            g_object_ref, g_object_unref, g_object_ref_sink, g_signal_connect_data, g_signal_connect,
            gtk_window_new, gtk_window_set_title, gtk_button_new_with_label, gtk_label_new, gtk_label_set_text,
            gtk_box_new, gtk_box_append, gtk_box_pack_start, gtk_container_add, gtk_window_set_child,
            gtk_widget_show_all, gtk_window_present,
            gtk_init,
            g_signal_emit_by_name,
            gtk_label_get_text,
        })
    }
}
