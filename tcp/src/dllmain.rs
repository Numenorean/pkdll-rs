use wchar::wchz;

use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, LPCWSTR};
use winapi::um::{consoleapi::AllocConsole, wincon::FreeConsole};

//use crate::CACHE;

const AUTHOR: &[u16] = wchz!("_Skill_");
const VER: &[u16] = wchz!("0.1");
const DESC: &[u16] = wchz!("Работаем с tcp/udp без лишних серваков");
const DEBUG: bool = true;

#[no_mangle]
extern "stdcall" fn info_getAuthor() -> LPCWSTR {
    AUTHOR.as_ptr()
}

#[no_mangle]
extern "stdcall" fn info_getVersion() -> LPCWSTR {
    VER.as_ptr()
}

#[no_mangle]
extern "stdcall" fn info_getDescription() -> LPCWSTR {
    DESC.as_ptr()
}

#[no_mangle]
#[allow(non_snake_case)]
extern "stdcall" fn DllMain(h_module: HINSTANCE, dw_reason: DWORD, _: LPVOID) -> BOOL {
    match dw_reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                // We don't need to know if PK creates new threads
                DisableThreadLibraryCalls(h_module);
                if DEBUG {
                    AllocConsole();
                }
            }
        }
        DLL_PROCESS_DETACH => {
            if DEBUG {
                unsafe {
                    FreeConsole();
                }
            }
        }
        _ => {}
    }
    TRUE
}
