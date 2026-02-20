#include "display_driver.h"
#include "lvgl.h"
#include "esp_lcd_panel_io.h"
#include "esp_lcd_panel_rgb.h"
#include "esp_lcd_panel_ops.h"
#include "driver/gpio.h"
#include "driver/i2c.h"
#include "esp_log.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "esp_rom_sys.h"
#include <string.h>

static const char *TAG = "DISPLAY";

// =============================================================================
// OFFICIAL SDK REFERENCE: components/bsp/src/boards/sensecap_indicator_board.c
// =============================================================================

// RGB Interface GPIOs - 16-bit parallel data bus
// From: sensecap_indicator_board.c GPIO configuration
#define LCD_GPIO_DATA0   15  // B0
#define LCD_GPIO_DATA1   14  // B1
#define LCD_GPIO_DATA2   13  // B2
#define LCD_GPIO_DATA3   12  // B3
#define LCD_GPIO_DATA4   11  // B4
#define LCD_GPIO_DATA5   10  // G0
#define LCD_GPIO_DATA6    9  // G1
#define LCD_GPIO_DATA7    8  // G2
#define LCD_GPIO_DATA8    7  // G3
#define LCD_GPIO_DATA9    6  // G4
#define LCD_GPIO_DATA10   5  // G5
#define LCD_GPIO_DATA11   4  // R0
#define LCD_GPIO_DATA12   3  // R1
#define LCD_GPIO_DATA13   2  // R2
#define LCD_GPIO_DATA14   1  // R3
#define LCD_GPIO_DATA15   0  // R4

#define LCD_GPIO_VSYNC   17
#define LCD_GPIO_HSYNC   16
#define LCD_GPIO_DE      18
#define LCD_GPIO_PCLK    21
#define LCD_GPIO_BL      45  // Backlight (active high)

// Display timing parameters for 480x480 ST7701
// From: sensecap_indicator_board.c timing configuration
#define HSYNC_BACK_PORCH     50
#define HSYNC_FRONT_PORCH    10
#define HSYNC_PULSE_WIDTH     8
#define VSYNC_BACK_PORCH     50
#define VSYNC_FRONT_PORCH    10
#define VSYNC_PULSE_WIDTH     8
#define LCD_FREQ         (16000000)  // 16MHz

// =============================================================================
// OFFICIAL SDK REFERENCE: components/bsp/src/boards/lcd_panel_config.c
// =============================================================================
// SPI pins for ST7701S initialization (bit-banging via IO expander)
// The ST7701S requires SPI initialization before RGB interface works

// TCA9535 IO Expander configuration
// From: sensecap_indicator_board.c and lcd_panel_config.c
#define TCA9535_I2C_ADDR        0x39    // IO Expander I2C address
#define EXPANDER_IO_LCD_CS      4       // LCD CS pin on IO expander
#define EXPANDER_IO_LCD_RESET   5       // LCD RESET pin on IO expander

// Direct GPIO pins for SPI bit-banging
// From: sensecap_indicator_board.c GPIO_SPI_* definitions
#define SPI_GPIO_CLK    41              // GPIO_SPI_SCLK
#define SPI_GPIO_MOSI   48              // GPIO_SPI_MOSI

// TCA9535 register definitions
// From: components/i2c_devices/io_expander/tca9535.c
#define TCA9535_INPUT_PORT_REG          0x00
#define TCA9535_OUTPUT_PORT_REG         0x02
#define TCA9535_CONFIGURATION_REG       0x06

// I2C configuration for IO expander
// From: sensecap_indicator_board.c GPIO_I2C_* definitions
#define I2C_MASTER_NUM      I2C_NUM_0
#define I2C_MASTER_SDA_IO   39          // GPIO_I2C_SDA
#define I2C_MASTER_SCL_IO   40          // GPIO_I2C_SCL
#define I2C_MASTER_FREQ_HZ  400000

static lv_disp_drv_t disp_drv;
static lv_disp_draw_buf_t draw_buf;
static lv_color_t *buf1 = NULL;
static esp_lcd_panel_handle_t panel_handle = NULL;

// IO Expander state
static uint16_t io_expander_output = 0;
static uint16_t io_expander_config = 0xFFFF;  // All inputs by default
static bool io_expander_initialized = false;

// =============================================================================
// TCA9535 IO EXPANDER DRIVER
// Reference: components/i2c_devices/io_expander/tca9535.c
// =============================================================================

static esp_err_t tca9535_write_reg(uint8_t reg, uint16_t data)
{
    i2c_cmd_handle_t cmd = i2c_cmd_link_create();
    i2c_master_start(cmd);
    i2c_master_write_byte(cmd, (TCA9535_I2C_ADDR << 1) | I2C_MASTER_WRITE, true);
    i2c_master_write_byte(cmd, reg, true);
    i2c_master_write_byte(cmd, data & 0xFF, true);  // Low byte
    i2c_master_write_byte(cmd, (data >> 8) & 0xFF, true);  // High byte
    i2c_master_stop(cmd);
    esp_err_t ret = i2c_master_cmd_begin(I2C_MASTER_NUM, cmd, pdMS_TO_TICKS(100));
    i2c_cmd_link_delete(cmd);
    return ret;
}

static esp_err_t tca9535_init(void)
{
    ESP_LOGI(TAG, "Initializing TCA9535 IO expander at 0x%02X", TCA9535_I2C_ADDR);
    
    // Initialize I2C master
    i2c_config_t conf = {
        .mode = I2C_MODE_MASTER,
        .sda_io_num = I2C_MASTER_SDA_IO,
        .scl_io_num = I2C_MASTER_SCL_IO,
        .sda_pullup_en = GPIO_PULLUP_ENABLE,
        .scl_pullup_en = GPIO_PULLUP_ENABLE,
        .master.clk_speed = I2C_MASTER_FREQ_HZ,
    };
    ESP_ERROR_CHECK(i2c_param_config(I2C_MASTER_NUM, &conf));
    ESP_ERROR_CHECK(i2c_driver_install(I2C_MASTER_NUM, conf.mode, 0, 0, 0));
    
    // Test communication by reading input port
    i2c_cmd_handle_t cmd = i2c_cmd_link_create();
    i2c_master_start(cmd);
    i2c_master_write_byte(cmd, (TCA9535_I2C_ADDR << 1) | I2C_MASTER_WRITE, true);
    i2c_master_write_byte(cmd, TCA9535_INPUT_PORT_REG, true);
    i2c_master_start(cmd);
    i2c_master_write_byte(cmd, (TCA9535_I2C_ADDR << 1) | I2C_MASTER_READ, true);
    uint8_t data[2];
    i2c_master_read(cmd, data, 2, I2C_MASTER_LAST_NACK);
    i2c_master_stop(cmd);
    esp_err_t ret = i2c_master_cmd_begin(I2C_MASTER_NUM, cmd, pdMS_TO_TICKS(100));
    i2c_cmd_link_delete(cmd);
    
    if (ret != ESP_OK) {
        ESP_LOGE(TAG, "TCA9535 not found at 0x%02X", TCA9535_I2C_ADDR);
        return ret;
    }
    
    // Set LCD CS and RESET pins as outputs
    io_expander_config &= ~(BIT(EXPANDER_IO_LCD_CS) | BIT(EXPANDER_IO_LCD_RESET));
    ESP_ERROR_CHECK(tca9535_write_reg(TCA9535_CONFIGURATION_REG, io_expander_config));
    
    // Set default levels (CS=1, RESET=1)
    io_expander_output |= BIT(EXPANDER_IO_LCD_CS);
    io_expander_output |= BIT(EXPANDER_IO_LCD_RESET);
    ESP_ERROR_CHECK(tca9535_write_reg(TCA9535_OUTPUT_PORT_REG, io_expander_output));
    
    io_expander_initialized = true;
    ESP_LOGI(TAG, "TCA9535 initialized successfully");
    return ESP_OK;
}

static void tca9535_set_level(uint8_t pin, bool level)
{
    if (!io_expander_initialized) return;
    
    if (level) {
        io_expander_output |= BIT(pin);
    } else {
        io_expander_output &= ~BIT(pin);
    }
    tca9535_write_reg(TCA9535_OUTPUT_PORT_REG, io_expander_output);
}

// =============================================================================
// SPI BIT-BANGING FOR ST7701S INITIALIZATION
// Reference: components/bsp/src/boards/lcd_panel_config.c
// =============================================================================

#define CS(n)   tca9535_set_level(EXPANDER_IO_LCD_CS, n)
#define RST(n)  tca9535_set_level(EXPANDER_IO_LCD_RESET, n)
#define CLK(n)  gpio_set_level(SPI_GPIO_CLK, n)
#define MOSI(n) gpio_set_level(SPI_GPIO_MOSI, n)
#define Delay(t) vTaskDelay(pdMS_TO_TICKS(t))
#define udelay(_t) esp_rom_delay_us(_t)

static void spi_init_gpio(void)
{
    // Configure SPI CLK and MOSI as outputs
    gpio_config_t io_conf = {
        .mode = GPIO_MODE_OUTPUT,
        .pin_bit_mask = (1ULL << SPI_GPIO_CLK) | (1ULL << SPI_GPIO_MOSI),
    };
    ESP_ERROR_CHECK(gpio_config(&io_conf));
    
    // Set initial state
    CLK(1);
    MOSI(1);
}

static void SPI_SendData(unsigned short i)
{
    // 9-bit SPI: 1 bit for data/command, 8 bits for data
    // Reference: lcd_panel_config.c SPI_SendData function
    for (int n = 0; n < 9; n++) {
        if (i & 0x0100) {
            MOSI(1);
        } else {
            MOSI(0);
        }
        i = i << 1;
        CLK(1);
        udelay(10);
        CLK(0);
        udelay(10);
    }
}

static void SPI_WriteComm(unsigned short c)
{
    // Send command (bit 8 = 0 for command)
    CS(0);
    udelay(10);
    CLK(0);
    udelay(10);
    
    SPI_SendData(((c >> 8) & 0x00FF) | 0x2000);  // High byte with command bit
    
    CLK(1);
    udelay(10);
    CLK(0);
    
    CS(1);
    udelay(10);
    CS(0);
    udelay(10);
    
    SPI_SendData((c & 0x00FF));  // Low byte
    CS(1);
    udelay(10);
}

static void SPI_WriteData(unsigned short d)
{
    // Send data (bit 8 = 1 for data)
    CS(0);
    udelay(10);
    CLK(0);
    udelay(10);
    
    d &= 0x00FF;
    d |= 0x0100;  // Set data bit
    SPI_SendData(d);
    
    CLK(1);
    udelay(10);
    CLK(0);
    udelay(10);
    
    CS(1);
    udelay(10);
}

// =============================================================================
// ST7701S INITIALIZATION SEQUENCE
// Reference: components/bsp/src/boards/lcd_panel_config.c lcd_panel_st7701s_init()
// =============================================================================

static void st7701s_init_sequence(void)
{
    ESP_LOGI(TAG, "Starting ST7701S initialization sequence");
    
    // Reset sequence
    RST(0);
    Delay(10);
    RST(1);
    
    // Command 2 BK0 (PAGE1)
    SPI_WriteComm(0xFF);
    SPI_WriteData(0x77);
    SPI_WriteData(0x01);
    SPI_WriteData(0x00);
    SPI_WriteData(0x00);
    SPI_WriteData(0x10);

    // Display resolution
    SPI_WriteComm(0xC0);
    SPI_WriteData(0x3B);  // 480*480
    SPI_WriteData(0x00);

    SPI_WriteComm(0xC1);
    SPI_WriteData(0x0D);
    SPI_WriteData(0x02);

    SPI_WriteComm(0xC2);
    SPI_WriteData(0x31);
    SPI_WriteData(0x05);

    SPI_WriteComm(0xC7);
    SPI_WriteData(0x04);

    SPI_WriteComm(0xCD);
    SPI_WriteData(0x08);

    // Gamma settings
    SPI_WriteComm(0xB0);
    SPI_WriteData(0x00);
    SPI_WriteData(0x11);
    SPI_WriteData(0x18);
    SPI_WriteData(0x0E);
    SPI_WriteData(0x11);
    SPI_WriteData(0x06);
    SPI_WriteData(0x07);
    SPI_WriteData(0x08);
    SPI_WriteData(0x07);
    SPI_WriteData(0x22);
    SPI_WriteData(0x04);
    SPI_WriteData(0x12);
    SPI_WriteData(0x0F);
    SPI_WriteData(0xAA);
    SPI_WriteData(0x31);
    SPI_WriteData(0x18);

    SPI_WriteComm(0xB1);
    SPI_WriteData(0x00);
    SPI_WriteData(0x11);
    SPI_WriteData(0x19);
    SPI_WriteData(0x0E);
    SPI_WriteData(0x12);
    SPI_WriteData(0x07);
    SPI_WriteData(0x08);
    SPI_WriteData(0x08);
    SPI_WriteData(0x08);
    SPI_WriteData(0x22);
    SPI_WriteData(0x04);
    SPI_WriteData(0x11);
    SPI_WriteData(0x11);
    SPI_WriteData(0xA9);
    SPI_WriteData(0x32);
    SPI_WriteData(0x18);

    // Command 2 BK1 (PAGE2)
    SPI_WriteComm(0xFF);
    SPI_WriteData(0x77);
    SPI_WriteData(0x01);
    SPI_WriteData(0x00);
    SPI_WriteData(0x00);
    SPI_WriteData(0x11);

    SPI_WriteComm(0xB0);
    SPI_WriteData(0x60);

    SPI_WriteComm(0xB1);
    SPI_WriteData(0x32);

    SPI_WriteComm(0xB2);
    SPI_WriteData(0x07);

    SPI_WriteComm(0xB3);
    SPI_WriteData(0x80);

    SPI_WriteComm(0xB5);
    SPI_WriteData(0x49);

    SPI_WriteComm(0xB7);
    SPI_WriteData(0x85);

    SPI_WriteComm(0xB8);
    SPI_WriteData(0x21);

    SPI_WriteComm(0xC1);
    SPI_WriteData(0x78);

    SPI_WriteComm(0xC2);
    SPI_WriteData(0x78);

    Delay(20);

    // VCOM settings
    SPI_WriteComm(0xE0);
    SPI_WriteData(0x00);
    SPI_WriteData(0x1B);
    SPI_WriteData(0x02);

    SPI_WriteComm(0xE1);
    SPI_WriteData(0x08);
    SPI_WriteData(0xA0);
    SPI_WriteData(0x00);
    SPI_WriteData(0x00);
    SPI_WriteData(0x07);
    SPI_WriteData(0xA0);
    SPI_WriteData(0x00);
    SPI_WriteData(0x00);
    SPI_WriteData(0x00);
    SPI_WriteData(0x44);
    SPI_WriteData(0x44);

    SPI_WriteComm(0xE2);
    SPI_WriteData(0x11);
    SPI_WriteData(0x11);
    SPI_WriteData(0x44);
    SPI_WriteData(0x44);
    SPI_WriteData(0xED);
    SPI_WriteData(0xA0);
    SPI_WriteData(0x00);
    SPI_WriteData(0x00);
    SPI_WriteData(0xEC);
    SPI_WriteData(0xA0);
    SPI_WriteData(0x00);
    SPI_WriteData(0x00);

    SPI_WriteComm(0xE3);
    SPI_WriteData(0x00);
    SPI_WriteData(0x00);
    SPI_WriteData(0x11);
    SPI_WriteData(0x11);

    SPI_WriteComm(0xE4);
    SPI_WriteData(0x44);
    SPI_WriteData(0x44);

    SPI_WriteComm(0xE5);
    SPI_WriteData(0x0A);
    SPI_WriteData(0xE9);
    SPI_WriteData(0xD8);
    SPI_WriteData(0xA0);
    SPI_WriteData(0x0C);
    SPI_WriteData(0xEB);
    SPI_WriteData(0xD8);
    SPI_WriteData(0xA0);
    SPI_WriteData(0x0E);
    SPI_WriteData(0xED);
    SPI_WriteData(0xD8);
    SPI_WriteData(0xA0);
    SPI_WriteData(0x10);
    SPI_WriteData(0xEF);
    SPI_WriteData(0xD8);
    SPI_WriteData(0xA0);

    SPI_WriteComm(0xE6);
    SPI_WriteData(0x00);
    SPI_WriteData(0x00);
    SPI_WriteData(0x11);
    SPI_WriteData(0x11);

    SPI_WriteComm(0xE7);
    SPI_WriteData(0x44);
    SPI_WriteData(0x44);

    SPI_WriteComm(0xE8);
    SPI_WriteData(0x09);
    SPI_WriteData(0xE8);
    SPI_WriteData(0xD8);
    SPI_WriteData(0xA0);
    SPI_WriteData(0x0B);
    SPI_WriteData(0xEA);
    SPI_WriteData(0xD8);
    SPI_WriteData(0xA0);
    SPI_WriteData(0x0D);
    SPI_WriteData(0xEC);
    SPI_WriteData(0xD8);
    SPI_WriteData(0xA0);
    SPI_WriteData(0x0F);
    SPI_WriteData(0xEE);
    SPI_WriteData(0xD8);
    SPI_WriteData(0xA0);

    SPI_WriteComm(0xEB);
    SPI_WriteData(0x02);
    SPI_WriteData(0x00);
    SPI_WriteData(0xE4);
    SPI_WriteData(0xE4);
    SPI_WriteData(0x88);
    SPI_WriteData(0x00);
    SPI_WriteData(0x40);

    SPI_WriteComm(0xEC);
    SPI_WriteData(0x3C);
    SPI_WriteData(0x00);

    SPI_WriteComm(0xED);
    SPI_WriteData(0xAB);
    SPI_WriteData(0x89);
    SPI_WriteData(0x76);
    SPI_WriteData(0x54);
    SPI_WriteData(0x02);
    SPI_WriteData(0xFF);
    SPI_WriteData(0xFF);
    SPI_WriteData(0xFF);
    SPI_WriteData(0xFF);
    SPI_WriteData(0xFF);
    SPI_WriteData(0xFF);
    SPI_WriteData(0x20);
    SPI_WriteData(0x45);
    SPI_WriteData(0x67);
    SPI_WriteData(0x98);
    SPI_WriteData(0xBA);

    // Memory access control
    SPI_WriteComm(0x36);
    SPI_WriteData(0x10);

    // Command 2 BK3 (PAGE3)
    SPI_WriteComm(0xFF);
    SPI_WriteData(0x77);
    SPI_WriteData(0x01);
    SPI_WriteData(0x00);
    SPI_WriteData(0x00);
    SPI_WriteData(0x13);

    SPI_WriteComm(0xE5);
    SPI_WriteData(0xE4);

    // Return to CMD1
    SPI_WriteComm(0xFF);
    SPI_WriteData(0x77);
    SPI_WriteData(0x01);
    SPI_WriteData(0x00);
    SPI_WriteData(0x00);
    SPI_WriteData(0x00);

    // Interface pixel format: RGB666
    SPI_WriteComm(0x3A);
    SPI_WriteData(0x60);  // 0x70 RGB888, 0x60 RGB666, 0x50 RGB565

    // Display Inversion On
    SPI_WriteComm(0x21);

    // Sleep Out
    SPI_WriteComm(0x11);
    Delay(120);

    // Display On
    SPI_WriteComm(0x29);
    Delay(120);

    // Set pins high
    CS(1);
    CLK(1);
    MOSI(1);
    
    ESP_LOGI(TAG, "ST7701S initialization complete");
}

// =============================================================================
// RGB DISPLAY INTERFACE
// =============================================================================

void display_init(void)
{
    ESP_LOGI(TAG, "Initializing SenseCAP Indicator D1 Display");
    ESP_LOGI(TAG, "Reference: Seeed Studio SDK - sensecap_indicator_esp32");
    
    // Step 1: Initialize IO Expander (TCA9535)
    // Reference: components/i2c_devices/io_expander/tca9535.c
    ESP_ERROR_CHECK(tca9535_init());
    
    // Step 2: Initialize SPI GPIOs for bit-banging
    // Reference: lcd_panel_config.c init_gpios()
    spi_init_gpio();
    
    // Step 3: Configure backlight
    gpio_config_t bk_gpio_config = {
        .mode = GPIO_MODE_OUTPUT,
        .pin_bit_mask = 1ULL << LCD_GPIO_BL
    };
    ESP_ERROR_CHECK(gpio_config(&bk_gpio_config));
    gpio_set_level(LCD_GPIO_BL, 0);  // Off initially
    
    // Step 4: Initialize ST7701S via SPI
    // Reference: lcd_panel_config.c lcd_panel_st7701s_init()
    st7701s_init_sequence();
    
    // Step 5: Configure RGB panel
    // Reference: bsp_lcd.c bsp_lcd_init() with RGB interface
    esp_lcd_rgb_panel_config_t panel_config = {
        .clk_src = LCD_CLK_SRC_PLL160M,
        .data_width = 16,
        .disp_gpio_num = GPIO_NUM_NC,
        .pclk_gpio_num = LCD_GPIO_PCLK,
        .vsync_gpio_num = LCD_GPIO_VSYNC,
        .hsync_gpio_num = LCD_GPIO_HSYNC,
        .de_gpio_num = LCD_GPIO_DE,
        .data_gpio_nums = {
            LCD_GPIO_DATA0, LCD_GPIO_DATA1, LCD_GPIO_DATA2, LCD_GPIO_DATA3,
            LCD_GPIO_DATA4, LCD_GPIO_DATA5, LCD_GPIO_DATA6, LCD_GPIO_DATA7,
            LCD_GPIO_DATA8, LCD_GPIO_DATA9, LCD_GPIO_DATA10, LCD_GPIO_DATA11,
            LCD_GPIO_DATA12, LCD_GPIO_DATA13, LCD_GPIO_DATA14, LCD_GPIO_DATA15,
        },
        .timings = {
            .pclk_hz = LCD_FREQ,
            .h_res = DISP_HOR_RES,
            .v_res = DISP_VER_RES,
            .hsync_back_porch = HSYNC_BACK_PORCH,
            .hsync_front_porch = HSYNC_FRONT_PORCH,
            .hsync_pulse_width = HSYNC_PULSE_WIDTH,
            .vsync_back_porch = VSYNC_BACK_PORCH,
            .vsync_front_porch = VSYNC_FRONT_PORCH,
            .vsync_pulse_width = VSYNC_PULSE_WIDTH,
            .flags.pclk_active_neg = false,
        },
        .flags.fb_in_psram = 1,
        .num_fbs = 1,
    };
    
    ESP_LOGI(TAG, "Creating RGB panel: %dx%d @ %d Hz", DISP_HOR_RES, DISP_VER_RES, LCD_FREQ);
    ESP_ERROR_CHECK(esp_lcd_new_rgb_panel(&panel_config, &panel_handle));
    ESP_ERROR_CHECK(esp_lcd_panel_reset(panel_handle));
    ESP_ERROR_CHECK(esp_lcd_panel_init(panel_handle));
    ESP_ERROR_CHECK(esp_lcd_panel_disp_on_off(panel_handle, true));
    
    // Turn on backlight
    gpio_set_level(LCD_GPIO_BL, 1);
    
    ESP_LOGI(TAG, "Display initialization complete");
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
    
    size_t buffer_size = DISP_HOR_RES * DISP_VER_RES;
    
    // Allocate from PSRAM
    buf1 = heap_caps_malloc(buffer_size * sizeof(lv_color_t), MALLOC_CAP_SPIRAM | MALLOC_CAP_8BIT);
    if (buf1 == NULL) {
        ESP_LOGW(TAG, "PSRAM not available, using internal RAM");
        buf1 = heap_caps_malloc(buffer_size * sizeof(lv_color_t), MALLOC_CAP_INTERNAL | MALLOC_CAP_8BIT);
        if (buf1 == NULL) {
            ESP_LOGE(TAG, "Failed to allocate display buffer");
            return;
        }
    }
    
    lv_disp_draw_buf_init(&draw_buf, buf1, NULL, buffer_size);
    
    lv_disp_drv_init(&disp_drv);
    disp_drv.hor_res = DISP_HOR_RES;
    disp_drv.ver_res = DISP_VER_RES;
    disp_drv.flush_cb = display_flush_cb;
    disp_drv.draw_buf = &draw_buf;
    disp_drv.full_refresh = 1;
    lv_disp_drv_register(&disp_drv);
    
    ESP_LOGI(TAG, "LVGL display driver initialized");
}
