use cli::Cell;
use std::borrow::Cow;
use std::ops::Deref;

#[derive(Clone, PartialEq, Eq)]
pub struct Text {
    vec: Vec<Cell>,
}

impl Deref for Text {
    type Target = Vec<Cell>;

    fn deref(&self) -> &Vec<Cell> {
        &self.vec
    }
}

impl<'a> From<Cow<'a, str>> for Text {
    fn from(s: Cow<'a, str>) -> Self {
        s.to_string().into()
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
