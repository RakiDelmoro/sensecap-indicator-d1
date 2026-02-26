use std::sync::{Arc, Mutex};

/// Backend state management for the SenseCAP Indicator D1
/// Handles light states, water level, and MQTT communication callbacks
pub struct Backend {
    bright_state: bool,
    relax_state: bool,
    water_level: u8,
    mqtt_publish_callback: Option<Arc<dyn Fn(&str, i32) + Send + Sync>>,
}

impl Backend {
    pub fn new() -> Self {
        Self {
            bright_state: false,
            relax_state: false,
            water_level: 50, // Default 50%
            mqtt_publish_callback: None,
        }
    }

    pub fn set_mqtt_callback<F>(&mut self, callback: F)
    where
        F: Fn(&str, i32) + Send + Sync + 'static,
    {
        self.mqtt_publish_callback = Some(Arc::new(callback));
    }

    /// Set the bright light state
    pub fn set_bright(&mut self, state: bool) {
        log::info!("[Backend] Bright state set to: {}", state);
        self.bright_state = state;

        // Mutual exclusion: if bright is on, turn off relax
        if state {
            self.relax_state = false;
        }

        // Publish to MQTT
        if let Some(ref callback) = self.mqtt_publish_callback {
            callback("bright", if state { 1 } else { 0 });
        }
    }

    /// Set the relax light state
    pub fn set_relax(&mut self, state: bool) {
        log::info!("[Backend] Relax state set to: {}", state);
        self.relax_state = state;

        // Mutual exclusion: if relax is on, turn off bright
        if state {
            self.bright_state = false;
        }

        // Publish to MQTT
        if let Some(ref callback) = self.mqtt_publish_callback {
            callback("relax", if state { 1 } else { 0 });
        }
    }

    /// Toggle the bright light state
    pub fn toggle_bright(&mut self) {
        let current = self.bright_state;
        self.set_bright(!current);
    }

    /// Toggle the relax light state
    pub fn toggle_relax(&mut self) {
        let current = self.relax_state;
        self.set_relax(!current);
    }

    /// Get the current bright state
    pub fn get_bright_state(&self) -> bool {
        self.bright_state
    }

    /// Get the current relax state
    pub fn get_relax_state(&self) -> bool {
        self.relax_state
    }

    /// Update water level from MQTT subscription
    pub fn update_water_level(&mut self, level: u8) {
        // Clamp level to 0-100
        let level = level.min(100);
        self.water_level = level;
        log::info!("[Backend] Water level updated to: {}%", level);
    }

    /// Get current water level
    pub fn get_water_level(&self) -> u8 {
        self.water_level
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe wrapper for Backend
pub type SharedBackend = Arc<Mutex<Backend>>;
