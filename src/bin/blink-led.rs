#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use atsamd_hal::prelude::*;
use atsamd_hal::delay::Delay;
use atsamd_hal::clock::GenericClockController;
use atsamd_hal::hal::digital::OutputPin;
use atsamd_hal::target_device::{Peripherals, CorePeripherals};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let port = peripherals.PORT;
    let mut parts = port.split();
    // let mut led_pin = parts.pa13.into_open_drain_output(&mut parts.port);
    let mut led_pin = parts.pa23.into_open_drain_output(&mut parts.port);

    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        led_pin.set_high();
        delay.delay_ms(2000u16);
        led_pin.set_low();
        delay.delay_ms(2000u16);
    }
}
