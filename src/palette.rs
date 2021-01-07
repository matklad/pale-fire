#![allow(clippy::declare_interior_mutable_const, unused_attributes)]

use once_cell::unsync::Lazy;
use tincture::{Hue, Oklch};

// Colors are from https://github.com/bbatsov/zenburn_emacs/blob/master/zenburn_theme.el

const FG_CHROMA: f32 = 0.022;
const FG_HUE: f32 = 107.0;
pub(crate) const ZENBURN_FG: Lazy<Oklch> = Lazy::new(|| oklch(0.8901145, FG_CHROMA, FG_HUE));
pub(crate) const ZENBURN_FG_PLUS_1: Lazy<Oklch> = Lazy::new(|| oklch(0.9957194, FG_CHROMA, FG_HUE));

pub(crate) const ZENBURN_BG_MINUS_1: Lazy<Oklch> = Lazy::new(|| oklch(0.28908, 0.0, 0.0));
pub(crate) const ZENBURN_BG_MINUS_05: Lazy<Oklch> = Lazy::new(|| oklch(0.34069705, 0.0, 0.0));
pub(crate) const ZENBURN_BG: Lazy<Oklch> = Lazy::new(|| oklch(0.36768097, 0.0, 0.0));
pub(crate) const ZENBURN_BG_PLUS_05: Lazy<Oklch> = Lazy::new(|| oklch(0.4053975, 0.0, 0.0));
pub(crate) const ZENBURN_BG_PLUS_1: Lazy<Oklch> = Lazy::new(|| oklch(0.42760777, 0.0, 0.0));
pub(crate) const ZENBURN_BG_PLUS_2: Lazy<Oklch> = Lazy::new(|| oklch(0.4854972, 0.0, 0.0));
pub(crate) const ZENBURN_BG_PLUS_3: Lazy<Oklch> = Lazy::new(|| oklch(0.5417056, 0.0, 0.0));

pub(crate) struct ColorLightness(u32);

impl From<ColorLightness> for f32 {
    fn from(lightness: ColorLightness) -> Self {
        match lightness.0 {
            0 => 0.65,
            1 => 0.75,
            2 => 0.8,
            3 => 0.85,
            4 => 0.9,
            _ => unreachable!(),
        }
    }
}

impl From<u32> for ColorLightness {
    fn from(lightness: u32) -> Self {
        assert!((0..=4).contains(&lightness));
        Self(lightness)
    }
}

pub(crate) enum ColorLightnessPreset {
    TerminalAnsi,
    TerminalAnsiBright,
    DiffFg,
    DiffBg,
    Gutter,
    OverviewRuler,
    GitDecoration,
    Minimap,
}

impl From<ColorLightnessPreset> for ColorLightness {
    fn from(preset: ColorLightnessPreset) -> Self {
        Self(match preset {
            ColorLightnessPreset::TerminalAnsi => 1,
            ColorLightnessPreset::TerminalAnsiBright => 3,
            ColorLightnessPreset::DiffFg => 3,
            ColorLightnessPreset::DiffBg => 0,
            ColorLightnessPreset::Gutter => 1,
            ColorLightnessPreset::OverviewRuler => 2,
            ColorLightnessPreset::GitDecoration => 3,
            ColorLightnessPreset::Minimap => 2,
        })
    }
}

impl From<ColorLightnessPreset> for f32 {
    fn from(preset: ColorLightnessPreset) -> Self {
        let lightness = ColorLightness::from(preset);
        lightness.into()
    }
}

const COLOR_CHROMA: f32 = 0.065;

macro_rules! def_color_fn {
    ($name:ident, hue: $hue:literal) => {
        pub(crate) fn $name(lightness: impl Into<ColorLightness>) -> Oklch {
            oklch(f32::from(lightness.into()), COLOR_CHROMA, $hue)
        }
    };
}

def_color_fn!(red, hue: 19.0);
def_color_fn!(orange, hue: 55.0);
def_color_fn!(yellow, hue: 91.0);
def_color_fn!(green, hue: 145.0);
def_color_fn!(cyan, hue: 200.0);

pub(crate) fn blue(lightness: impl Into<ColorLightness>) -> Oklch {
    let lightness = lightness.into();

    let chroma = if lightness.0 == 4 {
        COLOR_CHROMA * 0.7
    } else {
        COLOR_CHROMA
    };

    oklch(f32::from(lightness), chroma, 243.0)
}

fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h),
    }
}
