use crate::error as werror;
use crate::error::Result;
use crate::input::keyboard::KeySC;
use core::time::Duration;
use windows::Win32::{
    Foundation::{BOOL, HWND},
    UI::WindowsAndMessaging::SetForegroundWindow,
};

pub mod inner;

/// Trait for handling Window
pub trait Window {
    /// Foreground this window.
    fn foreground(&self) -> Result<()> {
        let hwnd = self.handle();
        unsafe {
            match SetForegroundWindow(hwnd) {
                BOOL(0) => werror::WindowNotForegroundSnafu { hwnd }.fail(),
                _ => Ok(()),
            }
        }
    }
    /// Press the key once for `hold` time.
    fn press(&self, keys: &[KeySC], hold: Duration) -> Result<u32>;
    /// Get the Windows Handle of the application window.
    fn handle(&self) -> HWND;
}
