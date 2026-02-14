use log::info;
use std::sync::Mutex;

pub mod lights;
pub mod water;
pub mod ffi;

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
}

impl Default for Backend {
    fn default() -> Self {
        Self {
            light_mode: LightMode::Off,
            water_level: 0,
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
