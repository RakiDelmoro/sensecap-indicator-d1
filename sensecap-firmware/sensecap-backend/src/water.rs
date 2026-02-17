use crate::ffi::trigger_ui_update;
use crate::with_backend;
use log::info;

/// Maximum water level
pub const MAX_WATER_LEVEL: u8 = 100;

/// MQTT topic for water level
pub const MQTT_TOPIC_WATER: &str = "sensecap/water/level";

/// Get current water level
pub fn get_level() -> Option<u8> {
    with_backend(|backend| backend.water_level)
}

/// Set water level (clamped to 0-100)
pub fn set_level(level: u8) {
    let clamped = level.min(MAX_WATER_LEVEL);
    with_backend(|backend| {
        backend.water_level = clamped;
        info!("Water level set to {}%", clamped);
    });
    // Trigger UI update callback
    trigger_ui_update();
}

/// Read water level from sensor
/// This is a placeholder that simulates reading from a sensor
pub fn read_sensor() -> u8 {
    // In a real implementation, this would read from an actual sensor
    // For now, return a simulated value
    50 // Placeholder: 50% full
}

/// Update water level by reading from sensor
pub fn update_from_sensor() {
    let level = read_sensor();
    set_level(level);
}

/// Check if water level is low (below 20%)
pub fn is_low() -> Option<bool> {
    with_backend(|backend| backend.water_level < 20)
}

/// Check if water level is critical (below 10%)
pub fn is_critical() -> Option<bool> {
    with_backend(|backend| backend.water_level < 10)
}
