mod cell;
mod color;
mod console;
mod display;
pub mod event;
mod input;

pub use self::cell::Cell;
pub use self::color::Color;
pub use self::console::Console;
pub use self::display::Display;
pub use self::event::Event;
pub use self::input::{Control, Key};

pub trait Write {
    fn write_text<T: Into<Display>>(&mut self, text: T);
    fn write_cell(&mut self, index: usize, cell: Cell);
}
