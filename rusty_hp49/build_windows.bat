@echo off
echo ==============================================
echo  Building Rusty HP 49G for Windows (x86_64)   
echo ==============================================

cargo build --release --target x86_64-pc-windows-msvc
if not exist "dist" mkdir dist
copy target\x86_64-pc-windows-msvc\release\rusty_hp49.exe dist\rusty_hp49_windows_x64.exe

echo Build complete: dist\rusty_hp49_windows_x64.exe
pause
