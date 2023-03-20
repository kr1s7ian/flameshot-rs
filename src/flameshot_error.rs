use std::{error::Error, fmt::Display};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Ord)]
pub enum FlameshotError {
    Os(String),
    Flameshot(String),
    #[cfg(feature = "image")]
    Image(String),
}

impl Display for FlameshotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for FlameshotError {}
