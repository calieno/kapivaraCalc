#!/bin/bash
set -e
echo "=============================================="
echo " Building kapivaraCalc (kC49) Android APK     "
echo "=============================================="

cd android
gradle assembleDebug

mkdir -p ../dist
find app/build/outputs/apk/debug -name "*.apk" -exec cp {} ../dist/kC49.apk \;
echo "Android APK build complete: dist/kC49.apk"
