#[no_mangle]
pub extern "C" fn generate_lyrics() -> *const i8 {
    let lyrics = "CRRRRR YEET FUCK ME A-A-A-A";
    std::ffi::CString::new(lyrics).unwrap().into_raw()
}
    