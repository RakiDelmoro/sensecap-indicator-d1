use anyhow::Result;
use log::info;
use std::sync::{Arc, Mutex};

/// MQTT Client wrapper for ESP-IDF
pub struct MqttClient {
    client: Option<esp_idf_svc::mqtt::client::EspMqttClient<'static>>,
    connected: Arc<Mutex<bool>>,
}

impl MqttClient {
    /// Create a new MQTT client
    pub fn new(broker_url: &str, client_id: &str) -> Result<Self> {
        let connected = Arc::new(Mutex::new(false));
        let connected_clone = connected.clone();

        let client = esp_idf_svc::mqtt::client::EspMqttClient::new(
            broker_url,
            &esp_idf_svc::mqtt::client::MqttClientConfiguration {
                client_id: Some(client_id),
                ..Default::default()
            },
            move |event| match event {
                esp_idf_svc::mqtt::client::Event::Connected(_) => {
                    info!("MQTT Connected");
                    *connected_clone.lock().unwrap() = true;
                }
                esp_idf_svc::mqtt::client::Event::Disconnected => {
                    info!("MQTT Disconnected");
                    *connected_clone.lock().unwrap() = false;
                }
                _ => {}
            },
        )?;

        Ok(Self {
            client: Some(client),
            connected,
        })
    }

    /// Subscribe to a topic
    pub fn subscribe(&mut self, topic: &str) -> Result<()> {
        if let Some(ref mut client) = self.client {
            client.subscribe(topic, esp_idf_svc::mqtt::client::QoS::AtLeastOnce)?;
            info!("Subscribed to topic: {}", topic);
        }
        Ok(())
    }

    /// Publish a message to a topic
    pub fn publish(&mut self, topic: &str, payload: &[u8]) -> Result<()> {
        if let Some(ref mut client) = self.client {
            client.publish(
                topic,
                esp_idf_svc::mqtt::client::QoS::AtLeastOnce,
                false,
                payload,
            )?;
            info!("Published to {}: {:?}", topic, payload);
        }
        Ok(())
    }

    /// Check if connected to MQTT broker
    pub fn is_connected(&self) -> bool {
        *self.connected.lock().unwrap()
    }
}

/// Global MQTT client instance
static MQTT_CLIENT: Mutex<Option<MqttClient>> = Mutex::new(None);

/// Initialize MQTT client
pub fn init_mqtt(broker_url: &str, client_id: &str) -> Result<()> {
    let client = MqttClient::new(broker_url, client_id)?;
    let mut global = MQTT_CLIENT.lock().unwrap();
    *global = Some(client);
    info!("MQTT client initialized");
    Ok(())
}

/// Get MQTT client reference
pub fn with_mqtt<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut MqttClient) -> R,
{
    let mut client = MQTT_CLIENT.lock().unwrap();
    client.as_mut().map(f)
}

/// Subscribe to a topic
pub fn subscribe(topic: &str) -> Result<()> {
    with_mqtt(|client| client.subscribe(topic)).unwrap_or(Ok(()))
}

/// Publish a message
pub fn publish(topic: &str, payload: &[u8]) -> Result<()> {
    with_mqtt(|client| client.publish(topic, payload)).unwrap_or(Ok(()))
}
