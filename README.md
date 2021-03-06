# rust feather M4

experiments with rust on the [Adafruit Feather M4 Express (Adafruit Industries 3857)](https://www.adafruit.com/product/3857) board with the ATSAMD51J19 microprocessor featuring the ARM Cortex-M4F microprocessor core

## run

```
./build.sh {name of one in src/bin}
```

```
./flash.sh {name of one in src/bin}
```

```
./build-and-flash.sh {name of one in src/bin}
```

## install

install rust:
```
curl https://sh.rustup.rs -sSf | sh
rustup update
```

install component for ARM M4 targets:
```
rustup target add thumbv7em-none-eabihf
```

install `bossac` (https://github.com/shumatech/BOSSA) which we'll use to upload our binary to the feather M4s flash memory:
```
git clone https://github.com/shumatech/BOSSA.git
cd BOSSA
make
cp bin/bossac ~/bin
```

install `arm-none-eabi-objcopy` which makes `*.bin` files out of `*.elf` files (https://stackoverflow.com/questions/2427011/what-is-the-difference-between-elf-files-and-bin-files):
```
brew tap PX4/homebrew-px4
brew update
brew install gcc-arm-none-eabi
```

enter this repository:
```
git clone git@github.com:snd/rust-feather-m4.git
cd rust-feather-m4
```

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
    - source: https://www.adafruit.com/product/3857
- flash length = `512KB`
    - source: https://www.adafruit.com/product/3857
- RAM length =  `192KB`
    - source: https://www.adafruit.com/product/3857
- flash origin address = `0x00000000`
  - source: https://www.mouser.com/datasheet/2/268/60001507A-1130176.pdf
    - `After Reset has been released, the CPU starts fetching PC and SP values from the Reset address,0x00000000. This points to the first executable address in the internal Flash memory`
    - search for `Physical Memory Map`
- RAM origin address = `0x20000000`
  - source: https://www.mouser.com/datasheet/2/268/60001507A-1130176.pdf
    - search for `Product Memory Mapping Overview`
    - search for `Physical Memory Map`
  - RAM is commonly located at address `0x20000000`
    - source: https://github.com/rust-embedded/cortex-m-quickstart

## awesome resources

- https://github.com/atsamd-rs/atsamd - for using the GPIO, clock, serial interfaces, etc on the M4
- https://rust-embedded.github.io/book/
- https://github.com/rust-embedded/awesome-embedded-rust
- https://github.com/rust-embedded/alloc-cortex-m - a heap allocator for Cortex-M processors
- https://github.com/johnthagen/min-sized-rust - How to minimize Rust binary size
- https://github.com/RazrFalcon/cargo-bloat - Find out what takes most of the space in your executable
- https://jamesmunns.com/blog/tinyrocket/ - making tiny rust executables

## files

- [memory.x](memory.x) contains specification of the memory layout of the target chip

## steps taken to initialize this project

```
cargo install cargo-generate

cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart --name rust-feather-m4
```
