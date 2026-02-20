// Simulator backend - mock implementation for PC testing
// This replaces the Rust backend for pure C implementation

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "backend.h"

static int water_level = 75;  // Simulated water tank level (0-100)
static bool light_bright = false;
static bool light_relax = false;

void backend_init(void) {
    printf("[SIMULATOR] Backend initialized\n");
    printf("[SIMULATOR] Mock WiFi: Connected to 'Simulator-Network'\n");
    printf("[SIMULATOR] Mock MQTT: Connected to localhost:1883\n");
}

void backend_set_light_mode(light_mode_t mode) {
    switch(mode) {
        case LIGHT_MODE_BRIGHT:
            light_bright = true;
            light_relax = false;
            printf("[SIMULATOR] Light mode: BRIGHT\n");
            break;
        case LIGHT_MODE_RELAX:
            light_bright = false;
            light_relax = true;
            printf("[SIMULATOR] Light mode: RELAX\n");
            break;
        case LIGHT_MODE_OFF:
            light_bright = false;
            light_relax = false;
            printf("[SIMULATOR] Light mode: OFF\n");
            break;
    }
}

int backend_get_water_level(void) {
    // Simulate slowly changing water level
    static int direction = -1;
    water_level += direction;
    if (water_level <= 10) direction = 1;
    if (water_level >= 95) direction = -1;
    return water_level;
}

bool backend_is_wifi_connected(void) {
    return true;  // Always connected in simulator
}

bool backend_is_mqtt_connected(void) {
    return true;  // Always connected in simulator
}

void backend_loop(void) {
    // Simulator loop - can add periodic tasks here
    static int counter = 0;
    counter++;
    if (counter % 60 == 0) {  // Every ~1 second at 60fps
        printf("[SIMULATOR] Water level: %d%%\n", water_level);
    }
}
