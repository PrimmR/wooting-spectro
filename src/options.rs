use crate::themes::ThemeChoice;
use crate::Result;

use std::fs::{File, OpenOptions};
use std::path::Path;

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
    pub fn read_from_file(path: impl AsRef<Path>) -> Self {
        Self::read_result(path).unwrap_or_default()
    }

    fn read_result(path: impl AsRef<Path>) -> Result<Self> {
        let f = File::open(path)?;
        let reader = std::io::BufReader::new(f);

        Ok(serde_json::from_reader(reader)?)
    }

    pub fn write_to_file(&self, path: impl AsRef<Path>) {
        let f = OpenOptions::new().create(true).write(true).truncate(true).open(path).unwrap();
        serde_json::to_writer_pretty(f, self).unwrap()
    }
}
