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

impl Style {
    pub(crate) fn as_json_value(&self, in_textmate_rule: bool) -> json::Value {
        let mut map = json::Map::new();

        if let Some(ref color) = self.color {
            map.insert("foreground".to_string(), color.into());
        }

        if let Some(ref font_style) = self.font_style {
            if in_textmate_rule {
                map.insert("fontStyle".to_string(), font_style.into());
            } else {
                let (key, value) = match font_style {
                    FontStyle::Bold => ("bold", json::Value::Bool(true)),
                    FontStyle::Italic => ("italic", json::Value::Bool(true)),
                    FontStyle::Underline => ("underline", json::Value::Bool(true)),
                    FontStyle::Clear => ("fontStyle", json::Value::String(String::new())),
                };

                map.insert(key.to_string(), value);
            }
        }

        json::Value::Object(map)
    }
}

pub(crate) enum FontStyle {
    Bold,
    Italic,
    Underline,
    Clear,
}

impl From<&FontStyle> for json::Value {
    fn from(font_style: &FontStyle) -> Self {
        match font_style {
            FontStyle::Bold => Self::String("bold".to_string()),
            FontStyle::Italic => Self::String("italic".to_string()),
            FontStyle::Underline => Self::String("underline".to_string()),
            FontStyle::Clear => Self::String(String::new()),
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

impl From<&Color> for json::Value {
    fn from(color: &Color) -> Self {
        let hex = if let Some(alpha) = color.alpha {
            format!("#{:06X}{:02X}", color.rgb.0, alpha)
        } else {
            format!("#{:06X}", color.rgb.0)
        };

        Self::String(hex)
    }
}

pub(crate) struct Rgb(pub(crate) u32);
