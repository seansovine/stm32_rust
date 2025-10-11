//! STM32F4DISCOVERY UART example.
//!
//! This communicates with a PC that the USART1 is attached to
//! via a USB virtual com port adapter.
//!
//! The code here was adapted from several places, but mostly from
//! [this](https://dev.to/theembeddedrustacean/stm32f4-embedded-rust-at-the-hal-uart-serial-communication-1oc8)
//! blog post.
//!
//! TODO: Document this fully, including hardware and setup steps.

#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use debouncr::{Edge, debounce_3};

use stm32f4d_test as _; // global logger + panicking-behavior + memory layout

use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
    serial::Config,
};

/// NOTE: Some explanatory comments here are adapted from Omar Hiari's blog post above.

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Configure the LED pin as a push pull ouput and obtain handler.
    //
    // -- We use the STM32F4DISCOVERY's organge LED; pin is from data sheet.
    let gpiod = dp.GPIOD.split();
    let mut led = gpiod.pd13.into_push_pull_output();

    // Configure the button pin (if needed) and obtain handler.
    // Pin is input by default.
    //
    // -- We use the B1 user pushbutton on STM32F4DISCOVERY.
    let gpioa = dp.GPIOA.split();
    let button = gpioa.pa0;

    // Serial config steps (Hiari):
    // 1) Need to configure the system clocks
    // - Promote RCC structure to HAL to be able to configure clocks
    let rcc = dp.RCC.constrain();
    // - Configure system clocks
    //
    // -- 8 MHz is also used for the STM32F4DISCOVERY board.
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    let gpiob = dp.GPIOB.split();
    let tx_pin = gpiob.pb6.into_alternate();

    // 3) Configure Serial perihperal channel
    //
    // -- We somewhat arbitrarily chose the USART1 peripheral from the datasheet.
    let mut uart_tx = dp
        .USART1
        .tx(
            tx_pin,
            Config::default()
                .baudrate(115200.bps())
                .wordlength_8()
                .parity_none(),
            &clocks,
        )
        .unwrap();

    // Variable for number of delay loop iterations.
    let mut delay_iters = 7_0000_i32;

    // Initialize LED to on (active low button).
    led.set_low();

    // Hiari: Initialize debouncer to false because button is active low.
    //        Chose 3 consecutive states based on testing.
    let mut debouncer = debounce_3(false);

    let mut num_button_presses: u8 = 0;
    let mut active = true;

    // Main loop.
    loop {
        // Explicitly copy upper bound to immutable; makes clippy happy.
        let loop_bound = delay_iters;

        // LED flash delay and input handling loop.
        for _i in 1..loop_bound {
            // Check for button press.
            if active && debouncer.update(button.is_low()) == Some(Edge::Falling) {
                // Hiari: If button is pressed print to derial and decrease the delay value.
                writeln!(uart_tx, "Button Press {:02} Woohoo!!\r", num_button_presses).unwrap();

                num_button_presses = num_button_presses.wrapping_add(1);
                // Reduce delay.
                delay_iters -= 3_0000_i32;
                // When delay reaches minimum reset to initial.
                if delay_iters < 1_0000 {
                    delay_iters = 7_0000_i32;
                }

                // Hiari: Exit delay loop since button was pressed.
                break;
            }
        }

        // Toggle LED
        led.toggle();

        // After 5 button presses inter inactive state (unresponsive to button
        // presses then) as a demonstration of basic state handling.
        if active && num_button_presses >= 5 {
            led.set_high();
            writeln!(uart_tx, "Deactivating program...\r").unwrap();
            active = false;
        }
    }
}

// On our machine can receive UART data with: minicom -D /dev/ttyUSB0
