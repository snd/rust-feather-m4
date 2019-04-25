//! 
#![no_std]
#![no_main]

// set the panicking behavior to halt
extern crate panic_halt;

extern crate feather_m4 as hal;

use hal::{
    prelude::*,
    entry,
    Peripherals,
    CorePeripherals,
    clock::GenericClockController,
    delay::Delay,
};

use smart_leds::{
    Color,
    colors::{RED, BLUE},
    brightness,
    SmartLedsWrite,
};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core_peripherals = CorePeripherals::take().unwrap();

    let mut clock_controller = GenericClockController::with_external_32kosc(
        // generic clock generator
        peripherals.GCLK,
        // main clock
        &mut peripherals.MCLK,
        // 32HZ oscillators control
        &mut peripherals.OSC32KCTRL,
        // oscillators control
        &mut peripherals.OSCCTRL,
        // non-volatile memory controller
        &mut peripherals.NVMCTRL,
    );

    let mut pins = hal::Pins::new(peripherals.PORT);

    let mut neopixel_pin = pins.d8.into_push_pull_output(&mut pins.port);
    // TC3 = timer counter 3
    let mut neopixel = rust_feather_m4::new_neopixel(peripherals.TC3, &mut peripherals.MCLK, &mut clock_controller, &mut neopixel_pin);

    // SYST = system timer
    let mut delay = Delay::new(core_peripherals.SYST, &mut clock_controller);

    loop {
        let color = [RED; 1];
        neopixel.write(brightness(color.iter().cloned(), 32)).unwrap();
        delay.delay_ms(500u16);
        let color = [BLUE; 1];
        neopixel.write(brightness(color.iter().cloned(), 32)).unwrap();
        delay.delay_ms(500u16);
    }
}
