use core::time::Duration;
use windows::Win32::Foundation::HWND;

use crate::error::Result;
use crate::input::keyboard::KeySC;

pub mod inner;

/// Trait for handling Window
pub trait Window {
    /// Press the key once for `hold` time.
    fn press(&self, keys: &[KeySC], hold: Duration) -> Result<u32>;
    /// Get the Windows Handle of the application window.
    fn handle(&self) -> HWND;
}
