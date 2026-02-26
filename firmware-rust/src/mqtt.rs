use anyhow::Result;
use esp_idf_svc::mqtt::client::QoS;
use esp_idf_svc::mqtt::client::{EspMqttClient, Event, Message, MqttClientConfiguration};
use log::{error, info, warn};
use std::sync::mpsc::{channel, Receiver, Sender};

const MQTT_TOPIC_LIGHT_STATE: &str = "sensecap/indicator/light/state";
const MQTT_TOPIC_WATER_LEVEL: &str = "sensecap/indicator/water/level";

pub enum MqttMessage {
    WaterLevel(u8),
    Connected,
    Disconnected,
}

pub struct MqttClient {
    client: EspMqttClient<'static>,
    receiver: Receiver<MqttMessage>,
}

impl MqttClient {
    pub fn new(broker_url: &str, _username: Option<&str>, _password: Option<&str>) -> Result<Self> {
        info!("Initializing MQTT client...");
        info!("Broker URL: {}", broker_url);

        let (tx, rx) = channel::<MqttMessage>();

        let config = MqttClientConfiguration {
            client_id: Some("sensecap_indicator_d1"),
            keep_alive_interval: Some(std::time::Duration::from_secs(60)),
            ..Default::default()
        };

        let mut client = EspMqttClient::new(broker_url, &config, move |event| {
            match event {
                Event::Connected(_) => {
                    info!("MQTT connected");
                    let _ = tx.send(MqttMessage::Connected);
                }
                Event::Disconnected => {
                    warn!("MQTT disconnected");
                    let _ = tx.send(MqttMessage::Disconnected);
                }
                Event::Received(msg) => {
                    if let Some(topic) = msg.topic() {
                        let topic_str = std::str::from_utf8(topic).unwrap_or("<invalid utf8>");
                        info!("MQTT message received on topic: {}", topic_str);

                        if topic_str == MQTT_TOPIC_WATER_LEVEL {
                            if let Some(data) = msg.data() {
                                if let Ok(data_str) = std::str::from_utf8(data) {
                                    if let Ok(level) = data_str.trim().parse::<u8>() {
                                        info!("Water level received: {}%", level);
                                        let _ = tx.send(MqttMessage::WaterLevel(level));
                                    } else {
                                        warn!("Failed to parse water level: {}", data_str);
                                    }
                                }
                            }
                        }
                    }
                }
                Event::Published(_) => {
                    // Message published successfully
                }
                Event::Error(err) => {
                    error!("MQTT error: {:?}", err);
                }
                _ => {}
            }
        })?;

        // Subscribe to water level topic
        client.subscribe(MQTT_TOPIC_WATER_LEVEL, QoS::AtLeastOnce)?;

        info!("MQTT client initialized");

        Ok(MqttClient {
            client,
            receiver: rx,
        })
    }

    pub fn publish_light_state(&mut self, mode: &str, state: bool) -> Result<()> {
        let payload = format!(
            "{{\"mode\":\"{}\",\"state\":{}}}",
            mode,
            if state { 1 } else { 0 }
        );
        info!("Publishing light state: {}", payload);

        self.client.publish(
            MQTT_TOPIC_LIGHT_STATE,
            QoS::AtLeastOnce,
            false,
            payload.as_bytes(),
        )?;

        Ok(())
    }

    pub fn try_publish_light_state(&mut self, mode: &str, state: bool) -> Result<()> {
        let payload = format!(
            "{{\"mode\":\"{}\",\"state\":{}}}",
            mode,
            if state { 1 } else { 0 }
        );
        info!("Publishing light state: {}", payload);

        self.client.publish(
            MQTT_TOPIC_LIGHT_STATE,
            QoS::AtLeastOnce,
            false,
            payload.as_bytes(),
        )?;

        Ok(())
    }

    pub fn try_recv(&self) -> Option<MqttMessage> {
        self.receiver.try_recv().ok()
    }

    pub fn get_receiver(&self) -> &Receiver<MqttMessage> {
        &self.receiver
    }
}
