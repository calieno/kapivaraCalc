/*
 * YUser.cpp - HP 49G Calculator Emulator User Interface
 * Reconstructed from YUser.exe / YUserfast.exe (Sebastien Carlier, ACO / HP 1999)
 *
 * Reverse Engineered via REA (Reverse Engineer Anything):
 * Evidence IDs:
 *   ev_8387e03e0bb16ceb91a4c4d2994e4625b561a3f46db3255e7ee8daa67acaadc5
 *   ev_65043a20b59152a1ec306ebdeae89d03413747394a98c59791974c20387082e7
 *   ev_a56fe679d217fb50eed97076237451c301039e6337a69a1428b2a31ec0ba77b1
 *   ev_d4c3624b0dbe7fcc374048c4feec523352e80d3e85ec0d77a86e74c80fbe0d76
 *   ev_8dc5917020d46ef211353cbb49fc891e6a55c5f031e23b81e9c9f80005910529
 */

#include "YUser.h"
#include <stdio.h>
#include <math.h>

/* Exact 54-Key Table extracted from YUser.exe 0x401390 static initializer */
const CKey g_HP49Keys[54] = {
    // Left, Top, Right, Bottom, Row, Col, Radius, VK
    { { 331,   0, 347,  16 }, 10, 0x8003,  7, 0xff }, // 0: Top-right minimize button
    { {   0,   0,  16,  16 }, 10, 0x8011,  7, 0xff }, // 1: Top-left close button
    { {  21, 275,  66, 302 },  5, 0x0000, 12, 0x41 }, // 2: F1  (A)
    { {  72, 275, 117, 302 },  5, 0x0001, 12, 0x42 }, // 3: F2  (B)
    { { 124, 275, 169, 302 },  5, 0x0002, 12, 0x43 }, // 4: F3  (C)
    { { 176, 275, 221, 302 },  5, 0x0003, 12, 0x44 }, // 5: F4  (D)
    { { 228, 275, 273, 302 },  5, 0x0004, 12, 0x45 }, // 6: F5  (E)
    { { 281, 275, 326, 302 },  5, 0x0005, 12, 0x46 }, // 7: F6  (F)
    { {  27, 322,  79, 353 },  5, 0x0007, 14, 0x47 }, // 8: APPS (G)
    { {  87, 322, 139, 353 },  4, 0x0007, 14, 0x48 }, // 9: MODE (H)
    { { 146, 322, 198, 353 },  3, 0x0007, 14, 0x49 }, // 10: TOOL (I)
    { {  27, 363,  79, 394 },  2, 0x0007, 14, 0x4a }, // 11: VAR  (J)
    { {  87, 363, 139, 394 },  1, 0x0007, 14, 0x4b }, // 12: STO  (K)
    { { 146, 363, 198, 394 },  0, 0x0007, 14, 0x4c }, // 13: NXT  (L)
    { { 237, 305, 285, 338 },  6, 0x0003, 15, 0x26 }, // 14: UP D-PAD
    { { 237, 385, 285, 418 },  6, 0x0001, 15, 0x28 }, // 15: DOWN D-PAD
    { { 206, 343, 253, 377 },  6, 0x0002, 15, 0x25 }, // 16: LEFT D-PAD
    { { 271, 343, 318, 377 },  6, 0x0000, 15, 0x27 }, // 17: RIGHT D-PAD
    { {  27, 420,  81, 449 },  4, 0x0006, 13, 0x4d }, // 18: HIST (M)
    { {  87, 420, 141, 449 },  3, 0x0006, 13, 0x4e }, // 19: EVAL (N)
    { { 146, 420, 200, 449 },  2, 0x0006, 13, 0x4f }, // 20: '    (O)
    { { 206, 420, 260, 449 },  1, 0x0006, 13, 0x50 }, // 21: SYMB (P)
    { { 266, 420, 320, 449 },  0, 0x0006, 13, 0x08 }, // 22: DEL/BS
    { {  27, 460,  81, 489 },  4, 0x0005, 13, 0x51 }, // 23: Y^X  (Q)
    { {  87, 460, 141, 489 },  3, 0x0005, 13, 0x52 }, // 24: SQRT (R)
    { { 146, 460, 200, 489 },  2, 0x0005, 13, 0x53 }, // 25: SIN  (S)
    { { 206, 460, 260, 489 },  1, 0x0005, 13, 0x54 }, // 26: COS  (T)
    { { 266, 460, 320, 489 },  0, 0x0005, 13, 0x55 }, // 27: TAN  (U)
    { {  27, 500,  81, 529 },  4, 0x0004, 13, 0x56 }, // 28: EEX  (V)
    { {  87, 500, 141, 529 },  3, 0x0004, 13, 0x57 }, // 29: +/-  (W)
    { { 146, 500, 200, 529 },  2, 0x0004, 13, 0x58 }, // 30: X    (X)
    { { 206, 500, 260, 529 },  1, 0x0004, 13, 0x59 }, // 31: 1/X  (Y)
    { { 266, 500, 320, 529 },  0, 0x0004, 13, 0x5a }, // 32: /    (Z)
    { {  27, 543,  81, 578 },  7, 0x0003, 16, 0x09 }, // 33: ALPHA
    { {  87, 543, 141, 578 },  3, 0x0003, 16, 0x37 }, // 34: 7
    { { 146, 543, 200, 578 },  2, 0x0003, 16, 0x38 }, // 35: 8
    { { 206, 543, 260, 578 },  1, 0x0003, 16, 0x39 }, // 36: 9
    { { 266, 543, 320, 578 },  0, 0x0003, 16, 0x2a }, // 37: *
    { {  27, 588,  81, 623 },  7, 0x0002, 16, 0x10 }, // 38: LEFT SHIFT (Red)
    { {  87, 588, 141, 623 },  3, 0x0002, 16, 0x34 }, // 39: 4
    { { 146, 588, 200, 623 },  2, 0x0002, 16, 0x35 }, // 40: 5
    { { 206, 588, 260, 623 },  1, 0x0002, 16, 0x36 }, // 41: 6
    { { 266, 588, 320, 623 },  0, 0x0002, 16, 0x2d }, // 42: -
    { {  27, 632,  81, 667 },  7, 0x0001, 16, 0x11 }, // 43: RIGHT SHIFT (Cyan)
    { {  87, 632, 141, 667 },  3, 0x0001, 16, 0x31 }, // 44: 1
    { { 146, 632, 200, 667 },  2, 0x0001, 16, 0x32 }, // 45: 2
    { { 206, 632, 260, 667 },  1, 0x0001, 16, 0x33 }, // 46: 3
    { { 266, 632, 320, 667 },  0, 0x0001, 16, 0xbb }, // 47: +
    { {  27, 677,  81, 712 },  9, 0x0009, 16, 0x1b }, // 48: ON / Cancel
    { {  87, 677, 141, 712 },  3, 0x0000, 16, 0x30 }, // 49: 0
    { { 146, 677, 200, 712 },  2, 0x0000, 16, 0x2e }, // 50: . (Dot)
    { { 146, 677, 200, 712 },  2, 0x0000, 16, 0xbe }, // 51: . (Numpad dot)
    { { 206, 677, 260, 712 },  1, 0x0000, 16, 0x20 }, // 52: SPC
    { { 266, 677, 320, 712 },  0, 0x0000, 16, 0x0d }  // 53: ENTER
};

/* Global application variables */
static HWND    g_hMainWnd = NULL;
static HBITMAP g_hSkinBitmap = NULL;
static HDC     g_hSkinDC = NULL;
static int     g_nPressedKeyIdx = -1;
static BOOL    g_bIsDragging = FALSE;
static POINT   g_ptDragStart;
static int     g_nSerialMode = 0; // 0=Console, 1=Kermit, 2=XModem

/* Forward declarations */
static LRESULT CALLBACK MainWndProc(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam);
static DWORD WINAPI SerialThread(LPVOID lpParam);
static void Draw3DHighlight(HDC hdc, int cx, int cy, int radius);
static void SnapToScreenEdges(HWND hWnd);

/* WinMain entry point */
int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow)
{
    WNDCLASSEX wc = {0};
    wc.cbSize        = sizeof(WNDCLASSEX);
    wc.style         = CS_HREDRAW | CS_VREDRAW;
    wc.lpfnWndProc   = MainWndProc;
    wc.hInstance     = hInstance;
    wc.hIcon         = LoadIcon(NULL, IDI_APPLICATION);
    wc.hCursor       = LoadCursor(NULL, IDC_ARROW);
    wc.hbrBackground = (HBRUSH)GetStockObject(BLACK_BRUSH);
    wc.lpszClassName = "YorkeM_UI_Class";

    if (!RegisterClassEx(&wc)) {
        MessageBoxA(NULL, "Failed to register window class", "YorkeM Error", MB_ICONERROR);
        return 1;
    }

    g_hMainWnd = CreateWindowExA(
        WS_EX_ACCEPTFILES,
        "YorkeM_UI_Class",
        "YorkeM User Interface",
        WS_POPUP | WS_VISIBLE,
        CW_USEDEFAULT, CW_USEDEFAULT,
        HP49_SKIN_WIDTH, HP49_SKIN_HEIGHT,
        NULL, NULL, hInstance, NULL
    );

    if (!g_hMainWnd) {
        MessageBoxA(NULL, "Failed to create main window", "YorkeM Error", MB_ICONERROR);
        return 1;
    }

    ShowWindow(g_hMainWnd, nCmdShow);
    UpdateWindow(g_hMainWnd);

    MSG msg;
    while (GetMessage(&msg, NULL, 0, 0)) {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }

    return (int)msg.wParam;
}

/* Main window procedure */
static LRESULT CALLBACK MainWndProc(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam)
{
    switch (msg) {
    case WM_CREATE: {
        // Configure speed: param 1 = 1400 cycles/quantum for realistic ~4MHz (YUser.exe)
        // YUserfast.exe passes param 2 = 1 (fast as possible)
        _YMConfigure(1, 1400);
        _YMConfigure(2, 1);

        // Load HP49 ROM image and initialize RAM ports
        if (!_YHLoadRom("ROM.E49")) {
            MessageBoxA(hWnd, "Cannot find ROM.E49!", "YorkeM Error", MB_ICONERROR);
        }
        _YHCreatePort0(0x80000); // 256 KB Port 0
        _YHCreatePort1(0x40000); // 128 KB Port 1
        _YHCreatePort2(0x40000); // 128 KB Port 2

        // Load skin bitmap (HP49.BMP, 347x751, 24bpp)
        g_hSkinBitmap = (HBITMAP)LoadImageA(
            NULL, "HP49.BMP", IMAGE_BITMAP, 0, 0,
            LR_LOADFROMFILE | LR_CREATEDIBSECTION
        );
        if (g_hSkinBitmap) {
            HDC hDC = GetDC(hWnd);
            g_hSkinDC = CreateCompatibleDC(hDC);
            SelectObject(g_hSkinDC, g_hSkinBitmap);
            ReleaseDC(hWnd, hDC);
        }

        // Attach LCD display to window DC
        _YHSetDisplay(hWnd, HP49_LCD_X, HP49_LCD_Y, HP49_LCD_WIDTH, HP49_LCD_HEIGHT, HP49_LCD_SCALE);

        // Start serial communications background worker thread
        CreateThread(NULL, 0, SerialThread, hWnd, 0, NULL);

        // Start local debug/SDK server on port 4200
        _YHStartServer(4200);

        // Annunciator update timer (100ms)
        SetTimer(hWnd, 1, 100, NULL);

        // Start Saturn CPU execution
        _YMRun();
        return 0;
    }

    case WM_PAINT: {
        PAINTSTRUCT ps;
        HDC hdc = BeginPaint(hWnd, &ps);
        if (g_hSkinDC) {
            BitBlt(hdc, 0, 0, HP49_SKIN_WIDTH, HP49_SKIN_HEIGHT, g_hSkinDC, 0, 0, SRCCOPY);
        }
        _YHRefreshDisplay();
        EndPaint(hWnd, &ps);
        return 0;
    }

    case WM_TIMER: {
        // Poll annunciators from CPU register
        unsigned char ann = _YMGetAnnunciators();
        HDC hdc = GetDC(hWnd);
        if (g_hSkinDC && hdc) {
            for (int i = 0; i < ANNUNCIATOR_COUNT; ++i) {
                int x_dst = 60 + i * 51;
                if (ann & (1 << i)) {
                    // Draw active annunciator icon from bottom strip of HP49.BMP (y=740)
                    BitBlt(hdc, x_dst, ANNUNCIATOR_Y_DST, ANNUNCIATOR_WIDTH, ANNUNCIATOR_HEIGHT,
                           g_hSkinDC, 60 + i * 51, ANNUNCIATOR_Y_SRC, SRCCOPY);
                } else {
                    // Erase annunciator (restore skin background)
                    BitBlt(hdc, x_dst, ANNUNCIATOR_Y_DST, ANNUNCIATOR_WIDTH, ANNUNCIATOR_HEIGHT,
                           g_hSkinDC, x_dst, ANNUNCIATOR_Y_DST, SRCCOPY);
                }
            }
        }
        ReleaseDC(hWnd, hdc);
        return 0;
    }

    case WM_LBUTTONDOWN: {
        int x = (short)LOWORD(lParam);
        int y = (short)HIWORD(lParam);
        POINT pt = { x, y };

        // Test if click is on skin keys
        int hit_key = -1;
        for (int i = 0; i < 54; ++i) {
            if (PtInRect(&g_HP49Keys[i].rect, pt)) {
                hit_key = i;
                break;
            }
        }

        if (hit_key >= 0) {
            g_nPressedKeyIdx = hit_key;
            if (g_HP49Keys[hit_key].row == 10) {
                // Window control buttons
                if (g_HP49Keys[hit_key].col == ID_WINDOW_MINIMIZE) {
                    ShowWindow(hWnd, SW_MINIMIZE);
                } else if (g_HP49Keys[hit_key].col == ID_APP_EXIT) {
                    PostQuitMessage(0);
                }
            } else {
                // Physical calculator key: draw 3D translucent highlight & press in Saturn
                HDC hdc = GetDC(hWnd);
                int cx = (g_HP49Keys[hit_key].rect.left + g_HP49Keys[hit_key].rect.right) / 2;
                int cy = (g_HP49Keys[hit_key].rect.top + g_HP49Keys[hit_key].rect.bottom) / 2;
                Draw3DHighlight(hdc, cx, cy, g_HP49Keys[hit_key].radius);
                ReleaseDC(hWnd, hdc);

                _YMPressKey(g_HP49Keys[hit_key].col, g_HP49Keys[hit_key].row);
            }
        } else {
            // Dragging window by skin surface
            g_bIsDragging = TRUE;
            SetCapture(hWnd);
            GetCursorPos(&g_ptDragStart);
        }
        return 0;
    }

    case WM_LBUTTONUP: {
        if (g_bIsDragging) {
            g_bIsDragging = FALSE;
            ReleaseCapture();
            SnapToScreenEdges(hWnd);
        }

        if (g_nPressedKeyIdx >= 0) {
            int idx = g_nPressedKeyIdx;
            g_nPressedKeyIdx = -1;

            if (g_HP49Keys[idx].row != 10) {
                _YMReleaseKey(g_HP49Keys[idx].col, g_HP49Keys[idx].row);
                InvalidateRect(hWnd, &g_HP49Keys[idx].rect, FALSE);
            }
        }
        return 0;
    }

    case WM_MOUSEMOVE: {
        if (g_bIsDragging) {
            POINT ptCur;
            GetCursorPos(&ptCur);
            RECT rcWnd;
            GetWindowRect(hWnd, &rcWnd);

            int dx = ptCur.x - g_ptDragStart.x;
            int dy = ptCur.y - g_ptDragStart.y;
            g_ptDragStart = ptCur;

            SetWindowPos(hWnd, NULL, rcWnd.left + dx, rcWnd.top + dy, 0, 0,
                         SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE);
        }
        return 0;
    }

    case WM_KEYDOWN: {
        WORD vk = (WORD)wParam;
        for (int i = 2; i < 54; ++i) {
            if (g_HP49Keys[i].vk == vk) {
                _YMPressKey(g_HP49Keys[i].col, g_HP49Keys[i].row);
                break;
            }
        }
        return 0;
    }

    case WM_KEYUP: {
        WORD vk = (WORD)wParam;
        for (int i = 2; i < 54; ++i) {
            if (g_HP49Keys[i].vk == vk) {
                _YMReleaseKey(g_HP49Keys[i].col, g_HP49Keys[i].row);
                break;
            }
        }
        return 0;
    }

    case WM_DESTROY: {
        KillTimer(hWnd, 1);
        _YMStop();
        _YHShutdownServer();
        if (g_hSkinDC) DeleteDC(g_hSkinDC);
        if (g_hSkinBitmap) DeleteObject(g_hSkinBitmap);
        PostQuitMessage(0);
        return 0;
    }
    }
    return DefWindowProcA(hWnd, msg, wParam, lParam);
}

/* Snap window to desktop borders (reconstructed from 0x402f60) */
static void SnapToScreenEdges(HWND hWnd)
{
    RECT rc;
    GetWindowRect(hWnd, &rc);

    int scrW = GetSystemMetrics(SM_CXSCREEN);
    int scrH = GetSystemMetrics(SM_CYSCREEN);
    int w = rc.right - rc.left;
    int h = rc.bottom - rc.top;

    int newX = rc.left;
    int newY = rc.top;

    if (abs(rc.left) < 16) newX = 0;
    else if (abs(rc.right - scrW) < 16) newX = scrW - w;

    if (abs(rc.top) < 16) newY = 0;
    else if (abs(rc.bottom - scrH) < 16) newY = scrH - h;

    if (newX != rc.left || newY != rc.top) {
        SetWindowPos(hWnd, NULL, newX, newY, 0, 0, SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE);
    }
}

/* Render 3D button press translucent circular highlight (reconstructed from 0x403060..0x403240) */
static void Draw3DHighlight(HDC hdc, int cx, int cy, int radius)
{
    for (int dy = -radius; dy <= radius; ++dy) {
        int span = (int)sqrt((double)(radius * radius - dy * dy));
        for (int dx = -span; dx <= span; ++dx) {
            int px = cx + dx;
            int py = cy + dy;
            COLORREF c = GetPixel(hdc, px, py);
            int r = GetRValue(c);
            int g = GetGValue(c);
            int b = GetBValue(c);

            // Add lighting gradient based on distance from center (specular reflection)
            int light = (int)(120.0 * (1.0 - (double)(dx * dx + dy * dy) / (double)(radius * radius)));
            if (light < 0) light = 0;

            r = min(255, r + light);
            g = min(255, g + light);
            b = min(255, b + light);
            SetPixel(hdc, px, py, RGB(r, g, b));
        }
    }
}

/* Serial communication background thread (reconstructed from 0x4025d0) */
static DWORD WINAPI SerialThread(LPVOID lpParam)
{
    HANDLE hSerial = _YHSerialGetHandle();
    if (!hSerial) return 0;

    unsigned char byte_buf;
    DWORD bytes_read;

    while (TRUE) {
        WaitForSingleObject(hSerial, INFINITE);
        if (ReadFile(hSerial, &byte_buf, 1, &bytes_read, NULL) && bytes_read == 1) {
            if (g_nSerialMode == 1) {
                _YHKermitOnByteReceived(byte_buf);
            } else if (g_nSerialMode == 2) {
                _YHXModemOnByteReceived(byte_buf);
            }
        }
    }
    return 0;
}
