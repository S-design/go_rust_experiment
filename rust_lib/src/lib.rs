use sha2::{Sha256, Digest};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn hash_sha256(input: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(input) };
    let str_slice = c_str.to_str().unwrap_or("");
//creating the hasher and input process
    let mut hasher = Sha256::new();
    hasher.update(str_slice.as_bytes());
    let result = hasher.finalize();

//returning th C-compatible string that Go can read
    let hex_string = format!("{:x}", result);
    CString::new(hex_string).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {return}
        let _ = CString::from_raw(s);
    }
}