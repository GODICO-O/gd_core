// Entry point C-ABI untuk dipanggil APK Debug Launcher
#[no_mangle]
pub extern "C" fn gd_core_init() -> i32 {
    // Return 1 sebagai tanda core berhasil berjalan
    1
}

#[no_mangle]
pub extern "C" fn gd_core_get_version() -> *const std::os::raw::c_char {
    "v0.1.0-debug\0".as_ptr() as *const std::os::raw::c_char
}
