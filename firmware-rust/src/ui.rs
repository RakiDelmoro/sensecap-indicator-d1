use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use log::info;

use crate::backend::SharedBackend;
use crate::display::DisplayDriver;
use crate::lvgl_ui::{LvglUi, UiAction};
use crate::mqtt::{MqttClient, MqttMessage};
use crate::touch::TouchDriver;

/// UI Manager handles the main event loop and coordinates
/// between display, touch, backend, and MQTT
pub struct UiManager {
    display: DisplayDriver,
    touch: TouchDriver,
    backend: SharedBackend,
    lvgl_ui: LvglUi,
    last_flush: std::time::Instant,
}

impl UiManager {
    pub fn new(display: DisplayDriver, touch: TouchDriver, backend: SharedBackend) -> Result<Self> {
        info!("Initializing UI Manager");

        Ok(UiManager {
            display,
            touch,
            backend,
            lvgl_ui: LvglUi::new(),
            last_flush: std::time::Instant::now(),
        })
    }

    /// Main event loop - runs indefinitely
    pub fn run_event_loop(&mut self, mqtt: &mut MqttClient) -> Result<()> {
        info!("Starting main event loop...");

        // Initial draw
        self.update_ui_from_backend();
        self.lvgl_ui.draw(&mut self.display)?;

        let mut last_touch_state = false;

        loop {
            // Read touch input
            match self.touch.read() {
                Ok(touch_state) => {
                    if touch_state.pressed {
                        if let Some(action) =
                            self.lvgl_ui.handle_touch(touch_state.x, touch_state.y)
                        {
                            self.process_ui_action(action, mqtt)?;
                        }
                        last_touch_state = true;
                    } else if last_touch_state {
                        // Touch released
                        last_touch_state = false;
                    }
                }
                Err(e) => {
                    log::error!("Touch read error: {:?}", e);
                }
            }

            // Process MQTT messages
            if let Err(e) = self.process_mqtt_messages(mqtt) {
                log::error!("MQTT processing error: {:?}", e);
            }

            // Update UI from backend state
            if let Err(e) = self.update_ui_from_backend() {
                log::error!("UI update error: {:?}", e);
            }

            // Draw UI if needed
            if self.lvgl_ui.needs_redraw() {
                if let Err(e) = self.lvgl_ui.draw(&mut self.display) {
                    log::error!("UI draw error: {:?}", e);
                }
            }

            // Periodic flush (30 FPS)
            let now = std::time::Instant::now();
            if now.duration_since(self.last_flush).as_millis() >= 33 {
                if let Err(e) = self.display.flush(0, 0, 480, 480) {
                    log::error!("Display flush error: {:?}", e);
                }
                self.last_flush = now;
            }

            // Small delay to prevent busy-waiting
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    /// Process UI actions
    fn process_ui_action(&mut self, action: UiAction, mqtt: &MqttClient) -> Result<()> {
        match action {
            UiAction::ToggleBright => {
                info!("UI Action: Toggle Bright");
                let mut backend = self.backend.lock().unwrap();
                backend.toggle_bright();

                // Update UI state
                let state = backend.get_bright_state();
                drop(backend);
                self.lvgl_ui.set_bright_state(state);

                // Publish to MQTT
                if let Err(e) = mqtt.try_publish_light_state("bright", state) {
                    log::error!("Failed to publish bright state: {:?}", e);
                }
            }
            UiAction::ToggleRelax => {
                info!("UI Action: Toggle Relax");
                let mut backend = self.backend.lock().unwrap();
                backend.toggle_relax();

                // Update UI state
                let state = backend.get_relax_state();
                drop(backend);
                self.lvgl_ui.set_relax_state(state);

                // Publish to MQTT
                if let Err(e) = mqtt.try_publish_light_state("relax", state) {
                    log::error!("Failed to publish relax state: {:?}", e);
                }
            }
        }
        Ok(())
    }

    /// Process MQTT messages
    fn process_mqtt_messages(&self, mqtt: &MqttClient) -> Result<()> {
        if let Ok(msg) = mqtt.get_receiver().try_recv() {
            match msg {
                MqttMessage::WaterLevel(level) => {
                    let mut backend = self.backend.lock().unwrap();
                    backend.update_water_level(level);
                    info!("Water level updated from MQTT: {}%", level);
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
    fn update_ui_from_backend(&mut self) -> Result<()> {
        let backend = self.backend.lock().unwrap();

        // Get current states
        let bright_state = backend.get_bright_state();
        let relax_state = backend.get_relax_state();
        let water_level = backend.get_water_level();

        // Update LVGL UI
        self.lvgl_ui.set_bright_state(bright_state);
        self.lvgl_ui.set_relax_state(relax_state);
        self.lvgl_ui.set_water_level(water_level);

        Ok(())
    }
}
