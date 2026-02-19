/**
 * SenseCAP Indicator D1 Firmware
 * 
 * ESP32-S3 based firmware with LVGL UI and C backend
 * Handles WiFi connectivity, MQTT communication, and display management
 */

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "freertos/event_groups.h"
#include "esp_system.h"
#include "esp_log.h"
#include "nvs_flash.h"
#include "esp_wifi.h"
#include "esp_event.h"
#include "mqtt_client.h"

#include "lvgl.h"
#include "ui.h"
#include "display_driver.h"
#include "touch_driver.h"
#include "wifi_manager.h"
#include "backend.h"

static const char *TAG = "SENSECAP_FW";

// Event group for WiFi and MQTT status
#define WIFI_CONNECTED_BIT  BIT0
#define MQTT_CONNECTED_BIT   BIT1
static EventGroupHandle_t s_network_event_group;

// UI update callbacks from backend
void ui_update_water_level_async(int level);
void ui_set_bright_state(int state);
void ui_set_relax_state(int state);

// MQTT topics
#define MQTT_TOPIC_LIGHT_STATE "sensecap/indicator/light/state"
#define MQTT_TOPIC_WATER_LEVEL "sensecap/indicator/water/level"

static esp_mqtt_client_handle_t mqtt_client = NULL;

// MQTT event handler
static void mqtt_event_handler(void *handler_args, esp_event_base_t base, int32_t event_id, void *event_data)
{
    esp_mqtt_event_handle_t event = event_data;
    
    switch ((esp_mqtt_event_id_t)event_id) {
        case MQTT_EVENT_CONNECTED:
            ESP_LOGI(TAG, "MQTT connected");
            xEventGroupSetBits(s_network_event_group, MQTT_CONNECTED_BIT);
            // Subscribe to water level topic
            esp_mqtt_client_subscribe(mqtt_client, MQTT_TOPIC_WATER_LEVEL, 1);
            break;
            
        case MQTT_EVENT_DISCONNECTED:
            ESP_LOGI(TAG, "MQTT disconnected");
            xEventGroupClearBits(s_network_event_group, MQTT_CONNECTED_BIT);
            break;
            
        case MQTT_EVENT_DATA:
            ESP_LOGI(TAG, "MQTT data received: topic=%.*s, data=%.*s", 
                     event->topic_len, event->topic, 
                     event->data_len, event->data);
            
            // Handle water level updates
            if (strncmp(event->topic, MQTT_TOPIC_WATER_LEVEL, event->topic_len) == 0) {
                char data_str[16];
                int len = event->data_len < 15 ? event->data_len : 15;
                memcpy(data_str, event->data, len);
                data_str[len] = '\0';
                int water_level = atoi(data_str);
                ui_update_water_level_async(water_level);
            }
            break;
            
        case MQTT_EVENT_ERROR:
            ESP_LOGE(TAG, "MQTT error occurred");
            break;
            
        default:
            break;
    }
}

// Initialize MQTT client
static void mqtt_init(void)
{
    esp_mqtt_client_config_t mqtt_cfg = {
        .broker.address.uri = CONFIG_MQTT_BROKER_URL,
        .credentials.client_id = "sensecap_indicator_d1",
        .session.keepalive = 60,
    };
    
    // Add authentication if username is configured
    if (strlen(CONFIG_MQTT_USERNAME) > 0) {
        mqtt_cfg.credentials.username = CONFIG_MQTT_USERNAME;
        mqtt_cfg.credentials.authentication.password = CONFIG_MQTT_PASSWORD;
        ESP_LOGI(TAG, "MQTT using authentication with username: %s", CONFIG_MQTT_USERNAME);
    }
    
    mqtt_client = esp_mqtt_client_init(&mqtt_cfg);
    esp_mqtt_client_register_event(mqtt_client, ESP_EVENT_ANY_ID, mqtt_event_handler, NULL);
    esp_mqtt_client_start(mqtt_client);
}

// Publish light state to MQTT
void publish_light_state(const char* mode, int state)
{
    if (mqtt_client == NULL) return;
    
    char payload[64];
    snprintf(payload, sizeof(payload), "{\"mode\":\"%s\",\"state\":%d}", mode, state);
    esp_mqtt_client_publish(mqtt_client, MQTT_TOPIC_LIGHT_STATE, payload, 0, 1, 0);
}

// LVGL task - handles rendering
static void lvgl_task(void *pvParameter)
{
    ESP_LOGI(TAG, "LVGL task started");
    
    while (1) {
        uint32_t time_till_next = lv_timer_handler();
        if (time_till_next > 0) {
            vTaskDelay(pdMS_TO_TICKS(time_till_next));
        } else {
            vTaskDelay(pdMS_TO_TICKS(1));
        }
    }
}

// Network status task
static void network_status_task(void *pvParameter)
{
    while (1) {
        EventBits_t bits = xEventGroupGetBits(s_network_event_group);
        
        bool wifi_connected = (bits & WIFI_CONNECTED_BIT) != 0;
        bool mqtt_connected = (bits & MQTT_CONNECTED_BIT) != 0;
        
        ESP_LOGD(TAG, "Network status: WiFi=%s, MQTT=%s",
                 wifi_connected ? "connected" : "disconnected",
                 mqtt_connected ? "connected" : "disconnected");
        
        vTaskDelay(pdMS_TO_TICKS(5000));
    }
}

// Initialize NVS
static esp_err_t nvs_init(void)
{
    esp_err_t ret = nvs_flash_init();
    if (ret == ESP_ERR_NVS_NO_FREE_PAGES || ret == ESP_ERR_NVS_NEW_VERSION_FOUND) {
        ESP_ERROR_CHECK(nvs_flash_erase());
        ret = nvs_flash_init();
    }
    return ret;
}

void app_main(void)
{
    ESP_LOGI(TAG, "======================================");
    ESP_LOGI(TAG, "SenseCAP Indicator D1 Firmware v1.0");
    ESP_LOGI(TAG, "======================================");
    
    // Initialize NVS
    ESP_ERROR_CHECK(nvs_init());
    
    // Initialize network event group
    s_network_event_group = xEventGroupCreate();
    
    // Initialize display
    ESP_LOGI(TAG, "Initializing display...");
    display_init();
    
    // Initialize touch
    ESP_LOGI(TAG, "Initializing touch...");
    touch_init();
    
    // Initialize LVGL
    ESP_LOGI(TAG, "Initializing LVGL...");
    lv_init();
    
    // Initialize display driver for LVGL
    display_driver_init();
    
    // Initialize touch driver for LVGL
    touch_driver_init();
    
    // Initialize UI
    ESP_LOGI(TAG, "Initializing UI...");
    ui_init();
    
    // Initialize WiFi
    ESP_LOGI(TAG, "Initializing WiFi...");
    wifi_init();
    wifi_connect(CONFIG_WIFI_SSID, CONFIG_WIFI_PASSWORD);
    
    // Wait for WiFi connection
    ESP_LOGI(TAG, "Waiting for WiFi connection...");
    xEventGroupWaitBits(s_network_event_group, WIFI_CONNECTED_BIT, pdFALSE, pdTRUE, portMAX_DELAY);
    ESP_LOGI(TAG, "WiFi connected!");
    
    // Initialize MQTT
    ESP_LOGI(TAG, "Initializing MQTT...");
    mqtt_init();
    
    // Initialize backend
    ESP_LOGI(TAG, "Initializing backend...");
    backend_init();
    
    // Create tasks
    ESP_LOGI(TAG, "Creating tasks...");
    xTaskCreatePinnedToCore(lvgl_task, "lvgl_task", 4096, NULL, 5, NULL, 1);
    xTaskCreatePinnedToCore(network_status_task, "network_status", 2048, NULL, 3, NULL, 0);
    
    ESP_LOGI(TAG, "Setup complete!");
    ESP_LOGI(TAG, "Display: 480x480, Touch: enabled");
    ESP_LOGI(TAG, "MQTT broker: %s", CONFIG_MQTT_BROKER_URL);
    
    // Main task can now exit, other tasks handle the work
    vTaskDelete(NULL);
}
