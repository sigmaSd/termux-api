use std::ffi::CString;

#[no_mangle]
pub extern "C" fn battery_status() -> *mut i8 {
    (|| -> crate::Result<_> {
        Ok(CString::new(serde_json::to_string(&crate::battery_status()?)?)?.into_raw())
    })()
    // TODO: propagate the error as a string
    .unwrap_or(std::ptr::null_mut())
}
