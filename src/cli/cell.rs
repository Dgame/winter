use cli::Color;

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
        Self::from(DEFAULT_CH)
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
        self.fg.fg_attributes() | self.bg.bg_attributes()
    }

    pub fn is_empty(&self) -> bool {
        self.ch == DEFAULT_CH
    }
}

impl From<char> for Cell {
    fn from(ch: char) -> Self {
        Cell::new(ch, DEFAULT_FG, DEFAULT_BG)
    }
}

impl Into<char> for Cell {
    fn into(self) -> char {
        self.ch
    }
}
