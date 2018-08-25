use basic::{Coord, Size};
use cli::input::{Button, Control, Key};
use enumflags::BitFlags;
use num::FromPrimitive;
use winapi::um::wincon::{INPUT_RECORD, KEY_EVENT};

#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub is_pressed: bool,
    pub repeat_count: u16,
    pub code: Key,
    pub control: BitFlags<Control>,
}

#[derive(Debug, Clone)]
pub struct MouseButtonEvent {
    pub button: Button,
    pub is_double_click: bool,
    pub pos: Coord,
}

#[derive(Debug, Clone)]
pub struct MouseMotionEvent {
    pub button: Button,
    pub pos: Coord,
}

#[derive(Debug, Clone)]
pub struct MouseWheelEvent {
    pub scroll: Coord,
}

#[derive(Debug, Clone)]
pub enum MouseEvent {
    Button(MouseButtonEvent),
    Motion(MouseMotionEvent),
    Wheel(MouseWheelEvent),
}

#[derive(Debug, Clone)]
pub struct WindowEvent {
    pub size: Size,
}

#[derive(Debug, Clone)]
pub enum Event {
    Unknown,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Window(WindowEvent),
}

impl Into<Event> for INPUT_RECORD {
    fn into(self) -> Event {
        match self.EventType {
            KEY_EVENT => {
                Event::Key(KeyEvent {
                    is_pressed: unsafe { self.Event.KeyEvent().bKeyDown == 1 },
                    code: FromPrimitive::from_u16(unsafe { self.Event.KeyEvent().wVirtualKeyCode })
                        .unwrap_or(Key::Unknown),
                    control: Control::interpret(unsafe { self.Event.KeyEvent().dwControlKeyState }),
                    repeat_count: unsafe { self.Event.KeyEvent().wRepeatCount },
                })
            }
            _ => Event::Unknown
        }
    }
}

impl ToString for KeyEvent {
    fn to_string(&self) -> String {
        self.code.to_string(self.control)
    }
}
