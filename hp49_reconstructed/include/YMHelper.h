#ifndef YMHELPER_H
#define YMHELPER_H

#ifdef __cplusplus
extern "C" {
#endif

#ifdef _WIN32
  #ifdef YMHELPER_EXPORTS
    #define YH_API __declspec(dllexport) __stdcall
  #else
    #define YH_API __declspec(dllimport) __stdcall
  #endif
  #include <windows.h>
#else
  #define YH_API
  typedef void* HWND;
  typedef void* HANDLE;
#endif

/* YMHelper - Helper services for Yorke HP49 Emulator
 * Reconstructed from YMHelper.dll (version 1.0, 1999)
 * Reverse Engineered via REA Evidence IDs:
 * ev_00326ef0180cb48543a0b31e6706b076250ceca7123351cc250544ebb3b76283
 * ev_3b18ccb91156525ee1d1903cc011c213b9aac63bbe633cd99cf54699fee17152
 * ev_d671d5368c936745fb698279b60ca8f4fed3aaee8665f4d45e27e29da2d8b3bc
 */

/* ROM & Memory Port Management */
int  YH_API _YHLoadRom(const char* rom_path);
int  YH_API _YHCreateRom(int size_nibbles);
void YH_API _YHDestroyRom(void);
void YH_API _YHGetRom(unsigned char** ptr, int* size);

int  YH_API _YHCreatePort0(int size_nibbles);
void YH_API _YHDestroyPort0(void);
void YH_API _YHGetPort0(unsigned char** ptr, int* size);

int  YH_API _YHCreatePort1(int size_nibbles);
void YH_API _YHDestroyPort1(void);
void YH_API _YHGetPort1(unsigned char** ptr, int* size);

int  YH_API _YHCreatePort2(int size_nibbles);
void YH_API _YHDestroyPort2(void);
void YH_API _YHGetPort2(unsigned char** ptr, int* size);

/* Direct Nibble Read/Write */
unsigned char YH_API _YHReadNibble(unsigned int address);
void YH_API _YHWriteNibble(unsigned int address, unsigned char nibble);

/* Display Interface */
int  YH_API _YHSetDisplay(HWND hWnd, int x, int y, int width, int height, int scale);
void YH_API _YHRefreshDisplay(void);
int  YH_API _YHGetDisplayedFPS(void);
int  YH_API _YHGetRenderedFPS(void);

/* Object & File Loader */
int  YH_API _YHPushObject(const unsigned char* obj_nibbles, int length_nibbles);
int  YH_API _YHLoadInPort1(const char* name, const unsigned char* obj_nibbles, int length_nibbles);

/* Communications (Kermit / XModem) */
HANDLE YH_API _YHSerialGetHandle(void);
int  YH_API _YHSerialSendByte(unsigned char byte, int timeout_ms);
void YH_API _YHKermitOnByteReceived(unsigned char byte);
int  YH_API _YHKermitSendFile(void (*status_callback)(int), const char* remote_name, const unsigned char* data, int length);
void YH_API _YHXModemOnByteReceived(unsigned char byte);
int  YH_API _YHXModemSendFile(void (*status_callback)(int), const unsigned char* data, int length);

/* Breakpoints & Disassembler */
int  YH_API _YHAddBreakpoint(unsigned int addr);
int  YH_API _YHRemoveBreakpoint(unsigned int addr);
int  YH_API _YHAddRomBreakpoint(unsigned int addr);
int  YH_API _YHRemoveRomBreakpoint(unsigned int addr);
void YH_API _YHSetBreakpointAction(unsigned int addr, int action, void* user_data);
int  YH_API _YHDisassemble(unsigned int addr, char* buffer, int max_chars);
void YH_API _YHAddSymbol(unsigned int addr, const char* name);
int  YH_API _YHGetSymbol(unsigned int addr, char* buffer, int max_chars);
void YH_API _YHSetSymbolBase(unsigned int base_addr);

/* Debug / Remote Server */
int  YH_API _YHStartServer(int port);
void YH_API _YHShutdownServer(void);

#ifdef __cplusplus
}
#endif

#endif /* YMHELPER_H */
