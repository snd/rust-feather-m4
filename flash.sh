#!/usr/bin/env bash

if [ $# -eq 0 ]
  then
    echo "please provide executable name"
    exit 1
fi

set -eva

# found by running ./list-usbmodems.sh
PORT=/dev/cu.usbmodem1421

BIN=target/thumbv7em-none-eabihf/release/$1.bin

# For M4 boards, which have a 16kB bootloader, you must specify -offset=0x4000
# https://learn.adafruit.com/adafruit-feather-m4-express-atsamd51/uf2-bootloader-details
bossac \
  --port $PORT \
  --offset 0x4000 \
  --write \
  --verify \
  --reset \
  --info \
  --debug \
  $BIN | tee flash.log
