
use std::io;
use std::fs::File;
use std::io::Read;
use std::fmt;
use std::fmt::Formatter;
use std::error;

use serde_yaml;

use core;

pub fn read_config_from_path(path: &str) -> Result<core::config::Config, ConfigError> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let config: core::config::Config = serde_yaml::from_str(&buffer)?;

    info!("{:?}", config);

    Ok(config)
}

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Serde(serde_yaml::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ConfigError::Io(ref err) => write!(f, "IO error: {}", err),
            ConfigError::Serde(ref err) => write!(f, "serde yaml error: {}", err),
        }
    }
}

impl error::Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::Io(ref err) => err.description(),
            ConfigError::Serde(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ConfigError::Io(ref err) => Some(err),
            ConfigError::Serde(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        ConfigError::Io(e)
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(e: serde_yaml::Error) -> Self {
        ConfigError::Serde(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn read_config_from_path_test() {
        let path = "config/collector.yaml";
        match read_config_from_path(path) {
            Ok(config) => println!("config: {:?}", config),
            Err(err) => println!("error: {}" , err),
        }
    }
}