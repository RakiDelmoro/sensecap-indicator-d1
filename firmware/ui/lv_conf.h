/**
 * @file lv_conf.h
 * Configuration file for LVGL v8.3.11 on ESP32-S3
 */

#ifndef LV_CONF_H
#define LV_CONF_H

#include <stdint.h>

/*====================
   COLOR SETTINGS
 *====================*/

/*Color depth: 1 (1 byte per pixel), 8 (RGB332), 16 (RGB565), 32 (ARGB8888)*/
#define LV_COLOR_DEPTH 16

/*Swap the 2 bytes of RGB565 color. Useful if the display has an 8-bit interface (e.g. SPI)*/
#define LV_COLOR_16_SWAP 0

/*Enable features to draw on transparent background.*/
#define LV_COLOR_SCREEN_TRANSP 0

/*Images pixels with this color will not be drawn if they are chroma keyed)*/
#define LV_COLOR_CHROMA_KEY lv_color_hex(0x00ff00)

/*=========================
   MEMORY SETTINGS
 *=========================*/

/*1: use custom malloc/free, 0: use the built-in `lv_mem_alloc()` and `lv_mem_free()`*/
#define LV_MEM_CUSTOM 1
#define LV_MEM_CUSTOM_INCLUDE <stdlib.h>
#define LV_MEM_CUSTOM_ALLOC   malloc
#define LV_MEM_CUSTOM_FREE    free
#define LV_MEM_CUSTOM_REALLOC realloc

/*Number of the intermediate memory buffer used during rendering*/
#define LV_MEM_BUF_MAX_NUM 16

/*Use the standard `memcpy` and `memset` instead of LVGL's own functions.*/
#define LV_MEMCPY_MEMSET_STD 0

/*====================
   HAL SETTINGS
 *====================*/

/*Default display refresh period*/
#define LV_DISP_DEF_REFR_PERIOD 10

/*Input device read period in milliseconds*/
#define LV_INDEV_DEF_READ_PERIOD 30

/*Default Dot Per Inch*/
#define LV_DPI_DEF 130

/*========================
   FEATURE CONFIGURATION
 *========================*/

/*------------- Drawing -----------*/
#define LV_DRAW_COMPLEX 1
#define LV_SHADOW_CACHE_SIZE 0
#define LV_CIRCLE_CACHE_SIZE 4
#define LV_USE_DRAW_MASKS 1
#define LV_DRAW_TRANSFORM_USE_MATRIX 0
#define LV_IMG_CACHE_DEF_SIZE       1
#define LV_GRADIENT_MAX_STOPS 2
#define LV_GRAD_CACHE_DEF_SIZE      256
#define LV_DITHER_GRADIENT 0
#define LV_DISP_ROT_MAX_BUF (10*1024)

/*------------- GPU -----------*/
#define LV_USE_GPU_STM32_DMA2D 0
#define LV_USE_GPU_NXP_PXP 0
#define LV_USE_GPU_NXP_VG_LITE 0
#define LV_USE_GPU_SDL 0

/*------------- Logging -----------*/
#define LV_USE_LOG 1
#define LV_LOG_LEVEL LV_LOG_LEVEL_WARN
#define LV_LOG_PRINTF 0
#define LV_LOG_TRACE_MEM        1
#define LV_LOG_TRACE_TIMER      1
#define LV_LOG_TRACE_INDEV      1
#define LV_LOG_TRACE_DISP_REFR  1
#define LV_LOG_TRACE_EVENT      1
#define LV_LOG_TRACE_OBJ_CREATE 1
#define LV_LOG_TRACE_LAYOUT     1
#define LV_LOG_TRACE_ANIM       1

/*------------- Asserts -----------*/
#define LV_USE_ASSERT_NULL          1
#define LV_USE_ASSERT_MALLOC        1
#define LV_USE_ASSERT_STYLE         0
#define LV_USE_ASSERT_MEM_INTEGRITY 0
#define LV_USE_ASSERT_OBJ           0
#define LV_ASSERT_HANDLER while(1);

/*------------- Others -----------*/
#define LV_USE_CHECK_STYLES 0
#define LV_USE_REFR_DEBUG 0
#define LV_USE_MASKS_DEBUG 0
#define LV_USE_LINE_DEBUG 0
#define LV_USE_DRAW_MASKS_DEBUG 0
#define LV_USE_DRAW_UNIT_DEBUG 0
#define LV_USE_PERF_MONITOR 0
#define LV_USE_MEM_MONITOR 0

/*==================
   FONT USAGE
 *==================*/

#define LV_FONT_MONTSERRAT_8  0
#define LV_FONT_MONTSERRAT_10 0
#define LV_FONT_MONTSERRAT_12 0
#define LV_FONT_MONTSERRAT_14 1
#define LV_FONT_MONTSERRAT_16 0
#define LV_FONT_MONTSERRAT_18 0
#define LV_FONT_MONTSERRAT_20 0
#define LV_FONT_MONTSERRAT_22 1
#define LV_FONT_MONTSERRAT_24 1
#define LV_FONT_MONTSERRAT_26 0
#define LV_FONT_MONTSERRAT_28 0
#define LV_FONT_MONTSERRAT_30 1
#define LV_FONT_MONTSERRAT_32 0
#define LV_FONT_MONTSERRAT_34 0
#define LV_FONT_MONTSERRAT_36 0
#define LV_FONT_MONTSERRAT_38 0
#define LV_FONT_MONTSERRAT_40 0
#define LV_FONT_MONTSERRAT_42 0
#define LV_FONT_MONTSERRAT_44 0
#define LV_FONT_MONTSERRAT_46 0
#define LV_FONT_MONTSERRAT_48 0

#define LV_FONT_MONTSERRAT_12_SUBPX      0
#define LV_FONT_MONTSERRAT_28_COMPRESSED  0
#define LV_FONT_DEJAVU_16_PERSIAN_HEBREW  0
#define LV_FONT_SIMSUN_16_CJK             0
#define LV_FONT_SIMSUN_14_CJK             0
#define LV_FONT_SIMSUN_14_CIK             0
#define LV_FONT_UNSCII_8  0
#define LV_FONT_UNSCII_16 0

#define LV_FONT_CUSTOM_DECLARE
#define LV_FONT_DEFAULT &lv_font_montserrat_14
#define LV_USE_FONT_COMPRESSED 1
#define LV_USE_FONT_SUBPX 0
#define LV_USE_FONT_PLACEHOLDER 1

/*================
  THEME USAGE
 *================*/

#define LV_USE_THEME_DEFAULT 1
#define LV_THEME_DEFAULT_DARK 0
#define LV_THEME_DEFAULT_GROW 1
#define LV_THEME_DEFAULT_TRANSITION_TIME 80

#define LV_USE_THEME_BASIC 1
#define LV_USE_THEME_MONO 1

/*===================
  KNOB USAGE
 *===================*/
#define LV_USE_KNOB 1

/*==================
 LAYOUTS
 *==================*/

#define LV_USE_FLEX 1
#define LV_USE_GRID 1

/*==================
 3D TEXTURE
 *==================*/
#define LV_USE_3DTEXTURE 0

/*=====================
  COMPILER SETTINGS
 *====================*/

#define LV_BIG_ENDIAN_SYSTEM 0
#define LV_ATTRIBUTE_TICK_INC
#define LV_ATTRIBUTE_TIMER_HANDLER
#define LV_ATTRIBUTE_FLUSH_READY
#define LV_ATTRIBUTE_MEM_ALIGN_SIZE 1
#define LV_ATTRIBUTE_MEM_ALIGN
#define LV_ATTRIBUTE_LARGE_CONST
#define LV_ATTRIBUTE_LARGE_RAM_ARRAY
#define LV_ATTRIBUTE_FAST_MEM
#define LV_ATTRIBUTE_DMA
#define LV_EXPORT_CONST_INT(int_value) struct _silence_gcc_warning ## int_value
#define LV_USE_LARGE_COORD 0

/*==================
  WIDGETS
 *==================*/

#define LV_USE_ARC       1
#define LV_USE_BAR       1
#define LV_USE_BTN       1
#define LV_USE_BTNMATRIX 1
#define LV_USE_CANVAS    1
#define LV_USE_CHECKBOX  1
#define LV_USE_DROPDOWN  1
#define LV_USE_IMG       1
#define LV_USE_LABEL     1
#define LV_LABEL_TEXT_SELECTION 1
#define LV_LABEL_LONG_TXT_HINT 1

#define LV_USE_LINE      1
#define LV_USE_ROLLER    1
#define LV_ROLLER_INF_PAGES 7

#define LV_USE_SLIDER    1
#define LV_USE_SWITCH    1
#define LV_USE_TEXTAREA  1
#define LV_TEXTAREA_DEF_PWD_SHOW_TIME 1500

#define LV_USE_TABLE     1
#define LV_USE_CALENDAR  1
#define LV_CALENDAR_WEEK_STARTS_MONDAY 0
#define LV_CALENDAR_HIGHLIGHT_TODAY 0
#define LV_USE_CALENDAR_HEADER_ARROW 1
#define LV_USE_CALENDAR_HEADER_DROPDOWN 1

#define LV_USE_CHART     1
#define LV_USE_COLORWHEEL    1
#define LV_USE_IMGBTN        1
#define LV_USE_KEYBOARD      1
#define LV_USE_LED           1
#define LV_USE_LIST          1
#define LV_USE_METER         1
#define LV_USE_MSGBOX        1
#define LV_USE_SPINBOX       1
#define LV_USE_SPINNER       1
#define LV_USE_TABVIEW       1
#define LV_USE_TILEVIEW      1
#define LV_USE_WIN           1
#define LV_USE_SPAN          1
#define LV_SPAN_DEF_LINE_BREAK_LEN 64

/*==================
  LVGL EXAMPLES
 *==================*/
#define LV_BUILD_EXAMPLES 0

/*==================
  OTHERS
 *==================*/

#define LV_USE_FS_STDIO 0
#define LV_USE_PNG 0
#define LV_USE_BMP 0
#define LV_USE_SJPG 0
#define LV_USE_GIF 0
#define LV_USE_QRCODE 0
#define LV_USE_FREETYPE 0
#define LV_USE_TINY_TTF 0
#define LV_USE_RLOTTIE 0
#define LV_USE_FFMPEG 0
#define LV_USE_SNAPSHOT 0
#define LV_USE_MONKEY   0
#define LV_USE_GRIDNAV  0
#define LV_USE_FRAGMENT 0
#define LV_USE_IMGFONT  0
#define LV_USE_IME_PINYIN 0

/*==================
  DEVICES
 *==================*/

#define LV_USE_SDL 0
#define LV_USE_LINUX_FBDEV 0
#define LV_USE_LINUX_DRM  0
#define LV_USE_TFT_ESPI 0

/*==================
  DEMO USAGE
 *==================*/

#define LV_USE_DEMO_WIDGETS 0
#define LV_USE_DEMO_KEYPAD_AND_ENCODER 0
#define LV_USE_DEMO_BENCHMARK 0
#define LV_USE_DEMO_STRESS 0
#define LV_USE_DEMO_MUSIC 0
#define LV_USE_DEMO_FLEX_LAYOUT 0

/*!--END OF LV_CONF_H--*/

#endif /*LV_CONF_H*/
