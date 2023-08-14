/*
   TODO:
       - rework this crate to be a binary crate
       - run the code from here
*/

mod mqtt_client;

use gethostname::gethostname;
use mqtt_client::MqttClient;

fn main() {
    let username = "lpaulic";
    let password = "lpaulic";

    let mut mqtt_client = MqttClient::new("localhost", 1883, username, password);
    let topic = format!(
        "device/{}/system/stats",
        gethostname().into_string().unwrap()
    );
    if let Err(e) = mqtt_client.publish(&topic, "test message".as_bytes()) {
        println!("Error {:?}", e);
    }
}
