use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn clean_garbage(ptr: *mut c_char) {
    unsafe {
        if !ptr.is_null() {
            // تصاحب حافظه و تبدیل به CString
            let cstr = CString::from_raw(ptr);

            // تلاش برای تبدیل به &str و چاپ محتوا
            match cstr.to_str() {
                Ok(text) => println!("Garbage says: {}", text),
                Err(_) => println!("Garbage contains invalid UTF-8."),
            }

            // حافظه در پایان drop می‌شه
            println!("Rust cleaned the garbage!");
        }
    }
}
