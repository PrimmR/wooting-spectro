mod cols;
mod freq_weight;
mod keyboard;
mod spectrum_memory;
pub mod themes;
mod traits;

pub use cols::*;
pub use freq_weight::*;
pub use keyboard::*;
pub use spectrum_memory::*;
pub use traits::*;

// pub type Rgb = palette::rgb::Rgb<palette::Srgb, u8>;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;