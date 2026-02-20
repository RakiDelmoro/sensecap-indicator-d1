use std::sync::Arc;

use anyhow::Result;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::timer::EspTaskTimerService;
use log::info;

mod backend;
mod config;
mod display;
mod mqtt;
mod touch;
mod ui;
mod wifi;

use crate::backend::Backend;
use crate::config::Config;
use crate::display::DisplayDriver;
use crate::mqtt::MqttClient;
use crate::touch::TouchDriver;
use crate::ui::UiManager;
use crate::wifi::WifiManager;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("======================================");
    info!("SenseCAP Indicator D1 Firmware v0.1.0");
    info!("======================================");

    let config = Config::from_env_or_default()?;

    let event_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    let timer_service = EspTaskTimerService::new()?;

    info!("Initializing display...");
    let display = DisplayDriver::new()?;

    info!("Initializing touch...");
    let touch = TouchDriver::new()?;

    info!("Initializing WiFi...");
    let wifi = WifiManager::new(
        event_loop.clone(),
        nvs.clone(),
        &config.wifi_ssid,
        &config.wifi_password,
    )?;

    info!("Initializing MQTT...");
    let mqtt = MqttClient::new(
        &config.mqtt_broker_url,
        config.mqtt_username.as_deref(),
        config.mqtt_password.as_deref(),
    )?;

    info!("Initializing backend...");
    let backend = Arc::new(std::sync::Mutex::new(Backend::new()));

    info!("Initializing UI...");
    let ui = UiManager::new(
        display,
        touch,
        Arc::clone(&backend),
    )?;

    info!("Starting main event loop...");
    ui.run_event_loop()
}
