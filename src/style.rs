use std::fmt;

pub(crate) struct Style {
    color: Option<Color>,
    font_style: Option<FontStyle>,
}

impl<C: Into<Color>> From<C> for Style {
    fn from(color: C) -> Self {
        Self {
            color: Some(color.into()),
            font_style: None,
        }
    }
}

impl From<FontStyle> for Style {
    fn from(font_style: FontStyle) -> Self {
        Self {
            color: None,
            font_style: Some(font_style),
        }
    }
}

impl<C: Into<Color>> From<(C, FontStyle)> for Style {
    fn from((color, font_style): (C, FontStyle)) -> Self {
        Self {
            color: Some(color.into()),
            font_style: Some(font_style),
        }
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.font_style.is_some() || f.alternate() {
            writeln!(f, "{{")?;

            if let Some(ref color) = self.color {
                write!(f, "\t\t\t\"foreground\": {}", color)?;
            }

            if let Some(ref font_style) = self.font_style {
                if self.color.is_some() {
                    writeln!(f, ",")?;
                }

                writeln!(f, "\t\t\t\"fontStyle\": {}", font_style)?;
            } else {
                writeln!(f)?;
            }

            write!(f, "\t\t}}")?;
        } else {
            // Style cannot be created without a color.
            write!(f, "{}", self.color.as_ref().unwrap())?;
        }

        Ok(())
    }
}

pub(crate) enum FontStyle {
    Bold,
    Italic,
    Underline,
}

impl fmt::Display for FontStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bold => write!(f, "\"bold\""),
            Self::Italic => write!(f, "\"italic\""),
            Self::Underline => write!(f, "\"underline\""),
        }
    }
}

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
