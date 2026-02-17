use log::info;
use std::sync::Mutex;

pub mod ffi;
pub mod lights;
pub mod mqtt;
pub mod water;

/// Light modes for the device
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LightMode {
    Off,
    Bright,
    Relax,
}

/// Global backend state
pub struct Backend {
    pub light_mode: LightMode,
    pub water_level: u8,
    /// Bright switch: 0 = off, 1 = on
    pub bright_switch: u8,
    /// Relax switch: 0 = off, 1 = on
    pub relax_switch: u8,
}

impl Default for Backend {
    fn default() -> Self {
        Self {
            light_mode: LightMode::Off,
            water_level: 0,
            bright_switch: 0,
            relax_switch: 0,
        }
    }
}

// Global backend state - accessible across the crate
static BACKEND: Mutex<Option<Backend>> = Mutex::new(None);

/// Initialize the backend
pub fn init() {
    let mut backend = BACKEND.lock().unwrap();
    *backend = Some(Backend::default());
    info!("SenseCap backend initialized");
}

/// Get a reference to the backend state
pub fn with_backend<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut Backend) -> R,
{
    let mut backend = BACKEND.lock().unwrap();
    backend.as_mut().map(f)
}
