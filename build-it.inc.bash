#!/bin/bash

DEBUG=0

if [ "$DEBUG" -eq 1 ]; then
    cargo tauri build --debug
else
    cargo tauri build
    find "../../../target/release/bundle/dmg" -type f -name "*.dmg" -exec mv {} ./releases/macos \;
fi

