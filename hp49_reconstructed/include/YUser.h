#ifndef YUSER_H
#define YUSER_H

#include <windows.h>
#include "YorkeM.h"
#include "YMHelper.h"

/* Command IDs reconstructed from YUser.exe .rsrc and disassembly */
#define IDD_ABOUTBOX               100
#define IDD_CONSOLE                131
#define IDR_MAINFRAME              130
#define IDC_CONSOLE_INPUT          1005
#define IDC_CONSOLE_OUTPUT         1006

#define ID_OBJECT_LOAD_KERMIT      0x800D
#define ID_OBJECT_LOAD_XMODEM      0x800E
#define ID_OBJECT_LOAD_STACK       0x8008
#define ID_OBJECT_LOAD_PORT1       0x800F
#define ID_STATE_LOAD              0x8010
#define ID_STATE_SAVE              0x8011
#define ID_ENGINE_START            0x8004
#define ID_ENGINE_STOP             0x8005
#define ID_ENGINE_RESET            0x800B
#define ID_CONSOLE_SHOW            0x3ED
#define ID_WINDOW_MINIMIZE         0x8003
#define ID_APP_ABOUT               0x800A
#define ID_APP_EXIT                0x8011

/* Display configuration (reconstructed from YUser.exe 0x402ac9) */
#define HP49_LCD_X                 42
#define HP49_LCD_Y                 77
#define HP49_LCD_WIDTH             131
#define HP49_LCD_HEIGHT            64
#define HP49_LCD_SCALE             2

#define HP49_SKIN_WIDTH            347
#define HP49_SKIN_HEIGHT           740
#define HP49_BMP_FULL_HEIGHT       751

/* Annunciator pixel positions on HP49.BMP (reconstructed from 0x4040b0) */
#define ANNUNCIATOR_Y_SRC          740
#define ANNUNCIATOR_Y_DST          60
#define ANNUNCIATOR_WIDTH          16
#define ANNUNCIATOR_HEIGHT         11
#define ANNUNCIATOR_COUNT          6

/* Key representation (reconstructed from 0x402580) */
struct CKey {
    RECT rect;         // Hitbox: left, top, right, bottom
    int  row;          // Saturn keyboard matrix row
    int  col;          // Saturn keyboard matrix col (or control ID)
    int  radius;       // Circle radius for click animation = min(w, h)/2 - 1
    WORD vk;           // PC Virtual Key code
};

extern const CKey g_HP49Keys[54];

#endif /* YUSER_H */
