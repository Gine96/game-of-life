#!/usr/bin/env bash

PROFILE=${1:-dev}

# Determine path based on profile
if [ "$PROFILE" = "release" ]; then
    TARGET_PATH="target/release/game-of-life"
else
    TARGET_PATH="target/debug/game-of-life"
fi

# Run the binary
if [ -f "$TARGET_PATH" ]; then
    "$TARGET_PATH"
else
    echo "Binary not found at $TARGET_PATH. Did you build it?"
    exit 1
fi
