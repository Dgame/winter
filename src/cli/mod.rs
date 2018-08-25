mod cell;
mod color;
mod console;
pub mod event;
mod input;
mod text;

pub use self::cell::Cell;
pub use self::color::Color;
pub use self::console::Console;
pub use self::event::Event;
pub use self::input::{Control, Key};
pub use self::text::Text;

pub trait Write {
    fn write_text<T: Into<Text>>(&mut self, text: T);
    fn write_cell(&mut self, width: usize, cell: Cell);
}
