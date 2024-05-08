pub mod inner;

use core::time::Duration;
use anyhow::Result;
use windows::Win32::Foundation::HWND;
use crate::input::keyboard::KeySC;

/// Trait for handling Window
pub trait Window {
    /// Press the key once for `hold` time.
    fn press(&self, key: KeySC, hold: Duration) -> Result<u32>;
    /// Get the Windows Handle of the application window.
    fn handle(&self) -> HWND;
}
