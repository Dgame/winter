use cli::Cell;
use std::borrow::Cow;
use std::ops::Deref;

#[derive(Clone, PartialEq, Eq)]
pub struct Display {
    vec: Vec<Cell>,
}

impl Display {
    pub fn error<S: ToString>(s: S) -> Self {
        Self {
            vec: s.to_string().chars().map(|ch| Cell::error(ch)).collect(),
        }
    }

    pub fn success<S: ToString>(s: S) -> Self {
        Self {
            vec: s.to_string().chars().map(|ch| Cell::success(ch)).collect(),
        }
    }
}

impl Deref for Display {
    type Target = Vec<Cell>;

    fn deref(&self) -> &Vec<Cell> {
        &self.vec
    }
}

impl<'a> From<Cow<'a, str>> for Display {
    fn from(s: Cow<'a, str>) -> Self {
        s.to_string().into()
    }
}

impl From<String> for Display {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

impl<'a> From<&'a str> for Display {
    fn from(s: &'a str) -> Self {
        Display {
            vec: s.chars().map(|ch| ch.into()).collect(),
        }
    }
}

impl Into<String> for Display {
    fn into(self) -> String {
        self.vec.iter().map(|cell| cell.ch).collect()
    }
}
