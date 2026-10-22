#!/bin/bash
set -e
echo "=============================================="
echo " Building Rusty HP 49G Android APK            "
echo "=============================================="

cd android
if [ -f "./gradlew" ]; then
    ./gradlew assembleRelease
else
    gradle assembleRelease
fi

mkdir -p ../dist
cp app/build/outputs/apk/release/app-release-unsigned.apk ../dist/rusty_hp49.apk || true
echo "Android APK build complete in dist/rusty_hp49.apk"
