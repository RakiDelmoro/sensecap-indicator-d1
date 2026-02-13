/**
 * @file lv_conf.h
 * Configuration file for LVGL PC Simulator
 * LVGL version: 8.3.11
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

/*Enable features to draw on transparent background.
 *It's required if opa, and transform_* style properties are used.
 *Can be also used by the user to draw something with transparent background.*/
#define LV_COLOR_SCREEN_TRANSP 0

/*Images pixels with this color will not be drawn if they are chroma keyed)*/
#define LV_COLOR_CHROMA_KEY lv_color_hex(0x00ff00)

/*=========================
   MEMORY SETTINGS
 *=========================*/

/*1: use custom malloc/free, 0: use the built-in `lv_mem_alloc()` and `lv_mem_free()`*/
#define LV_MEM_CUSTOM 1
#if LV_MEM_CUSTOM == 0
    /*Size of the memory available for `lv_mem_alloc()` in bytes (>= 2kB)*/
    #define LV_MEM_SIZE (64U * 1024U)

    /*Set an address for the memory pool instead of allocating it as a normal array. Can be in external SRAM too.*/
    #define LV_MEM_ADR 0

    /*Instead of an address give a memory allocator that will be called to get a memory pool for LVGL. E.g. my_malloc*/
    #if LV_MEM_ADR == 0
        #define LV_MEM_POOL_INCLUDE <stdlib.h>
        #define LV_MEM_POOL_ALLOC malloc
    #endif

#else       /*LV_MEM_CUSTOM*/
    #define LV_MEM_CUSTOM_INCLUDE <stdlib.h>
    #define LV_MEM_CUSTOM_ALLOC   malloc
    #define LV_MEM_CUSTOM_FREE    free
    #define LV_MEM_CUSTOM_REALLOC realloc
#endif     /*LV_MEM_CUSTOM*/

/*Number of the intermediate memory buffer used during rendering and other internal processing mechanisms.
 *You will see an error at compile time if the buffer is not large enough.*/
#define LV_MEM_BUF_MAX_NUM 16

/*Use the standard `memcpy` and `memset` instead of LVGL's own functions. (Might or might not be faster).*/
#define LV_MEMCPY_MEMSET_STD 0

/*====================
   HAL SETTINGS
 *====================*/

/*Default display refresh period. LVG will redraw changed areas with this period period*/
#define LV_DISP_DEF_REFR_PERIOD 10      /*[ms]*/

/*Input device read period in milliseconds*/
#define LV_INDEV_DEF_READ_PERIOD 30     /*[ms]*/

/*Use a custom tick source that tells the elapsed time in milliseconds.
 *It removes the need to manually update the tick with `lv_tick_inc()`)*/
#define LV_TICK_CUSTOM 0
#if LV_TICK_CUSTOM
    #define LV_TICK_CUSTOM_INCLUDE "SDL.h"         /*Header for the system time function*/
    #define LV_TICK_CUSTOM_SYS_TIME_EXPR (SDL_GetTicks())    /*Expression evaluating to current system time in ms*/
#endif   /*LV_TICK_CUSTOM*/

/*Default Dot Per Inch. Used to initialize default sizes such as widgets sized, style paddings.
 *(Not so important, you can adjust it to modify default sizes and spaces)*/
#define LV_DPI_DEF 130     /*[px/inch]*/

/*========================
   FEATURE CONFIGURATION
 *========================*/

/*-------------
 * Drawing
 *-----------*/

/*Enable complex draw engine.
 *Required to draw shadow, gradient, rounded corners, circles, arc, skew lines, image transformations or any masks*/
#define LV_DRAW_COMPLEX 1
#if LV_DRAW_COMPLEX != 0
    /*Allow buffering some shadow calculation.
    *LV_SHADOW_CACHE_SIZE is the max. shadow size to buffer, where shadow size is `shadow_width + radius`
    *Caching has LV_SHADOW_CACHE_SIZE^2 RAM cost*/
    #define LV_SHADOW_CACHE_SIZE 0

    /* Set number of maximally cached circle data.
    * The circumference of 1/4 circle are saved for anti-aliasing
    * radius * 4 bytes are used per circle (the most often used shape)
    * 0: to disable caching */
    #define LV_CIRCLE_CACHE_SIZE 4
#endif /*LV_DRAW_COMPLEX*/

/**
 * "Transform" features transformation (zoom, rotation, etc.) on layer's whole coordinate plane
 * and on image draw. Can be used independent from `LV_DRAW_COMPLEX`.
 * Requires `LV_DRAW_COMPLEX = 1`.
 * 0: Disable all transformations
 * 1: Enable transformation (but with affect all rendering as coordinate plane)
 */
#define LV_USE_DRAW_MASKS 1

/*Include `lv_draw_transform_rect()` with support for transforms in draw functions.*/
#define LV_DRAW_TRANSFORM_USE_MATRIX 0

/* The number of units in a line of a cache.
 * Higher = better performance, but more memory used. */
#define LV_IMG_CACHE_DEF_SIZE       1

/*Number of stops allowed per gradient. Increase this to allow more stops in a gradient linear or radial line.*/
#define LV_GRADIENT_MAX_STOPS 2

/*Default gradient buffer size.
 *Set to 0 will use the entire screen's size as buffer (inefficient, do not do that!)
 *Set to 256 by default (256 x 4 bytes = 1kB for each gradient)
 *LV_GRAD_CACHE_DEF_SIZE sets the size of this buffer.*/
#define LV_GRAD_CACHE_DEF_SIZE      256

/*Allow dithering the gradients (to achieve visual smooth color gradients on limited color depth display)*/
#define LV_DITHER_GRADIENT 0
#if LV_DITHER_GRADIENT
    /*Add support for error diffusion dithering.
    *Error diffusion dithering is a high quality dithering method but it requires more CPU and memory during calculation (and some extra memory for the `lv_grad`)*/
    #define LV_DITHER_ERROR_DIFFUSION 0
#endif

/*Maximum buffer size to allocate for rotation.
 *Only used if software rotation is enabled in the display driver.*/
#define LV_DISP_ROT_MAX_BUF (10*1024)

/*-------------
 * GPU
 *-----------*/

/*Use STM32's DMA2D (aka Chrom Art) GPU*/
#define LV_USE_GPU_STM32_DMA2D 0

/*Use NXP's PXP GPU iMX RTxxx platforms*/
#define LV_USE_GPU_NXP_PXP 0

/*Use NXP's VG-Lite GPU iMX RTxxx platforms*/
#define LV_USE_GPU_NXP_VG_LITE 0

/*Use SDL renderer API. Requires LV_USE_DRAW_MASKS to be enabled.*/
#define LV_USE_GPU_SDL 0

/*-------------
 * Logging
 *-----------*/

/*Enable the log module*/
#define LV_USE_LOG 1
#if LV_USE_LOG

    /*How important log should be added:
    *LV_LOG_LEVEL_TRACE       A lot of logs to give detailed information
    *LV_LOG_LEVEL_INFO        Log important events
    *LV_LOG_LEVEL_WARN        Log if something unwanted happened but didn't cause a problem
    *LV_LOG_LEVEL_ERROR       Only critical issue, when the system may fail
    *LV_LOG_LEVEL_USER        Only logs added by the user
    *LV_LOG_LEVEL_NONE        Do not log anything*/
    #define LV_LOG_LEVEL LV_LOG_LEVEL_WARN

    /*1: Print the log with 'printf'; 0: User needs to register a callback with `lv_log_register_print_cb()`*/
    #define LV_LOG_PRINTF 1

    /*Enable/disable LV_LOG_TRACE in modules that produces a huge number of logs*/
    #define LV_LOG_TRACE_MEM        1
    #define LV_LOG_TRACE_TIMER      1
    #define LV_LOG_TRACE_INDEV      1
    #define LV_LOG_TRACE_DISP_REFR  1
    #define LV_LOG_TRACE_EVENT      1
    #define LV_LOG_TRACE_OBJ_CREATE 1
    #define LV_LOG_TRACE_LAYOUT     1
    #define LV_LOG_TRACE_ANIM       1

#endif  /*LV_USE_LOG*/

/*-------------
 * Asserts
 *-----------*/

/*Enable asserts if an operation is failed or an invalid data is found.
 *If LV_USE_LOG is enabled an error message will be printed on failure*/
#define LV_USE_ASSERT_NULL          1
#define LV_USE_ASSERT_MALLOC        1
#define LV_USE_ASSERT_STYLE         0
#define LV_USE_ASSERT_MEM_INTEGRITY 0
#define LV_USE_ASSERT_OBJ           0

/*Add a custom handler when assert happens e.g. to restart the MCU*/
#define LV_ASSERT_HANDLER_INCLUDE <stdint.h>
#define LV_ASSERT_HANDLER while(1);   /*Halt by default*/

/*-------------
 * Others
 *-----------*/

/*Check if LVGL is configured correctly*/
#define LV_USE_CHECK_STYLES 0

/*1: Draw random colored rectangles over the redrawn areas*/
#define LV_USE_REFR_DEBUG 0

/*1: Draw a red overlay to the layers with masks*/
#define LV_USE_MASKS_DEBUG 0

/*1: Draw a red line to indicate the coordinate system's y=0 position (Useful to debug an LVGL port)*/
#define LV_USE_LINE_DEBUG 0

/*1: Draw overlays with different colors for each draw unit
 *1: Draw overlays with different colors for each draw unit (mask, fill, border, etc.)*/
#define LV_USE_DRAW_MASKS_DEBUG 0

/*1: Draw overlays with different colors for each draw unit
 *1: Draw overlays with different colors for each draw unit (mask, fill, border, etc.)*/
#define LV_USE_DRAW_UNIT_DEBUG 0

/*1: Keep the display refreshed even if nothing is happening
 *1: Keep the display refreshed even if nothing is happening*/
#define LV_USE_PERF_MONITOR 0

/*1: Show the used memory and the memory fragmentation
 * Requires `LV_MEM_CUSTOM = 0`*/
#define LV_USE_MEM_MONITOR 0

/*Change the built in fonts (Enable the fonts you need in lv_conf.h and set it to 1.)
 *Use the built-in fonts or declare custom fonts here.
 *The order is important, as it determines the order in which the font fallback works.*/
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

/*Demonstrate special features*/
#define LV_FONT_MONTSERRAT_12_SUBPX      0
#define LV_FONT_MONTSERRAT_28_COMPRESSED  0
#define LV_FONT_DEJAVU_16_PERSIAN_HEBREW  0
#define LV_FONT_SIMSUN_16_CJK             0
#define LV_FONT_SIMSUN_14_CJK             0

/*Pixel perfect monospace fonts*/
#define LV_FONT_UNSCII_8  0
#define LV_FONT_UNSCII_16 0

/*Optionally declare custom fonts here*/
#define LV_FONT_CUSTOM_DECLARE

/*Always set a default font*/
#define LV_FONT_DEFAULT &lv_font_montserrat_14

/*Enable handling large font and/or characters which are stored as arrays
 *in `lv_font_fmt_txt_glyph_dsc_t` (instead of simple pointer to glyph bitmap)*/
#define LV_USE_FONT_COMPRESSED 1

/*Enable subpixel rendering*/
#define LV_USE_FONT_SUBPX 0

/*Enable drawing placeholders when glyph dsc is not found*/
#define LV_USE_FONT_PLACEHOLDER 1

/*================
 *  THEME USAGE
 *================*/

/*Always enable at least one theme*/

/*A simple, impressive and very complete theme*/
#define LV_USE_THEME_DEFAULT 1
#if LV_USE_THEME_DEFAULT
    #define LV_THEME_DEFAULT_DARK 0
    #define LV_THEME_DEFAULT_GROW 1
    #define LV_THEME_DEFAULT_TRANSITION_TIME 80
#endif

/*A very simple theme that is a good starting point for custom themes*/
#define LV_USE_THEME_BASIC 1

/*A theme designed for monochrome displays*/
#define LV_USE_THEME_MONO 1

/*================
 *  LAYOUTS
 *================*/

/*A layout similar to Flexbox in CSS.*/
#define LV_USE_FLEX 1

/*A layout similar to Grid in CSS.*/
#define LV_USE_GRID 1

/*==================
 *  WIDGETS
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
#define LV_USE_LINE      1
#define LV_USE_ROLLER    1
#define LV_USE_SLIDER    1
#define LV_USE_SWITCH    1
#define LV_USE_TEXTAREA  1
#define LV_USE_TABLE     1
#define LV_USE_CALENDAR  1
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

/*==================
 *  EXAMPLES
 *==================*/

#define LV_BUILD_EXAMPLES 0

/*==================
 * OTHERS
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
 * DEVICES
 *==================*/

/*Use SDL to open window on PC and simulate a display and mouse or keyboard*/
#define LV_USE_SDL 1
#if LV_USE_SDL
    #define LV_SDL_INCLUDE_PATH    <SDL2/SDL.h>
    #define LV_SDL_FULLSCREEN       0
    #define LV_SDL_MOUSEWHEEL_MODE  LV_SDL_MOUSEWHEEL_MODE_ENCODER
#endif

/*==================
 * COMPILER SETTINGS
 *================*/

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

/*!--END OF LV_CONF_H--*/

#endif /*LV_CONF_H*/
