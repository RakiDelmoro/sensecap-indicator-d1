#ifndef WIFI_MANAGER_H
#define WIFI_MANAGER_H

#include <stdbool.h>

// WiFi initialization
void wifi_init(void);

// Connect to WiFi network
void wifi_connect(const char *ssid, const char *password);

// Get WiFi connection status
bool wifi_is_connected(void);

// Get IP address as string
const char* wifi_get_ip(void);

#endif // WIFI_MANAGER_H
