#!/bin/bash

me="$(realpath "$(dirname "$0")")"

if [ ! -d "$1" ]; then
    echo "$0: No such file or directory"
    exit
fi

if [ ! -f "$1/chrome-sandbox" ]; then
    echo "$0: Application is not a valid Atomic application"
    exit
fi

original=$(du -hs "$1" | awk '{print $1;}')

echo "$0: Removing built-in Electron"
rm -rf "$1/locales" "$1/"*".pak" "$1/chrome_crashpad_handler" "$1/chrome-sandbox" "$1/icudtl.dat" "$1/"*".so" "$1/libvulkan.so.1" "$1/LICENSE" "$1/LICENSES.chromium.html" "$1/"*".bin" "$1/version" "$1/vk_swiftshader_icd.json"

executable=""
for i in "$1/"*; do
    if [[ "$(basename "$i")" == *"."* ]] || [ -d "$i" ]; then
        true
    else
        executable="$i"
    fi
done

echo "$0: Executable found at $executable"

echo "$0: Copying launcher"
rm -f "$executable"
cp "$me/launcher-linux-arm64" "$executable" || cp "$me/../Launcher/launcher-linux-arm64" "$executable"

new=$(du -hs "$1" | awk '{print $1;}')

echo "$0: Completed. Was $original, is now $new."
