#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

picker=${1:-"default"}
flag_rsync=t

while :; do
	case $picker in
	-c)
		flag_rsync=f
		shift
		break
		;;
	--) # End of all options.
		shift
		break
	;;
	*) # Default case: No more options, so break out of the loop.
		break
	;;
	esac
	shift
done

# Should only have to change these two
readonly APP_NAME="ast"
readonly TARGET_HOST="quub-pi-remote"

readonly TARGET_PATH="/home/pi/$APP_NAME"
# Change depending on if running on 64-bit ARM (such as RPi 4 with Ubuntu)
# readonly TARGET_ARCH=armv7-unknown-linux-musleabihf
readonly TARGET_ARCH="aarch64-unknown-linux-musl"
readonly SOURCE_PATH="."

# Breaks my setup on mac
# if ! command -v docker &> /dev/null
# then
#     echo "Docker could not be found! Ensure it is installed and the daemon is running."
#     exit 1
# fi

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

cross build --target aarch64-unknown-linux-musl --release

[[ $flag_rsync == t ]] && rsync -r "${SOURCE_PATH}/target/${TARGET_ARCH}/release/ast" ${TARGET_HOST}:${TARGET_PATH}

[[ $flag_rsync == t ]] && echo "Upload successful!"

# This is a bit janky, but essentially kill the service if it exists, otherwise
# don't worry, and keep deploying.
# ssh $TARGET_HOST "kill \$(lsof -t -i:4533) &> /dev/null"
[[ $flag_rsync == t ]] && TERM="xterm" ssh $TARGET_HOST "kill \$(lsof -t -i:4533) &> /dev/null || true && $TARGET_PATH"
