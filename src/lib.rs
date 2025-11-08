// lib.rs
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn clean_garbage(ptr: *mut c_char) {
    unsafe {
        if !ptr.is_null() {
            let _ = CString::from_raw(ptr); // Rust takes ownership and drops it
            println!("Rust cleaned the garbage!");
        }
    }
}
