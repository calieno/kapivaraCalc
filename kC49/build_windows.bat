@echo off
echo ==============================================
echo  Building kapivaraCalc (kC49) for Windows
echo ==============================================

cargo build --release --target x86_64-pc-windows-msvc
if not exist "dist" mkdir dist
copy target\x86_64-pc-windows-msvc\release\kC49.exe dist\kC49_windows_x64.exe
copy target\x86_64-pc-windows-msvc\release\kC49.exe dist\kC49.exe

echo Build complete: dist\kC49_windows_x64.exe (e kC49.exe)
pause
