use anyhow::Result;

pub struct Config {
    pub wifi_ssid: String,
    pub wifi_password: String,
    pub mqtt_broker_url: String,
    pub mqtt_username: Option<String>,
    pub mqtt_password: Option<String>,
}

impl Config {
    pub fn from_env_or_default() -> Result<Self> {
        Ok(Config {
            wifi_ssid: std::env::var("WIFI_SSID").unwrap_or_else(|_| "your_wifi_ssid".to_string()),
            wifi_password: std::env::var("WIFI_PASSWORD")
                .unwrap_or_else(|_| "your_wifi_password".to_string()),
            mqtt_broker_url: std::env::var("MQTT_BROKER_URL")
                .unwrap_or_else(|_| "mqtt://broker.hivemq.com:1883".to_string()),
            mqtt_username: std::env::var("MQTT_USERNAME").ok(),
            mqtt_password: std::env::var("MQTT_PASSWORD").ok(),
        })
    }
}
