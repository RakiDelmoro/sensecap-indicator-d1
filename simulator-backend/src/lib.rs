//! Simulator Backend - Mock MQTT + Real Logic for PC Testing
//! 
//! This runs on PC (not embedded) for testing UI interactions

pub mod mock_mqtt;
pub mod lights;
pub mod water;
pub mod ffi;

use std::sync::Mutex;

/// Global backend state
pub struct Backend {
    pub bright_switch: u8,
    pub relax_switch: u8,
    pub water_level: u8,
}

impl Default for Backend {
    fn default() -> Self {
        Self {
            bright_switch: 0,
            relax_switch: 0,
            water_level: 50,
        }
    }
}

static BACKEND: Mutex<Option<Backend>> = Mutex::new(None);

/// Initialize the backend
pub fn init() {
    let mut backend = BACKEND.lock().unwrap();
    *backend = Some(Backend::default());
    println!("[INFO] Simulator backend initialized");
}

/// Get mutable access to backend
pub fn with_backend<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut Backend) -> R,
{
    let mut backend = BACKEND.lock().unwrap();
    backend.as_mut().map(f)
}
