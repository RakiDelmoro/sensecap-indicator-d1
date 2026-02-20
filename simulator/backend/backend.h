#ifndef BACKEND_H
#define BACKEND_H

#include <stdbool.h>

// Light mode enum
typedef enum {
    LIGHT_MODE_OFF = 0,
    LIGHT_MODE_BRIGHT = 1,
    LIGHT_MODE_RELAX = 2
} light_mode_t;

// Initialize backend (WiFi, MQTT, etc.)
void backend_init(void);

// Set light mode
void backend_set_light_mode(light_mode_t mode);

// Get current water tank level (0-100)
int backend_get_water_level(void);

// Check connection status
bool backend_is_wifi_connected(void);
bool backend_is_mqtt_connected(void);

// Main loop - call this regularly
void backend_loop(void);

#endif // BACKEND_H
