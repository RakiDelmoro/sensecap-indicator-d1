//! Mock MQTT Client - Simulates incoming water level messages

use crate::ffi::ui_update_water_level_async;
use crate::{water, with_backend};
use rand::Rng;
use std::thread;
use std::time::Duration;

/// Start mock MQTT subscriber thread
/// Generates water level every 5 seconds (0-100)
pub fn start_subscriber() {
    thread::spawn(|| {
        println!("[MQTT] Connected to broker (MOCK)");
        println!("[MQTT] Subscribed to: sensecap/water/level");

        let mut rng = rand::thread_rng();

        loop {
            // Generate random water level 0-100
            let level: u8 = rng.gen_range(0..=100);

            println!("[MQTT] ← RECEIVED: sensecap/water/level = {}", level);

            // Update backend
            water::set_level(level);

            // PUSH to C UI - Schedule async update from main LVGL thread
            unsafe {
                ui_update_water_level_async(level as i32);
            }
            println!("[UI] Water level async update scheduled: {}%", level);

            // Get current light states for display
            let (bright, relax) =
                with_backend(|b| (b.bright_switch, b.relax_switch)).unwrap_or((0, 0));

            println!(
                "[STATE] Water: {}%, Bright: {}, Relax: {}",
                level, bright, relax
            );

            thread::sleep(Duration::from_secs(5));
        }
    });
}

/// Publish lights state to MQTT (logs to console)
pub fn publish_lights(bright: u8, relax: u8) {
    println!(
        "[MQTT] → PUBLISH: sensecap/lights/state = {{\"bright\":{},\"relax\":{}}}",
        bright, relax
    );
}

/// Publish water level to MQTT (logs to console)  
pub fn publish_water(level: u8) {
    println!("[MQTT] → PUBLISH: sensecap/water/level = {}", level);
}
