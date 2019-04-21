# rust feather M4

experiments with rust on the [Adafruit Feather M4 Express (Adafruit Industries 3857 
)](https://www.adafruit.com/product/3857
) board featuring the ARM Cortex-M4F microprocessor core


## steps you need to do to compile this and put it onto the feather M4

install rust:
```
curl https://sh.rustup.rs -sSf | sh
rustup update
```

install components for ARM M4 targets:
```
rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

install the [bossa flashing utility](https://github.com/shumatech/BOSSA)
which we'll later use to flash the chip:
```
git clone https://github.com/shumatech/BOSSA.git
cd BOSSA
make
cp bin/bossac ~/bin
```

enter this repository:
```
git clone git@github.com:snd/rust-feather-m4.git
cd rust-feather-m4
```

build:
```
cargo build --release
```

before executing this put the feather into bootloader mode by doublepressing the reset button:
```
bossac --port /dev/cu.usbmodem1421 --offset 0x4000 --erase --write --verify --reset --info --debug target/thumbv7em-none-eabihf/release/rust-feather-m4
```

`--offset 0x4000` source https://learn.adafruit.com/adafruit-feather-m4-express-atsamd51/uf2-bootloader-details

## board

https://www.alliedelec.com/product/adafruit-industries/3857/71458403
https://www.digikey.com/product-detail/en/adafruit-industries-llc/3857/1528-2648-ND/9553567

## ATSAMD51J19 microprocessor

by microchip (source: https://twitter.com/microchipmakes/status/1017405803349839872)

microprocessor: [](https://www.findchips.com/search/ATSAMD51J19)
https://www.microchip.com/wwwproducts/en/ATSAMD51J19A
https://learn.adafruit.com/adafruit-feather-m4-express-atsamd51/downloads
https://www.digikey.com/product-detail/en/adafruit-industries-llc/3857/1528-2648-ND/9553567

- 120MHZ
    - source: https://www.mouser.com/ProductDetail/Microchip-Technology/ATSAMD51J19A-AUT
- flash length = `512KB`
    - source: https://www.mouser.com/ProductDetail/Microchip-Technology/ATSAMD51J19A-AUT
- RAM length =  `192KB`
    - source: https://www.mouser.com/ProductDetail/Microchip-Technology/ATSAMD51J19A-AUT
- flash origin address = `0x00000000`
  - source: https://www.mouser.com/datasheet/2/268/60001507A-1130176.pdf
    - `After Reset has been released, the CPU starts fetching PC and SP values from the Reset address,0x00000000. This points to the first executable address in the internal Flash memory`
    - 9.2 Physical Memory Map
- RAM origin address = `0x20000000`
  - source: https://www.mouser.com/datasheet/2/268/60001507A-1130176.pdf
    - 8. Product Memory Mapping Overview
    - 9.2 Physical Memory Map
  - RAM is commonly located at address 0x2000_0000
    - https://github.com/rust-embedded/cortex-m-quickstart


## core

## 

ARM 

## files

- [memory.x](memory.x) contains specification of the memory layout of the target chip

## steps taken to initialize this project

```
cargo install cargo-generate

cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart --name rust-feather-m4
```
