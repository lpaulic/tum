use clap::Parser;
use std::fmt;
use std::path::PathBuf;

use crate::configuration::{Configuration, ConfigurationError};
use crate::monitor::{ResourceMonitor, ResourceMonitorError};
use crate::mqtt_client::MqttClient;

const DEFAULT_CONFIGURATION_PATH_STR: &str = "/etc/tum/configuration.yml";

#[derive(Debug, Parser)]
#[command(name = "tum")]
#[command(bin_name = "tum")]
#[command(author, version, about, long_about = None)]
pub struct TumArgs {
    #[arg(short, long, default_value = DEFAULT_CONFIGURATION_PATH_STR)]
    configuration_path: String,
}

pub struct Tum {
    resource_monitor: ResourceMonitor,
}

#[derive(Debug)]
pub enum TumError {
    ParseArguments(clap::Error),
    SignalRegistration(ctrlc::Error),
    Configuration(ConfigurationError),
    ResourceMonitor(ResourceMonitorError),
}

impl From<clap::Error> for TumError {
    fn from(item: clap::Error) -> TumError {
        TumError::ParseArguments(item)
    }
}

impl From<ctrlc::Error> for TumError {
    fn from(item: ctrlc::Error) -> TumError {
        TumError::SignalRegistration(item)
    }
}

impl From<ConfigurationError> for TumError {
    fn from(item: ConfigurationError) -> TumError {
        TumError::Configuration(item)
    }
}

impl From<ResourceMonitorError> for TumError {
    fn from(item: ResourceMonitorError) -> TumError {
        TumError::ResourceMonitor(item)
    }
}

impl fmt::Display for TumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TumError::ParseArguments(ref err) => write!(f, "{}", err),
            TumError::SignalRegistration(ref err) => write!(f, "{}", err),
            TumError::Configuration(ref err) => write!(f, "{}", err),
            TumError::ResourceMonitor(ref err) => write!(f, "{}", err),
        }
    }
}

impl Tum {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Tum, TumError> {
        let tum_args = TumArgs::try_parse_from(args)?;
        let tum_config = Configuration::from_file(&PathBuf::from(&tum_args.configuration_path))?;
        let tum_mqtt_client = MqttClient::new(
            &tum_config.server_addr,
            tum_config.server_port,
            &tum_config.username,
            &tum_config.password,
        );
        let resource_monitor = ResourceMonitor::new(tum_config.monitoring_rate_s, tum_mqtt_client);

        Ok(Tum { resource_monitor })
    }

    pub fn run(&self) -> Result<(), TumError> {
        self.resource_monitor.start()?;
        Ok(())
    }

    pub fn halt(&self) -> Result<(), TumError> {
        self.resource_monitor.stop()?;
        Ok(())
    }
}
