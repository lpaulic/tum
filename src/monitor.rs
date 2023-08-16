use crate::mqtt_client::{MqttClient, MqttClientError};
use crate::resource::Resource;
use gethostname::gethostname;
use std::any::Any;
use std::boxed::Box;
use std::fmt;
use std::marker::Send;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub enum ResourceMonitorError {
    ResourceSynchronization,
    TransmissionSynchronization,
    ExecutionSynchronization,
    StringConversion,
    Serialization,
    Transmission(MqttClientError),
}

impl fmt::Display for ResourceMonitorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ResourceMonitorError::ResourceSynchronization => {
                write!(f, "Synchronization issues for 'resource' attribute.")
            }
            ResourceMonitorError::TransmissionSynchronization => {
                write!(f, "Synchronization issues for 'transmitter' attribute.")
            }
            ResourceMonitorError::ExecutionSynchronization => {
                write!(f, "Synchronization issues for 'execution' attribute.")
            }
            ResourceMonitorError::StringConversion => {
                write!(f, "Failed to convert data to string.")
            }
            ResourceMonitorError::Serialization => {
                write!(f, "Failed to serialize resource data.")
            }
            ResourceMonitorError::Transmission(ref err) => write!(f, "MqttClient error: {}", err),
        }
    }
}

impl From<MqttClientError> for ResourceMonitorError {
    fn from(item: MqttClientError) -> ResourceMonitorError {
        ResourceMonitorError::Transmission(item)
    }
}

impl From<Box<dyn Any + Send>> for ResourceMonitorError {
    fn from(item: Box<dyn Any + Send>) -> ResourceMonitorError {
        item.into()
    }
}

pub struct ResourceMonitor {
    sampling_rate_s: u64,
    resource: Arc<Mutex<Resource>>,
    transmitter: Arc<Mutex<MqttClient>>,
    stop_monitoring: Arc<Mutex<bool>>,
}

impl ResourceMonitor {
    pub fn new(sampling_rate_s: u64, transmitter: MqttClient) -> ResourceMonitor {
        ResourceMonitor {
            sampling_rate_s: if sampling_rate_s > 0 {
                sampling_rate_s
            } else {
                5
            },
            resource: Arc::new(Mutex::new(Resource::new())),
            transmitter: Arc::new(Mutex::new(transmitter)),
            stop_monitoring: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&self) -> Result<(), ResourceMonitorError> {
        let resource = Arc::clone(&self.resource);
        let transmitter = Arc::clone(&self.transmitter);
        let stop_monitoring = Arc::clone(&self.stop_monitoring);
        let sampling_rate_s = self.sampling_rate_s;

        let handler = thread::spawn(move || -> Result<(), ResourceMonitorError> {
            loop {
                match resource.lock() {
                    Ok(mut res) => res.refresh(),
                    Err(_) => return Err(ResourceMonitorError::ResourceSynchronization),
                }

                let topic = match gethostname().to_str() {
                    Some(hostname) => format!("device/{}/system/stats", hostname),
                    None => return Err(ResourceMonitorError::StringConversion),
                };

                let data = match serde_json::to_vec(resource.as_ref()) {
                    Ok(data) => data,
                    Err(_) => return Err(ResourceMonitorError::Serialization),
                };

                match transmitter.lock() {
                    Ok(mut tx) => tx.publish(&topic, data.as_slice())?,
                    Err(_) => return Err(ResourceMonitorError::TransmissionSynchronization),
                };

                match stop_monitoring.lock() {
                    Ok(stop_monitoring) => {
                        if *stop_monitoring {
                            break;
                        }
                    }
                    Err(_) => return Err(ResourceMonitorError::ExecutionSynchronization),
                }

                thread::sleep(std::time::Duration::from_secs(sampling_rate_s));
            }
            Ok(())
        });

        handler.join()?
    }

    pub fn stop(&self) -> Result<(), ResourceMonitorError> {
        match self.stop_monitoring.lock() {
            Ok(mut stop_monitoring) => *stop_monitoring = true,
            Err(_) => return Err(ResourceMonitorError::ExecutionSynchronization),
        }

        Ok(())
    }
}
