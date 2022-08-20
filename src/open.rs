use {
    crate::{ParseHandednessError, Specs, Variant},
    std::{
        error, f64, fmt, io,
        num::{ParseFloatError, ParseIntError},
        path,
    },
    svg::{node::element::tag, parser::Event},
};

#[derive(Debug)]
pub enum Error {
    /// Error reading the svg file
    Io(io::Error),
    /// Error parsing a float type from the file's metadata
    ParseFloat(ParseFloatError),
    /// Error parsing an integer type from the file's metadata
    ParseInt(ParseIntError),
    /// Error parsing the neck's handedness from the file's metadata
    ParseHandedness,
    /// The file does not contain a Description element
    NoMetadata,
    /// The file's description is missing a metadata field
    MissingField(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{e}"),
            Self::ParseFloat(e) => write!(f, "{e}"),
            Self::ParseInt(e) => write!(f, "{e}"),
            Self::ParseHandedness => write!(f, "Parse handedness error"),
            Self::NoMetadata => write!(f, "No metadata"),
            Self::MissingField(s) => write!(f, "Missing field: {s}"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::ParseFloat(e) => Some(e),
            Self::ParseInt(e) => Some(e),
            Self::ParseHandedness => Some(&ParseHandednessError),
            Self::NoMetadata | Self::MissingField(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ParseFloatError> for Error {
    fn from(e: ParseFloatError) -> Self {
        Self::ParseFloat(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}

impl From<ParseHandednessError> for Error {
    fn from(_: ParseHandednessError) -> Self {
        Self::ParseHandedness
    }
}

/// Opens an svg file and extracts a Specs struct from it if it was created
/// by this library previously
/// # Errors
/// See `OpenError` for a list of potential errors
pub fn open<T: AsRef<path::Path>>(path: T) -> Result<Specs, Error> {
    let mut content = String::new();
    let event_iter = svg::open(path, &mut content)?;
    for event in event_iter {
        if let Event::Tag(tag::Description, _, attributes) = event {
            let scale = attributes
                .get("Scale")
                .ok_or(Error::MissingField("Scale"))?
                .parse()?;
            let bridge = attributes
                .get("BridgeSpacing")
                .ok_or(Error::MissingField("BridgeSpacing"))?
                .parse()?;
            let nut = attributes
                .get("NutWidth")
                .ok_or(Error::MissingField("NutWidth"))?
                .parse()?;
            let count = attributes
                .get("FretCount")
                .ok_or(Error::MissingField("FretCount"))?
                .parse()?;
            let variant = match attributes.get("ScaleTreble") {
                Some(scl) => Variant::Multiscale {
                    scale: scl.parse::<f64>()?,
                    handedness: attributes
                        .get("Handedness")
                        .ok_or(Error::MissingField("Handedness"))?
                        .parse()?,
                    pfret: attributes
                        .get("PerpendicularFret")
                        .ok_or(Error::MissingField("PerpendicularFret"))?
                        .parse()?,
                },
                None => Variant::Monoscale,
            };
            return Ok(Specs::init(scale, count, variant, nut, bridge));
        }
    }
    Err(Error::NoMetadata)
}

#[cfg(test)]
mod tests {
    use {super::*, crate::Handedness};

    #[test]
    fn test_open() {
        let specs = open("src/test.svg").unwrap();
        assert_eq!(specs.scale, 648.0);
        assert_eq!(specs.variant.scale(), Some(610.0));
        assert_eq!(specs.variant.pfret(), Some(8.5));
        assert_eq!(specs.variant.handedness(), Some(Handedness::Right));
        assert_eq!(specs.bridge, 56.0);
        assert_eq!(specs.nut, 43.0);
        assert_eq!(specs.count, 24);
    }
}
