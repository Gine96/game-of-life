#!/bin/bash

PROFILE=${1:-dev}  # default to 'dev' if not specified

cargo build --color=always --package game-of-life --bin game-of-life --profile "$PROFILE"
