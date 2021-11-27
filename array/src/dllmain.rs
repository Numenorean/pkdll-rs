use wchar::{wchz};

use winapi::um::{
    consoleapi::AllocConsole,
    wincon::FreeConsole,
};
use winapi::shared::minwindef::{BOOL, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};


const AUTHOR: &[u16] = wchz!("_Skill_");
const VER: &[u16] = wchz!("0.1");
const DESC: &[u16] = wchz!("Функции для работы с массивами");
const DEBUG: bool = false;

#[no_mangle]
pub extern "stdcall" fn info_getAuthor() ->  *const u16 {
    AUTHOR.as_ptr()
}

#[no_mangle]
pub extern "stdcall" fn info_getVersion() ->  *const u16 {
    VER.as_ptr()
}

#[no_mangle]
pub extern "stdcall" fn info_getDescription() ->  *const u16 {
    DESC.as_ptr()
}

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(h_module: HINSTANCE, dw_reason: u32, _: LPVOID) -> BOOL {
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
            unsafe {
                FreeConsole();
            }
        }
        _ => {}
    }
    TRUE
}
