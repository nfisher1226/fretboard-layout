use {
    crate::Variant,
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize)]
// This struct contains multiplication factors used to convert the raw lengths
// from bridge to fret into x,y coordinates. It also contains an offset distance
// used to correctly orient the two scales in a multiscale design so that the
// desired fret is perpendicular to the centerline.
pub struct Factors {
    pub x_ratio: f64,
    pub y_ratio: f64,
    // How far forward the treble side of the bridge should start with respect
    // to the trable side
    pub treble_offset: f64,
}

impl Default for Factors {
    fn default() -> Self {
        Self::init(655.0, &Variant::default(), 43.0, 56.0)
    }
}

impl Factors {
    /// Uses trigonometry to place the fret ends, based on visualizing their
    /// locations as a triangle where the hypotenuse is the string, and the
    /// opposite is the distance from the bridge parallel to the centerline.
    pub fn init(scale: f64, variant: &Variant, nut: f64, bridge: f64) -> Self {
        let height = (bridge - nut) / 2.0;
        let y_ratio = height / scale;
        let x_ratio = y_ratio.acos().sin();
        let pfret = variant.pfret().unwrap_or(8.0);
        let factor = 2.0_f64.powf(pfret / 12.0);
        let length_bass = scale / factor;
        let length_treble = match variant {
            Variant::Monoscale => length_bass,
            Variant::Multiscale { scale: s, .. } => s / factor,
        };
        let bass_pfret = x_ratio * length_bass;
        let treble_pfret = x_ratio * length_treble;
        let treble_offset = bass_pfret - treble_pfret;
        Self {
            x_ratio,
            y_ratio,
            treble_offset,
        }
    }
}

