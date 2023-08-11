use random_string::generate;
use rumqttc::{
    Client, ClientError, Connection, ConnectionError, Event, Incoming, MqttOptions, Outgoing, QoS,
};
use std::time::Duration;

#[derive(Debug)]
pub enum MqttClientError {
    Client(ClientError),
    Connection(ConnectionError),
}

impl From<ClientError> for MqttClientError {
    fn from(item: ClientError) -> MqttClientError {
        MqttClientError::Client(item)
    }
}

impl From<ConnectionError> for MqttClientError {
    fn from(item: ConnectionError) -> MqttClientError {
        MqttClientError::Connection(item)
    }
}

pub struct MqttClient {
    client: Client,
    connection: Connection,
}

impl MqttClient {
    pub fn new(server_addr: &str, server_port: u16, username: &str, password: &str) -> MqttClient {
        let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let client_id = format!("client{}", generate(7, charset));

        let mut client_options = MqttOptions::new(client_id, server_addr, server_port);
        client_options.set_keep_alive(Duration::from_secs(5));
        client_options.set_credentials(username, password);

        let (client, connection) = Client::new(client_options, 10);
        MqttClient { client, connection }
    }

    pub fn publish(&mut self, topic: &String, data: &[u8]) -> Result<(), MqttClientError> {
        self.client.publish(topic, QoS::ExactlyOnce, false, data)?;

        let mut message_id = 0;
        let mut is_message_sent = false;
        for notification in self.connection.iter() {
            match notification? {
                Event::Outgoing(outgoing) => {
                    if let Outgoing::Publish(publish_id) = outgoing {
                        message_id = publish_id;
                        is_message_sent = true;
                    }
                }
                Event::Incoming(incoming) => {
                    if let Incoming::PubComp(publish_completed) = incoming {
                        if is_message_sent && publish_completed.pkid == message_id {
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
