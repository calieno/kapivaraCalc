# HP 49G Calculator Emulator (Reconstructed Source)

> **Reverse-engineered C/C++ source code of Hewlett-Packard's official HP 49G emulator (`YUser.exe`, `Yorkem.dll`, `YMHelper.dll`) and ROM `rom.e49`.**  
> Reconstructed using REA (*Reverse Engineer Anything*) and the Antigravity Analysis Framework.

---

## 📁 Repository Structure

```text
hp49_reconstructed/
├── CMakeLists.txt              # Modern CMake build configuration
├── Makefile                    # MinGW/GCC Makefile
├── README.md                   # This overview
├── REVERSE_ENGINEERING_REPORT.md # Full technical reverse engineering report
├── include/
│   ├── YUser.h                 # Window, layout, button geometry and keymap definitions
│   ├── YorkeM.h                # Saturn CPU emulator and hardware engine exports (38 functions)
│   └── YMHelper.h               # ROM/RAM memory manager and display helper exports (38 functions)
└── src/
    └── YUser.cpp               # Fully reconstructed Win32 application, event loop, LCD render & skinning
```

---

## 🛠️ Build Instructions

### Building with MinGW on Windows or Linux (Cross-Compilation)
```bash
cd hp49_reconstructed
make
```

### Building with CMake
```bash
cd hp49_reconstructed
mkdir build && cd build
cmake ..
cmake --build .
```

---

## 🔬 Reverse Engineering Highlights
- **Exact Geometry:** All 54 keys mapped with pixel coordinates, rounded corners, and Saturn row/column matrix addresses.
- **Translucent Keypress Effect:** Original 1999 visual highlight blending formula $(R/2 + 64, G/2 + 64, B/2 + 64)$ faithfully restored.
- **ROM Extraction:** 4 MiB unpacked nibble format converted and verified against HP 49G Revision 1.19-6 (VGER) containing Bernard Parisse's Erable CAS.

For the modern single-file Rust reimagining with cross-platform desktop and mobile builds (Windows, Mac, Linux, Android APK, and iOS), see [`rusty_hp49/`](../rusty_hp49/).
