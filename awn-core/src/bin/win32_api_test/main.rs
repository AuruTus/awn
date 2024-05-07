use std::mem;
use std::{thread, time};
use std::os::windows::ffi::OsStrExt;
use windows::core::PCWSTR;
use windows::Win32::{
    Foundation::GetLastError,
    UI::WindowsAndMessaging::*,
    UI::Input::KeyboardAndMouse::*,
};
use std::ffi::OsStr;
use std::iter::once;
use std::io::Read;

const TITLE: &str = "Granblue Fantasy: Relink";

fn main() {
    let title = OsStr::new(TITLE);
    let title: Vec<_> = title.encode_wide().chain(once(0)).collect();
    let title = PCWSTR::from_raw(title.as_ptr());
    let dpy_title = unsafe {title.to_string().unwrap()};
    println!("title str: {:?}, title pcstr: {:?}, title content: {}", TITLE, title, dpy_title);
    unsafe {
        let hwnd = FindWindowW(None, title);
        match GetLastError().ok() {
            Ok(_) => {
                println!("window handler: {:?}", hwnd);
            }
            Err(e) => {
                panic!("get window error: {e}")
            }
        }

        let _ = SetForegroundWindow(hwnd);
        
        let hex_key_code = 0x11; // press w once
        let pinputs = &mut [INPUT::default(); 2][..];
        pinputs[0].r#type = INPUT_KEYBOARD;
        pinputs[0].Anonymous.ki.wScan = hex_key_code;
        pinputs[0].Anonymous.ki.dwFlags = KEYEVENTF_SCANCODE;
        SendInput(&pinputs[0..1], mem::size_of::<INPUT>() as _);

        thread::sleep(time::Duration::from_millis(250));

        pinputs[1].r#type = INPUT_KEYBOARD;
        pinputs[1].Anonymous.ki.wScan = hex_key_code;
        pinputs[1].Anonymous.ki.dwFlags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;
        SendInput(&pinputs[1..], mem::size_of::<INPUT>() as _);
        
        let curr_hwnd = GetForegroundWindow();
        println!("curr_hwnd: {:?}, target_hwnd: {:?}", curr_hwnd, hwnd);
        println!("finish window control...")
    }
    let _: Option<i32> = std::io::stdin()
    .bytes() 
    .next()
    .and_then(|result| result.ok())
    .map(|byte| byte as i32);
}
