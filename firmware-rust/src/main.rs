use std::sync::Arc;

use anyhow::Result;
use esp_idf_hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::units::Hertz;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use log::info;

mod backend;
mod config;
mod display;
mod lvgl_ui;
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

// Shared I2C bus for display and touch
const I2C_FREQ_HZ: u32 = 400000;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("======================================");
    info!("SenseCAP Indicator D1 Firmware v0.1.0");
    info!("======================================");

    let config = Config::from_env_or_default()?;

    let event_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    // Take peripherals once
    let mut peripherals = Peripherals::take()?;

    // Initialize shared I2C bus for display and touch
    let i2c_config = I2cConfig::new()
        .baudrate(Hertz(I2C_FREQ_HZ))
        .sda_io_num(39)
        .scl_io_num(40);

    let i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio39,
        peripherals.pins.gpio40,
        &i2c_config,
    )?;

    // Wrap I2C in Arc<Mutex<>> for sharing between drivers
    let shared_i2c = Arc::new(std::sync::Mutex::new(i2c));

    info!("Initializing display...");
    let display = DisplayDriver::new(&mut peripherals, Arc::clone(&shared_i2c))?;

    info!("Initializing touch...");
    let touch = TouchDriver::new(&mut peripherals, Arc::clone(&shared_i2c))?;

    info!("Initializing WiFi...");
    let _wifi = WifiManager::new(
        event_loop.clone(),
        nvs.clone(),
        &config.wifi_ssid,
        &config.wifi_password,
    )?;

    info!("Initializing MQTT...");
    let mut mqtt = MqttClient::new(
        &config.mqtt_broker_url,
        config.mqtt_username.as_deref(),
        config.mqtt_password.as_deref(),
    )?;

    info!("Initializing backend...");
    let backend = Arc::new(std::sync::Mutex::new(Backend::new()));

    info!("Initializing UI...");
    let mut ui = UiManager::new(display, touch, Arc::clone(&backend))?;

    info!("Starting main event loop...");
    ui.run_event_loop(&mut mqtt)
}
