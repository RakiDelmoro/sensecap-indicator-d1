use crate::{init, lights, water};
use log::info;

/// FFI bridge for C code to call Rust functions

/// Initialize the backend from C
#[no_mangle]
pub extern "C" fn rust_backend_init() {
    init();
    info!("Backend initialized via FFI");
}

/// Handle bright toggle from C
#[no_mangle]
pub extern "C" fn rust_handle_bright_toggle() {
    lights::handle_bright_toggle();
}

/// Handle relax toggle from C
#[no_mangle]
pub extern "C" fn rust_handle_relax_toggle() {
    lights::handle_relax_toggle();
}

/// Get water level from C
/// Returns -1 if backend not initialized, otherwise 0-100
#[no_mangle]
pub extern "C" fn rust_get_water_level() -> i32 {
    match water::get_level() {
        Some(level) => level as i32,
        None => -1,
    }
}

/// Update water level from sensor (called from C)
#[no_mangle]
pub extern "C" fn rust_update_water_level() {
    water::update_from_sensor();
}

/// Set water level from C
#[no_mangle]
pub extern "C" fn rust_set_water_level(level: u8) {
    water::set_level(level);
}
