mod controll;

use awn_core::input::keyboard::KeySC;
use awn_core::window::inner::WindowInner;
use awn_core::{error::Result, window::Window};
use controll::Controll;
use snafu::ErrorCompat;
use std::time;

const TITLE: &str = "Granblue Fantasy: Relink";

fn run() -> Result<()> {
    let window = WindowInner::new(TITLE.to_string())?;
    let window = Controll::new(window)?;

    window.make_foreground()?;

    loop {
        window.press(&[KeySC::KC_SPACE][..], time::Duration::from_micros(250))?;
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        if let Some(bt) = ErrorCompat::backtrace(&e) {
            eprintln!("{}", bt);
        }
    }
}
