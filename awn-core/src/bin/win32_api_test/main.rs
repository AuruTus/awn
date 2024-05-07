use std::os::windows::ffi::OsStrExt;
use windows::core::PCWSTR;
use windows::Win32::{
    Foundation::GetLastError,
    UI::WindowsAndMessaging::*,
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
                println!("hello {:?}", hwnd.0);
            }
            Err(e) => {
                println!("{e}")
            }
        }
    }
    let _: Option<i32> = std::io::stdin()
    .bytes() 
    .next()
    .and_then(|result| result.ok())
    .map(|byte| byte as i32);
}
