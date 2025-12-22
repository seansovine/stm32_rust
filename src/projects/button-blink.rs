//! Modified from :
//!  `https://blog.theembeddedrustacean.com/stm32f4-embedded-rust-at-the-hal-gpio-button-controlled-blinking

#![no_std]
#![no_main]

use cortex_m_rt::entry;

use stm32f4d as _; // global logger + panicking-behavior + memory layout

use stm32f4xx_hal::{
    gpio::Pin,
    pac::{self},
    prelude::*,
};

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Configure the LED pin as a push pull ouput and obtain handler.
    // On the Nucleo FR401 theres an on-board LED connected to pin PA5.
    let gpiod = dp.GPIOD.split();
    let mut led = gpiod.pd13.into_push_pull_output();

    // Configure the button pin (if needed) and obtain handler.
    // On the Nucleo FR401 there is a button connected to pin PC13.
    // Pin is input by default
    let gpioa = dp.GPIOA.split();
    let button = gpioa.pa0;

    // Create and initialize a delay variable to manage delay loop
    let mut del_var = 10_0000_u32;

    // Initialize LED to on or off
    led.set_low();

    // Application Loop
    loop {
        // Call delay function and update delay variable once done
        del_var = loop_delay(del_var, &button);

        // Toggle LED
        led.toggle();
    }
}

// Delay Function
fn loop_delay<const P: char, const N: u8>(mut del: u32, but: &Pin<P, N>) -> u32 {
    let loop_bound = del;
    // Loop for until value of del
    for _i in 1..loop_bound {
        // Check if button got pressed
        if but.is_low() {
            // If button pressed decrease the delay value
            del -= 2_5000_u32;
            // If updated delay value reaches zero then reset it back to starting value
            if del < 2_5000 {
                del = 10_0000_u32;
            }
            // Exit function returning updated delay value if button pressed
            return del;
        }
    }
    // Exit function returning original delay value if button no pressed (for loop ending naturally)
    del
}
