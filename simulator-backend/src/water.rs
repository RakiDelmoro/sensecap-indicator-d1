//! Water level management

use crate::with_backend;

/// Maximum water level
pub const MAX_WATER_LEVEL: u8 = 100;

/// Get current water level
pub fn get_level() -> Option<u8> {
    with_backend(|backend| backend.water_level)
}

/// Set water level (clamped to 0-100)
pub fn set_level(level: u8) {
    let clamped = level.min(MAX_WATER_LEVEL);
    with_backend(|backend| {
        backend.water_level = clamped;
    });
    println!("[WATER] Level set to: {}%", clamped);
}

/// Check if water level is low (below 20%)
pub fn is_low() -> Option<bool> {
    with_backend(|backend| backend.water_level < 20)
}

/// Check if water level is critical (below 10%)
pub fn is_critical() -> Option<bool> {
    with_backend(|backend| backend.water_level < 10)
}
