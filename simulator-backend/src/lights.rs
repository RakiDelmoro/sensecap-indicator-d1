//! Light control - same API as embedded version

use crate::{mock_mqtt, with_backend};

pub fn set_bright(state: u8) {
    let clamped = if state > 0 { 1 } else { 0 };

    with_backend(|backend| {
        backend.bright_switch = clamped;

        // Mutual exclusion: turn off relax when bright is on
        if clamped == 1 {
            backend.relax_switch = 0;
            println!("[LIGHT] Mode: BRIGHT");
        } else if backend.relax_switch == 0 {
            println!("[LIGHT] Mode: OFF");
        }

        // PUBLISH to MQTT (mock)
        mock_mqtt::publish_lights(clamped, backend.relax_switch);
    });

    println!("[LIGHT] Bright switch set to: {}", clamped);
}

pub fn set_relax(state: u8) {
    let clamped = if state > 0 { 1 } else { 0 };

    with_backend(|backend| {
        backend.relax_switch = clamped;

        // Mutual exclusion: turn off bright when relax is on
        if clamped == 1 {
            backend.bright_switch = 0;
            println!("[LIGHT] Mode: RELAX");
        } else if backend.bright_switch == 0 {
            println!("[LIGHT] Mode: OFF");
        }

        // PUBLISH to MQTT (mock)
        mock_mqtt::publish_lights(backend.bright_switch, clamped);
    });

    println!("[LIGHT] Relax switch set to: {}", clamped);
}

pub fn get_bright() -> Option<u8> {
    with_backend(|backend| backend.bright_switch)
}

pub fn get_relax() -> Option<u8> {
    with_backend(|backend| backend.relax_switch)
}

/// Toggle bright switch (0→1, 1→0), returns new state
pub fn toggle_bright() -> Option<u8> {
    with_backend(|backend| {
        let current = backend.bright_switch;
        let new_state = if current == 0 { 1 } else { 0 };

        // Set directly (don't call set_bright to avoid double lock)
        backend.bright_switch = new_state;
        if new_state == 1 {
            backend.relax_switch = 0;
            println!("[LIGHT] Mode: BRIGHT");
        } else if backend.relax_switch == 0 {
            println!("[LIGHT] Mode: OFF");
        }

        println!("[LIGHT] Bright toggled: {} -> {}", current, new_state);

        // PUBLISH to MQTT (mock)
        mock_mqtt::publish_lights(new_state, backend.relax_switch);

        new_state
    })
}

/// Toggle relax switch (0→1, 1→0), returns new state
pub fn toggle_relax() -> Option<u8> {
    with_backend(|backend| {
        let current = backend.relax_switch;
        let new_state = if current == 0 { 1 } else { 0 };

        // Set directly (don't call set_relax to avoid double lock)
        backend.relax_switch = new_state;
        if new_state == 1 {
            backend.bright_switch = 0;
            println!("[LIGHT] Mode: RELAX");
        } else if backend.bright_switch == 0 {
            println!("[LIGHT] Mode: OFF");
        }

        println!("[LIGHT] Relax toggled: {} -> {}", current, new_state);

        // PUBLISH to MQTT (mock)
        mock_mqtt::publish_lights(backend.bright_switch, new_state);

        new_state
    })
}
