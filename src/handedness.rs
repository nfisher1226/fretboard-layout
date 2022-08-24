use std::{error::Error, fmt, str::FromStr};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Whether the output represents a right handed or left handed neck style
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Handedness {
    #[default]
    Right,
    Left,
}

/// An error occurred parsing the neck's Handedness from a str
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
