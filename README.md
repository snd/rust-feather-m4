# rust feather M4

experiments with rust on the [Adafruit Feather M4 Express (Adafruit Industries 3857 
)](https://www.adafruit.com/product/3857
) board featuring the ARM Cortex-M4F microprocessor core


## steps you need to do to initialize the project

```
rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf

cargo build
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



## steps taken to initialize this project

```
cargo install cargo-generate

cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart --name rust-feather-m4
```
