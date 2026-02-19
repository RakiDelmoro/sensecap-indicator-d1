#ifndef TOUCH_DRIVER_H
#define TOUCH_DRIVER_H

#include <stdint.h>
#include <stdbool.h>

// Touch initialization
void touch_init(void);
void touch_driver_init(void);

// Touch read callback for LVGL
void touch_read_cb(lv_indev_drv_t *drv, lv_indev_data_t *data);

#endif // TOUCH_DRIVER_H
