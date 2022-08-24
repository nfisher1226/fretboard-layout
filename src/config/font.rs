#![warn(clippy::all, clippy::pedantic)]

use std::{error::Error, fmt, str::FromStr};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The style of the font
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Style {
    Normal,
    Oblique,
    Italic,
}

impl Default for Style {
    fn default() -> Self {
        Self::Normal
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Style {
    type Err = ParseFontError;

    #[allow(clippy::must_use_candidate)]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "Style::Normal" | "Style::normal" => Ok(Self::Normal),
            "Style::Oblique" | "Style::oblique" => Ok(Self::Oblique),
            "Style::Italic" | "Style::italic" => Ok(Self::Italic),
            _ => Err(ParseFontError),
        }
    }
}

#[cfg(feature = "pango")]
impl From<pango::Style> for Style {
    fn from(style: pango::Style) -> Self {
        match style {
            pango::Style::Oblique => Self::Oblique,
            pango::Style::Italic => Self::Italic,
            _ => Self::Normal,
        }
    }
}

#[cfg(feature = "pango")]
impl From<Style> for pango::Style {
    fn from(style: Style) -> Self {
        match style {
            Style::Normal => pango::Style::Normal,
            Style::Oblique => pango::Style::Oblique,
            Style::Italic => pango::Style::Italic,
        }
    }
}

impl Style {
    pub fn css_value(&self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Oblique => "Oblique",
            Self::Italic => "Italic",
        }
    }
}

/// The weight of the font
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Weight {
    Thin,
    Ultralight,
    Light,
    Semilight,
    Book,
    Normal,
    Medium,
    Semibold,
    Bold,
    Ultrabold,
    Heavy,
    Ultraheavy,
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for Weight {
    fn default() -> Self {
        Self::Normal
    }
}

impl FromStr for Weight {
    type Err = ParseFontError;

    #[allow(clippy::must_use_candidate)]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "Weight::Thin" | "Weight::thin" => Ok(Self::Thin),
            "Weight::Ultralight" | "Weight::ultralight" => Ok(Self::Ultralight),
            "Weight::Light" | "Weight::light" => Ok(Self::Light),
            "Weight::Semilight" | "Weight::semilight" => Ok(Self::Semilight),
            "Weight::Book" | "Weight::book" => Ok(Self::Book),
            "Weight::Normal" | "Weight::normal" | "Weight::Regular" | "Weight::regular" => {
                Ok(Self::Normal)
            }
            "Weight::Medium" | "Weight::medium" => Ok(Self::Medium),
            "Weight::Semibold" | "Weight::semibold" => Ok(Self::Semibold),
            "Weight::Bold" | "Weight::bold" => Ok(Self::Bold),
            "Weight::Ultrabold" | "Weight::ultrabold" => Ok(Self::Ultrabold),
            "Weight::Heavy" | "Weight::heavy" => Ok(Self::Heavy),
            "Weight::Ultraheavy" | "Weight::ultraheavy" => Ok(Self::Ultraheavy),
            _ => Err(ParseFontError),
        }
    }
}

#[cfg(feature = "pango")]
impl From<pango::Weight> for Weight {
    fn from(weight: pango::Weight) -> Self {
        match weight {
            pango::Weight::Thin => Self::Thin,
            pango::Weight::Ultralight => Self::Ultralight,
            pango::Weight::Light => Self::Light,
            pango::Weight::Semilight => Self::Semilight,
            pango::Weight::Book => Self::Book,
            pango::Weight::Medium => Self::Medium,
            pango::Weight::Semibold => Self::Semibold,
            pango::Weight::Bold => Self::Bold,
            pango::Weight::Ultrabold => Self::Ultrabold,
            pango::Weight::Heavy => Self::Heavy,
            pango::Weight::Ultraheavy => Self::Ultraheavy,
            _ => Self::Normal,
        }
    }
}

#[cfg(feature = "pango")]
impl From<Weight> for pango::Weight {
    fn from(weight: Weight) -> Self {
        match weight {
            Weight::Thin => Self::Thin,
            Weight::Ultralight => Self::Ultralight,
            Weight::Light => Self::Light,
            Weight::Semilight => Self::Semilight,
            Weight::Book => Self::Book,
            Weight::Normal => Self::Normal,
            Weight::Medium => Self::Medium,
            Weight::Semibold => Self::Semibold,
            Weight::Bold => Self::Bold,
            Weight::Ultrabold => Self::Ultrabold,
            Weight::Heavy => Self::Heavy,
            Weight::Ultraheavy => Self::Ultraheavy,
        }
    }
}

impl Weight {
    pub fn css_value(&self) -> &'static str {
        match self {
            Self::Thin => "100",
            Self::Ultralight => "200",
            Self::Light => "300",
            Self::Semilight => "350",
            Self::Book | Self::Normal => "400",
            Self::Medium => "500",
            Self::Semibold => "600",
            Self::Bold => "700",
            Self::Ultrabold => "800",
            Self::Heavy => "900",
            Self::Ultraheavy => "950",
        }
    }
}

/// The stretch of the font
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Stretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl fmt::Display for Stretch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for Stretch {
    fn default() -> Self {
        Self::Normal
    }
}

impl FromStr for Stretch {
    type Err = ParseFontError;

    #[allow(clippy::must_use_candidate)]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "Stretch::UltraCondensed" => Ok(Self::UltraCondensed),
            "Stretch::ExtraCondensed" => Ok(Self::ExtraCondensed),
            "Stretch::Condensed" => Ok(Self::Condensed),
            "Stretch::SemiCondensed" => Ok(Self::SemiCondensed),
            "Stretch::Normal" => Ok(Self::Normal),
            "Stretch::SemiExpanded" => Ok(Self::SemiExpanded),
            "Stretch::Expanded" => Ok(Self::Expanded),
            "Stretch::ExtraExpanded" => Ok(Self::ExtraExpanded),
            "Stretch::UltraExpanded" => Ok(Self::UltraExpanded),
            _ => Err(ParseFontError),
        }
    }
}

#[cfg(feature = "pango")]
impl From<pango::Stretch> for Stretch {
    fn from(stretch: pango::Stretch) -> Self {
        match stretch {
            pango::Stretch::UltraCondensed => Self::UltraCondensed,
            pango::Stretch::ExtraCondensed => Self::ExtraCondensed,
            pango::Stretch::Condensed => Self::Condensed,
            pango::Stretch::SemiCondensed => Self::SemiCondensed,
            pango::Stretch::SemiExpanded => Self::SemiExpanded,
            pango::Stretch::Expanded => Self::Expanded,
            pango::Stretch::ExtraExpanded => Self::ExtraExpanded,
            pango::Stretch::UltraExpanded => Self::UltraExpanded,
            _ => Self::Normal,
        }
    }
}

#[cfg(feature = "pango")]
impl From<Stretch> for pango::Stretch {
    fn from(stretch: Stretch) -> Self {
        match stretch {
            Stretch::UltraCondensed => Self::UltraCondensed,
            Stretch::ExtraCondensed => Self::ExtraCondensed,
            Stretch::Condensed => Self::Condensed,
            Stretch::SemiCondensed => Self::SemiCondensed,
            Stretch::Normal => Self::Normal,
            Stretch::SemiExpanded => Self::SemiExpanded,
            Stretch::Expanded => Self::Expanded,
            Stretch::ExtraExpanded => Self::ExtraExpanded,
            Stretch::UltraExpanded => Self::UltraExpanded,
        }
    }
}

impl Stretch {
    pub fn css_value(&self) -> &'static str {
        match self {
            Self::UltraCondensed => "ultra-condensed",
            Self::ExtraCondensed => "extra-condensed",
            Self::Condensed => "condensed",
            Self::SemiCondensed => "semi-condensed",
            Self::Normal => "normal",
            Self::SemiExpanded => "semi-expanded",
            Self::Expanded => "expanded",
            Self::ExtraExpanded => "extra-expanded",
            Self::UltraExpanded => "ultra-expanded",
        }
    }
}

pub struct FontBuilder {
    family: String,
    style: Style,
    weight: Weight,
    stretch: Stretch,
    size: i32,
}

impl Default for FontBuilder {
    fn default() -> Self {
        Self {
            family: String::from("Sans"),
            style: Style::default(),
            weight: Weight::default(),
            stretch: Stretch::default(),
            size: 12288,
        }
    }
}

impl FontBuilder {
    pub fn family(mut self, family: &str) -> Self {
        self.family = String::from(family);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn weight(mut self, weight: Weight) -> Self {
        self.weight = weight;
        self
    }

    pub fn stretch(mut self, stretch: Stretch) -> Self {
        self.stretch = stretch;
        self
    }

    pub fn size(mut self, size: i32) -> Self {
        self.size = size;
        self
    }

    pub fn build(self) -> Font {
        Font {
            family: self.family,
            style: self.style,
            weight: self.weight,
            stretch: self.stretch,
            size: self.size,
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Font {
    family: String,
    style: Style,
    weight: Weight,
    stretch: Stretch,
    size: i32,
}

/// Error returned if unable to parse a font from a given `str`
#[derive(Debug, Eq, PartialEq)]
pub struct ParseFontError;

impl fmt::Display for ParseFontError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseFontError {}

impl Default for Font {
    /// Returns "Sans Normal"
    fn default() -> Self {
        Self {
            family: String::from("Sans"),
            style: Style::default(),
            weight: Weight::default(),
            stretch: Stretch::default(),
            size: 12288,
        }
    }
}

impl Font {
    pub fn builder() -> FontBuilder {
        FontBuilder::default()
    }

    /// Get the *family* of the font
    #[must_use]
    pub fn family(&self) -> String {
        String::from(&self.family)
    }

    /// Set the *family* of the font
    pub fn set_family(&mut self, family: String) {
        self.family = family;
    }

    /// Get the *style* of the font
    #[must_use]
    pub fn style(&self) -> Style {
        self.style
    }

    /// Set the *style* or *style* of the font
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    /// Get the *weight* of the font
    #[must_use]
    pub fn weight(&self) -> Weight {
        self.weight
    }

    /// Set the *weight* of the font
    pub fn set_weight(&mut self, weight: Weight) {
        self.weight = weight;
    }

    /// Get the *size* of the font
    #[must_use]
    pub fn size(&self) -> i32 {
        self.size
    }

    /// Set the *size* of the font
    pub fn set_size(&mut self, size: i32) {
        self.size = size;
    }

    pub fn stretch(&self) -> Stretch {
        self.stretch
    }

    pub fn set_stretch(&mut self, stretch: Stretch) {
        self.stretch = stretch;
    }
}

#[cfg(feature = "pango")]
impl From<pango::FontDescription> for Font {
    fn from(font: pango::FontDescription) -> Self {
        Self {
            family: match font.family() {
                Some(f) => f.to_string(),
                None => "Sans".to_string(),
            },
            style: font.style().into(),
            weight: font.weight().into(),
            stretch: font.stretch().into(),
            size: font.size(),
        }
    }
}

#[cfg(feature = "pango")]
impl From<&Font> for pango::FontDescription {
    fn from(input: &Font) -> Self {
        let mut font = pango::FontDescription::new();
        font.set_family(&input.family);
        font.set_style(input.style.into());
        font.set_weight(input.weight.into());
        font.set_stretch(input.stretch.into());
        font.set_size(input.size);
        font
    }
}
