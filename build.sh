#!/usr/bin/env bash

# compiles rust into an elf executable, then converts the elf excutable into a bin file
# (https://stackoverflow.com/questions/2427011/what-is-the-difference-between-elf-files-and-bin-files)
# also prints out sizes of both files

if [ $# -eq 0 ]
  then
    echo "please provide executable name"
    exit 1
fi

set -eva

cargo build --release
ELF=target/thumbv7em-none-eabihf/release/$1
BIN=$ELF.bin
ls -lah $ELF
arm-none-eabi-objcopy -O binary $ELF $BIN
ls -lah $BIN
