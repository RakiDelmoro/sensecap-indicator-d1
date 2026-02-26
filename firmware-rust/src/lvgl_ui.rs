use anyhow::Result;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Arc, Circle, PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Text},
};
use log::info;

use crate::display::{DisplayDriver, DISP_HOR_RES, DISP_VER_RES};

// Color definitions (RGB565)
pub const COLOR_BLACK: u16 = 0x0000;
pub const COLOR_WHITE: u16 = 0xFFFF;
pub const COLOR_RED: u16 = 0xF800;
pub const COLOR_GREEN: u16 = 0x07E0;
pub const COLOR_BLUE: u16 = 0x001F;
pub const COLOR_YELLOW: u16 = 0xFFE0;
pub const COLOR_CYAN: u16 = 0x07FF;
pub const COLOR_MAGENTA: u16 = 0xF81F;
pub const COLOR_ORANGE: u16 = 0xFD20;
pub const COLOR_GRAY: u16 = 0x8410;
pub const COLOR_DARK_GRAY: u16 = 0x4208;

// Background and panel colors
pub const COLOR_BG: u16 = 0x0841; // Dark blue-gray #0B0B0B
pub const COLOR_PANEL_BG: u16 = 0x2104; // Dark gray #282828
pub const COLOR_LIGHTS_BORDER: u16 = 0xFF00; // Yellow #F1E144
pub const COLOR_WATER_BORDER: u16 = 0x041F; // Cyan #0087C8
pub const COLOR_WATER_NORMAL: u16 = 0x041F; // Blue #1F84D8
pub const COLOR_WATER_LOW: u16 = 0xFD20; // Orange #FFA500
pub const COLOR_WATER_CRITICAL: u16 = 0xF800; // Red #FF0000
pub const COLOR_SWITCH_ON: u16 = 0xFF80; // Yellow #FFF800

/// LVGL UI structure
pub struct LvglUi {
    bright_state: bool,
    relax_state: bool,
    water_level: u8,
    needs_redraw: bool,
}

impl LvglUi {
    pub fn new() -> Self {
        info!("Creating LVGL UI");

        Self {
            bright_state: false,
            relax_state: false,
            water_level: 50,
            needs_redraw: true,
        }
    }

    /// Draw the complete UI
    pub fn draw(&mut self, display: &mut DisplayDriver) -> Result<()> {
        if !self.needs_redraw {
            return Ok(());
        }

        // Clear screen
        display.fill_screen(COLOR_BG);

        // Draw light mode section
        self.draw_lights_section(display)?;

        // Draw water level section
        self.draw_water_section(display)?;

        // Flush to display
        display.flush(0, 0, DISP_HOR_RES as i32, DISP_VER_RES as i32)?;

        self.needs_redraw = false;
        Ok(())
    }

    fn draw_lights_section(&self, display: &mut DisplayDriver) -> Result<()> {
        // Draw panel background (upper half)
        self.draw_panel(
            display,
            12,
            12,
            DISP_HOR_RES - 24,
            DISP_VER_RES / 2 - 18,
            COLOR_PANEL_BG,
            COLOR_LIGHTS_BORDER,
        );

        // Draw "Lights" label
        self.draw_text(display, DISP_HOR_RES / 2, 40, "LIGHTS", COLOR_YELLOW);

        // Draw divider line
        self.draw_line(
            display,
            DISP_HOR_RES / 2,
            60,
            DISP_HOR_RES / 2,
            DISP_VER_RES / 2 - 30,
            COLOR_GRAY,
        );

        // Draw Bright switch
        self.draw_switch(
            display,
            DISP_HOR_RES / 2 - 100,
            DISP_VER_RES / 4,
            "BRIGHT",
            self.bright_state,
        );

        // Draw Relax switch
        self.draw_switch(
            display,
            DISP_HOR_RES / 2 + 100,
            DISP_VER_RES / 4,
            "RELAX",
            self.relax_state,
        );

        Ok(())
    }

    fn draw_water_section(&self, display: &mut DisplayDriver) -> Result<()> {
        // Draw panel background (lower half)
        self.draw_panel(
            display,
            12,
            DISP_VER_RES / 2 - 6,
            DISP_HOR_RES - 24,
            DISP_VER_RES / 2 - 18,
            COLOR_PANEL_BG,
            COLOR_WATER_BORDER,
        );

        // Draw "Water Level" label
        self.draw_text(
            display,
            DISP_HOR_RES / 2,
            DISP_VER_RES / 2 + 20,
            "WATER LEVEL",
            COLOR_CYAN,
        );

        // Draw water arc
        self.draw_water_arc(display)?;

        // Draw water level percentage
        self.draw_text(
            display,
            DISP_HOR_RES / 2,
            DISP_VER_RES / 2 + 80,
            &format!("{}%", self.water_level),
            self.get_water_color(),
        );

        Ok(())
    }

    fn draw_water_arc(&self, display: &mut DisplayDriver) -> Result<()> {
        let center_x = DISP_HOR_RES / 2;
        let center_y = DISP_VER_RES / 2 + 60;
        let radius = 80;

        // Draw arc background
        self.draw_arc(
            display,
            center_x,
            center_y,
            radius,
            180,
            360,
            COLOR_DARK_GRAY,
        );

        // Calculate arc fill based on water level
        let fill_angle = (self.water_level as u16) * 180 / 100;
        let water_color = self.get_water_color();

        // Draw filled portion
        self.draw_arc(
            display,
            center_x,
            center_y,
            radius,
            180,
            180 + fill_angle,
            water_color,
        );

        Ok(())
    }

    fn draw_switch(&self, display: &mut DisplayDriver, x: u16, y: u16, label: &str, state: bool) {
        // Draw switch background
        let width: u16 = 100;
        let height: u16 = 50;

        // Draw switch track
        let color = if state { COLOR_SWITCH_ON } else { COLOR_GRAY };
        self.draw_rounded_rect(display, x - width / 2, y - height / 2, width, height, color);

        // Draw switch handle
        let handle_color = if state { COLOR_WHITE } else { COLOR_WHITE };
        if state {
            self.draw_circle(display, x + width / 4, y, 18, handle_color);
        } else {
            self.draw_circle(display, x - width / 4, y, 18, handle_color);
        }

        // Draw label
        self.draw_text(display, x, y - height / 2 - 15, label, COLOR_WHITE);
    }

    fn draw_panel(
        &self,
        display: &mut DisplayDriver,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        bg_color: u16,
        border_color: u16,
    ) {
        // Draw border (2px)
        for i in 0..2 {
            self.draw_rect(
                display,
                x + i,
                y + i,
                width - i * 2,
                height - i * 2,
                border_color,
            );
        }

        // Draw background
        self.draw_rect(display, x + 2, y + 2, width - 4, height - 4, bg_color);
    }

    fn draw_text(&self, display: &mut DisplayDriver, x: u16, y: u16, text: &str, color: u16) {
        // Simple text rendering using pixel placement
        // For now, just a placeholder - full font rendering would require embedded_graphics
        let start_x = x - (text.len() as u16 * 3);
        for (i, _ch) in text.chars().enumerate() {
            self.draw_rect(display, start_x + i as u16 * 6, y, 4, 8, color);
        }
    }

    fn draw_line(
        &self,
        display: &mut DisplayDriver,
        x1: u16,
        y1: u16,
        x2: u16,
        y2: u16,
        color: u16,
    ) {
        // Simple line drawing (vertical/horizontal only for now)
        if x1 == x2 {
            // Vertical line
            for y in y1.min(y2)..=y1.max(y2) {
                display.draw_pixel(x1, y, color);
            }
        } else if y1 == y2 {
            // Horizontal line
            for x in x1.min(x2)..=x1.max(x2) {
                display.draw_pixel(x, y1, color);
            }
        }
    }

    fn draw_rect(
        &self,
        display: &mut DisplayDriver,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        color: u16,
    ) {
        for dy in 0..height {
            for dx in 0..width {
                display.draw_pixel(x + dx, y + dy, color);
            }
        }
    }

    fn draw_rounded_rect(
        &self,
        display: &mut DisplayDriver,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        color: u16,
    ) {
        // Simple rounded rectangle (just a regular rect for now)
        self.draw_rect(display, x, y, width, height, color);
    }

    fn draw_circle(
        &self,
        display: &mut DisplayDriver,
        center_x: u16,
        center_y: u16,
        radius: u16,
        color: u16,
    ) {
        for y in 0..=radius * 2 {
            for x in 0..=radius * 2 {
                let dx = x as i32 - radius as i32;
                let dy = y as i32 - radius as i32;
                if dx * dx + dy * dy <= (radius as i32) * (radius as i32) {
                    display.draw_pixel(center_x - radius + x, center_y - radius + y, color);
                }
            }
        }
    }

    fn draw_arc(
        &self,
        display: &mut DisplayDriver,
        center_x: u16,
        center_y: u16,
        radius: u16,
        start_angle: u16,
        end_angle: u16,
        color: u16,
    ) {
        for angle in start_angle..=end_angle {
            let rad = (angle as f32) * std::f32::consts::PI / 180.0;
            let x = (center_x as f32 + (radius as f32) * rad.cos()) as u16;
            let y = (center_y as f32 + (radius as f32) * rad.sin()) as u16;
            display.draw_pixel(x, y, color);

            // Draw thicker line
            for r in (radius - 5)..=radius {
                let x = (center_x as f32 + (r as f32) * rad.cos()) as u16;
                let y = (center_y as f32 + (r as f32) * rad.sin()) as u16;
                display.draw_pixel(x, y, color);
            }
        }
    }

    fn get_water_color(&self) -> u16 {
        match self.water_level {
            0..=10 => COLOR_WATER_CRITICAL,
            11..=20 => COLOR_WATER_LOW,
            _ => COLOR_WATER_NORMAL,
        }
    }

    // Public API
    pub fn set_bright_state(&mut self, state: bool) {
        if self.bright_state != state {
            self.bright_state = state;
            self.needs_redraw = true;
            info!("UI: Bright state set to {}", state);
        }
    }

    pub fn set_relax_state(&mut self, state: bool) {
        if self.relax_state != state {
            self.relax_state = state;
            self.needs_redraw = true;
            info!("UI: Relax state set to {}", state);
        }
    }

    pub fn set_water_level(&mut self, level: u8) {
        let level = level.clamp(0, 100);
        if self.water_level != level {
            self.water_level = level;
            self.needs_redraw = true;
            info!("UI: Water level set to {}%", level);
        }
    }

    pub fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    pub fn mark_for_redraw(&mut self) {
        self.needs_redraw = true;
    }

    /// Handle touch events
    pub fn handle_touch(&mut self, x: i16, y: i16) -> Option<UiAction> {
        // Bright switch region (left side, upper half)
        if x < (DISP_HOR_RES / 2) as i16 && y < (DISP_VER_RES / 2) as i16 {
            // Toggle bright
            return Some(UiAction::ToggleBright);
        }
        // Relax switch region (right side, upper half)
        else if x >= (DISP_HOR_RES / 2) as i16 && y < (DISP_VER_RES / 2) as i16 {
            return Some(UiAction::ToggleRelax);
        }
        // Water level area (just for visual feedback)
        else if y >= (DISP_VER_RES / 2) as i16 {
            // Could add water level adjustment here
        }

        None
    }
}

/// UI Actions
#[derive(Debug, Clone, Copy)]
pub enum UiAction {
    ToggleBright,
    ToggleRelax,
}
