use anyhow::Result;
use log::info;

// Import from sensecap_backend
use sensecap_backend;

// External C functions from LVGL UI
extern "C" {
    fn ui_init();
    fn lv_timer_handler();
    fn rust_register_ui_callback(callback: extern "C" fn(i32, i32, i32));
}

/// WiFi SSID - configure via environment or sdkconfig
const WIFI_SSID: &str = env!("WIFI_SSID", "Set WIFI_SSID environment variable");
const WIFI_PASS: &str = env!("WIFI_PASS", "Set WIFI_PASS environment variable");

/// MQTT broker URL
const MQTT_BROKER: &str = env!("MQTT_BROKER", "mqtt://192.168.1.100:1883");
const MQTT_CLIENT_ID: &str = "sensecap-indicator";

/// MQTT Topics
const MQTT_TOPIC_LIGHTS: &str = "sensecap/lights/state";
const MQTT_TOPIC_WATER: &str = "sensecap/water/level";

/// UI update callback - called by Rust when state changes
extern "C" fn ui_update_callback(water_level: i32, bright_state: i32, relax_state: i32) {
    info!(
        "UI Update: water={}, bright={}, relax={}",
        water_level, bright_state, relax_state
    );
    // TODO: Call C UI update functions to update display
}

/// Connect to WiFi
fn connect_wifi() -> Result<esp_idf_svc::wifi::EspWifi<'static>> {
    let peripherals = esp_idf_svc::hal::peripherals::Peripherals::take()?;
    let sys_loop = esp_idf_svc::eventloop::EspSystemEventLoop::take()?;
    let nvs = esp_idf_svc::nvs::EspDefaultNvsPartition::take()?;

    let mut wifi = esp_idf_svc::wifi::EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?;

    wifi.set_configuration(&esp_idf_svc::wifi::Configuration::Client(
        esp_idf_svc::wifi::ClientConfiguration {
            ssid: WIFI_SSID
                .try_into()
                .map_err(|_| anyhow::anyhow!("SSID too long"))?,
            password: WIFI_PASS
                .try_into()
                .map_err(|_| anyhow::anyhow!("Password too long"))?,
            ..Default::default()
        },
    ))?;

    wifi.wait_netif_up()?;

    Ok(wifi)
}

fn main() -> Result<()> {
    // Initialize ESP-IDF
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Starting SenseCap firmware...");

    // Initialize the backend
    sensecap_backend::init();

    // Initialize WiFi
    info!("Connecting to WiFi...");
    let _wifi = connect_wifi()?;
    info!("WiFi connected!");

    // Initialize MQTT
    info!("Initializing MQTT...");
    unsafe {
        let broker = std::ffi::CString::new(MQTT_BROKER)?;
        let client_id = std::ffi::CString::new(MQTT_CLIENT_ID)?;
        sensecap_backend::ffi::rust_mqtt_init(
            broker.as_ptr() as *const u8,
            client_id.as_ptr() as *const u8,
        );
    }

    // Subscribe to topics
    unsafe {
        let topic = std::ffi::CString::new(MQTT_TOPIC_WATER)?;
        sensecap_backend::ffi::rust_mqtt_subscribe(topic.as_ptr() as *const u8);
    }

    // Register UI update callback
    unsafe {
        rust_register_ui_callback(ui_update_callback);
    }

    // Initialize LVGL UI (from C)
    unsafe {
        ui_init();
    }

    info!("UI initialized, entering main loop...");

    // Main loop
    loop {
        // Handle LVGL timer events
        unsafe {
            lv_timer_handler();
        }

        // Small delay to prevent busy-waiting
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

