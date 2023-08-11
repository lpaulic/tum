use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sysinfo::{CpuExt, NetworkExt, System, SystemExt};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
struct CPUResource {
    id: u8,
    load: f32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
struct MemoryResource {
    used_bytes: u64,
    total_bytes: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct NetworkResource {
    interface: String,
    rx_bytes: u64,
    tx_bytes: u64,
    rx_error_bytes: u64,
    tx_error_bytes: u64,
    rx_speed_bps: f64,
    tx_speed_bps: f64,
    #[serde(skip)]
    tx_bytes_old: u64,
    #[serde(skip)]
    rx_bytes_old: u64,
    #[serde(skip)]
    last_synced_timestamp: u128,
}

impl PartialEq for NetworkResource {
    fn eq(&self, other: &Self) -> bool {
        self.interface.eq(&other.interface)
            && self.rx_bytes == other.rx_bytes
            && self.tx_bytes == other.tx_bytes
    }
}

// NOTE: this approach is chosen rather than deriving Eq
//       because we use f64 in NetworkResource structure
//       but we do not use it for comparison
impl Eq for NetworkResource {}

impl PartialOrd for NetworkResource {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NetworkResource {
    fn cmp(&self, other: &Self) -> Ordering {
        self.interface.cmp(&other.interface)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Resource {
    #[serde(skip)]
    system_info: System,
    cpus: Vec<CPUResource>,
    memory: MemoryResource,
    networks: Vec<NetworkResource>,
}

impl Resource {
    pub fn new() -> Resource {
        let mut resource = Resource {
            system_info: System::new(),
            cpus: Vec::new(),
            memory: MemoryResource {
                used_bytes: 0,
                total_bytes: 0,
            },
            networks: Vec::new(),
        };

        resource.system_info.refresh_cpu();
        for i in 0..resource.system_info.cpus().len() {
            resource.cpus.push(CPUResource {
                id: i as u8,
                load: -1.0,
            });
        }

        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::new(0, 0))
            .as_micros();

        resource.system_info.refresh_networks_list();
        resource.system_info.refresh_networks();
        for (interface, _) in resource.system_info.networks() {
            resource.networks.push(NetworkResource {
                interface: interface.to_owned(),
                rx_bytes: 0,
                tx_bytes: 0,
                rx_error_bytes: 0,
                tx_error_bytes: 0,
                rx_speed_bps: 0.0,
                tx_speed_bps: 0.0,
                tx_bytes_old: 0,
                rx_bytes_old: 0,
                last_synced_timestamp: time,
            });
        }
        resource.networks.sort();

        resource
    }

    pub fn refresh(&mut self) {
        self.system_info.refresh_cpu();
        self.system_info
            .cpus()
            .iter()
            .enumerate()
            .for_each(
                |(i, x)| match self.cpus.iter_mut().find(|y| y.id == i as u8) {
                    Some(cpu) => cpu.load = x.cpu_usage(),
                    None => print!("WRN: can't find cpu with id: '{}'", i),
                },
            );

        self.system_info.refresh_networks();
        self.system_info
            .networks()
            .into_iter()
            .for_each(|(interface, network_data)| {
                match self.networks.iter_mut().find(|y| y.interface.eq(interface)) {
                    Some(network) => {
                        network.tx_bytes = network_data.transmitted();
                        network.rx_bytes = network_data.received();
                        network.rx_error_bytes = network_data.total_errors_on_received();
                        network.rx_error_bytes = network_data.total_errors_on_transmitted();

                        let time = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_micros();
                        network.rx_speed_bps = (network.rx_bytes - network.rx_bytes_old) as f64
                            / (time - network.last_synced_timestamp) as f64;
                        network.tx_speed_bps = (network.tx_bytes - network.tx_bytes_old) as f64
                            / (time - network.last_synced_timestamp) as f64;
                        network.rx_bytes_old = network.rx_bytes;
                        network.tx_bytes_old = network.tx_bytes;
                        network.last_synced_timestamp = time;
                    }
                    None => print!(
                        "WRN: can't find network interface with interface name: '{}'",
                        interface
                    ),
                }
            });

        self.system_info.refresh_memory();
        self.memory.used_bytes = self.system_info.used_memory();
        self.memory.total_bytes = self.system_info.total_memory();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_json_diff::assert_json_eq;
    use serde_json::json;
    use std::process::{Command, Stdio};

    fn crate_resource() -> Resource {
        let mut resource = Resource {
            system_info: System::new(),
            cpus: Vec::new(),
            memory: MemoryResource {
                used_bytes: 0,
                total_bytes: 0,
            },
            networks: Vec::new(),
        };

        resource.system_info.refresh_cpu();
        for i in 0..resource.system_info.cpus().len() {
            resource.cpus.push(CPUResource {
                id: i as u8,
                load: -1.0,
            });
        }

        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::new(0, 0))
            .as_micros();

        resource.system_info.refresh_networks_list();
        resource.system_info.refresh_networks();
        for (interface, _) in resource.system_info.networks() {
            resource.networks.push(NetworkResource {
                interface: interface.to_owned(),
                rx_bytes: 0,
                tx_bytes: 0,
                rx_error_bytes: 0,
                tx_error_bytes: 0,
                rx_speed_bps: 0.0,
                tx_speed_bps: 0.0,
                tx_bytes_old: 0,
                rx_bytes_old: 0,
                last_synced_timestamp: time,
            });
        }
        resource.networks.sort();

        resource
    }

    #[test]
    fn test_create_resource() {
        let def_resource = crate_resource();
        let resource = Resource::new();

        assert_eq!(def_resource.cpus.iter().eq(resource.cpus.iter()), true);
        assert!(def_resource.memory == resource.memory);
        def_resource.networks.iter().for_each(|x| {
            println!(
                "interface: {}\ndef received: {}\ndef transmitted: {}\n",
                x.interface, x.rx_bytes, x.tx_bytes
            )
        });
        resource.networks.iter().for_each(|x| {
            println!(
                "interface: {}\nreceived: {}\ntransmitted: {}\n",
                x.interface, x.rx_bytes, x.tx_bytes
            )
        });
        assert_eq!(
            def_resource.networks.iter().eq(resource.networks.iter()),
            true
        );
    }

    #[test]
    fn test_refresh_resources() {
        let def_resource = crate_resource();
        let mut resource = Resource::new();
        let status = Command::new("ping")
            .arg("-c 2")
            .arg("localhost")
            .stdout(Stdio::null())
            .status()
            .expect("failed to run ping");

        resource.refresh();

        assert_eq!(status.success(), true);

        assert!(def_resource.cpus.len() == resource.cpus.len());
        assert_eq!(def_resource.cpus.iter().eq(resource.cpus.iter()), false);

        assert!(def_resource.memory != resource.memory);

        assert!(def_resource.networks.len() == resource.networks.len());
        assert_eq!(
            def_resource.networks.iter().eq(resource.networks.iter()),
            false
        );
    }

    #[test]
    fn test_resource_serialization() {
        let mut resource = Resource {
            system_info: System::new(),
            cpus: Vec::new(),
            memory: MemoryResource {
                used_bytes: 0,
                total_bytes: 0,
            },
            networks: Vec::new(),
        };

        resource.cpus.push(CPUResource { id: 0, load: 38.0 });
        resource.cpus.push(CPUResource { id: 1, load: 11.0 });
        resource.cpus.push(CPUResource { id: 2, load: 12.0 });

        resource.memory.total_bytes = 15;
        resource.memory.used_bytes = 1;

        resource.networks.push(NetworkResource {
            interface: "eth0".to_owned(),
            rx_bytes: 100,
            tx_bytes: 200,
            rx_error_bytes: 0,
            tx_error_bytes: 0,
            rx_speed_bps: 300.0,
            tx_speed_bps: 400.0,
            tx_bytes_old: 0,
            rx_bytes_old: 0,
            last_synced_timestamp: 0,
        });

        resource.networks.push(NetworkResource {
            interface: "eth1".to_owned(),
            rx_bytes: 10,
            tx_bytes: 20,
            rx_error_bytes: 0,
            tx_error_bytes: 0,
            rx_speed_bps: 30.0,
            tx_speed_bps: 40.0,
            tx_bytes_old: 0,
            rx_bytes_old: 0,
            last_synced_timestamp: 0,
        });

        let expected_resource = json!({
                "cpus": [
                    {
                        "id": resource.cpus[0].id,
                        "load": resource.cpus[0].load
                    },
                    {
                        "id": resource.cpus[1].id,
                        "load": resource.cpus[1].load
                    },
                    {
                        "id": resource.cpus[2].id,
                        "load": resource.cpus[2].load
                    }
                ],
                "memory": {
                    "used_bytes": resource.memory.used_bytes,
                    "total_bytes": resource.memory.total_bytes
                },
                "networks": [
                    {
                        "interface": resource.networks[0].interface,
                        "rx_bytes": resource.networks[0].rx_bytes,
                        "tx_bytes": resource.networks[0].tx_bytes,
                        "rx_error_bytes": resource.networks[0].rx_error_bytes,
                        "tx_error_bytes": resource.networks[0].tx_error_bytes,
                        "rx_speed_bps": resource.networks[0].rx_speed_bps,
                        "tx_speed_bps":resource.networks[0].tx_speed_bps
                    },
                    {
                        "interface": resource.networks[1].interface,
                        "rx_bytes": resource.networks[1].rx_bytes,
                        "tx_bytes": resource.networks[1].tx_bytes,
                        "rx_error_bytes": resource.networks[1].rx_error_bytes,
                        "tx_error_bytes": resource.networks[1].tx_error_bytes,
                        "rx_speed_bps": resource.networks[1].rx_speed_bps,
                        "tx_speed_bps":resource.networks[1].tx_speed_bps
                    }
                ]
        });

        assert_json_eq!(expected_resource, resource);
    }
}
