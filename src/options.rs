use crate::themes::ThemeChoice;
use crate::Result;
use crate::OPTIONS_FILE;

use std::fs::{create_dir, File, OpenOptions};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActiveDevice {
    #[default]
    Default,
    Named(String),
}

impl ActiveDevice {
    pub fn from_string(s: String) -> Self {
        if s == "Default" {
            Self::Default
        } else {
            Self::Named(s)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Options {
    pub device: ActiveDevice,
    pub theme: ThemeChoice,
    pub caps_active: bool,
}

impl std::default::Default for Options {
    fn default() -> Self {
        Self { caps_active: true, device: Default::default(), theme: Default::default() }
    }
}

impl Options {
    #[allow(unused_mut)]
    fn get_options_path() -> PathBuf {
        let mut path = PathBuf::from("./").join(OPTIONS_FILE);

        #[cfg(feature = "cli")]
        if let Some(p) = dirs::config_dir() {
            path = p.join(env!("CARGO_PKG_NAME")).join(OPTIONS_FILE)
        }

        path
    }

    pub fn read_from_file() -> Self {
        println!("{:?}",Self::get_options_path());
        Self::read_result(Self::get_options_path()).unwrap_or_default()
    }

    fn read_result(path: impl AsRef<Path>) -> Result<Self> {
        let f = File::open(path)?;
        let reader = std::io::BufReader::new(f);

        Ok(serde_json::from_reader(reader)?)
    }

    pub fn write_to_file(&self) {
        let path = Self::get_options_path();
        let dir = path.parent().unwrap();
        if !dir.exists() {
            create_dir(dir).unwrap()
        }

        let f = OpenOptions::new().create(true).write(true).truncate(true).open(path).unwrap();
        serde_json::to_writer_pretty(f, self).unwrap()
    }
}
