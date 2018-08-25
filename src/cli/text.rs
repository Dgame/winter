use cli::Cell;

#[derive(Clone, PartialEq, Eq)]
pub struct Text {
    vec: Vec<Cell>
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

impl<'a> From<&'a str> for Text {
    fn from(s: &'a str) -> Self {
        Text {
            vec: s.chars().map(|ch| ch.into()).collect()
        }
    }
}