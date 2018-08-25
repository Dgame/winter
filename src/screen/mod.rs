pub mod buffer;
pub mod line;
pub mod manager;
pub mod screen;

pub use self::buffer::Buffer;
pub use self::line::Line;
pub use self::manager::ScreenManager;
pub use self::screen::Screen;

pub trait CursorDel {
    fn del_left(&mut self);
    fn del_right(&mut self);
}

trait CursorShift {
    fn shift_left(&mut self);
    fn shift_right(&mut self);
}
