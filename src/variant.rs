use {
    crate::Handedness,
    serde::{Deserialize, Serialize},
};

/// Whether to output a traditional `Monoscale` style neck with the same scale
/// across it's entire width, or a modern `Multiscale` neck, with a shorter scale
/// along the treble side, also known as *fan fret*.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Variant {
    /// A traditional fretbaord where the same scale length is used all of the
    /// way across the fretboard.
    Monoscale,
    /// A modern style of neck where there is a longer scale length along the
    /// bass side of the neck and a shorter scale along the treble side of the
    /// neck, allowing for more natural string tension, greater flexibility in
    /// tuning, and better ergonomics.
    Multiscale {
        /// The scale length along the treble side of the neck
        scale: f64,
        /// Right or left handed output
        handedness: Handedness,
        /// Which fret is perpendicular to the centerline
        pfret: f64,
    },
}

impl Default for Variant {
    fn default() -> Self {
        Self::Monoscale
    }
}

impl Variant {
    pub fn multi() -> Self {
        Self::Multiscale {
            scale: 610.0,
            handedness: Handedness::default(),
            pfret: 8.0,
        }
    }

    /// Return the treble side scale length if the neck is `Multiscale`, or else
    /// `None`
    pub fn scale(&self) -> Option<f64> {
        match self {
            Self::Monoscale => None,
            Self::Multiscale { scale: x, .. } => Some(*x),
        }
    }

    /// Returns whether the resulting neck is right or left handed, or `None` if
    /// the neck is `Monoscale`
    pub fn handedness(&self) -> Option<Handedness> {
        match self {
            Self::Monoscale => None,
            Self::Multiscale { handedness: x, .. } => Some(*x),
        }
    }

    /// Returns which fret is perpendicular to the centerline, or `None` if the
    /// fretboard is Monoscale
    pub fn pfret(&self) -> Option<f64> {
        match self {
            Self::Monoscale => None,
            Self::Multiscale { pfret: x, .. } => Some(*x),
        }
    }
}

/// A builder pattern struct for building a `Variant::Multiscale` struct
pub struct MultiscaleBuilder {
    scale: f64,
    handedness: Handedness,
    pfret: f64,
}

impl Default for MultiscaleBuilder {
    fn default() -> Self {
        Self {
            scale: 610.0,
            handedness: Handedness::default(),
            pfret: 8.0,
        }
    }
}

impl MultiscaleBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    #[must_use]
    pub fn handedness(mut self, handedness: Handedness) -> Self {
        self.handedness = handedness;
        self
    }

    #[must_use]
    pub fn pfret(mut self, pfret: f64) -> Self {
        self.pfret = pfret;
        self
    }

    #[must_use]
    pub fn build(self) -> Variant {
        Variant::Multiscale {
            scale: self.scale,
            handedness: self.handedness,
            pfret: self.pfret,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_default() {
        let var = Variant::default();
        assert_eq!(Variant::Monoscale, var);
    }

    #[test]
    fn variant_value() {
        let var = Variant::Multiscale {
            scale: 23.5,
            handedness: Handedness::Right,
            pfret: 8.0,
        };
        let val = var.scale();
        assert_eq!(val.unwrap(), 23.5);
        let hand = var.handedness();
        assert_eq!(hand.unwrap(), Handedness::Right);
    }
}
