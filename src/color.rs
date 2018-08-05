use winapi::um::wincon::{
    BACKGROUND_BLUE, BACKGROUND_GREEN, BACKGROUND_INTENSITY, BACKGROUND_RED, FOREGROUND_BLUE,
    FOREGROUND_GREEN, FOREGROUND_INTENSITY, FOREGROUND_RED,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    DarkBlack,
    DarkRed,
    DarkGreen,
    DarkYellow,
    DarkBlue,
    DarkMagenta,
    DarkCyan,
    DarkWhite
}

pub fn into_fg_attributes(color: Color) -> u16 {
    match color {
        Color::Blue => FOREGROUND_INTENSITY | FOREGROUND_BLUE,
        Color::DarkBlue => FOREGROUND_BLUE,
        Color::Cyan => FOREGROUND_INTENSITY | FOREGROUND_GREEN | FOREGROUND_BLUE,
        Color::DarkCyan => FOREGROUND_GREEN | FOREGROUND_BLUE,
        Color::White => {
            FOREGROUND_INTENSITY | FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE
        }
        Color::DarkWhite => FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE,
        Color::Black => FOREGROUND_INTENSITY,
        Color::DarkBlack => 0,
        Color::Green => FOREGROUND_INTENSITY | FOREGROUND_GREEN,
        Color::DarkGreen => FOREGROUND_GREEN,
        Color::Magenta => FOREGROUND_INTENSITY | FOREGROUND_RED | FOREGROUND_BLUE,
        Color::DarkMagenta => FOREGROUND_RED | FOREGROUND_BLUE,
        Color::Red => FOREGROUND_INTENSITY | FOREGROUND_RED,
        Color::DarkRed => FOREGROUND_RED,
        Color::Yellow => FOREGROUND_INTENSITY | FOREGROUND_RED | FOREGROUND_GREEN,
        Color::DarkYellow => FOREGROUND_RED | FOREGROUND_GREEN,
    }
}

pub fn into_bg_attributes(color: Color) -> u16 {
    match color {
        Color::Blue => BACKGROUND_INTENSITY | BACKGROUND_BLUE,
        Color::DarkBlue => BACKGROUND_BLUE,
        Color::Cyan => BACKGROUND_INTENSITY | BACKGROUND_GREEN | BACKGROUND_BLUE,
        Color::DarkCyan => BACKGROUND_GREEN | BACKGROUND_BLUE,
        Color::White => {
            BACKGROUND_INTENSITY | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE
        }
        Color::DarkWhite => BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE,
        Color::Black => BACKGROUND_INTENSITY,
        Color::DarkBlack => 0,
        Color::Green => BACKGROUND_INTENSITY | BACKGROUND_GREEN,
        Color::DarkGreen => BACKGROUND_GREEN,
        Color::Magenta => BACKGROUND_INTENSITY | BACKGROUND_RED | BACKGROUND_BLUE,
        Color::DarkMagenta => BACKGROUND_RED | BACKGROUND_BLUE,
        Color::Red => BACKGROUND_INTENSITY | BACKGROUND_RED,
        Color::DarkRed => BACKGROUND_RED,
        Color::Yellow => BACKGROUND_INTENSITY | BACKGROUND_RED | BACKGROUND_GREEN,
        Color::DarkYellow => BACKGROUND_RED | BACKGROUND_GREEN,
    }
}

