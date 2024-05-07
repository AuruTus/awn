pub mod inner;

use anyhow::Result;
use crate::input::keyboard::KeySC;

/// Trait for handling Window
pub trait Window {
    fn press(key: KeySC) -> Result<u32>;
}
