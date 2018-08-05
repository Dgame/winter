use color::Color;
use color::{into_bg_attributes, into_fg_attributes};

const DEFAULT_CH: char = ' ';
const BORDER_CH: char = '~';

static DEFAULT_FG: Color = Color::DarkWhite;
static DEFAULT_BG: Color = Color::DarkBlack;

static BORDER_FG: Color = Color::DarkWhite;
static BORDER_BG: Color = Color::Black;

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
        Self::new(BORDER_CH, BORDER_FG, BORDER_BG)
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

    pub fn is_border(&self) -> bool {
        self.ch == BORDER_CH && self.fg == BORDER_FG && self.bg == BORDER_BG
    }
}
