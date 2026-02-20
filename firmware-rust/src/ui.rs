use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use log::info;

use crate::backend::SharedBackend;
use crate::display::DisplayDriver;
use crate::mqtt::{MqttClient, MqttMessage};
use crate::touch::{TouchDriver, TouchState};

/// UI Manager handles the main event loop and coordinates
/// between display, touch, backend, and MQTT
pub struct UiManager {
    _display: DisplayDriver,
    _touch: TouchDriver,
    backend: SharedBackend,
}

impl UiManager {
    pub fn new(display: DisplayDriver, touch: TouchDriver, backend: SharedBackend) -> Result<Self> {
        info!("Initializing UI Manager");

        Ok(UiManager {
            _display: display,
            _touch: touch,
            backend,
        })
    }

    /// Main event loop - runs indefinitely
    pub fn run_event_loop(&self, mqtt: &MqttClient) -> Result<()> {
        info!("Starting main event loop...");

        // This is where LVGL would be integrated
        // For now, we'll use a simplified event loop

        loop {
            // Read touch input
            // let touch_state = self.touch.read()?;
            // self.process_touch(touch_state)?;

            // Process MQTT messages
            self.process_mqtt_messages(mqtt)?;

            // Update UI based on backend state
            self.update_ui()?;

            // Small delay to prevent busy-waiting
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    /// Process touch events
    fn process_touch(&self, _touch_state: TouchState) -> Result<()> {
        // Handle touch events
        // This would update switches, buttons, etc.
        Ok(())
    }

    /// Process MQTT messages
    fn process_mqtt_messages(&self, mqtt: &MqttClient) -> Result<()> {
        if let Ok(msg) = mqtt.get_receiver().try_recv() {
            match msg {
                MqttMessage::WaterLevel(level) => {
                    let mut backend = self.backend.lock().unwrap();
                    backend.update_water_level(level);
                    // self.update_water_level_display(level)?;
                }
                MqttMessage::Connected => {
                    info!("MQTT connected - UI updated");
                }
                MqttMessage::Disconnected => {
                    info!("MQTT disconnected - UI updated");
                }
            }
        }
        Ok(())
    }

    /// Update UI based on current backend state
    fn update_ui(&self) -> Result<()> {
        let backend = self.backend.lock().unwrap();

        // Update bright switch state
        let _bright_state = backend.get_bright_state();

        // Update relax switch state
        let _relax_state = backend.get_relax_state();

        // Update water level display
        let _water_level = backend.get_water_level();

        Ok(())
    }

    /// Set water level display (would be called from backend)
    pub fn set_water_level(&self, level: i32) {
        info!("[UI] Updating water level display: {}%", level);

        // Clamp level to 0-100
        let _level = level.clamp(0, 100) as u8;

        // Update the arc value
        // lv_arc_set_value(ui_WaterTankArc, level);

        // Update label text
        // lv_label_set_text(ui_WaterLevel, format!("{}", level));

        // Change arc color based on level
        // match level {
        //     0..=10 => lv_obj_set_style_arc_color(ui_WaterTankArc, RED, ...),
        //     11..=20 => lv_obj_set_style_arc_color(ui_WaterTankArc, ORANGE, ...),
        //     _ => lv_obj_set_style_arc_color(ui_WaterTankArc, BLUE, ...),
        // }
    }

    /// Set bright switch state (would be called from backend)
    pub fn set_bright_state(&self, state: bool) {
        info!("[UI] Setting bright state: {}", state);

        // Update switch state
        // if state {
        //     lv_obj_add_state(ui_BrightSwitch, LV_STATE_CHECKED);
        // } else {
        //     lv_obj_clear_state(ui_BrightSwitch, LV_STATE_CHECKED);
        // }
    }

    /// Set relax switch state (would be called from backend)
    pub fn set_relax_state(&self, state: bool) {
        info!("[UI] Setting relax state: {}", state);

        // Update switch state
        // if state {
        //     lv_obj_add_state(ui_RelaxSwitch, LV_STATE_CHECKED);
        // } else {
        //     lv_obj_clear_state(ui_RelaxSwitch, LV_STATE_CHECKED);
        // }
    }
}
