use crate::{init, lights, water};
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
