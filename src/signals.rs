use crate::symbols::GSignalConnectData;
use crate::symbols::GSignalConnect;
use std::ffi::CString;
use std::os::raw::c_void;

// Trampoline and destroy notify for clicked handler
// We'll define a small C-ABI trampoline that converts user_data pointer into a Box<dyn FnMut()>

#[no_mangle]
pub extern "C" fn gtk_compat_clicked_trampoline(widget: *mut c_void, user_data: *mut c_void) {
    unsafe {
        if user_data.is_null() { return; }
        // user_data is a pointer produced by Box::into_raw on a Box<Box<dyn FnMut()>>.
        // Box::into_raw returns a *mut Box<dyn FnMut()>, so cast accordingly.
        let inner_ptr = user_data as *mut Box<dyn FnMut()>;
        if inner_ptr.is_null() { return; }
        // Deref twice: Box<dyn FnMut()> -> dyn FnMut()
        let closure_ref: &mut dyn FnMut() = &mut **inner_ptr;
        closure_ref();
    }
}

#[no_mangle]
pub extern "C" fn gtk_compat_destroy_notify(data: *mut c_void, _closure: *mut c_void) {
    unsafe {
        if data.is_null() { return; }
        // data is a *mut Box<dyn FnMut()> produced by Box::into_raw
        let inner_ptr = data as *mut Box<dyn FnMut()>;
        // reconstruct the outer Box<Box<dyn FnMut()>> and drop it (frees the closure)
        let _boxed: Box<Box<dyn FnMut()>> = Box::from_raw(inner_ptr);
        // dropped here
    }
}

// helper to connect a "clicked" handler using either g_signal_connect_data or g_signal_connect
pub unsafe fn connect_clicked(lib_symbols: &crate::symbols::Symbols, instance: *mut c_void, cb: Box<dyn FnMut()>) -> Result<u64, String> {
    // box twice so the pointer to Box remains stable (Box<dyn FnMut()> -> *mut Box<dyn FnMut()>)
    let boxed: Box<Box<dyn FnMut()>> = Box::new(Box::new(cb));
    let raw = Box::into_raw(boxed) as *mut c_void;

    let sig_name = CString::new("clicked").unwrap();
    if let Some(gscd) = lib_symbols.g_signal_connect_data {
        // connect with destroy notify
        let handler_ptr = gtk_compat_clicked_trampoline as *const () as *mut c_void;
        let destroy_ptr = Some(gtk_compat_destroy_notify as unsafe extern "C" fn(*mut c_void, *mut c_void));
        let id = gscd(instance, sig_name.as_ptr(), handler_ptr, raw, destroy_ptr, 0);
        Ok(id)
    } else if let Some(gsc) = lib_symbols.g_signal_connect {
        let handler_ptr = gtk_compat_clicked_trampoline as *const () as *mut c_void;
        let id = gsc(instance, sig_name.as_ptr(), handler_ptr, raw);
        // We didn't register a destroy notify; closure will leak. It's acceptable for the demo.
        Ok(id)
    } else {
        Err("no g_signal_connect available".into())
    }
}
