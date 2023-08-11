mod monitor;
mod mqtt_client;
mod resource;

use mqtt_client::MqttClient;
use std::sync::Arc;

fn main() {
    let username = "lpaulic";
    let password = "lpaulic";
    let mqtt_client = MqttClient::new("localhost", 1883, username, password);
    let resource_monitor = Arc::new(monitor::ResourceMonitor::new(1, mqtt_client));

    // TODO: make OS specific signal handling, handle for Windows OS, currently unix systems are only supported
    let sighandler_monitor = Arc::clone(&resource_monitor);
    if let Err(e) = ctrlc::set_handler(move || match sighandler_monitor.stop() {
        Ok(_) => std::process::exit(0),
        Err(_) => std::process::exit(1),
    }) {
        println!("ERR: interrupt signal registration failed: {}", e)
    }

    if let Err(e) = resource_monitor.start() {
        println!("ERR: resource monitor start issue: {}", e);
    }
}
