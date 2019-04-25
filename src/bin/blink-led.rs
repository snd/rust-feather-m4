//! alternates red led being on for 2 seconds and off for 2 seconds
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

#[entry]
fn main() -> ! {
    // all the hardware devices available on the chip
    let mut peripherals = Peripherals::take().unwrap();
    let core_peripherals = CorePeripherals::take().unwrap();

    let mut pins = feather_m4::Pins::new(peripherals.PORT);

    let mut led_pin = pins.d13.into_open_drain_output(&mut pins.port);

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

    // SYST = system timer
    let mut delay = Delay::new(core_peripherals.SYST, &mut clock_controller);

    loop {
        led_pin.set_high();
        delay.delay_ms(2000u16);
        led_pin.set_low();
        delay.delay_ms(2000u16);
    }
}
