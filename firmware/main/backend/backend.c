/**
 * @file backend.c
 * @brief C Backend Implementation for SenseCAP Indicator D1
 *
 * This module provides the backend logic for the SenseCAP Indicator D1 firmware.
 * Replaces the previous Rust backend with pure C implementation.
 */

#include "backend.h"
#include <stdio.h>
#include <string.h>

// Static state storage - using simple static variables
// For thread safety in embedded systems, we can use critical sections if needed
static volatile uint8_t bright_state = 0;
static volatile uint8_t relax_state = 0;
static volatile uint8_t water_level = 50; // Default 50%

// External C callbacks - these are implemented in the UI layer
extern void ui_update_water_level_async(int level);
extern void ui_set_bright_state(int state);
extern void ui_set_relax_state(int state);
extern void publish_light_state(const char* mode, int state);

/**
 * @brief Initialize the backend
 *
 * Must be called once before using any other backend functions.
 */
void backend_init(void)
{
    bright_state = 0;
    relax_state = 0;
    water_level = 50;
    printf("[Backend] Initialized\n");
}

/**
 * @brief Set the bright light state
 *
 * @param state 0 for off, 1 for on
 */
void backend_set_bright(uint8_t state)
{
    bright_state = state;
    printf("[Backend] Bright state set to: %d\n", state);

    // If bright is on, turn off relax (mutual exclusion)
    if (state != 0) {
        relax_state = 0;
        ui_set_relax_state(0);
    }

    // Publish to MQTT
    publish_light_state("bright", state);
}

/**
 * @brief Set the relax light state
 *
 * @param state 0 for off, 1 for on
 */
void backend_set_relax(uint8_t state)
{
    relax_state = state;
    printf("[Backend] Relax state set to: %d\n", state);

    // If relax is on, turn off bright (mutual exclusion)
    if (state != 0) {
        bright_state = 0;
        ui_set_bright_state(0);
    }

    // Publish to MQTT
    publish_light_state("relax", state);
}

/**
 * @brief Toggle the bright light state
 */
void backend_toggle_bright(void)
{
    uint8_t current = backend_get_bright_state();
    backend_set_bright(current == 0 ? 1 : 0);
}

/**
 * @brief Toggle the relax light state
 */
void backend_toggle_relax(void)
{
    uint8_t current = backend_get_relax_state();
    backend_set_relax(current == 0 ? 1 : 0);
}

/**
 * @brief Get the current bright state
 *
 * @return 0 if off, 1 if on
 */
uint8_t backend_get_bright_state(void)
{
    return bright_state;
}

/**
 * @brief Get the current relax state
 *
 * @return 0 if off, 1 if on
 */
uint8_t backend_get_relax_state(void)
{
    return relax_state;
}

/**
 * @brief Update water level from MQTT subscription
 *
 * @param level Water level percentage (0-100)
 */
void backend_update_water_level(uint8_t level)
{
    // Clamp level to 0-100
    if (level > 100) {
        level = 100;
    }
    water_level = level;
    printf("[Backend] Water level updated to: %d%%\n", level);

    // Update UI
    ui_update_water_level_async((int)level);
}

/**
 * @brief Get current water level
 *
 * @return Water level percentage (0-100)
 */
uint8_t backend_get_water_level(void)
{
    return water_level;
}

/**
 * @brief Connect to WiFi (placeholder - actual WiFi managed in main)
 *
 * @param ssid WiFi SSID
 * @param password WiFi password
 */
void backend_wifi_connect(const char* ssid, const char* password)
{
    // WiFi connection is handled by ESP-IDF in main.c
    // This is a placeholder for any backend-side WiFi logic
    (void)ssid;
    (void)password;
    printf("[Backend] WiFi connect placeholder called\n");
}

/**
 * @brief Connect to MQTT broker (placeholder - actual MQTT managed in main)
 *
 * @param broker_url MQTT broker URL
 */
void backend_mqtt_connect(const char* broker_url)
{
    // MQTT connection is handled by ESP-IDF in main.c
    // This is a placeholder for any backend-side MQTT logic
    (void)broker_url;
    printf("[Backend] MQTT connect placeholder called\n");
}
