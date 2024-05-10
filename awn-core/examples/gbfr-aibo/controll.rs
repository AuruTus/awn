use awn_core::error as awn_error;
use awn_core::error::*;
use awn_core::input::keyboard::KeySC;
use awn_core::window::Window;
use rand::prelude::*;
use snafu::ensure;
use std::{thread, time};
use windows::Win32::{
    Foundation::{BOOL, HWND},
    UI::WindowsAndMessaging::{GetForegroundWindow, SetForegroundWindow},
};

pub(crate) struct Controll<W>
where
    W: Window,
{
    inner: W,
}

impl<W> Controll<W>
where
    W: Window,
{
    pub fn new(inner: W) -> Result<Self> {
        Ok(Self { inner })
    }

    pub fn make_foreground(&self) -> Result<()> {
        unsafe {
            let hwnd = self.handle();
            match SetForegroundWindow(hwnd) {
                BOOL(0) => awn_error::WindowNotForegroundSnafu { hwnd }.fail(),
                _ => Ok(()),
            }
        }
    }
}

impl<W> Window for Controll<W>
where
    W: Window,
{
    fn handle(&self) -> HWND {
        self.inner.handle()
    }

    fn press(&self, keys: &[KeySC], hold: std::time::Duration) -> Result<u32> {
        let rand_time = rand::thread_rng().gen_range(1..=100);
        thread::sleep(time::Duration::from_millis(rand_time));

        let hwnd = self.handle();
        let fore_hwnd = unsafe { GetForegroundWindow() };
        ensure!(
            hwnd == fore_hwnd,
            awn_error::WindowNotForegroundSnafu { hwnd }
        );
        self.inner.press(keys, hold)
    }
}
