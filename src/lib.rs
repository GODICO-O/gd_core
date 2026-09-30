use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn Java_com_godico_gdlauncher_host_MainActivity_gd_1core_1init() -> i32 {
    // Return status code 1 (Sukses)
    1
}

#[no_mangle]
pub extern "C" fn Java_com_godico_gdlauncher_host_MainActivity_gd_1core_1get_1version() -> *const c_char {
    let version = CString::new("v0.1.0-debug").unwrap();
    version.into_raw()
}
