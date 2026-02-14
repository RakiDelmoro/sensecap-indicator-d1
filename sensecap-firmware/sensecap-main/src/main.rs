use anyhow::Result;
use log::info;

// Import from sensecap_backend
use sensecap_backend;

// External C functions from LVGL UI
extern "C" {
    fn ui_init();
    fn lv_timer_handler();
}

fn main() -> Result<()> {
    // Initialize ESP-IDF
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Starting SenseCap firmware...");

    // Initialize the backend
    sensecap_backend::init();

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
