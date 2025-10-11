//! STM32F4DISCOVERY UART example.
//!
//! This communicates with a PC that the USART1 peripheral is
//! attached to via a USB virtual com port adapter, while the user
//! controls the rate of a blinking LED via a pushbutton.
//!
//! The code here was adapted from several places, but mostly from
//! [this](https://dev.to/theembeddedrustacean/stm32f4-embedded-rust-at-the-hal-uart-serial-communication-1oc8)
//! blog post.
//!
//! TODO: Document all steps to set this up from scratch.

#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use debouncr::{Edge, debounce_3};

// From Knurling template setup:
// global logger + panicking-behavior + memory layout
use stm32f4d_test as _;

use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
    serial::Config,
};

enum State {
    Active,
    Inactive,
}

#[entry]
fn main() -> ! {
    // Take ownership of peripheral interface.
    let dp = pac::Peripherals::take().unwrap();

    // Configure orange LED pin.
    let gpiod = dp.GPIOD.split();
    let mut led = gpiod.pd13.into_push_pull_output();

    // Configure B! user pushbutton pin.
    let gpioa = dp.GPIOA.split();
    let button = gpioa.pa0;

    // Get system clock peripheral.
    let rcc = dp.RCC.constrain();
    // Configure peripheral to use on-board oscillator.
    // 8 MHz was suggested by Hiari for other board and reflected in datasheet.
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    // Setup UART transmit pin via multiplexer config.
    let gpiob = dp.GPIOB.split();
    // Pin configuration type is inferred from use below.
    let tx_pin = gpiob.pb6.into_alternate();

    // Configure USART/UART peripheral with chosen pin.
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

    // Start with LED off.
    led.set_low();

    // Hiari: Initialize debouncer to false because button is
    // active low. Chose 3 consecutive states based on testing.
    let mut debouncer = debounce_3(false);
    // Controls spin loop iterations between LED toggles.
    let mut toggle_delay_iters = 7_0000_i32;
    // Count button presses.
    let mut num_button_presses: u8 = 0;
    // Current program state.
    let mut system_state = State::Active;

    // # presses before entering inactive state.
    const ALLOWED_PRESSES: u8 = 5;

    // Program main loop.
    loop {
        // Explicitly copy upper bound to immutable; makes clippy happy.
        let loop_bound = toggle_delay_iters;

        // LED flash delay and input handling loop.
        for _i in 1..loop_bound {
            // Check for button press.
            if let State::Active = system_state
                && debouncer.update(button.is_low()) == Some(Edge::Falling)
            {
                num_button_presses = num_button_presses.wrapping_add(1);
                writeln!(uart_tx, "Button Press {:02} Woohoo!!\r", num_button_presses).unwrap();

                // Reduce LED toggle delay on button press.
                toggle_delay_iters -= 3_0000_i32;
                // When delay reaches minimum reset to initial.
                if toggle_delay_iters < 1_0000 {
                    toggle_delay_iters = 7_0000_i32;
                }

                // Immediately trigger blink rate change.
                break;
            }
        }

        // Toggle LED
        led.toggle();

        // After 5 button presses enter inactive state (unresponsive to
        // button presses then) as a demonstration of basic state handling.
        if let State::Active = system_state
            && num_button_presses >= ALLOWED_PRESSES
        {
            led.set_high();
            writeln!(uart_tx, "Deactivating program...\r").unwrap();
            system_state = State::Inactive;

            // TODO: Start timer that triggers switch back to active.
        }
    }
}

// On our machine can receive UART data with: minicom -D /dev/ttyUSB0
