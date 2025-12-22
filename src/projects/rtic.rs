//! Example for using
//!
//! Based on an example from The Embedded Rustacean.

#![no_main]
#![no_std]

// For panic_handler.
use stm32f4d as _;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true)]
mod app {
    // Resources shared between tasks
    #[shared]
    struct Shared {}

    // Local resources to specific tasks (cannot be shared)
    #[local]
    struct Local {}

    #[init]
    fn init(_ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        (
            // Initialization of shared resources
            Shared {},
            // Initialization of task local resources
            Local {},
            // Move the monotonic timer to the RTIC run-time, this enables
            // scheduling
            init::Monotonics(),
        )
    }
}
