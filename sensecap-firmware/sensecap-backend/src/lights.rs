use crate::{with_backend, LightMode};
use log::info;

/// Toggle bright mode
pub fn handle_bright_toggle() {
    with_backend(|backend| match backend.light_mode {
        LightMode::Bright => {
            backend.light_mode = LightMode::Off;
            info!("Bright mode turned off");
        }
        _ => {
            backend.light_mode = LightMode::Bright;
            info!("Bright mode activated");
        }
    });
}

/// Toggle relax mode
pub fn handle_relax_toggle() {
    with_backend(|backend| match backend.light_mode {
        LightMode::Relax => {
            backend.light_mode = LightMode::Off;
            info!("Relax mode turned off");
        }
        _ => {
            backend.light_mode = LightMode::Relax;
            info!("Relax mode activated");
        }
    });
}

/// Get current light mode
pub fn get_light_mode() -> Option<LightMode> {
    with_backend(|backend| backend.light_mode)
}

/// Set light mode directly
pub fn set_light_mode(mode: LightMode) {
    with_backend(|backend| {
        backend.light_mode = mode;
        info!("Light mode set to {:?}", mode);
    });
}
