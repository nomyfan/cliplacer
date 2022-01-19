use std::ffi::{OsStr, OsString};

use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;
use windows::Win32::System::{
    DataExchange::{GetClipboardData, IsClipboardFormatAvailable, OpenClipboard, SetClipboardData},
    Memory::{GlobalAlloc, GlobalLock, GlobalSize, GlobalUnlock},
};
use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, MOD_ALT, MOD_NOREPEAT};
use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG};
use windows::Win32::{Foundation::HANDLE, System::Memory::GlobalFree};
use windows::Win32::{Foundation::HWND, System::DataExchange::CloseClipboard};

const WM_HOTKEY: u32 = 0x0312;
const CF_UNICODETEXT: u32 = 13;
const GMEM_MOVEABLE: u32 = 0x0002;
const VK_C: u32 = 0x43;

macro_rules! NULL_HWND {
    () => {
        HWND::default()
    };
}

fn main() {
    match unsafe { RegisterHotKey(NULL_HWND!(), 1, MOD_ALT | MOD_NOREPEAT, VK_C).as_bool() } {
        true => {
            println!("Alt+C was registered")
        }
        false => std::process::exit(1),
    }

    let mut msg = MSG::default();
    let msg_ptr = &mut msg as *mut MSG;
    while unsafe { GetMessageW(msg_ptr, NULL_HWND!(), 0, 0).as_bool() } {
        if msg.message != WM_HOTKEY {
            continue;
        }
        println!("WM_HOTKEY received");

        if !unsafe { IsClipboardFormatAvailable(CF_UNICODETEXT).as_bool() } {
            continue;
        }

        if !unsafe { OpenClipboard(NULL_HWND!()).as_bool() } {
            eprintln!("cannot open clipboard");
            continue;
        }

        let hw = unsafe { GetClipboardData(CF_UNICODETEXT) };
        unsafe { CloseClipboard() };
        if hw.is_invalid() {
            continue;
        }

        let data_size = unsafe { GlobalSize(hw.0) } / std::mem::size_of::<u16>();
        let mut dst = Vec::with_capacity(data_size);
        let data_ptr = hw.0 as *const u16;
        unsafe {
            std::ptr::copy_nonoverlapping(data_ptr, dst.as_mut_ptr(), data_size);
            dst.set_len(data_size);
        }

        let text_content: String = OsString::from_wide(&dst[..]).to_string_lossy().into_owned();

        let wnl: String = text_content.lines().collect();
        let wnl_u16_raw = OsStr::new(&wnl[..])
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>();

        let dw_bytes = std::mem::size_of::<u16>() * wnl_u16_raw.len();

        unsafe {
            let alloc_mem = GlobalAlloc(GMEM_MOVEABLE, dw_bytes);
            let dst_ptr = GlobalLock(alloc_mem);
            std::ptr::copy_nonoverlapping(wnl_u16_raw.as_ptr(), dst_ptr as _, dw_bytes);
            GlobalUnlock(alloc_mem);

            OpenClipboard(NULL_HWND!());
            SetClipboardData(CF_UNICODETEXT, HANDLE(alloc_mem));
            GlobalFree(alloc_mem);
            CloseClipboard();
        }
    }
}
