use crate::loader::Loader;
use crate::symbols::*;
use crate::error::Error;
use std::ffi::CString;
use std::marker::PhantomData;
use std::os::raw::c_void;
use std::rc::Rc;
use std::sync::Arc;

pub struct BoxWidget {
    inner: *mut c_void,
    loader: Arc<Loader>,
    _not_send: PhantomData<Rc<()>>,
}

pub enum Orientation { Horizontal = 0, Vertical = 1 }

impl BoxWidget {
    pub fn new(loader: Arc<Loader>, orientation: Orientation, spacing: i32) -> Result<Self, Error> {
        let symbols = &loader.symbols;
        let gtk_box_new = symbols.gtk_box_new.ok_or(Error::MissingSymbol("gtk_box_new".into()))?;
        let inner = unsafe { gtk_box_new(orientation as i32, spacing) };
        // ref sink if available
        if let Some(ref_sink) = symbols.g_object_ref_sink { unsafe { ref_sink(inner); } }
        else if let Some(gref) = symbols.g_object_ref { unsafe { gref(inner); } }
        Ok(BoxWidget { inner, loader, _not_send: PhantomData })
    }

    pub fn append(&self, child: &impl AsRef<*mut c_void>) {
        let symbols = &self.loader.symbols;
        let child_ptr = *child.as_ref();
        if let Some(box_append) = symbols.gtk_box_append {
            unsafe { box_append(self.inner, child_ptr); }
        } else if let Some(pack) = symbols.gtk_box_pack_start {
            unsafe { pack(self.inner, child_ptr, 1, 1, 0); }
        }
    }
}

impl AsRef<*mut c_void> for BoxWidget {
    fn as_ref(&self) -> &*mut c_void { &self.inner }
}

impl Drop for BoxWidget {
    fn drop(&mut self) {
        if let Some(unref) = self.loader.symbols.g_object_unref { unsafe { unref(self.inner); } }
    }
}

pub struct Window {
    inner: *mut c_void,
    loader: Arc<Loader>,
    _not_send: PhantomData<Rc<()>>,
}

impl Window {
    pub fn new(loader: Arc<Loader>) -> Result<Self, Error> {
        let symbols = &loader.symbols;
        let gtk_window_new = symbols.gtk_window_new.ok_or(Error::MissingSymbol("gtk_window_new".into()))?;
        // GTK_WINDOW_TOPLEVEL is 0
        let inner = unsafe { gtk_window_new(0) };
        if let Some(ref_sink) = symbols.g_object_ref_sink { unsafe { ref_sink(inner); } }
        else if let Some(gref) = symbols.g_object_ref { unsafe { gref(inner); } }
        Ok(Window { inner, loader, _not_send: PhantomData })
    }

    pub fn set_title(&self, title: &str) {
        if let Some(set_title) = self.loader.symbols.gtk_window_set_title {
            let c = CString::new(title).unwrap();
            unsafe { set_title(self.inner, c.as_ptr()); }
        }
    }

    pub fn set_child(&self, child: &impl AsRef<*mut c_void>) {
        let symbols = &self.loader.symbols;
        let child_ptr = *child.as_ref();
        if let Some(set_child) = symbols.gtk_window_set_child {
            unsafe { set_child(self.inner, child_ptr); }
        } else if let Some(container_add) = symbols.gtk_container_add {
            unsafe { container_add(self.inner, child_ptr); }
        }
    }

    pub fn present(&self) {
        // Ensure children are shown first (GTK3 needs gtk_widget_show_all in many cases),
        // then present the window if possible.
        if let Some(show_all) = self.loader.symbols.gtk_widget_show_all { unsafe { show_all(self.inner); } }
        if let Some(present) = self.loader.symbols.gtk_window_present { unsafe { present(self.inner); return; } }
    }
}

impl AsRef<*mut c_void> for Window { fn as_ref(&self) -> &*mut c_void { &self.inner } }

impl Drop for Window {
    fn drop(&mut self) {
        if let Some(unref) = self.loader.symbols.g_object_unref { unsafe { unref(self.inner); } }
    }
}

pub struct Button {
    inner: *mut c_void,
    loader: Arc<Loader>,
    _not_send: PhantomData<Rc<()>>,
}

impl Button {
    pub fn with_label(loader: Arc<Loader>, label: &str) -> Result<Self, Error> {
        let symbols = &loader.symbols;
        let ctor = symbols.gtk_button_new_with_label.ok_or(Error::MissingSymbol("gtk_button_new_with_label".into()))?;
        let c = CString::new(label).unwrap();
        let inner = unsafe { ctor(c.as_ptr()) };
        if let Some(ref_sink) = symbols.g_object_ref_sink { unsafe { ref_sink(inner); } }
        else if let Some(gref) = symbols.g_object_ref { unsafe { gref(inner); } }
        Ok(Button { inner, loader, _not_send: PhantomData })
    }

    pub fn connect_clicked<F: FnMut() + 'static>(&self, f: F) -> Result<u64, Error> {
        let symbols = &self.loader.symbols;
        let boxed: Box<dyn FnMut()> = Box::new(f);
        unsafe {
            match crate::signals::connect_clicked(symbols, self.inner, boxed) {
                Ok(id) => Ok(id),
                Err(e) => Err(Error::Other(e)),
            }
        }
    }

    pub fn emit_clicked(&self) -> Result<u64, Error> {
        if let Some(emit) = self.loader.symbols.g_signal_emit_by_name {
            use std::ffi::CString;
            let name = CString::new("clicked").unwrap();
            let id = unsafe { emit(self.inner, name.as_ptr()) };
            Ok(id)
        } else { Err(Error::MissingSymbol("g_signal_emit_by_name".into())) }
    }
}

impl AsRef<*mut c_void> for Button { fn as_ref(&self) -> &*mut c_void { &self.inner } }

impl Drop for Button {
    fn drop(&mut self) {
        if let Some(unref) = self.loader.symbols.g_object_unref { unsafe { unref(self.inner); } }
    }
}

pub struct Label {
    inner: *mut c_void,
    loader: Arc<Loader>,
    _not_send: PhantomData<Rc<()>>,
}

impl Label {
    pub fn new(loader: Arc<Loader>, text: &str) -> Result<Self, Error> {
        let symbols = &loader.symbols;
        let ctor = symbols.gtk_label_new.ok_or(Error::MissingSymbol("gtk_label_new".into()))?;
        let c = CString::new(text).unwrap();
        let inner = unsafe { ctor(c.as_ptr()) };
        if let Some(ref_sink) = symbols.g_object_ref_sink { unsafe { ref_sink(inner); } }
        else if let Some(gref) = symbols.g_object_ref { unsafe { gref(inner); } }
        Ok(Label { inner, loader, _not_send: PhantomData })
    }

    pub fn set_text(&self, text: &str) {
        if let Some(set_text) = self.loader.symbols.gtk_label_set_text {
            let c = CString::new(text).unwrap();
            unsafe { set_text(self.inner, c.as_ptr()); }
        }
    }

    pub fn get_text(&self) -> Option<String> {
        if let Some(get_text) = self.loader.symbols.gtk_label_get_text {
            unsafe {
                let s = get_text(self.inner);
                if s.is_null() { return None; }
                let c = std::ffi::CStr::from_ptr(s);
                return Some(c.to_string_lossy().into_owned());
            }
        }
        None
    }
}

impl AsRef<*mut c_void> for Label { fn as_ref(&self) -> &*mut c_void { &self.inner } }

impl Drop for Label { fn drop(&mut self) { if let Some(unref) = self.loader.symbols.g_object_unref { unsafe { unref(self.inner); } } } }

impl Clone for Label {
    fn clone(&self) -> Self {
        // increment ref
        if let Some(gref) = self.loader.symbols.g_object_ref {
            unsafe { gref(self.inner); }
        }
        Label { inner: self.inner, loader: self.loader.clone(), _not_send: PhantomData }
    }
}

// Application wrapper (simplified): uses gtk_application_new + g_application_run from libgio
pub struct Application {
    inner: *mut c_void,
    loader: Arc<Loader>,
}

impl Application {
    pub fn new(loader: Arc<Loader>, id: Option<&str>) -> Result<Self, Error> {
        // We expect libgio and the symbols to be present; for this task we won't fallback
        // We'll look up gtk_application_new in libgtk and g_application_run in libgio via symbols indirectly (not defined yet)
        // For now, call gtk_init (optional) and return an Application with inner == null and let run use GMainLoop via glib
        Ok(Application { inner: std::ptr::null_mut(), loader })
    }

    pub fn run(self) -> Result<(), Error> {
        // For simplicity call glib GMainLoop
        let symbols = &self.loader.symbols;
        let loop_new = symbols.g_main_loop_new.ok_or(Error::MissingSymbol("g_main_loop_new".into()))?;
        let loop_run = symbols.g_main_loop_run.ok_or(Error::MissingSymbol("g_main_loop_run".into()))?;
        let loop_ptr = unsafe { loop_new(std::ptr::null_mut(), 0) };
        unsafe { loop_run(loop_ptr); }
        Ok(())
    }
}
