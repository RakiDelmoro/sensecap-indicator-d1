#include "touch_driver.h"
#include "lvgl.h"
#include "driver/i2c.h"
#include "driver/gpio.h"
#include "esp_log.h"

static const char *TAG = "TOUCH";

// Touch controller pins (GT911 for SenseCAP Indicator D1)
// From official SDK: sensecap_indicator_board.c
#define TOUCH_I2C_NUM       I2C_NUM_0
#define TOUCH_PIN_NUM_SDA   39  // GPIO_I2C_SDA
#define TOUCH_PIN_NUM_SCL   40  // GPIO_I2C_SCL
#define TOUCH_PIN_NUM_INT   3
#define TOUCH_PIN_NUM_RST   2

// GT911 registers
#define GT911_REG_X_LOW     0x8140
#define GT911_REG_X_HIGH    0x8141
#define GT911_REG_Y_LOW     0x8142
#define GT911_REG_Y_HIGH    0x8143
#define GT911_REG_STATUS    0x814E
#define GT911_REG_POINTS    0x814F

static lv_indev_drv_t indev_drv;
static int16_t last_x = 0;
static int16_t last_y = 0;
static bool last_pressed = false;

// I2C read function
static esp_err_t gt911_read(uint16_t reg, uint8_t *data, size_t len)
{
    i2c_cmd_handle_t cmd = i2c_cmd_link_create();
    i2c_master_start(cmd);
    i2c_master_write_byte(cmd, (0x5D << 1) | I2C_MASTER_WRITE, true);
    i2c_master_write_byte(cmd, reg >> 8, true);
    i2c_master_write_byte(cmd, reg & 0xFF, true);
    i2c_master_start(cmd);
    i2c_master_write_byte(cmd, (0x5D << 1) | I2C_MASTER_READ, true);
    if (len > 1) {
        i2c_master_read(cmd, data, len - 1, I2C_MASTER_ACK);
    }
    i2c_master_read_byte(cmd, data + len - 1, I2C_MASTER_NACK);
    i2c_master_stop(cmd);
    esp_err_t ret = i2c_master_cmd_begin(TOUCH_I2C_NUM, cmd, pdMS_TO_TICKS(1000));
    i2c_cmd_link_delete(cmd);
    return ret;
}

void touch_init(void)
{
    ESP_LOGI(TAG, "Initializing touch hardware");
    
    // Configure reset and interrupt GPIOs
    gpio_config_t rst_gpio_config = {
        .mode = GPIO_MODE_OUTPUT,
        .pin_bit_mask = 1ULL << TOUCH_PIN_NUM_RST
    };
    ESP_ERROR_CHECK(gpio_config(&rst_gpio_config));
    
    gpio_config_t int_gpio_config = {
        .mode = GPIO_MODE_INPUT,
        .pin_bit_mask = 1ULL << TOUCH_PIN_NUM_INT,
        .pull_up_en = GPIO_PULLUP_ENABLE
    };
    ESP_ERROR_CHECK(gpio_config(&int_gpio_config));
    
    // Reset touch controller
    gpio_set_level(TOUCH_PIN_NUM_RST, 0);
    vTaskDelay(pdMS_TO_TICKS(10));
    gpio_set_level(TOUCH_PIN_NUM_RST, 1);
    vTaskDelay(pdMS_TO_TICKS(100));
    
    // Initialize I2C
    i2c_config_t i2c_conf = {
        .mode = I2C_MODE_MASTER,
        .sda_io_num = TOUCH_PIN_NUM_SDA,
        .scl_io_num = TOUCH_PIN_NUM_SCL,
        .sda_pullup_en = GPIO_PULLUP_ENABLE,
        .scl_pullup_en = GPIO_PULLUP_ENABLE,
        .master.clk_speed = 400000,
    };
    
    ESP_ERROR_CHECK(i2c_param_config(TOUCH_I2C_NUM, &i2c_conf));
    ESP_ERROR_CHECK(i2c_driver_install(TOUCH_I2C_NUM, I2C_MODE_MASTER, 0, 0, 0));
    
    ESP_LOGI(TAG, "Touch hardware initialized");
}

void touch_read_cb(lv_indev_drv_t *drv, lv_indev_data_t *data)
{
    uint8_t status = 0;
    
    // Read touch status
    if (gt911_read(GT911_REG_STATUS, &status, 1) != ESP_OK) {
        data->state = LV_INDEV_STATE_RELEASED;
        return;
    }
    
    // Check if touch is detected
    if (status & 0x80) {
        uint8_t buf[4];
        if (gt911_read(GT911_REG_X_LOW, buf, 4) == ESP_OK) {
            int16_t x = buf[0] | (buf[1] << 8);
            int16_t y = buf[2] | (buf[3] << 8);
            
            // Transform coordinates (display is 480x480)
            last_x = x;
            last_y = y;
            last_pressed = true;
            
            data->point.x = last_x;
            data->point.y = last_y;
            data->state = LV_INDEV_STATE_PRESSED;
        } else {
            data->state = LV_INDEV_STATE_RELEASED;
        }
        
        // Clear status
        uint8_t clear = 0;
        i2c_cmd_handle_t cmd = i2c_cmd_link_create();
        i2c_master_start(cmd);
        i2c_master_write_byte(cmd, (0x5D << 1) | I2C_MASTER_WRITE, true);
        i2c_master_write_byte(cmd, GT911_REG_STATUS >> 8, true);
        i2c_master_write_byte(cmd, GT911_REG_STATUS & 0xFF, true);
        i2c_master_write_byte(cmd, clear, true);
        i2c_master_stop(cmd);
        i2c_master_cmd_begin(TOUCH_I2C_NUM, cmd, pdMS_TO_TICKS(1000));
        i2c_cmd_link_delete(cmd);
    } else {
        last_pressed = false;
        data->state = LV_INDEV_STATE_RELEASED;
    }
}

void touch_driver_init(void)
{
    ESP_LOGI(TAG, "Initializing LVGL touch driver");
    
    lv_indev_drv_init(&indev_drv);
    indev_drv.type = LV_INDEV_TYPE_POINTER;
    indev_drv.read_cb = touch_read_cb;
    lv_indev_drv_register(&indev_drv);
    
    ESP_LOGI(TAG, "LVGL touch driver initialized");
}
