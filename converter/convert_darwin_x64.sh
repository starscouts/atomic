#!/bin/bash

me="$(realpath "$(dirname "$0")")"

if [ ! -d "$1" ]; then
    echo "$0: No such file or directory"
    exit
fi

if [ ! -f "$1/Contents/Frameworks/Electron Framework.framework/Electron Framework" ]; then
    echo "$0: Application is not a valid Atomic application"
    exit
fi

original=$(du -hs "$1" | awk '{print $1;}')

executable=""
for i in "$1/Contents/MacOS/"*; do executable="$i"; done

echo "$0: Executable found at $executable"

echo "$0: Copying launcher"
rm -f "$executable"
cp "$me/launcher-darwin-x64" "$executable" || cp "$me/../Launcher/launcher-darwin-x64" "$executable"

echo "$0: Removing built-in Electron"
rm -rf "$1/Contents/Frameworks"
for i in "$1/Contents/Resources/"*; do
    if [[ "$i" == */app ]] || [[ "$i" == */electron.icns ]]; then
        true
    else
        rm -rf "$i"
    fi
done

new=$(du -hs "$1" | awk '{print $1;}')

echo "$0: Completed. Was $original, is now $new."
