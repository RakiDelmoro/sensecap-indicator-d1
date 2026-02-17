//! FFI Bridge - Functions callable from C UI

use crate::{init, lights, mock_mqtt, water};

/// Initialize the backend from C
#[no_mangle]
pub extern "C" fn rust_backend_init() {
    init();
    mock_mqtt::start_subscriber();
    println!("[FFI] Backend + MockMQTT initialized");
}

// ==================== LIGHTS ====================

/// Set bright switch from C (0 = off, 1 = on)
#[no_mangle]
pub extern "C" fn rust_set_bright(state: u8) {
    lights::set_bright(state);
}

/// Set relax switch from C (0 = off, 1 = on)
#[no_mangle]
pub extern "C" fn rust_set_relax(state: u8) {
    lights::set_relax(state);
}

/// Get bright switch state (returns 0 or 1, -1 if error)
#[no_mangle]
pub extern "C" fn rust_get_bright() -> i32 {
    match lights::get_bright() {
        Some(state) => state as i32,
        None => -1,
    }
}

/// Get relax switch state (returns 0 or 1, -1 if error)
#[no_mangle]
pub extern "C" fn rust_get_relax() -> i32 {
    match lights::get_relax() {
        Some(state) => state as i32,
        None => -1,
    }
}

/// Toggle bright, returns new state
#[no_mangle]
pub extern "C" fn rust_toggle_bright() -> i32 {
    match lights::toggle_bright() {
        Some(state) => state as i32,
        None => -1,
    }
}

/// Toggle relax, returns new state
#[no_mangle]
pub extern "C" fn rust_toggle_relax() -> i32 {
    match lights::toggle_relax() {
        Some(state) => state as i32,
        None => -1,
    }
}

// ==================== WATER ====================

/// Get water level (returns 0-100, -1 if error)
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

/// Check if water is low (returns 1/0, -1 if error)
#[no_mangle]
pub extern "C" fn rust_is_water_low() -> i32 {
    match water::is_low() {
        Some(true) => 1,
        Some(false) => 0,
        None => -1,
    }
}

/// Check if water is critical (returns 1/0, -1 if error)
#[no_mangle]
pub extern "C" fn rust_is_water_critical() -> i32 {
    match water::is_critical() {
        Some(true) => 1,
        Some(false) => 0,
        None => -1,
    }
}

// ==================== C CALLBACKS ====================
/// These are implemented in C UI and called by Rust

extern "C" {
    /// Update water arc in C UI (LVGL thread only!)
    pub fn ui_set_water_level(level: i32);

    /// Update bright button state in C UI (LVGL thread only!)
    pub fn ui_set_bright_state(state: i32);

    /// Update relax button state in C UI (LVGL thread only!)
    pub fn ui_set_relax_state(state: i32);

    /// Thread-safe async water level update (can be called from any thread)
    pub fn ui_update_water_level_async(level: i32);
}
