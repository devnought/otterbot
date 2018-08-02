use serde_json;
use std::{fs::File, io, path::Path};

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    port: u32,
}

impl Config {
    pub fn load<P>(path: P) -> Result<Self, LoadConfigError>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn port(&self) -> u32 {
        self.port
    }
}

#[derive(Debug)]
pub enum LoadConfigError {
    Io(io::Error),
    Deserialize(serde_json::Error),
}

impl From<io::Error> for LoadConfigError {
    fn from(error: io::Error) -> Self {
        LoadConfigError::Io(error)
    }
}

impl From<serde_json::Error> for LoadConfigError {
    fn from(error: serde_json::Error) -> Self {
        LoadConfigError::Deserialize(error)
    }
}
