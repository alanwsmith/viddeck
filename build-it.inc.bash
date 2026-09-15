#!/bin/bash

SOURCE="../../../target/release/bundle/macos/vidDeck.app"
DEST="./releases/vidDeck.app"

if [ -e "$DEST" ]; then
   trash "$DEST"
fi

# cargo tauri build --debug
cargo tauri build

if [ -e "$SOURCE" ]; then
    mv "$SOURCE" "$DEST"
fi


