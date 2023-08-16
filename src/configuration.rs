use serde::Deserialize;
use std::fmt;
use std::path::PathBuf;

// TODO: think about making attributes public
#[derive(Debug, PartialEq, Deserialize)]
pub struct Configuration {
    pub username: String,
    pub password: String,
    pub server_addr: String,
    pub server_port: u16,
    pub monitoring_rate_s: u64,
}

#[derive(Debug)]
pub enum ConfigurationError {
    FileOperation(std::io::Error),
    ParsingConfiguration(serde_yaml::Error),
}

impl From<std::io::Error> for ConfigurationError {
    fn from(item: std::io::Error) -> ConfigurationError {
        ConfigurationError::FileOperation(item)
    }
}

impl From<serde_yaml::Error> for ConfigurationError {
    fn from(item: serde_yaml::Error) -> ConfigurationError {
        ConfigurationError::ParsingConfiguration(item)
    }
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigurationError::FileOperation(ref err) => write!(f, "{}", err),
            ConfigurationError::ParsingConfiguration(ref err) => write!(f, "{}", err),
        }
    }
}

impl Configuration {
    pub fn from_file(configuration_path: &PathBuf) -> Result<Configuration, ConfigurationError> {
        let configuration_file = std::fs::File::open(configuration_path)?;
        let configuration = serde_yaml::from_reader(configuration_file)?;

        Ok(configuration)
    }
}
