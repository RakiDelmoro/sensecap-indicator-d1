use std::sync::{Arc, Mutex};

use anyhow::Result;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::i2c::I2cDriver;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys::{
    esp_lcd_new_rgb_panel, esp_lcd_panel_del, esp_lcd_panel_disp_on_off, esp_lcd_panel_draw_bitmap,
    esp_lcd_panel_handle_t, esp_lcd_panel_init, esp_lcd_panel_reset, esp_lcd_rgb_panel_config_t,
    lcd_clk_src_t_LCD_CLK_SRC_PLL160M,
};
use log::info;

// Display resolution
pub const DISP_HOR_RES: u16 = 480;
pub const DISP_VER_RES: u16 = 480;

// RGB Interface GPIOs
const LCD_GPIO_DATA0: i32 = 15;
const LCD_GPIO_DATA1: i32 = 14;
const LCD_GPIO_DATA2: i32 = 13;
const LCD_GPIO_DATA3: i32 = 12;
const LCD_GPIO_DATA4: i32 = 11;
const LCD_GPIO_DATA5: i32 = 10;
const LCD_GPIO_DATA6: i32 = 9;
const LCD_GPIO_DATA7: i32 = 8;
const LCD_GPIO_DATA8: i32 = 7;
const LCD_GPIO_DATA9: i32 = 6;
const LCD_GPIO_DATA10: i32 = 5;
const LCD_GPIO_DATA11: i32 = 4;
const LCD_GPIO_DATA12: i32 = 3;
const LCD_GPIO_DATA13: i32 = 2;
const LCD_GPIO_DATA14: i32 = 1;
const LCD_GPIO_DATA15: i32 = 0;

const LCD_GPIO_VSYNC: i32 = 17;
const LCD_GPIO_HSYNC: i32 = 16;
const LCD_GPIO_DE: i32 = 18;
const LCD_GPIO_PCLK: i32 = 21;

// Display timing parameters
const HSYNC_BACK_PORCH: i32 = 50;
const HSYNC_FRONT_PORCH: i32 = 10;
const HSYNC_PULSE_WIDTH: i32 = 8;
const VSYNC_BACK_PORCH: i32 = 50;
const VSYNC_FRONT_PORCH: i32 = 10;
const VSYNC_PULSE_WIDTH: i32 = 8;
const LCD_FREQ: i32 = 16_000_000; // 16MHz

// TCA9535 IO Expander
const TCA9535_I2C_ADDR: u8 = 0x39;
const TCA9535_INPUT_PORT_REG: u8 = 0x00;
const TCA9535_OUTPUT_PORT_REG: u8 = 0x02;
const TCA9535_CONFIGURATION_REG: u8 = 0x06;

// IO Expander pins
const EXPANDER_IO_LCD_CS: u8 = 4;
const EXPANDER_IO_LCD_RESET: u8 = 5;

pub struct DisplayDriver {
    i2c: Arc<Mutex<I2cDriver<'static>>>,
    io_expander_output: u16,
    io_expander_config: u16,
    spi_clk: PinDriver<'static, esp_idf_hal::gpio::AnyOutputPin>,
    spi_mosi: PinDriver<'static, esp_idf_hal::gpio::AnyOutputPin>,
    backlight: PinDriver<'static, esp_idf_hal::gpio::AnyOutputPin>,
    panel_handle: esp_lcd_panel_handle_t,
    frame_buffer: *mut u16,
}

// Safety: frame buffer is allocated and valid for the lifetime of the driver
unsafe impl Send for DisplayDriver {}
unsafe impl Sync for DisplayDriver {}

impl DisplayDriver {
    pub fn new(
        peripherals: &mut Peripherals,
        shared_i2c: Arc<Mutex<I2cDriver<'static>>>,
    ) -> Result<Self> {
        info!("Initializing SenseCAP Indicator D1 Display");

        // Configure SPI GPIOs for bit-banging
        let spi_clk = PinDriver::output(
            peripherals
                .pins
                .gpio41
                .take()
                .expect("GPIO41 already taken"),
        )?;
        let spi_mosi = PinDriver::output(
            peripherals
                .pins
                .gpio48
                .take()
                .expect("GPIO48 already taken"),
        )?;

        // Configure backlight
        let mut backlight = PinDriver::output(
            peripherals
                .pins
                .gpio45
                .take()
                .expect("GPIO45 already taken"),
        )?;
        backlight.set_low()?;

        let mut driver = DisplayDriver {
            i2c: shared_i2c,
            io_expander_output: 0,
            io_expander_config: 0xFFFF,
            spi_clk,
            spi_mosi,
            backlight,
            panel_handle: std::ptr::null_mut(),
            frame_buffer: std::ptr::null_mut(),
        };

        // Set SPI pins high initially
        driver.spi_clk.set_high()?;
        driver.spi_mosi.set_high()?;

        // Initialize IO expander
        driver.tca9535_init()?;

        // Initialize ST7701S via SPI
        driver.st7701s_init_sequence()?;

        // Allocate frame buffer
        driver.allocate_framebuffer()?;

        // Initialize RGB panel
        driver.init_rgb_panel()?;

        // Turn on backlight
        driver.backlight.set_high()?;

        info!("Display initialization complete");

        Ok(driver)
    }

    fn allocate_framebuffer(&mut self) -> Result<()> {
        let buffer_size = (DISP_HOR_RES as usize) * (DISP_VER_RES as usize);

        info!(
            "Allocating frame buffer: {}x{} = {} pixels",
            DISP_HOR_RES, DISP_VER_RES, buffer_size
        );

        // Try to allocate from PSRAM first
        unsafe {
            self.frame_buffer = esp_idf_sys::heap_caps_malloc(
                buffer_size * 2, // 2 bytes per pixel (RGB565)
                esp_idf_sys::MALLOC_CAP_SPIRAM | esp_idf_sys::MALLOC_CAP_8BIT,
            ) as *mut u16;

            if self.frame_buffer.is_null() {
                info!("PSRAM not available, using internal RAM");
                self.frame_buffer = esp_idf_sys::heap_caps_malloc(
                    buffer_size * 2,
                    esp_idf_sys::MALLOC_CAP_INTERNAL | esp_idf_sys::MALLOC_CAP_8BIT,
                ) as *mut u16;

                if self.frame_buffer.is_null() {
                    anyhow::bail!("Failed to allocate frame buffer");
                }
            }
        }

        info!("Frame buffer allocated at {:?}", self.frame_buffer);
        Ok(())
    }

    fn init_rgb_panel(&mut self) -> Result<()> {
        info!(
            "Initializing RGB panel: {}x{} @ {} Hz",
            DISP_HOR_RES, DISP_VER_RES, LCD_FREQ
        );

        // RGB Panel configuration
        let panel_config = esp_lcd_rgb_panel_config_t {
            clk_src: lcd_clk_src_t_LCD_CLK_SRC_PLL160M,
            data_width: 16,
            bits_per_pixel: 16,
            sram_trans_align: 4,
            psram_trans_align: 64,
            num_fbs: 1,
            bounce_buffer_size_px: 0,
            hsync_gpio_num: LCD_GPIO_HSYNC,
            vsync_gpio_num: LCD_GPIO_VSYNC,
            de_gpio_num: LCD_GPIO_DE,
            pclk_gpio_num: LCD_GPIO_PCLK,
            disp_gpio_num: -1, // Not used
            data_gpio_nums: [
                LCD_GPIO_DATA0,
                LCD_GPIO_DATA1,
                LCD_GPIO_DATA2,
                LCD_GPIO_DATA3,
                LCD_GPIO_DATA4,
                LCD_GPIO_DATA5,
                LCD_GPIO_DATA6,
                LCD_GPIO_DATA7,
                LCD_GPIO_DATA8,
                LCD_GPIO_DATA9,
                LCD_GPIO_DATA10,
                LCD_GPIO_DATA11,
                LCD_GPIO_DATA12,
                LCD_GPIO_DATA13,
                LCD_GPIO_DATA14,
                LCD_GPIO_DATA15,
            ],
            timing: esp_idf_sys::esp_lcd_rgb_timing_t {
                pclk_hz: LCD_FREQ as u32,
                h_res: DISP_HOR_RES as i32,
                v_res: DISP_VER_RES as i32,
                hsync_pulse_width: HSYNC_PULSE_WIDTH,
                hsync_back_porch: HSYNC_BACK_PORCH,
                hsync_front_porch: HSYNC_FRONT_PORCH,
                vsync_pulse_width: VSYNC_PULSE_WIDTH,
                vsync_back_porch: VSYNC_BACK_PORCH,
                vsync_front_porch: VSYNC_FRONT_PORCH,
                flags: esp_idf_sys::esp_lcd_rgb_timing_t__bindgen_ty_1 {
                    _bitfield: 0, // pclk_active_neg = false
                },
            },
            flags: esp_idf_sys::esp_lcd_rgb_panel_config_t__bindgen_ty_1 {
                _bitfield: 1, // fb_in_psram = true
            },
            ..Default::default()
        };

        unsafe {
            // Create RGB panel
            let ret = esp_lcd_new_rgb_panel(&panel_config, &mut self.panel_handle);
            if ret != esp_idf_sys::ESP_OK {
                anyhow::bail!("Failed to create RGB panel: {}", ret);
            }

            // Reset panel
            let ret = esp_lcd_panel_reset(self.panel_handle);
            if ret != esp_idf_sys::ESP_OK {
                anyhow::bail!("Failed to reset RGB panel: {}", ret);
            }

            // Initialize panel
            let ret = esp_lcd_panel_init(self.panel_handle);
            if ret != esp_idf_sys::ESP_OK {
                anyhow::bail!("Failed to initialize RGB panel: {}", ret);
            }

            // Turn on display
            let ret = esp_lcd_panel_disp_on_off(self.panel_handle, true);
            if ret != esp_idf_sys::ESP_OK {
                anyhow::bail!("Failed to turn on RGB panel: {}", ret);
            }
        }

        info!("RGB panel initialized successfully");
        Ok(())
    }

    /// Flush a region of the frame buffer to the display
    pub fn flush(&self, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        if self.frame_buffer.is_null() || self.panel_handle.is_null() {
            anyhow::bail!("Display not properly initialized");
        }

        unsafe {
            let ret = esp_lcd_panel_draw_bitmap(
                self.panel_handle,
                x,
                y,
                x + width,
                y + height,
                self.frame_buffer
                    .add((y * DISP_HOR_RES as i32 + x) as usize) as *const _,
            );

            if ret != esp_idf_sys::ESP_OK {
                anyhow::bail!("Failed to draw bitmap: {}", ret);
            }
        }

        Ok(())
    }

    /// Get a mutable reference to the frame buffer
    pub fn get_framebuffer(&mut self) -> Option<&mut [u16]> {
        if self.frame_buffer.is_null() {
            return None;
        }

        let buffer_size = (DISP_HOR_RES as usize) * (DISP_VER_RES as usize);
        unsafe {
            Some(std::slice::from_raw_parts_mut(
                self.frame_buffer,
                buffer_size,
            ))
        }
    }

    /// Fill the entire screen with a color
    pub fn fill_screen(&mut self, color: u16) {
        if let Some(buffer) = self.get_framebuffer() {
            for pixel in buffer.iter_mut() {
                *pixel = color;
            }
        }
    }

    /// Draw a pixel
    pub fn draw_pixel(&mut self, x: u16, y: u16, color: u16) {
        if x >= DISP_HOR_RES || y >= DISP_VER_RES {
            return;
        }

        if let Some(buffer) = self.get_framebuffer() {
            let index = (y as usize) * (DISP_HOR_RES as usize) + (x as usize);
            buffer[index] = color;
        }
    }

    fn tca9535_init(&mut self) -> Result<()> {
        info!(
            "Initializing TCA9535 IO expander at 0x{:02X}",
            TCA9535_I2C_ADDR
        );

        // Test communication
        let mut data = [0u8; 2];
        {
            let mut i2c = self.i2c.lock().unwrap();
            i2c.write_read(TCA9535_I2C_ADDR, &[TCA9535_INPUT_PORT_REG], &mut data, 100)?;
        }

        info!("TCA9535 found, input state: {:02X}{:02X}", data[1], data[0]);

        // Set LCD CS and RESET pins as outputs
        self.io_expander_config &= !(1 << EXPANDER_IO_LCD_CS | 1 << EXPANDER_IO_LCD_RESET);
        self.tca9535_write_reg(TCA9535_CONFIGURATION_REG, self.io_expander_config)?;

        // Set default levels (CS=1, RESET=1)
        self.io_expander_output |= 1 << EXPANDER_IO_LCD_CS;
        self.io_expander_output |= 1 << EXPANDER_IO_LCD_RESET;
        self.tca9535_write_reg(TCA9535_OUTPUT_PORT_REG, self.io_expander_output)?;

        info!("TCA9535 initialized successfully");
        Ok(())
    }

    fn tca9535_write_reg(&mut self, reg: u8, data: u16) -> Result<()> {
        let bytes = [reg, (data & 0xFF) as u8, ((data >> 8) & 0xFF) as u8];
        let mut i2c = self.i2c.lock().unwrap();
        i2c.write(TCA9535_I2C_ADDR, &bytes, 100)?;
        Ok(())
    }

    fn tca9535_set_level(&mut self, pin: u8, level: bool) -> Result<()> {
        if level {
            self.io_expander_output |= 1 << pin;
        } else {
            self.io_expander_output &= !(1 << pin);
        }
        self.tca9535_write_reg(TCA9535_OUTPUT_PORT_REG, self.io_expander_output)
    }

    fn spi_send_data(&mut self, data: u16) -> Result<()> {
        // 9-bit SPI: 1 bit for data/command, 8 bits for data
        let mut i = data;
        for _ in 0..9 {
            if i & 0x0100 != 0 {
                self.spi_mosi.set_high()?;
            } else {
                self.spi_mosi.set_low()?;
            }
            i <<= 1;
            self.spi_clk.set_high()?;
            esp_idf_hal::delay::Ets::delay_us(10);
            self.spi_clk.set_low()?;
            esp_idf_hal::delay::Ets::delay_us(10);
        }
        Ok(())
    }

    fn spi_write_comm(&mut self, command: u16) -> Result<()> {
        // Send command (bit 8 = 0 for command)
        self.tca9535_set_level(EXPANDER_IO_LCD_CS, false)?;
        esp_idf_hal::delay::Ets::delay_us(10);
        self.spi_clk.set_low()?;
        esp_idf_hal::delay::Ets::delay_us(10);

        self.spi_send_data(((command >> 8) & 0x00FF) | 0x2000)?;

        self.spi_clk.set_high()?;
        esp_idf_hal::delay::Ets::delay_us(10);
        self.spi_clk.set_low()?;

        self.tca9535_set_level(EXPANDER_IO_LCD_CS, true)?;
        esp_idf_hal::delay::Ets::delay_us(10);
        self.tca9535_set_level(EXPANDER_IO_LCD_CS, false)?;
        esp_idf_hal::delay::Ets::delay_us(10);

        self.spi_send_data(command & 0x00FF)?;
        self.tca9535_set_level(EXPANDER_IO_LCD_CS, true)?;
        esp_idf_hal::delay::Ets::delay_us(10);
        Ok(())
    }

    fn spi_write_data(&mut self, data: u16) -> Result<()> {
        // Send data (bit 8 = 1 for data)
        self.tca9535_set_level(EXPANDER_IO_LCD_CS, false)?;
        esp_idf_hal::delay::Ets::delay_us(10);
        self.spi_clk.set_low()?;
        esp_idf_hal::delay::Ets::delay_us(10);

        let mut d = data & 0x00FF;
        d |= 0x0100; // Set data bit
        self.spi_send_data(d)?;

        self.spi_clk.set_high()?;
        esp_idf_hal::delay::Ets::delay_us(10);
        self.spi_clk.set_low()?;
        esp_idf_hal::delay::Ets::delay_us(10);

        self.tca9535_set_level(EXPANDER_IO_LCD_CS, true)?;
        esp_idf_hal::delay::Ets::delay_us(10);
        Ok(())
    }

    fn st7701s_init_sequence(&mut self) -> Result<()> {
        info!("Starting ST7701S initialization sequence");

        // Reset sequence
        self.tca9535_set_level(EXPANDER_IO_LCD_RESET, false)?;
        esp_idf_hal::delay::FreeRtos::delay_ms(10);
        self.tca9535_set_level(EXPANDER_IO_LCD_RESET, true)?;

        // Command 2 BK0 (PAGE1)
        self.spi_write_comm(0xFF)?;
        self.spi_write_data(0x77)?;
        self.spi_write_data(0x01)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x10)?;

        // Display resolution - 480x480
        self.spi_write_comm(0xC0)?;
        self.spi_write_data(0x3B)?;
        self.spi_write_data(0x00)?;

        self.spi_write_comm(0xC1)?;
        self.spi_write_data(0x0D)?;
        self.spi_write_data(0x02)?;

        self.spi_write_comm(0xC2)?;
        self.spi_write_data(0x31)?;
        self.spi_write_data(0x05)?;

        self.spi_write_comm(0xC7)?;
        self.spi_write_data(0x04)?;

        self.spi_write_comm(0xCD)?;
        self.spi_write_data(0x08)?;

        // Gamma settings B0
        self.spi_write_comm(0xB0)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x11)?;
        self.spi_write_data(0x18)?;
        self.spi_write_data(0x0E)?;
        self.spi_write_data(0x11)?;
        self.spi_write_data(0x06)?;
        self.spi_write_data(0x07)?;
        self.spi_write_data(0x08)?;
        self.spi_write_data(0x07)?;
        self.spi_write_data(0x22)?;
        self.spi_write_data(0x04)?;
        self.spi_write_data(0x12)?;
        self.spi_write_data(0x0F)?;
        self.spi_write_data(0xAA)?;
        self.spi_write_data(0x31)?;
        self.spi_write_data(0x18)?;

        // Gamma settings B1
        self.spi_write_comm(0xB1)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x11)?;
        self.spi_write_data(0x19)?;
        self.spi_write_data(0x0E)?;
        self.spi_write_data(0x12)?;
        self.spi_write_data(0x07)?;
        self.spi_write_data(0x08)?;
        self.spi_write_data(0x08)?;
        self.spi_write_data(0x08)?;
        self.spi_write_data(0x22)?;
        self.spi_write_data(0x04)?;
        self.spi_write_data(0x11)?;
        self.spi_write_data(0x11)?;
        self.spi_write_data(0xA9)?;
        self.spi_write_data(0x32)?;
        self.spi_write_data(0x18)?;

        // Command 2 BK1 (PAGE2)
        self.spi_write_comm(0xFF)?;
        self.spi_write_data(0x77)?;
        self.spi_write_data(0x01)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x11)?;

        self.spi_write_comm(0xB0)?;
        self.spi_write_data(0x60)?;

        self.spi_write_comm(0xB1)?;
        self.spi_write_data(0x32)?;

        self.spi_write_comm(0xB2)?;
        self.spi_write_data(0x07)?;

        self.spi_write_comm(0xB3)?;
        self.spi_write_data(0x80)?;

        self.spi_write_comm(0xB5)?;
        self.spi_write_data(0x49)?;

        self.spi_write_comm(0xB7)?;
        self.spi_write_data(0x85)?;

        self.spi_write_comm(0xB8)?;
        self.spi_write_data(0x21)?;

        self.spi_write_comm(0xC1)?;
        self.spi_write_data(0x78)?;

        self.spi_write_comm(0xC2)?;
        self.spi_write_data(0x78)?;

        esp_idf_hal::delay::FreeRtos::delay_ms(20);

        // VCOM settings
        self.spi_write_comm(0xE0)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x1B)?;
        self.spi_write_data(0x02)?;

        self.spi_write_comm(0xE1)?;
        self.spi_write_data(0x08)?;
        self.spi_write_data(0xA0)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x07)?;
        self.spi_write_data(0xA0)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x44)?;
        self.spi_write_data(0x44)?;

        self.spi_write_comm(0xE2)?;
        self.spi_write_data(0x11)?;
        self.spi_write_data(0x11)?;
        self.spi_write_data(0x44)?;
        self.spi_write_data(0x44)?;
        self.spi_write_data(0xED)?;
        self.spi_write_data(0xA0)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0xEC)?;
        self.spi_write_data(0xA0)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x00)?;

        self.spi_write_comm(0xE3)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x11)?;
        self.spi_write_data(0x11)?;

        self.spi_write_comm(0xE4)?;
        self.spi_write_data(0x44)?;
        self.spi_write_data(0x44)?;

        self.spi_write_comm(0xE5)?;
        self.spi_write_data(0x0A)?;
        self.spi_write_data(0xE9)?;
        self.spi_write_data(0xD8)?;
        self.spi_write_data(0xA0)?;
        self.spi_write_data(0x0C)?;
        self.spi_write_data(0xEB)?;
        self.spi_write_data(0xD8)?;
        self.spi_write_data(0xA0)?;
        self.spi_write_data(0x0E)?;
        self.spi_write_data(0xED)?;
        self.spi_write_data(0xD8)?;
        self.spi_write_data(0xA0)?;
        self.spi_write_data(0x10)?;
        self.spi_write_data(0xEF)?;
        self.spi_write_data(0xD8)?;
        self.spi_write_data(0xA0)?;

        self.spi_write_comm(0xE6)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x11)?;
        self.spi_write_data(0x11)?;

        self.spi_write_comm(0xE7)?;
        self.spi_write_data(0x44)?;
        self.spi_write_data(0x44)?;

        self.spi_write_comm(0xE8)?;
        self.spi_write_data(0x09)?;
        self.spi_write_data(0xE8)?;
        self.spi_write_data(0xD8)?;
        self.spi_write_data(0xA0)?;
        self.spi_write_data(0x0B)?;
        self.spi_write_data(0xEA)?;
        self.spi_write_data(0xD8)?;
        self.spi_write_data(0xA0)?;
        self.spi_write_data(0x0D)?;
        self.spi_write_data(0xEC)?;
        self.spi_write_data(0xD8)?;
        self.spi_write_data(0xA0)?;
        self.spi_write_data(0x0F)?;
        self.spi_write_data(0xEE)?;
        self.spi_write_data(0xD8)?;
        self.spi_write_data(0xA0)?;

        self.spi_write_comm(0xEB)?;
        self.spi_write_data(0x02)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0xE4)?;
        self.spi_write_data(0xE4)?;
        self.spi_write_data(0x88)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x40)?;

        self.spi_write_comm(0xEC)?;
        self.spi_write_data(0x3C)?;
        self.spi_write_data(0x00)?;

        self.spi_write_comm(0xED)?;
        self.spi_write_data(0xAB)?;
        self.spi_write_data(0x89)?;
        self.spi_write_data(0x76)?;
        self.spi_write_data(0x54)?;
        self.spi_write_data(0x02)?;
        self.spi_write_data(0xFF)?;
        self.spi_write_data(0xFF)?;
        self.spi_write_data(0xFF)?;
        self.spi_write_data(0xFF)?;
        self.spi_write_data(0xFF)?;
        self.spi_write_data(0xFF)?;
        self.spi_write_data(0x20)?;
        self.spi_write_data(0x45)?;
        self.spi_write_data(0x67)?;
        self.spi_write_data(0x98)?;
        self.spi_write_data(0xBA)?;

        // Memory access control
        self.spi_write_comm(0x36)?;
        self.spi_write_data(0x10)?;

        // Command 2 BK3 (PAGE3)
        self.spi_write_comm(0xFF)?;
        self.spi_write_data(0x77)?;
        self.spi_write_data(0x01)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x13)?;

        self.spi_write_comm(0xE5)?;
        self.spi_write_data(0xE4)?;

        // Return to CMD1
        self.spi_write_comm(0xFF)?;
        self.spi_write_data(0x77)?;
        self.spi_write_data(0x01)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x00)?;
        self.spi_write_data(0x00)?;

        // Interface pixel format: RGB666
        self.spi_write_comm(0x3A)?;
        self.spi_write_data(0x60)?;

        // Display Inversion On
        self.spi_write_comm(0x21)?;

        // Sleep Out
        self.spi_write_comm(0x11)?;
        esp_idf_hal::delay::FreeRtos::delay_ms(120);

        // Display On
        self.spi_write_comm(0x29)?;
        esp_idf_hal::delay::FreeRtos::delay_ms(120);

        // Set pins high
        self.tca9535_set_level(EXPANDER_IO_LCD_CS, true)?;
        self.spi_clk.set_high()?;
        self.spi_mosi.set_high()?;

        info!("ST7701S initialization complete");
        Ok(())
    }
}

impl Drop for DisplayDriver {
    fn drop(&mut self) {
        unsafe {
            if !self.panel_handle.is_null() {
                let _ = esp_lcd_panel_del(self.panel_handle);
            }
            if !self.frame_buffer.is_null() {
                esp_idf_sys::free(self.frame_buffer as *mut _);
            }
        }
    }
}
