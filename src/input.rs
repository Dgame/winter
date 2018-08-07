use enumflags::BitFlags;
use num;
use winapi::um::wincon::{
    CAPSLOCK_ON, ENHANCED_KEY, INPUT_RECORD, KEY_EVENT_RECORD, LEFT_ALT_PRESSED, LEFT_CTRL_PRESSED,
    NUMLOCK_ON, RIGHT_ALT_PRESSED, RIGHT_CTRL_PRESSED, SCROLLLOCK_ON, SHIFT_PRESSED,
};

#[derive(Debug, FromPrimitive)]
pub enum Key {
    Unknown = 0,
    Num0 = 0x30,
    Num1 = 0x31,
    Num2 = 0x32,
    Num3 = 0x33,
    Num4 = 0x34,
    Num5 = 0x35,
    Num6 = 0x36,
    Num7 = 0x37,
    Num8 = 0x38,
    Num9 = 0x39,
    A = 0x41,
    B = 0x42,
    C = 0x43,
    D = 0x44,
    E = 0x45,
    F = 0x46,
    G = 0x47,
    H = 0x48,
    I = 0x49,
    J = 0x4A,
    K = 0x4B,
    L = 0x4C,
    M = 0x4D,
    N = 0x4E,
    O = 0x4F,
    P = 0x50,
    Q = 0x51,
    R = 0x52,
    S = 0x53,
    T = 0x54,
    U = 0x55,
    V = 0x56,
    W = 0x57,
    X = 0x58,
    Y = 0x59,
    Z = 0x5A,
    Back = 0x08,
    Tab = 0x09,
    Clear = 0x0C,
    Return = 0x0D,
    Escape = 0x1B,
    Space = 0x20,
    Prior = 0x21,
    Next = 0x22,
    End = 0x23,
    Home = 0x24,
    Left = 0x25,
    Up = 0x26,
    Right = 0x27,
    Down = 0x28,
    Select = 0x29,
    Print = 0x2A,
    Execute = 0x2B,
    Snapshot = 0x2C,
    Insert = 0x2D,
    Delete = 0x2E,
    Help = 0x2F,
    Numpad0 = 0x60,
    Numpad1 = 0x61,
    Numpad2 = 0x62,
    Numpad3 = 0x63,
    Numpad4 = 0x64,
    Numpad5 = 0x65,
    Numpad6 = 0x66,
    Numpad7 = 0x67,
    Numpad8 = 0x68,
    Numpad9 = 0x69,
    Multiply = 0x6A,
    Add = 0x6B,
    Separator = 0x6C,
    Subtract = 0x6D,
    Decimal = 0x6E,
    Divide = 0x6F,
    F1 = 0x70,
    F2 = 0x71,
    F3 = 0x72,
    F4 = 0x73,
    F5 = 0x74,
    F6 = 0x75,
    F7 = 0x76,
    F8 = 0x77,
    F9 = 0x78,
    F10 = 0x79,
    F11 = 0x7A,
    F12 = 0x7B,
    Shift = 0x10,
    Control = 0x11,
    Alt = 0x12,
    Capslock = 0x14,
    LWin = 0x5B,
    RWin = 0x5C,
    Apps = 0x5D,
    Numlock = 0x90,
    Scroll = 0x91,
    LShift = 0xA0,
    RShift = 0xA1,
    LControL = 0xA2,
    RControl = 0xA3,
    LMenu = 0xA4,
    RMenu = 0xA5,
}

#[derive(EnumFlags, Copy, Clone, Debug)]
#[repr(u16)]
pub enum Control {
    Capslock = 1,
    Numlock = 2,
    Scrolllock = 4,
    Shift = 8,
    Enhanced = 16,
    LAlt = 32,
    RAlt = 64,
    LCtrl = 128,
    RCtrl = 256,
}

pub struct InputEvent {
    pub is_pressed: bool,
    pub key: Key,
    pub control: BitFlags<Control>,
}

impl InputEvent {
    pub fn from(record: &INPUT_RECORD) -> Self {
        Self {
            is_pressed: unsafe { record.Event.KeyEvent().bKeyDown == 1 },
            key: num::FromPrimitive::from_u16(unsafe { record.Event.KeyEvent().wVirtualKeyCode })
                .unwrap_or(Key::Unknown),
            control: Self::get_control(unsafe { *record.Event.KeyEvent() }),
        }
    }

    fn get_control(record: KEY_EVENT_RECORD) -> BitFlags<Control> {
        let mut control = BitFlags::empty();

        let state = record.dwControlKeyState;

        if (state & CAPSLOCK_ON) != 0 {
            control |= Control::Capslock;
        }

        if (state & SCROLLLOCK_ON) != 0 {
            control |= Control::Scrolllock;
        }

        if (state & SHIFT_PRESSED) != 0 {
            control |= Control::Shift;
        }

        if (state & ENHANCED_KEY) != 0 {
            control |= Control::Enhanced;
        }

        if (state & LEFT_ALT_PRESSED) != 0 {
            control |= Control::LAlt;
        }

        if (state & RIGHT_ALT_PRESSED) != 0 {
            control |= Control::RAlt;
        }

        if (state & LEFT_CTRL_PRESSED) != 0 {
            control |= Control::LCtrl;
        }

        if (state & RIGHT_CTRL_PRESSED) != 0 {
            control |= Control::RCtrl;
        }

        if (state & NUMLOCK_ON) != 0 {
            control |= Control::Numlock;
        }

        control
    }
}
