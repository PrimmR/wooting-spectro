use crate::Keyboard;
use crate::Rgb;

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString};

// const WHITE: Rgb = Rgb::from_hex(0xffffff);
const BLACK: Rgb = Rgb::from_hex(0x000000);

#[inline]
fn get_key_intensity(bar_height: impl Into<f32>, row: impl Into<f32>) -> f32 {
    (bar_height.into() - row.into()).clamp(0., 1.)
}

#[derive(
    Debug, Default, Serialize, Deserialize, Copy, Clone, PartialEq, Display, EnumString, AsRefStr, EnumIter,
)]
pub enum ThemeChoice {
    #[default]
    Classic,
    Grape,
    Copper,
    Citric,
    Blossom,
    Rainbow,
    Fire,
}

impl ThemeChoice {
    pub fn get_theme(&self) -> Box<dyn Theme> {
        match self {
            ThemeChoice::Classic => Box::new(ClassicTheme {}),
            ThemeChoice::Grape => Box::new(GrapeTheme {}),
            ThemeChoice::Copper => Box::new(CopperTheme {}),
            ThemeChoice::Citric => Box::new(CitricTheme {}),
            ThemeChoice::Blossom => Box::new(BlossomTheme {}),
            ThemeChoice::Rainbow => Box::new(RainbowTheme {}),
            ThemeChoice::Fire => Box::new(FireTheme {}),
        }
    }
}

pub trait Theme {
    fn get_led_color(&self, kbd: &Keyboard, cur_col: u8, cur_row: u8, bar_height: f32) -> Rgb;
}

#[derive(Debug, Default)]
pub struct ClassicTheme {}

impl Theme for ClassicTheme {
    fn get_led_color(&self, kbd: &Keyboard, _col: u8, row: u8, bar_h: f32) -> Rgb {
        const RED: Rgb = Rgb::from_hex(0xff0000);
        const YELLOW: Rgb = Rgb::from_hex(0xffff00);
        const GREEN: Rgb = Rgb::from_hex(0x00d000);
        const D_GREEN: Rgb = Rgb::from_hex(0x008000);

        let top_intensity = get_key_intensity(bar_h, row);

        if row == kbd.rows() - 1 {
            RED
        } else if row == kbd.rows() - 2 {
            YELLOW
        } else if row == 0 {
            D_GREEN
        } else {
            GREEN
        }
        .intensify(top_intensity)
    }
}

#[derive(Debug, Default)]
pub struct GrapeTheme {}

impl Theme for GrapeTheme {
    fn get_led_color(&self, kbd: &Keyboard, _col: u8, row: u8, bar_h: f32) -> Rgb {
        const HOT_PINK: Rgb = Rgb::from_hex(0xf61b83);
        const PINKPLE: Rgb = Rgb::from_hex(0xc015ee);
        const PURP: Rgb = Rgb::from_hex(0x8522e6);
        const VIOLET: Rgb = Rgb::from_hex(0x6424e1);
        const BLOO: Rgb = Rgb::from_hex(0x2a15ee);

        let top_intensity = get_key_intensity(bar_h, row);

        match kbd.rows().saturating_sub(row + 1) {
            0 => HOT_PINK,
            1 => PINKPLE,
            2 => PURP,
            3 => VIOLET,
            _ => BLOO,
        }
        .intensify(top_intensity)
    }
}

#[derive(Debug, Default)]
pub struct CopperTheme {}

impl Theme for CopperTheme {
    fn get_led_color(&self, _kbd: &Keyboard, _col: u8, row: u8, bar_height: f32) -> Rgb {
        const ORANGE: Rgb = Rgb::from_hex(0xff6600);
        const GREEN: Rgb = Rgb::from_hex(0x00ffaa);

        let key_intensity = get_key_intensity(bar_height, row);
        let second_blend = get_key_intensity(bar_height, row as f32 + 1.);

        if key_intensity == 1. {
            GREEN.interpolate(&ORANGE, second_blend * 2.)
        } else if key_intensity == 0. {
            BLACK
        } else {
            GREEN.intensify(key_intensity * 2.)
        }
    }
}

#[derive(Debug, Default)]
pub struct CitricTheme {}

impl Theme for CitricTheme {
    fn get_led_color(&self, _kbd: &Keyboard, col: u8, row: u8, bar_h: f32) -> Rgb {
        const ORANGE: Rgb = Rgb::from_hex(0xff570d);
        const LEMON: Rgb = Rgb::from_hex(0xeeff00);
        const LIME: Rgb = Rgb::from_hex(0x00ff0b);
        const GRAPEFRUIT: Rgb = Rgb::from_hex(0xff8080);

        let top_intensity = get_key_intensity(bar_h, row);

        let column_col = match col % 3 {
            0 => ORANGE,
            1 => LEMON,
            _ => LIME,
        };

        if top_intensity == 1. {
            column_col
        } else if top_intensity == 0. {
            BLACK
        } else {
            column_col.interpolate(&GRAPEFRUIT, top_intensity).intensify(top_intensity * 2.)
        }
    }
}

#[derive(Debug, Default)]
pub struct BlossomTheme {}

impl Theme for BlossomTheme {
    fn get_led_color(&self, kbd: &Keyboard, _col: u8, row: u8, bar_height: f32) -> Rgb {
        const PINK: Rgb = Rgb::from_hex(0xff0080);
        const LIGHT_PINK: Rgb = Rgb::from_hex(0xffe6f2);

        let top_intensity = get_key_intensity(bar_height, row);

        PINK.interpolate(&LIGHT_PINK, bar_height / kbd.rows() as f32).intensify(top_intensity)
    }
}

#[derive(Debug, Default)]
pub struct RainbowTheme {}

impl Theme for RainbowTheme {
    fn get_led_color(&self, kbd: &Keyboard, _col: u8, row: u8, bar_height: f32) -> Rgb {
        const RED: Rgb = Rgb::from_hex(0xff0000);
        const ORANGE: Rgb = Rgb::from_hex(0xffc300);
        const YELLOW: Rgb = Rgb::from_hex(0xffff00);
        const GREEN: Rgb = Rgb::from_hex(0x00ff00);
        const BLUE: Rgb = Rgb::from_hex(0x0000ff);
        const PURPLE: Rgb = Rgb::from_hex(0xff00ff);
        const PINK: Rgb = Rgb::from_hex(0xff009b);

        let fade = bar_height % 1.;
        let top_intensity = get_key_intensity(bar_height, row);

        if kbd.rows() == 5 {
            match bar_height.floor() as i16 - Into::<i16>::into(row) {
                i16::MIN..=-1 => BLACK,
                0 => RED,
                1 => RED.interpolate(&ORANGE, fade),
                2 => ORANGE.interpolate(&GREEN, fade),
                3 => GREEN.interpolate(&BLUE, fade),
                4 => BLUE.interpolate(&PURPLE, fade),
                5 => PURPLE.interpolate(&PINK, fade),
                6..=i16::MAX => PINK,
            }
        } else {
            match bar_height.floor() as i16 - Into::<i16>::into(row) {
                i16::MIN..=-1 => BLACK,
                0 => RED,
                1 => RED.interpolate(&ORANGE, fade),
                2 => ORANGE.interpolate(&YELLOW, fade),
                3 => YELLOW.interpolate(&GREEN, fade),
                4 => GREEN.interpolate(&BLUE, fade),
                5 => BLUE.interpolate(&PURPLE, fade),
                6 => PURPLE.interpolate(&PINK, fade),
                7..=i16::MAX => PINK,
            }
        }
        .intensify(top_intensity * 2.)
    }
}

#[derive(Debug, Default)]
pub struct FireTheme {}

impl Theme for FireTheme {
    fn get_led_color(&self, _kbd: &Keyboard, _col: u8, row: u8, bar_height: f32) -> Rgb {
        const RED: Rgb = Rgb::from_hex(0xff0000);
        const RORANGE: Rgb = Rgb::from_hex(0xd73502);

        const ORANGE: Rgb = Rgb::from_hex(0xfc6400);
        const YELLOW: Rgb = Rgb::from_hex(0xfac000);
        const WHITE: Rgb = Rgb::from_hex(0xffffcf);

        let fade = bar_height % 1.;
        let top_intensity = get_key_intensity(bar_height, row);

        match bar_height.floor() as i16 - Into::<i16>::into(row) {
            i16::MIN..=-1 => BLACK,
            0 => RED,
            1 => RED.interpolate(&RORANGE, fade),
            2 => RORANGE.interpolate(&ORANGE, fade),
            3 => ORANGE.interpolate(&YELLOW, fade),
            4 => YELLOW.interpolate(&WHITE, fade),
            5..=i16::MAX => WHITE,
        }
        .intensify(top_intensity * 2.)
    }
}
