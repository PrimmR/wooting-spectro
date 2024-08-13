pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub const fn from_hex(hex: u32) -> Self {
        Rgb(
            (hex >> 16 | 0b11) as u8,
            (hex >> 8 | 0b11) as u8,
            (hex | 0b11) as u8,
        )
    }

    pub fn intensify(&self, intensity: f32) -> Self {
       let intensity = intensity.clamp(0., 1.);
        Self(
            (self.0 as f32 * intensity) as u8,
            (self.1 as f32 * intensity) as u8,
            (self.2 as f32 * intensity) as u8,
        )
    }

    pub fn interpolate(&self, other:& Self, factor: f32) -> Self {
       let factor = factor.clamp(0., 1.);

        let a = self.to_f32_tuple();
        let b = other.to_f32_tuple();
        Self::from_f32(
            (b.0 - a.0) * factor + a.0,
            (b.1 - a.1) * factor + a.1,
            (b.2 - a.2) * factor + a.2,
        )
    }

    fn to_f32_tuple(&self) -> (f32, f32, f32) {
        (self.0.into(), self.1.into(), self.2.into())
    }

    fn from_f32(r: f32, g: f32, b: f32) -> Self {
        Self(r as u8, g as u8, b as u8)
    }
}
