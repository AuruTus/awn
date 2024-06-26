use snafu::ensure;
use windows::Win32::{
    UI::Input::KeyboardAndMouse::GetKeyState,
    UI::Input::KeyboardAndMouse::MapVirtualKeyW,
    UI::Input::KeyboardAndMouse::SendInput,
    UI::Input::KeyboardAndMouse::INPUT,
    UI::Input::KeyboardAndMouse::INPUT_0,
    UI::Input::KeyboardAndMouse::INPUT_KEYBOARD,
    UI::Input::KeyboardAndMouse::KEYBDINPUT,
    UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS,
    UI::Input::KeyboardAndMouse::MAPVK_VK_TO_VSC,
    UI::Input::KeyboardAndMouse::VIRTUAL_KEY,
    UI::Input::KeyboardAndMouse::{KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE},
};

use crate::error as awn_error;
use crate::error::Result;

const CBSIZE: usize = std::mem::size_of::<INPUT>();

#[allow(non_snake_case)]
fn new_kbd_input(
    wVk: u16,
    wScan: u16,
    dwFlags: KEYBD_EVENT_FLAGS,
    time: u32,
    dwExtraInfo: usize,
) -> KEYBDINPUT {
    KEYBDINPUT {
        wVk: VIRTUAL_KEY(wVk),
        wScan,
        dwFlags,
        time,
        dwExtraInfo,
    }
}

fn new_input(ki: KEYBDINPUT) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 { ki },
    }
}

fn get_num_lock_state() -> i16 {
    unsafe { GetKeyState(0x90) }
}

/// Keyboard scan code mappings.
///
/// !NOTE: arrow key scancodes (`up`, `down`, `left`, `right`) can be different on the hardware.
///     To use them, look up their scancodes with [`MapVirtualKeyA`][windows::Win32::UI::Input::KeyboardAndMouse::MapVirtualKeyA]
///     or [`MapVirtualKeyW`][windows::Win32::UI::Input::KeyboardAndMouse::MapVirtualKeyW].
///     See [Microsoft Win32 Doc][https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapvirtualkeya?redirectedfrom=MSDN] for more details.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeySC {
    KC_ESC = 0x01,
    KC_F1 = 0x3b,
    KC_F2 = 0x3c,
    KC_F3 = 0x3d,
    KC_F4 = 0x3e,
    KC_F5 = 0x3f,
    KC_F6 = 0x40,
    KC_F7 = 0x41,
    KC_F8 = 0x42,
    KC_F9 = 0x43,
    KC_F10 = 0x44,
    KC_F11 = 0x57,
    KC_F12 = 0x58,
    KC_PRINTSCREEN = 0xb7,
    KC_SCROLLLOCK = 0x46,
    KC_PAUSE = 0xc5,
    /// Key `
    KC_BACKTICK = 0x29,
    KC_1 = 0x02,
    KC_2 = 0x03,
    KC_3 = 0x04,
    KC_4 = 0x05,
    KC_5 = 0x06,
    KC_6 = 0x07,
    KC_7 = 0x08,
    KC_8 = 0x09,
    KC_9 = 0x0a,
    KC_0 = 0x0b,
    /// Key -
    KC_HYPHEN = 0x0c,
    /// Key =
    KC_EQUAL = 0x0d,
    KC_BACKSPACE = 0x0e,
    KC_INSERT = 0x4d2,
    KC_HOME = 0x4c7,
    KC_PAGEUP = 0x4c9,
    KC_PAGEDOWN = 0x4d1,
    KC_NUMLOCK = 0x45,
    KC_DIVIDE = 0x4b5,
    KC_MULTIPLY = 0x37,
    KC_SUBTRACT = 0x4a,
    KC_ADD = 0x4e,
    KC_DECIMAL = 0x53,
    KC_NUMPADENTER = 0x49c,
    KC_NUMPAD1 = 0x4f,
    KC_NUMPAD2 = 0x50,
    KC_NUMPAD3 = 0x51,
    KC_NUMPAD4 = 0x4b,
    KC_NUMPAD5 = 0x4c,
    KC_NUMPAD6 = 0x4d,
    KC_NUMPAD7 = 0x47,
    KC_NUMPAD8 = 0x48,
    KC_NUMPAD9 = 0x49,
    KC_NUMPAD0 = 0x52,
    KC_TAB = 0x0f,
    KC_Q = 0x10,
    KC_W = 0x11,
    KC_E = 0x12,
    KC_R = 0x13,
    KC_T = 0x14,
    KC_Y = 0x15,
    KC_U = 0x16,
    KC_I = 0x17,
    KC_O = 0x18,
    KC_P = 0x19,
    /// Key [
    KC_LEFT_BRACKET = 0x1a,
    /// Key ]
    KC_RIGHT_BRACKET = 0x1b,
    /// Key \
    KC_BACKSLASH = 0x2b,
    KC_DEL = 0x4d3,
    KC_END = 0x4cf,
    KC_CAPSLOCK = 0x3a,
    KC_A = 0x1e,
    KC_S = 0x1f,
    KC_D = 0x20,
    KC_F = 0x21,
    KC_G = 0x22,
    KC_H = 0x23,
    KC_J = 0x24,
    KC_K = 0x25,
    KC_L = 0x26,
    /// Key ;
    KC_SEMICOLON = 0x27,
    /// Key '
    KC_APOSTROPHE = 0x28,
    KC_ENTER = 0x1c,
    /// Key shift left
    KC_SHIFT = 0x2a,
    KC_Z = 0x2c,
    KC_X = 0x2d,
    KC_C = 0x2e,
    KC_V = 0x2f,
    KC_B = 0x30,
    KC_N = 0x31,
    KC_M = 0x32,
    /// Key ,
    KC_COMMA = 0x33,
    /// Key .
    KC_FULLSTOP = 0x34,
    /// Key /
    KC_SLASH = 0x35,
    KC_SHIFTRIGHT = 0x36,
    /// Key ctrl left
    KC_CTRL = 0x1d,
    /// Key win left
    KC_WIN = 0x4db,
    /// Key alt left
    KC_ALT = 0x38,
    KC_SPACE = 0x39,
    KC_ALTRIGHT = 0x4b8,
    KC_WINRIGHT = 0x4dc,
    KC_APPS = 0x4dd,
    KC_CTRLRIGHT = 0x49d,

    /// prefix to expand scancode sequences
    KC_PREFIX = 0xe0,

    /// Diffrent on hardwares, should not directly use them
    /// Every 8 bits are the Virtual Key Codes, use them to
    /// look up the win32 mapping.
    KC_UP = 0x2626,
    KC_DOWN = 0x2525,
    KC_LEFT = 0x2828,
    KC_RIGHT = 0x2727,
}

impl KeySC {
    fn arrow_vsc(self) -> u32 {
        unsafe { MapVirtualKeyW(self as u32 >> 8, MAPVK_VK_TO_VSC) }
    }

    pub fn is_arrow(self) -> bool {
        matches!(
            self,
            Self::KC_UP | Self::KC_DOWN | Self::KC_RIGHT | Self::KC_LEFT
        )
    }

    fn send_prefix(dw_flags: KEYBD_EVENT_FLAGS) -> Result<u32> {
        let input = new_input(new_kbd_input(
            0,
            Into::<u32>::into(Self::KC_PREFIX) as u16,
            dw_flags,
            0,
            0,
        ));
        match unsafe { SendInput(&[input][..], CBSIZE as i32) } {
            0 => awn_error::KeyDownSendFailedSnafu {
                key: stringify!("{:?}", Self::KC_PREFIX).to_owned(),
            }
            .fail(),
            event => Ok(event),
        }
    }

    #[cfg(feature = "foreground")]
    pub fn keydown(self) -> Result<u32> {
        let mut dw_flags = KEYEVENTF_SCANCODE;
        let mut expected_events = 1;
        let mut inserted_events = 0;

        if self.is_arrow() {
            dw_flags |= KEYEVENTF_EXTENDEDKEY;
            // if numlock is on, an additional scancode needs to be sent for arrow key
            // see refs below:
            // https://stackoverflow.com/questions/14026496/sendinput-sends-num8-when-i-want-to-send-vk-up-how-come
            // https://handmade.network/wiki/2823-keyboard_inputs_-_scancodes,_raw_input,_text_input,_key_names
            if get_num_lock_state() != 0 {
                expected_events = 2;
                inserted_events += Self::send_prefix(KEYEVENTF_SCANCODE)?;
            }
        }

        let input = new_input(new_kbd_input(
            0,
            Into::<u32>::into(self) as u16,
            dw_flags,
            0,
            0,
        ));
        inserted_events += unsafe { SendInput(&[input][..], CBSIZE as i32) };
        ensure!(
            expected_events == inserted_events,
            awn_error::KeyDownSendFailedSnafu {
                key: stringify!("{:?}", self).to_owned()
            }
        );
        Ok(inserted_events)
    }

    #[cfg(feature = "foreground")]
    pub fn keyup(self) -> Result<u32> {
        let mut dw_flags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;
        let mut expected_events = 1;
        let mut inserted_events = 0;

        if self.is_arrow() {
            dw_flags |= KEYEVENTF_EXTENDEDKEY;
        }

        let input = new_input(new_kbd_input(
            0,
            Into::<u32>::into(self) as u16,
            dw_flags,
            0,
            0,
        ));
        inserted_events += match unsafe { SendInput(&[input][..], CBSIZE as i32) } {
            0 => awn_error::KeyUpSendFailedSnafu {
                key: stringify!("{:?}", self).to_owned(),
            }
            .fail(),
            event => Ok(event),
        }?;

        if self.is_arrow() && get_num_lock_state() != 0 {
            // if numlock is on, an additional scancode needs to be sent for arrow key
            expected_events = 2;
            inserted_events += Self::send_prefix(KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP)?;
        }

        ensure!(
            expected_events == inserted_events,
            awn_error::KeyUpSendFailedSnafu {
                key: stringify!("{:?}", self).to_owned()
            }
        );
        Ok(inserted_events)
    }
}

#[allow(clippy::from_over_into)]
impl Into<u32> for KeySC {
    fn into(self) -> u32 {
        if self.is_arrow() {
            self.arrow_vsc()
        } else {
            self as u32
        }
    }
}
