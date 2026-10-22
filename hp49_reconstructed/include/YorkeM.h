#ifndef YORKEM_H
#define YORKEM_H

#ifdef __cplusplus
extern "C" {
#endif

#ifdef _WIN32
  #ifdef YORKEM_EXPORTS
    #define YM_API __declspec(dllexport) __stdcall
  #else
    #define YM_API __declspec(dllimport) __stdcall
  #endif
#else
  #define YM_API
#endif

/* YorkeM Core - Saturn CPU and HP49 ASIC Hardware Emulator Core
 * Reconstructed from Yorkem.dll (version 1.0.7.0, Sebastien Carlier 1999)
 * Reverse Engineered via REA Evidence IDs:
 * ev_e733e7d156259b16f4112321fc16c1156329f8f1767c8daa35a718c3ecf8b8fb
 * ev_471220e22b0dd269ce958f382e61ea5d67c0985c8c80cccad289a9fda08a54bb
 * ev_02969a90836108d597232c1bd70b2c032a109a1fad96f7cebd34a1fe64f6bb0c
 */

/* Execution control */
void YM_API _YMRun(void);
void YM_API _YMStop(void);
void YM_API _YMReset(void);
void YM_API _YMStep(void);
int  YM_API _YMIsRunning(void);
int  YM_API _YMConfigure(int param, int value);

/* Keyboard interface */
void YM_API _YMPressKey(int col, int row);
void YM_API _YMReleaseKey(int col, int row);
int  YM_API _YMIsKeyPressed(int col, int row);
void YM_API _YMSetKeyboardHandler(void (*handler)(int, int, int));

/* Display and Annunciators */
unsigned char YM_API _YMGetAnnunciators(void);
void YM_API _YMForceRefresh(void);
void YM_API _YMSetDisplayHandler(void (*handler)(const unsigned char* buffer, int width, int height));
void YM_API _YMSetDisplaySize(int width, int height);
void YM_API _YMDiscardFrame(int discard);

/* Memory and MMU */
int  YM_API _YMMemoryConfig(int bank);
void YM_API _YMMemoryReset(void);
int  YM_API _YMMemoryUncnfg(int bank);
int  YM_API _YMSetMemoryModule(int bank, unsigned char* ptr, int size_nibbles);
unsigned int YM_API _YMTranslateAddress(unsigned int logical_addr, int mode);

/* Registers and CPU State */
unsigned int YM_API _YMGetRegister(int reg_id);
void YM_API _YMSetRegister(int reg_id, unsigned int value);
unsigned int YM_API _YMRstkPop(void);
void YM_API _YMRstkPush(unsigned int addr);
int  YM_API _YMSaveState(void* buffer, int max_bytes);
int  YM_API _YMRestoreState(const void* buffer, int bytes);

/* Serial Communications & Hooks */
int  YM_API _YMSerialSendByte(unsigned char byte, int timeout_ms);
void YM_API _YMSetSerialHandler(void (*handler)(unsigned char));
void YM_API _YMSetInterruptHandler(int irq, void (*handler)(void));
void YM_API _YMSetBreakpointHandler(int (*handler)(unsigned int pc));
void YM_API _YMSetDebuggingHackHandler(void (*handler)(void));
int  YM_API _YMAddHook(unsigned int addr, void (*hook_fn)(void));
int  YM_API _YMRemoveHook(unsigned int addr);
void YM_API _YMChainNextHook(void);
void YM_API _YMExecuteRoutine(unsigned int addr);

/* Logging */
void YM_API _YMLogClear(void);
void YM_API _YMLogConfigure(int flags, int buffer_size);
int  YM_API _YMLogRead(char* dest, int max_len);

#ifdef __cplusplus
}
#endif

#endif /* YORKEM_H */
