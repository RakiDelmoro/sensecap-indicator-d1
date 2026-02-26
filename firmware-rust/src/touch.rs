use std::sync::{Arc, Mutex};

use anyhow::Result;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::i2c::I2cDriver;
use esp_idf_hal::peripherals::Peripherals;
use log::info;

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
    i2c: Arc<Mutex<I2cDriver<'static>>>,
    _reset: PinDriver<'static, esp_idf_hal::gpio::AnyOutputPin>,
    _int: PinDriver<'static, esp_idf_hal::gpio::AnyInputPin>,
    last_state: TouchState,
}

impl TouchDriver {
    pub fn new(
        peripherals: &mut Peripherals,
        shared_i2c: Arc<Mutex<I2cDriver<'static>>>,
    ) -> Result<Self> {
        info!("Initializing touch hardware (GT911)");

        // Configure reset and interrupt GPIOs
        let mut reset =
            PinDriver::output(peripherals.pins.gpio2.take().expect("GPIO2 already taken"))?;
        let int = PinDriver::input(peripherals.pins.gpio3.take().expect("GPIO3 already taken"))?;

        // Reset touch controller
        reset.set_low()?;
        esp_idf_hal::delay::FreeRtos::delay_ms(10);
        reset.set_high()?;
        esp_idf_hal::delay::FreeRtos::delay_ms(100);

        info!("Touch hardware initialized");

        Ok(TouchDriver {
            i2c: shared_i2c,
            _reset: reset,
            _int: int,
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

        let mut i2c = self.i2c.lock().unwrap();
        i2c.write_read(GT911_I2C_ADDR, &reg_bytes, data, 100)?;

        Ok(())
    }

    /// Write to a register in GT911
    fn gt911_write(&mut self, reg: u16, data: u8) -> Result<()> {
        let bytes = [(reg >> 8) as u8, (reg & 0xFF) as u8, data];

        let mut i2c = self.i2c.lock().unwrap();
        i2c.write(GT911_I2C_ADDR, &bytes, 100)?;

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
