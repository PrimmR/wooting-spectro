use crate::*;
use themes::Theme;

use std::mem::ManuallyDrop;
use std::result::Result;

use wooting_rgb::{Key, RgbKeyboard};
use wooting_rgb_sys::WOOTING_USB_META;
use wooting_rgb_sys::{
    WOOTING_DEVICE_TYPE_DEVICE_KEYBOARD, WOOTING_DEVICE_TYPE_DEVICE_KEYBOARD_60,
    WOOTING_DEVICE_TYPE_DEVICE_KEYBOARD_TKL, WOOTING_DEVICE_TYPE_DEVICE_KEYPAD_3KEY,
};

/// Coordinate system for the keyboard where (0,0) is L-Ctrl
/// The ordering is (x, y)
struct KeyboardCoord(u8, u8);

impl KeyboardCoord {
    pub fn to_absolute(&self, rows: u8, top_row: u8) -> (u8, u8) {
        let row = rows - 1 - self.1 + top_row;
        let coord = (self.0, row);
        // Correction for shift key
        if coord == (13, 4) {
            (12, 4)
        } else if coord == (12, 4) {
            (13, 4)
        } else {
            coord
        }
    }

    pub fn to_absolute_transposed(&self, rows: u8, top_row: u8) -> (u8, u8) {
        let abs = self.to_absolute(rows, top_row);
        (abs.1, abs.0)
    }
}

pub enum WootingDeviceType {
    Keyboard,
    Keyboard60,
    KeypadThreeKey,
    KeyboardTKL,
}

#[allow(non_snake_case)]
impl TryFrom<i32> for WootingDeviceType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            WOOTING_DEVICE_TYPE_DEVICE_KEYBOARD => Self::Keyboard,
            WOOTING_DEVICE_TYPE_DEVICE_KEYBOARD_60 => Self::Keyboard60,
            WOOTING_DEVICE_TYPE_DEVICE_KEYPAD_3KEY => Self::KeypadThreeKey,
            WOOTING_DEVICE_TYPE_DEVICE_KEYBOARD_TKL => Self::KeyboardTKL,
            _ => return Err(()),
        })
    }
}

// ManuallyDrop okay here because RgbKeyboard is a ZST
pub struct Keyboard {
    rgb_keys: ManuallyDrop<RgbKeyboard>,
    theme: Box<dyn Theme>,
    max_row: u8,
    first_row: u8,
    cols: u8,
}

impl Keyboard {
    // Management methods
    pub fn new(rgb_keys: RgbKeyboard, theme: Box<dyn Theme>, max_row: u8, first_row: u8, cols: u8) -> Self {
        Self { rgb_keys: ManuallyDrop::new(rgb_keys), theme, first_row, max_row, cols }
    }

    pub fn from_meta_and_theme(meta: WOOTING_USB_META, theme: Box<dyn Theme>) -> Self {
        let first_row = match WootingDeviceType::try_from(meta.device_type) {
            Ok(WootingDeviceType::Keyboard60) => 1,
            Ok(_) => 0,
            Err(_) => panic!("Unknown device type"),
        };
        Self::new(RgbKeyboard, theme, meta.max_rows, first_row, meta.max_columns)
    }

    #[must_use = "Contained RgbKeyboard needs to be manually dropped"]
    pub fn close(self) -> ManuallyDrop<RgbKeyboard> {
        self.rgb_keys
    }

    pub fn set_theme(&mut self, theme: Box<dyn Theme>) {
        self.theme = theme
    }

    // Draw methods

    pub fn display(&mut self, spec_mem: &SpectrumMemory, show_caps: bool) {
        for (col, (_freq, weight)) in spec_mem.weights.iter().enumerate() {
            self.draw_column(col as u8, *weight)
        }

        if show_caps && toggle_keys::get_caps_lock_state() {
            self.rgb_keys.array_set_single(Key::CapsLock, 255, 255, 255);
        }

        self.rgb_keys.array_update();
    }

    pub fn set_point(&mut self, row: u8, col: u8, rgb: Rgb) {
        let coord = self.rearrange_coord(col, row);
        self.rgb_keys.array_set_single(coord, rgb.0, rgb.1, rgb.2);
    }

    fn draw_column(&mut self, col: u8, db: f32) {
        let height = db - MIN_DB;
        let height_keys = height / self.get_db_step();

        for row in 0..self.rows() {
            let rgb = self.get_color(col, row, height_keys);

            self.set_point(row, col, rgb)
        }
    }

    fn get_color(&self, col: u8, row: u8, bar_height: f32) -> Rgb {
        self.theme.get_led_color(self, col, row, bar_height)
    }

    // Attribute methods
    pub fn cols(&self) -> u8 {
        self.cols
    }

    pub fn rows(&self) -> u8 {
        self.max_row - self.first_row
    }

    pub fn get_db_step(&self) -> f32 {
        (MAX_DB - MIN_DB) / self.rows() as f32
    }

    pub fn rearrange_coord(&self, col: u8, row: u8) -> (u8, u8) {
        KeyboardCoord(col, row).to_absolute_transposed(self.rows(), self.first_row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abs_coord_0() {
        let c = KeyboardCoord(1, 3);
        assert_eq!(c.to_absolute(6, 0), (1, 2))
    }

    #[test]
    fn abs_coord_1() {
        let c = KeyboardCoord(1, 3);
        assert_eq!(c.to_absolute(5, 1), (1, 2))
    }

    #[test]
    fn abs_coord_2() {
        let c = KeyboardCoord(0, 0);
        assert_eq!(c.to_absolute(5, 1), (0, 5))
    }

    #[test]
    fn abs_coord_t() {
        let c = KeyboardCoord(1, 2);
        assert_eq!(c.to_absolute_transposed(5, 1), (3, 1))
    }
}
