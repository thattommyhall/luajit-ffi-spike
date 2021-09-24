use cid::{Cid, Codec};
use libc::size_t;
use multihash::Sha2_256;
use std::convert::TryFrom;
use std::ffi::CString;
use std::os::raw::c_char;
use std::slice;
use std::str;

#[no_mangle]
pub extern "C" fn string_from_rust() -> *const c_char {
    let s = CString::new("Hello World").unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}

#[no_mangle]
pub extern "C" fn cid_test() -> *const c_char {
    let h = Sha2_256::digest(b"beep boop");
    let cid = Cid::new_v1(Codec::DagProtobuf, h);
    let cid_string = cid.to_string();
    let p = cid_string.as_ptr() as *const c_char;
    std::mem::forget(cid_string);
    p
}

#[no_mangle]
pub extern "C" fn v0_to_v1(v0: *const u8, v0_len: size_t, v1: *mut u8, v1_len: size_t) -> size_t {
    assert!(!v0.is_null());
    assert!(!v1.is_null());
    let message_slice = unsafe { slice::from_raw_parts(v0, v0_len) };
    let result_buffer = unsafe { slice::from_raw_parts_mut(v1, v1_len) };
    let cid_str = unsafe { str::from_utf8_unchecked(message_slice) };
    let cid_v0 = if let Ok(cid) = Cid::try_from(cid_str) {
        cid
    } else {
        return 0;
    };
    let cid_v1 = Cid::new_v1(Codec::DagProtobuf, cid_v0.hash().to_owned());
    let cid_string = cid_v1.to_string();
    let cid_bytes = cid_string.as_bytes();
    result_buffer[..cid_bytes.len()].copy_from_slice(&cid_bytes);
    cid_bytes.len()
}
