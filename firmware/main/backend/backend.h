/**
 * @file backend.h
 * @brief C Backend for SenseCAP Indicator D1
 *
 * This module provides the backend logic for the SenseCAP Indicator D1 firmware.
 * It handles:
 * - Light state management
 * - MQTT message processing
 * - Business logic
 *
 * Replaces the previous Rust backend with pure C implementation.
 */

#ifndef BACKEND_H
#define BACKEND_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Initialize the backend
 *
 * Must be called once before using any other backend functions.
 */
void backend_init(void);

/**
 * @brief Set the bright light state
 *
 * @param state 0 for off, 1 for on
 */
void backend_set_bright(uint8_t state);

/**
 * @brief Set the relax light state
 *
 * @param state 0 for off, 1 for on
 */
void backend_set_relax(uint8_t state);

/**
 * @brief Toggle the bright light state
 */
void backend_toggle_bright(void);

/**
 * @brief Toggle the relax light state
 */
void backend_toggle_relax(void);

/**
 * @brief Get the current bright state
 *
 * @return 0 if off, 1 if on
 */
uint8_t backend_get_bright_state(void);

/**
 * @brief Get the current relax state
 *
 * @return 0 if off, 1 if on
 */
uint8_t backend_get_relax_state(void);

/**
 * @brief Update water level from MQTT
 *
 * @param level Water level percentage (0-100)
 */
void backend_update_water_level(uint8_t level);

/**
 * @brief Get current water level
 *
 * @return Water level percentage (0-100)
 */
uint8_t backend_get_water_level(void);

/**
 * @brief Connect to WiFi (placeholder - actual WiFi managed in main)
 *
 * @param ssid WiFi SSID
 * @param password WiFi password
 */
void backend_wifi_connect(const char* ssid, const char* password);

/**
 * @brief Connect to MQTT broker (placeholder - actual MQTT managed in main)
 *
 * @param broker_url MQTT broker URL
 */
void backend_mqtt_connect(const char* broker_url);

#ifdef __cplusplus
}
#endif

#endif /* BACKEND_H */
