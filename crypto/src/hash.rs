use winapi::um::winnt::PWSTR;

use crate::utils::cstring;
use crate::utils::hash;

use crate::unwrap_or_err;

#[no_mangle]
pub extern "stdcall" fn hash(hash_type: PWSTR, data_ptr: PWSTR) -> PWSTR {
    let data = cstring::from_ptr(data_ptr);
    let data = unwrap_or_err!(base64::decode(data));
    let hash_type = cstring::from_ptr(hash_type);

    let hashed = unwrap_or_err!(hash::make_hash(data, &hash_type));

    cstring::to_widechar_ptr(&base64::encode(hashed))
}