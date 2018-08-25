use cli::Cell;
use std::slice::Iter;

#[derive(Clone, PartialEq, Eq)]
pub struct Text {
    vec: Vec<Cell>,
}

impl Text {
    pub fn iter(&self) -> Iter<Cell> {
        self.vec.iter()
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

impl<'a> From<&'a str> for Text {
    fn from(s: &'a str) -> Self {
        Text {
            vec: s.chars().map(|ch| ch.into()).collect(),
        }
    }
}

impl Into<String> for Text {
    fn into(self) -> String {
        self.vec.iter().map(|cell| cell.ch).collect()
    }
}
