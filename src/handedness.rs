use {
    serde::{Deserialize, Serialize},
    std::{error::Error, fmt, str::FromStr},
};

/// Whether the output represents a right handed or left handed neck style
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Handedness {
    #[default]
    Right,
    Left,
}

#[derive(Debug)]
pub struct ParseHandednessError;

impl fmt::Display for ParseHandednessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse Handedness Error")
    }
}

impl Error for ParseHandednessError {}

impl FromStr for Handedness {
    type Err = ParseHandednessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "right" | "Right" => Ok(Self::Right),
            "left" | "Left" => Ok(Self::Left),
            _ => Err(ParseHandednessError),
        }
    }
}

impl fmt::Display for Handedness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Right => "right",
                Self::Left => "left",
            }
        )
    }
}
