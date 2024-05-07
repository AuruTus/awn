use anyhow::{Result, bail};
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, GetLastError};
use std::{ffi::OsStr, iter::once, os::windows::ffi::OsStrExt};
use windows::Win32::UI::WindowsAndMessaging::FindWindowW;

use crate::input::keyboard::KeySC;
use crate::window::Window;

/// basic Window Handler
pub struct WindowInner {
    /// Windows application window handle
    hwnd: HWND,
    /// Window title discription
    title: PCWSTR
}

impl WindowInner {
    pub fn new(title_raw: &str) -> Result<Self> {
        let title = OsStr::new(title_raw);
        let title: Vec<_> = title.encode_wide().chain(once(0)).collect();
        let title = PCWSTR::from_raw(title.as_ptr());
        let hwnd = unsafe {
            let hwnd = FindWindowW(None, title);
            GetLastError().ok()?;
            if hwnd.0 == 0 {
                let title_converted = title.to_string().unwrap();
                bail!("no application find: pcwstr_title {}, raw_title {}", title_converted, title_raw);
            }
            hwnd
        };
        Ok(Self {
            hwnd,
            title
        })
    }
}

impl Window for WindowInner {
    fn press(key: KeySC) -> Result<u32> {
        todo!()
    }
}