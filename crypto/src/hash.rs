use std::mem;
use crate::utils::cstring;
use crate::utils::hash;

use crate::base64_decode_with_error;

#[no_mangle]
pub extern "stdcall" fn hash(hash_type: *const u16, data_ptr: *const u16) -> *const u16 {
    let data = cstring::from_ptr(data_ptr).unwrap();
    let data = base64_decode_with_error!(data);
    let hash_type = cstring::from_ptr(hash_type).unwrap();

    let hashed = match hash::hash_base64(data, &hash_type) {
        Some(hashed) => hashed,
        None => {
            let mut err_string = String::from("Hash not supported yet: ");
            err_string.insert_str(0, crate::ERR);
            err_string.push_str(&hash_type);
            let wstring = cstring::to_widechar(&err_string);
            return mem::ManuallyDrop::new(wstring).as_ptr();
        }
    };

    mem::ManuallyDrop::new(cstring::to_widechar(&hashed)).as_ptr()
}