use crate::ffi::trigger_ui_update;
use crate::with_backend;
use log::info;

/// MQTT topic for publishing light state
pub const MQTT_TOPIC_LIGHTS: &str = "sensecap/lights/state";

/// Set bright switch state (0 = off, 1 = on)
pub fn set_bright(state: u8) {
    let clamped = if state > 0 { 1 } else { 0 };
    with_backend(|backend| {
        backend.bright_switch = clamped;
        // Update light mode based on switch
        if clamped == 1 {
            backend.light_mode = crate::LightMode::Bright;
            backend.relax_switch = 0; // Turn off relax when bright is on
        } else if backend.relax_switch == 0 {
            backend.light_mode = crate::LightMode::Off;
        }
        info!("Bright switch set to: {}", clamped);
    });
    // Trigger UI update callback
    trigger_ui_update();
}

/// Set relax switch state (0 = off, 1 = on)
pub fn set_relax(state: u8) {
    let clamped = if state > 0 { 1 } else { 0 };
    with_backend(|backend| {
        backend.relax_switch = clamped;
        // Update light mode based on switch
        if clamped == 1 {
            backend.light_mode = crate::LightMode::Relax;
            backend.bright_switch = 0; // Turn off bright when relax is on
        } else if backend.bright_switch == 0 {
            backend.light_mode = crate::LightMode::Off;
        }
        info!("Relax switch set to: {}", clamped);
    });
    // Trigger UI update callback
    trigger_ui_update();
}

/// Get bright switch state (0 or 1)
pub fn get_bright() -> Option<u8> {
    with_backend(|backend| backend.bright_switch)
}

/// Get relax switch state (0 or 1)
pub fn get_relax() -> Option<u8> {
    with_backend(|backend| backend.relax_switch)
}

/// Toggle bright switch (0→1, 1→0), returns new state
/// NOTE: Does NOT call set_bright() to avoid deadlock
pub fn toggle_bright() -> Option<u8> {
    with_backend(|backend| {
        let current = backend.bright_switch;
        let new_state = if current == 0 { 1 } else { 0 };
        // Set directly (don't call set_bright to avoid double lock)
        backend.bright_switch = new_state;
        if new_state == 1 {
            backend.light_mode = crate::LightMode::Bright;
            backend.relax_switch = 0;
        } else if backend.relax_switch == 0 {
            backend.light_mode = crate::LightMode::Off;
        }
        info!("Bright toggled: {} -> {}", current, new_state);
        new_state
    })
}

/// Toggle relax switch (0→1, 1→0), returns new state
/// NOTE: Does NOT call set_relax() to avoid deadlock
pub fn toggle_relax() -> Option<u8> {
    with_backend(|backend| {
        let current = backend.relax_switch;
        let new_state = if current == 0 { 1 } else { 0 };
        // Set directly (don't call set_relax to avoid double lock)
        backend.relax_switch = new_state;
        if new_state == 1 {
            backend.light_mode = crate::LightMode::Relax;
            backend.bright_switch = 0;
        } else if backend.bright_switch == 0 {
            backend.light_mode = crate::LightMode::Off;
        }
        info!("Relax toggled: {} -> {}", current, new_state);
        new_state
    })
}

/// Get current light mode
pub fn get_light_mode() -> Option<crate::LightMode> {
    with_backend(|backend| backend.light_mode)
}

/// Set light mode directly
pub fn set_light_mode(mode: crate::LightMode) {
    with_backend(|backend| {
        backend.light_mode = mode;
        // Update switches to match mode
        match mode {
            crate::LightMode::Off => {
                backend.bright_switch = 0;
                backend.relax_switch = 0;
            }
            crate::LightMode::Bright => {
                backend.bright_switch = 1;
                backend.relax_switch = 0;
            }
            crate::LightMode::Relax => {
                backend.bright_switch = 0;
                backend.relax_switch = 1;
            }
        }
        info!("Light mode set to {:?}", mode);
    });
}

/// Legacy function: Handle bright toggle from C (maintains compatibility)
pub fn handle_bright_toggle() {
    toggle_bright();
}

/// Legacy function: Handle relax toggle from C (maintains compatibility)
pub fn handle_relax_toggle() {
    toggle_relax();
}
