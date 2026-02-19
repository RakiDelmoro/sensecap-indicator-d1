#include "display_driver.h"
#include "lvgl.h"
#include "esp_lcd_panel_io.h"
#include "esp_lcd_panel_vendor.h"
#include "esp_lcd_panel_ops.h"
#include "driver/gpio.h"
#include "driver/spi_master.h"
#include "esp_log.h"

static const char *TAG = "DISPLAY";

// Display pins for SenseCAP Indicator D1 (ESP32-S3)
#define LCD_PIN_NUM_MOSI  11
#define LCD_PIN_NUM_CLK   12
#define LCD_PIN_NUM_CS    10
#define LCD_PIN_NUM_DC    13
#define LCD_PIN_NUM_RST   9
#define LCD_PIN_NUM_BL    14  // Backlight

static lv_disp_drv_t disp_drv;
static lv_disp_draw_buf_t draw_buf;

// Display buffers - use PSRAM if available, otherwise internal RAM
static lv_color_t *buf1 = NULL;
static lv_color_t *buf2 = NULL;

static esp_lcd_panel_handle_t panel_handle = NULL;

void display_init(void)
{
    ESP_LOGI(TAG, "Initializing display hardware");
    
    // Configure backlight GPIO
    gpio_config_t bk_gpio_config = {
        .mode = GPIO_MODE_OUTPUT,
        .pin_bit_mask = 1ULL << LCD_PIN_NUM_BL
    };
    ESP_ERROR_CHECK(gpio_config(&bk_gpio_config));
    
    // Configure reset GPIO
    gpio_config_t rst_gpio_config = {
        .mode = GPIO_MODE_OUTPUT,
        .pin_bit_mask = 1ULL << LCD_PIN_NUM_RST
    };
    ESP_ERROR_CHECK(gpio_config(&rst_gpio_config));
    
    // Reset display
    gpio_set_level(LCD_PIN_NUM_RST, 0);
    vTaskDelay(pdMS_TO_TICKS(100));
    gpio_set_level(LCD_PIN_NUM_RST, 1);
    vTaskDelay(pdMS_TO_TICKS(100));
    
    // Initialize SPI bus
    spi_bus_config_t bus_config = {
        .sclk_io_num = LCD_PIN_NUM_CLK,
        .mosi_io_num = LCD_PIN_NUM_MOSI,
        .miso_io_num = -1,  // Not used
        .quadwp_io_num = -1,
        .quadhd_io_num = -1,
        .max_transfer_sz = DISP_HOR_RES * 20 * sizeof(uint16_t)
    };
    ESP_ERROR_CHECK(spi_bus_initialize(SPI2_HOST, &bus_config, SPI_DMA_CH_AUTO));
    
    // Initialize panel IO
    esp_lcd_panel_io_spi_config_t io_config = {
        .dc_gpio_num = LCD_PIN_NUM_DC,
        .cs_gpio_num = LCD_PIN_NUM_CS,
        .pclk_hz = 40 * 1000 * 1000,  // 40 MHz
        .lcd_cmd_bits = 8,
        .lcd_param_bits = 8,
        .spi_mode = 0,
        .trans_queue_depth = 10,
    };
    
    esp_lcd_panel_io_handle_t io_handle = NULL;
    ESP_ERROR_CHECK(esp_lcd_new_panel_io_spi((esp_lcd_spi_bus_handle_t)SPI2_HOST, &io_config, &io_handle));
    
    // Initialize panel (ST7701S driver for SenseCAP Indicator)
    esp_lcd_panel_dev_config_t panel_config = {
        .reset_gpio_num = LCD_PIN_NUM_RST,
        .color_space = ESP_LCD_COLOR_SPACE_RGB,
        .bits_per_pixel = 16,
    };
    ESP_ERROR_CHECK(esp_lcd_new_panel_st7701(io_handle, &panel_config, &panel_handle));
    
    // Initialize panel
    ESP_ERROR_CHECK(esp_lcd_panel_reset(panel_handle));
    ESP_ERROR_CHECK(esp_lcd_panel_init(panel_handle));
    ESP_ERROR_CHECK(esp_lcd_panel_disp_on_off(panel_handle, true));
    
    // Turn on backlight
    gpio_set_level(LCD_PIN_NUM_BL, 1);
    
    ESP_LOGI(TAG, "Display hardware initialized");
}

void display_flush_cb(lv_disp_drv_t *drv, const lv_area_t *area, lv_color_t *color_map)
{
    esp_lcd_panel_draw_bitmap(panel_handle, 
                              area->x1, area->y1, 
                              area->x2 + 1, area->y2 + 1, 
                              color_map);
    lv_disp_flush_ready(drv);
}

void display_driver_init(void)
{
    ESP_LOGI(TAG, "Initializing LVGL display driver");
    
    // Allocate display buffers
    size_t buffer_size = DISP_HOR_RES * 20;  // 20 lines buffer
    
    // Try to use internal RAM first
    buf1 = heap_caps_malloc(buffer_size * sizeof(lv_color_t), MALLOC_CAP_INTERNAL | MALLOC_CAP_8BIT);
    if (buf1 == NULL) {
        ESP_LOGE(TAG, "Failed to allocate buffer 1");
        return;
    }
    
    buf2 = heap_caps_malloc(buffer_size * sizeof(lv_color_t), MALLOC_CAP_INTERNAL | MALLOC_CAP_8BIT);
    if (buf2 == NULL) {
        ESP_LOGW(TAG, "Failed to allocate buffer 2, using single buffer");
        buf2 = NULL;
    }
    
    // Initialize draw buffer
    lv_disp_draw_buf_init(&draw_buf, buf1, buf2, buffer_size);
    
    // Initialize display driver
    lv_disp_drv_init(&disp_drv);
    disp_drv.hor_res = DISP_HOR_RES;
    disp_drv.ver_res = DISP_VER_RES;
    disp_drv.flush_cb = display_flush_cb;
    disp_drv.draw_buf = &draw_buf;
    disp_drv.full_refresh = 0;
    disp_drv.direct_mode = 0;
    lv_disp_drv_register(&disp_drv);
    
    ESP_LOGI(TAG, "LVGL display driver initialized: %dx%d", DISP_HOR_RES, DISP_VER_RES);
}
