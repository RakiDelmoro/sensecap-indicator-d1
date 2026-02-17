use crate::{init, lights, mqtt, water};
use log::info;

/// FFI bridge for C code to call Rust functions

/// Initialize the backend from C
#[no_mangle]
pub extern "C" fn rust_backend_init() {
    init();
    info!("Backend initialized via FFI");
}

// ==================== LIGHTS (SWITCH: 0/1) ====================

/// Set bright switch state from C (0 = off, 1 = on)
#[no_mangle]
pub extern "C" fn rust_set_bright(state: u8) {
    lights::set_bright(state);
}

/// Set relax switch state from C (0 = off, 1 = on)
#[no_mangle]
pub extern "C" fn rust_set_relax(state: u8) {
    lights::set_relax(state);
}

/// Get bright switch state for C (returns 0 or 1, -1 if not initialized)
#[no_mangle]
pub extern "C" fn rust_get_bright() -> i32 {
    match lights::get_bright() {
        Some(state) => state as i32,
        None => -1,
    }
}

/// Get relax switch state for C (returns 0 or 1, -1 if not initialized)
#[no_mangle]
pub extern "C" fn rust_get_relax() -> i32 {
    match lights::get_relax() {
        Some(state) => state as i32,
        None => -1,
    }
}

/// Toggle bright switch from C, returns new state (0 or 1, -1 if error)
#[no_mangle]
pub extern "C" fn rust_toggle_bright() -> i32 {
    match lights::toggle_bright() {
        Some(state) => state as i32,
        None => -1,
    }
}

/// Toggle relax switch from C, returns new state (0 or 1, -1 if error)
#[no_mangle]
pub extern "C" fn rust_toggle_relax() -> i32 {
    match lights::toggle_relax() {
        Some(state) => state as i32,
        None => -1,
    }
}

/// Legacy: Handle bright toggle from C (for backward compatibility)
#[no_mangle]
pub extern "C" fn rust_handle_bright_toggle() {
    lights::handle_bright_toggle();
}

/// Legacy: Handle relax toggle from C (for backward compatibility)
#[no_mangle]
pub extern "C" fn rust_handle_relax_toggle() {
    lights::handle_relax_toggle();
}

// ==================== WATER LEVEL ====================

/// Get water level from C
/// Returns -1 if backend not initialized, otherwise 0-100
#[no_mangle]
pub extern "C" fn rust_get_water_level() -> i32 {
    match water::get_level() {
        Some(level) => level as i32,
        None => -1,
    }
}

/// Set water level from C (0-100)
#[no_mangle]
pub extern "C" fn rust_set_water_level(level: u8) {
    water::set_level(level);
}

/// Update water level from sensor (called from C)
#[no_mangle]
pub extern "C" fn rust_update_water_level() {
    water::update_from_sensor();
}

/// Check if water level is low (below 20%)
/// Returns: 1 = low, 0 = not low, -1 = error
#[no_mangle]
pub extern "C" fn rust_is_water_low() -> i32 {
    match water::is_low() {
        Some(true) => 1,
        Some(false) => 0,
        None => -1,
    }
}

/// Check if water level is critical (below 10%)
/// Returns: 1 = critical, 0 = not critical, -1 = error
#[no_mangle]
pub extern "C" fn rust_is_water_critical() -> i32 {
    match water::is_critical() {
        Some(true) => 1,
        Some(false) => 0,
        None => -1,
    }
}

// ==================== MQTT & WIFI ====================

/// Initialize MQTT client
/// broker_url: MQTT broker URL (e.g., "mqtt://192.168.1.100:1883")
/// client_id: Unique client ID
#[no_mangle]
pub extern "C" fn rust_mqtt_init(broker_url: *const u8, client_id: *const u8) -> i32 {
    let broker = unsafe {
        std::ffi::CStr::from_ptr(broker_url as *const i8)
            .to_str()
            .unwrap_or("mqtt://localhost:1883")
    };
    let id = unsafe {
        std::ffi::CStr::from_ptr(client_id as *const i8)
            .to_str()
            .unwrap_or("sensecap")
    };

    match mqtt::init_mqtt(broker, id) {
        Ok(_) => {
            info!("MQTT initialized: {} as {}", broker, id);
            0
        }
        Err(e) => {
            log::error!("Failed to initialize MQTT: {}", e);
            -1
        }
    }
}

/// Subscribe to MQTT topic
/// Returns: 0 = success, -1 = error
#[no_mangle]
pub extern "C" fn rust_mqtt_subscribe(topic: *const u8) -> i32 {
    let topic_str = unsafe {
        std::ffi::CStr::from_ptr(topic as *const i8)
            .to_str()
            .unwrap_or("")
    };

    match mqtt::subscribe(topic_str) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// Publish MQTT message
/// Returns: 0 = success, -1 = error
#[no_mangle]
pub extern "C" fn rust_mqtt_publish(topic: *const u8, payload: *const u8, len: usize) -> i32 {
    let topic_str = unsafe {
        std::ffi::CStr::from_ptr(topic as *const i8)
            .to_str()
            .unwrap_or("")
    };
    let payload_bytes = unsafe { std::slice::from_raw_parts(payload, len) };

    match mqtt::publish(topic_str, payload_bytes) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// Check if MQTT is connected
/// Returns: 1 = connected, 0 = not connected
#[no_mangle]
pub extern "C" fn rust_mqtt_is_connected() -> i32 {
    match mqtt::with_mqtt(|client| client.is_connected()) {
        Some(true) => 1,
        _ => 0,
    }
}

// ==================== UI CALLBACKS (C to Rust) ====================

/// Callback function type for UI updates from Rust
/// This is implemented in C and called by Rust when data changes
pub type UiUpdateCallback = extern "C" fn(water_level: i32, bright_state: i32, relax_state: i32);

static mut UI_CALLBACK: Option<UiUpdateCallback> = None;

/// Register UI update callback from C
/// This callback is called whenever the backend state changes
#[no_mangle]
pub extern "C" fn rust_register_ui_callback(callback: UiUpdateCallback) {
    unsafe {
        UI_CALLBACK = Some(callback);
    }
    info!("UI callback registered");
}

/// Trigger UI update from Rust (called when MQTT receives data)
pub fn trigger_ui_update() {
    let water = water::get_level().unwrap_or(0) as i32;
    let bright = lights::get_bright().unwrap_or(0) as i32;
    let relax = lights::get_relax().unwrap_or(0) as i32;

    unsafe {
        if let Some(callback) = UI_CALLBACK {
            callback(water, bright, relax);
        }
    }
}

/// Set water level from MQTT (updates backend and triggers UI callback)
pub fn set_water_level_from_mqtt(level: u8) {
    water::set_level(level);
    trigger_ui_update();
}

/// Set bright state from MQTT (updates backend and triggers UI callback)
pub fn set_bright_from_mqtt(state: u8) {
    lights::set_bright(state);
    trigger_ui_update();
}

/// Set relax state from MQTT (updates backend and triggers UI callback)
pub fn set_relax_from_mqtt(state: u8) {
    lights::set_relax(state);
    trigger_ui_update();
}
