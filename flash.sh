#!/usr/bin/env bash

set -eva

cargo build --release
bossac --port /dev/cu.usbmodem1421 --offset 0x4000 --erase --write --verify --reset --info --debug target/thumbv7em-none-eabihf/release/rust-feather-m4
