use color::Color;
use color::{into_bg_attributes, into_fg_attributes};

pub const DEFAULT_CH: char = ' ';

static DEFAULT_FG: Color = Color::DarkWhite;
static DEFAULT_BG: Color = Color::DarkBlack;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub fg: Color,
    pub bg: Color,
    pub ch: char,
}

impl Cell {
    pub fn default() -> Self {
        Self::plain(DEFAULT_CH)
    }

    pub fn plain(ch: char) -> Self {
        Self {
            fg: DEFAULT_FG,
            bg: DEFAULT_BG,
            ch,
        }
    }

    pub fn border() -> Self {
        let mut cell = Self::default();
        cell.bg = Color::DarkWhite;

        cell
    }

    pub fn new(ch: char, fg: Color, bg: Color) -> Self {
        Self { fg, bg, ch }
    }

    pub fn get_color_attributes(&self) -> u16 {
        into_fg_attributes(self.fg) | into_bg_attributes(self.bg)
    }

    pub fn is_empty(&self) -> bool {
        self.ch == DEFAULT_CH
    }
}
