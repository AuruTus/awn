use snafu::ResultExt;
use std::thread::sleep;
use core::time::Duration;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, GetLastError};
use std::{ffi::OsStr, iter::once, os::windows::ffi::OsStrExt};
use windows::Win32::UI::WindowsAndMessaging::FindWindowW;

use crate::input::keyboard::KeySC;
use crate::window::Window;
use crate::error as awn_error;
use crate::error::Result;

/// basic Window Handler
#[derive(Debug)]
pub struct WindowInner {
    /// Windows application window handle
    pub hwnd: HWND,
    /// Window title discription
    pub title: PCWSTR
}

impl WindowInner {
    pub fn new(title_raw: String) -> Result<Self> {
        let title = OsStr::new(&title_raw[..]);
        let title: Vec<_> = title.encode_wide().chain(once(0)).collect();
        let title = PCWSTR::from_raw(title.as_ptr());
        let hwnd = unsafe {
            let hwnd = FindWindowW(None, title);
            GetLastError().ok().context(awn_error::Win32APISnafu {
                name: "FindWindowW: GetLastError",
            })?;
            if hwnd.0 == 0 {
                let title_converted = title.to_string().unwrap();
                return awn_error::ApplicationNotFoundSnafu{title_converted, title_raw}.fail();
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
    /// Press the key for `hold` time. Will block the current thread!
    fn press(&self, key: KeySC, hold: Duration) -> Result<u32> {
        let down = key.keydown()?;
        sleep(hold);
        let up = key.keyup()?;
        Ok(down + up)
    }

    fn handle(&self) -> HWND {
        self.hwnd
    }
}
