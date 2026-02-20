use anyhow::Result;
use esp_idf_hal::gpio::{InputPin, OutputPin, PinDriver};
use esp_idf_hal::i2c::{I2c, I2cConfig, I2cDriver, I2C0};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::units::Hertz;
use log::info;

// Touch controller pins (GT911 for SenseCAP Indicator D1)
const TOUCH_I2C_NUM: i2c_port_t = I2C_NUM_0;
const TOUCH_PIN_NUM_SDA: gpio_num_t = GPIO_NUM_39;
const TOUCH_PIN_NUM_SCL: gpio_num_t = GPIO_NUM_40;
const TOUCH_PIN_NUM_INT: gpio_num_t = GPIO_NUM_3;
const TOUCH_PIN_NUM_RST: gpio_num_t = GPIO_NUM_2;
const TOUCH_I2C_FREQ_HZ: u32 = 400000;

// GT911 I2C address
const GT911_I2C_ADDR: u8 = 0x5D;

// GT911 registers
const GT911_REG_X_LOW: u16 = 0x8140;
const GT911_REG_X_HIGH: u16 = 0x8141;
const GT911_REG_Y_LOW: u16 = 0x8142;
const GT911_REG_Y_HIGH: u16 = 0x8143;
const GT911_REG_STATUS: u16 = 0x814E;
const GT911_REG_POINTS: u16 = 0x814F;

/// Touch state structure
#[derive(Debug, Clone, Copy)]
pub struct TouchState {
    pub x: i16,
    pub y: i16,
    pub pressed: bool,
}

impl Default for TouchState {
    fn default() -> Self {
        TouchState {
            x: 0,
            y: 0,
            pressed: false,
        }
    }
}

pub struct TouchDriver {
    i2c: I2cDriver<'static>,
    last_state: TouchState,
}

impl TouchDriver {
    pub fn new() -> Result<Self> {
        info!("Initializing touch hardware (GT911)");

        let peripherals = Peripherals::take()?;

        // Configure reset and interrupt GPIOs
        // Note: In a real implementation, we'd use PinDriver

        // Reset touch controller
        // rst.set_low()?;
        std::thread::sleep(std::time::Duration::from_millis(10));
        // rst.set_high()?;
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Initialize I2C (shared with display)
        let i2c_config = I2cConfig::new()
            .baudrate(Hertz(TOUCH_I2C_FREQ_HZ))
            .sda_io_num(TOUCH_PIN_NUM_SDA)
            .scl_io_num(TOUCH_PIN_NUM_SCL);

        let i2c = I2cDriver::new(TOUCH_I2C_NUM, &i2c_config, 0)?;

        info!("Touch hardware initialized");

        Ok(TouchDriver {
            i2c,
            last_state: TouchState::default(),
        })
    }

    /// Read touch data from GT911
    pub fn read(&mut self) -> Result<TouchState> {
        let mut status = [0u8; 1];

        // Read touch status
        self.gt911_read(GT911_REG_STATUS, &mut status)?;

        // Check if touch is detected (bit 7 set)
        if status[0] & 0x80 != 0 {
            let mut buf = [0u8; 4];

            // Read X and Y coordinates
            self.gt911_read(GT911_REG_X_LOW, &mut buf)?;

            let x = (buf[0] as i16) | ((buf[1] as i16) << 8);
            let y = (buf[2] as i16) | ((buf[3] as i16) << 8);

            // Clear status register
            self.gt911_write(GT911_REG_STATUS, 0)?;

            self.last_state = TouchState {
                x,
                y,
                pressed: true,
            };
        } else {
            self.last_state.pressed = false;
        }

        Ok(self.last_state)
    }

    /// Read a register from GT911
    fn gt911_read(&mut self, reg: u16, data: &mut [u8]) -> Result<()> {
        let reg_bytes = [(reg >> 8) as u8, (reg & 0xFF) as u8];

        self.i2c.write_read(GT911_I2C_ADDR, &reg_bytes, data, 100)?;

        Ok(())
    }

    /// Write to a register in GT911
    fn gt911_write(&mut self, reg: u16, data: u8) -> Result<()> {
        let bytes = [(reg >> 8) as u8, (reg & 0xFF) as u8, data];

        self.i2c.write(GT911_I2C_ADDR, &bytes, 100)?;

        Ok(())
    }

    /// Check if screen is being touched
    pub fn is_pressed(&self) -> bool {
        self.last_state.pressed
    }

    /// Get last known touch position
    pub fn get_position(&self) -> (i16, i16) {
        (self.last_state.x, self.last_state.y)
    }
}
