#ifndef DISPLAY_DRIVER_H
#define DISPLAY_DRIVER_H

#include <stdint.h>

#define DISP_HOR_RES 480
#define DISP_VER_RES 480

// Display initialization
void display_init(void);
void display_driver_init(void);

// LVGL flush callback
void display_flush_cb(lv_disp_drv_t *drv, const lv_area_t *area, lv_color_t *color_map);

#endif // DISPLAY_DRIVER_H
