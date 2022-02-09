#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

# Should only have to change these two
readonly APP_NAME="automated-satellite-tracker"
readonly TARGET_HOST="odessen"

readonly TARGET_PATH=/home/ubuntu/$APP_NAME
# Change depending on if running on 64-bit ARM (such as RPi 4 with Ubuntu)
# readonly TARGET_ARCH=armv7-unknown-linux-musleabihf
readonly TARGET_ARCH="aarch64-unknown-linux-musl"
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/$APP_NAME

if ! command -v docker &> /dev/null
then
    echo "Docker could not be found! Ensure it is installed and the daemon is running."
    exit 1
fi

if ! command -v cargo &> /dev/null
then
    echo "Cargo could not be found! Ensure it is installed!"
    exit 1
fi

if ! command -v rsync &> /dev/null
then
    echo "Rsync could not be found! Ensure it is installed!"
    exit 1
fi

if ! command -v cross &> /dev/null
then
    echo "Cross could not be found! Installing now..."
    cargo install cross
fi

# TODO: Run Tests
# cross test

cross build --target=${TARGET_ARCH} --release
rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}

echo "Upload successful!"
