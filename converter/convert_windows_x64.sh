#!/bin/bash

me="$(realpath "$(dirname "$0")")"

if [ ! -d "$1" ]; then
    echo "$0: No such file or directory"
    exit
fi

if [ ! -f "$1/vk_swiftshader.dll" ]; then
    echo "$0: Application is not a valid Atomic application"
    exit
fi

original=$(du -hs "$1" | awk '{print $1;}')

echo "$0: Removing built-in Electron"
rm -rf "$1/locales" "$1/"*".pak" "$1/chrome_crashpad_handler" "$1/icudtl.dat" "$1/"*".dll" "$1/libvulkan.so.1" "$1/LICENSE" "$1/LICENSES.chromium.html" "$1/"*".bin" "$1/version" "$1/vk_swiftshader_icd.json"

executable=$(ls "$1/"*".exe")

echo "$0: Executable found at $executable"

echo "$0: Copying launcher"
rm -f "$executable"
cp "$me/launcher-windows-x64.exe" "$executable" || cp "$me/../Launcher/launcher-windows-x64.exe" "$executable"

new=$(du -hs "$1" | awk '{print $1;}')

echo "$0: Completed. Was $original, is now $new."
