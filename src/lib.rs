#![no_std]

//! extracted commonly used functionality
extern crate feather_m4 as hal;

// ws2812 = neopixel
extern crate ws2812_timer_delay as ws2812;

use hal::{
    prelude::*,
    Peripherals,
    timer::TimerCounter,
    clock::GenericClockController,
    gpio::{Output, PushPull, Pb3},
    TC3,
    MCLK,
};

pub fn new_neopixel<'a>(
    timer_counter_3: TC3,
    main_clock: &mut MCLK,
    clock_controller: &mut GenericClockController, 
    neopixel_pin: &'a mut Pb3<Output<PushPull>>
) -> ws2812::Ws2812<'a, TimerCounter<hal::TC3>, Pb3<Output<PushPull>>> {
    // wire up the peripherals

    let clock_generator_0 = clock_controller.gclk0();
    // configure clock generator for use with either the timer counter 2 or 3 peripherals
    // TODO error handling
    let clock_configured_for_timer_counter_3 = clock_controller.tc2_tc3(&clock_generator_0).unwrap();

    // hardware timer counter 3
    let mut timer_counter = TimerCounter::tc3_(
        &clock_configured_for_timer_counter_3,
        timer_counter_3,
        main_clock,
    );
    // repeatedly counts down from 3MHZ
    timer_counter.start(3_000_000u32.hz());

    ws2812::Ws2812::new(timer_counter, neopixel_pin)
}
