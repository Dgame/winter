use color::{into_fg_attributes, into_bg_attributes};
use color::Color;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub fg: Color,
    pub bg: Color,
    pub ch: char,
}

impl Cell {
    pub fn default() -> Self {
        Self::plain(' ')
    }

    pub fn new(ch: char, fg: Color, bg: Color) -> Self {
        Self { fg, bg, ch }
    }

    pub fn plain(ch: char) -> Self {
        Self {
            fg: Color::DarkWhite,
            bg: Color::DarkBlack,
            ch,
        }
    }

    pub fn border() -> Self {
        Self::new('~', Color::DarkWhite, Color::Black)
    }

    pub fn get_color_attributes(&self) -> u16 {
        into_fg_attributes(self.fg) | into_bg_attributes(self.bg)
    }
}
