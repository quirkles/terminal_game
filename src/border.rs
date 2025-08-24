use std::fmt::{Display, Formatter, Result};

pub enum BorderChars {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Vertical,
    Horizontal,
}

impl Display for BorderChars {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            BorderChars::TopLeft => write!(f, "\u{250c}"),
            BorderChars::TopRight => write!(f, "\u{2510}"),
            BorderChars::BottomLeft => write!(f, "\u{2514}"),
            BorderChars::BottomRight => write!(f, "\u{2518}"),
            BorderChars::Vertical => write!(f, "\u{2502}"),
            BorderChars::Horizontal => write!(f, "\u{2500}"),
        }
    }
}
