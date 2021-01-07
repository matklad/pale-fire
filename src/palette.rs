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

pub(crate) struct LightnessLevel(pub(crate) u32);

impl From<LightnessLevel> for f32 {
    fn from(lightness_level: LightnessLevel) -> Self {
        match lightness_level.0 {
            0 => 0.65,
            1 => 0.75,
            2 => 0.8,
            3 => 0.85,
            4 => 0.9,
            _ => unreachable!(),
        }
    }
}

pub(crate) enum LightnessLevelPreset {
    TerminalAnsi,
    TerminalAnsiBright,
    DiffFg,
    DiffBg,
    Gutter,
    OverviewRuler,
    GitDecoration,
    Minimap,
}

impl From<LightnessLevelPreset> for LightnessLevel {
    fn from(lightness_level_preset: LightnessLevelPreset) -> Self {
        Self(match lightness_level_preset {
            LightnessLevelPreset::TerminalAnsi => 1,
            LightnessLevelPreset::TerminalAnsiBright => 3,
            LightnessLevelPreset::DiffFg => 3,
            LightnessLevelPreset::DiffBg => 0,
            LightnessLevelPreset::Gutter => 1,
            LightnessLevelPreset::OverviewRuler => 2,
            LightnessLevelPreset::GitDecoration => 3,
            LightnessLevelPreset::Minimap => 2,
        })
    }
}

impl From<LightnessLevelPreset> for f32 {
    fn from(lightness_level_preset: LightnessLevelPreset) -> Self {
        let lightness_level = LightnessLevel::from(lightness_level_preset);
        lightness_level.into()
    }
}

const COLOR_CHROMA: f32 = 0.065;

macro_rules! def_color_fn {
    ($name:ident, hue: $hue:literal) => {
        pub(crate) fn $name(lightness_level: impl Into<LightnessLevel>) -> Oklch {
            oklch(f32::from(lightness_level.into()), COLOR_CHROMA, $hue)
        }
    };
}

def_color_fn!(red, hue: 19.0);
def_color_fn!(orange, hue: 55.0);
def_color_fn!(yellow, hue: 91.0);
def_color_fn!(green, hue: 145.0);
def_color_fn!(cyan, hue: 200.0);

pub(crate) fn blue(lightness_level: impl Into<LightnessLevel>) -> Oklch {
    let lightness_level = lightness_level.into();

    let chroma = if lightness_level.0 == 4 {
        COLOR_CHROMA * 0.7
    } else {
        COLOR_CHROMA
    };

    oklch(f32::from(lightness_level), chroma, 243.0)
}

fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h),
    }
}
