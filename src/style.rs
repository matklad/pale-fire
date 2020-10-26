use std::fmt;

pub(crate) struct Color {
    rgb: Rgb,
    alpha: Option<u8>,
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        Self { rgb, alpha: None }
    }
}

impl From<(Rgb, u8)> for Color {
    fn from((rgb, alpha): (Rgb, u8)) -> Self {
        Self {
            rgb,
            alpha: Some(alpha),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(alpha) = self.alpha {
            write!(f, "\"#{:06X}{:02X}\"", self.rgb.0, alpha)
        } else {
            write!(f, "\"#{:06X}\"", self.rgb.0)
        }
    }
}

pub(crate) struct Rgb(pub(crate) u32);

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"#{:06X}\"", self.0)
    }
}
