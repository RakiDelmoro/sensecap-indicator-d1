/**
 * LVGL PC Simulator for SenseCap Indicator
 * 
 * This simulator runs the SquareLine Studio generated UI on PC using SDL2
 */

#include <SDL2/SDL.h>
#include "lvgl/lvgl.h"
#include "ui.h"

/*Screen dimensions matching SenseCap Indicator D1 display (480x480 circular display)*/
#define DISP_HOR_RES 480
#define DISP_VER_RES 480

/*Mouse cursor icon for the simulator*/
#define USE_MOUSE_CURSOR 0

/*SDL window and renderer*/
static SDL_Window *window = NULL;
static SDL_Renderer *renderer = NULL;
static SDL_Texture *texture = NULL;

/*LVGL display buffer*/
static lv_color_t buf1[DISP_HOR_RES * DISP_VER_RES / 10];
static lv_color_t buf2[DISP_HOR_RES * DISP_VER_RES / 10];

/*Flush function for LVGL*/
static void sdl_flush_cb(lv_disp_drv_t *disp_drv, const lv_area_t *area, lv_color_t *color_p)
{
    lv_coord_t w = area->x2 - area->x1 + 1;
    lv_coord_t h = area->y2 - area->y1 + 1;
    
    /*Update SDL texture with the rendered area*/
    SDL_UpdateTexture(texture, &(SDL_Rect){area->x1, area->y1, w, h}, 
                      color_p, w * sizeof(lv_color_t));
    
    /*Render to screen*/
    SDL_RenderClear(renderer);
    SDL_RenderCopy(renderer, texture, NULL, NULL);
    SDL_RenderPresent(renderer);
    
    lv_disp_flush_ready(disp_drv);
}

/*Mouse read function*/
static void sdl_mouse_read(lv_indev_drv_t *indev_drv, lv_indev_data_t *data)
{
    (void)indev_drv;
    
    int x, y;
    SDL_GetMouseState(&x, &y);
    data->point.x = x;
    data->point.y = y;
    
    if(SDL_GetMouseState(NULL, NULL) & SDL_BUTTON(SDL_BUTTON_LEFT)) {
        data->state = LV_INDEV_STATE_PRESSED;
    } else {
        data->state = LV_INDEV_STATE_RELEASED;
    }
}

int main(int argc, char **argv)
{
    (void)argc; /*Unused*/
    (void)argv; /*Unused*/
    
    /*Initialize SDL*/
    if(SDL_Init(SDL_INIT_VIDEO | SDL_INIT_TIMER | SDL_INIT_EVENTS) != 0) {
        fprintf(stderr, "Failed to initialize SDL: %s\n", SDL_GetError());
        return 1;
    }
    
    /*Create SDL window*/
    window = SDL_CreateWindow(
        "SenseCap Indicator Simulator",
        SDL_WINDOWPOS_CENTERED,
        SDL_WINDOWPOS_CENTERED,
        DISP_HOR_RES,
        DISP_VER_RES,
        SDL_WINDOW_SHOWN
    );
    
    if(!window) {
        fprintf(stderr, "Failed to create window: %s\n", SDL_GetError());
        return 1;
    }
    
    /*Create SDL renderer*/
    renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC);
    if(!renderer) {
        fprintf(stderr, "Failed to create renderer: %s\n", SDL_GetError());
        return 1;
    }
    
    /*Create texture for LVGL rendering*/
    texture = SDL_CreateTexture(renderer, SDL_PIXELFORMAT_RGB565, SDL_TEXTUREACCESS_STREAMING, DISP_HOR_RES, DISP_VER_RES);
    if(!texture) {
        fprintf(stderr, "Failed to create texture: %s\n", SDL_GetError());
        return 1;
    }
    
    /*Initialize LVGL*/
    lv_init();
    
    /*Initialize display buffer*/
    static lv_disp_draw_buf_t draw_buf;
    lv_disp_draw_buf_init(&draw_buf, buf1, buf2, DISP_HOR_RES * DISP_VER_RES / 10);
    
    /*Initialize display driver*/
    static lv_disp_drv_t disp_drv;
    lv_disp_drv_init(&disp_drv);
    disp_drv.hor_res = DISP_HOR_RES;
    disp_drv.ver_res = DISP_VER_RES;
    disp_drv.flush_cb = sdl_flush_cb;
    disp_drv.draw_buf = &draw_buf;
    lv_disp_drv_register(&disp_drv);
    
    /*Initialize mouse input device*/
    static lv_indev_drv_t indev_drv;
    lv_indev_drv_init(&indev_drv);
    indev_drv.type = LV_INDEV_TYPE_POINTER;
    indev_drv.read_cb = sdl_mouse_read;
    lv_indev_drv_register(&indev_drv);
    
    /*Initialize the UI - this calls ui_init() which loads Screen_1*/
    ui_init();
    
    printf("SenseCap Indicator Simulator started!\n");
    printf("Window size: %dx%d\n", DISP_HOR_RES, DISP_VER_RES);
    printf("Close window to exit.\n");
    
    /*Main loop*/
    int running = 1;
    SDL_Event event;
    
    while(running) {
        /*Handle SDL events*/
        while(SDL_PollEvent(&event)) {
            if(event.type == SDL_QUIT) {
                running = 0;
            }
        }
        
        /*Handle LVGL tasks*/
        lv_timer_handler();
        
        /*Increment LVGL tick*/
        lv_tick_inc(5);
        
        /*Small delay to prevent 100% CPU usage*/
        SDL_Delay(5);
    }
    
    /*Cleanup*/
    ui_destroy();
    SDL_DestroyTexture(texture);
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();
    
    return 0;
}
