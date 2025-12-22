//! Example for using
//!
//! We learned from the Embedded Rustacean example available here:
//!  https://dev.to/theembeddedrustacean/stm32f4-embedded-rust-at-the-hal-the-rtic-framework-1j9i
//!

#![no_main]
#![no_std]

// For panic_handler.
use stm32f4d as _;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true)]
mod app {
    // Imports.
    use stm32f4xx_hal::{
        gpio::{self, Output, PushPull},
        pac::TIM2,
        prelude::*,
        timer::{self, Event, Flag},
    };

    // List of timeout delay values to cycle through.
    const TIMER_DELAYS_MS: &[u32] = &[50, 500];
    // Period after which we change to the next timeout.
    const TIMEOUT_CHANGE_INT_MS: u32 = 5000;

    pub struct TimerInfo {
        pub current_timeout: usize,
        pub cumul_timeout: u32,
    }

    // Resources shared between tasks
    #[shared]
    struct Shared {
        timer: timer::CounterMs<TIM2>,
        timer_info: TimerInfo,
    }

    // Local resources to specific tasks (cannot be shared)
    #[local]
    struct Local {
        led: gpio::PD13<Output<PushPull>>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        // Borrow peripherals handle.
        let dp = ctx.device;

        // Get system clock peripheral.
        let rcc = dp.RCC.constrain();
        // Configure peripheral to use on-board oscillator:
        //   8 MHz was suggested by Hiari for other board
        //   and also reflected in our board's datasheet.
        let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

        // Configure orange LED pin.
        let gpiod = dp.GPIOD.split();
        let mut led = gpiod.pd13.into_push_pull_output();

        // Start with LED on.
        led.set_high();

        // Create timer.
        let mut timer = dp.TIM2.counter_ms(&clocks);
        // Start timer with initial timeout rate.
        timer.start(TIMER_DELAYS_MS[0].millis()).unwrap();
        // Hiari: Set up to generate interrupt when timer expires
        timer.listen(Event::Update);

        (
            Shared {
                timer,
                timer_info: TimerInfo {
                    current_timeout: 0,
                    cumul_timeout: 0,
                },
            },
            Local { led },
            // Hiari: We aren't using these explicitly,
            //        but they still need initialized.
            init::Monotonics(),
        )
    }

    // Hiari: Not necessary to explicitly implement this.
    //
    // Us: -> But it's interesting to see it.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    // Code to run on timer expired interrupt.
    #[task(binds = TIM2, local=[led], shared=[timer, timer_info])]
    fn timer_expired(mut ctx: timer_expired::Context) {
        let mut current_timeout = 0;
        ctx.shared.timer_info.lock(|ti| {
            ti.cumul_timeout += TIMER_DELAYS_MS[ti.current_timeout];

            // If timeout update interval has expired, move to next rate.
            if ti.cumul_timeout > 0 && ti.cumul_timeout % TIMEOUT_CHANGE_INT_MS == 0 {
                ti.current_timeout = (ti.current_timeout + 1) % TIMER_DELAYS_MS.len();
            }

            current_timeout = ti.current_timeout;
        });

        ctx.local.led.toggle();
        ctx.shared.timer.lock(|timer| {
            // Clear interrupt flag so it doesn't immediately fire again.
            timer.clear_flags(Flag::Update);

            // Now restart timer with current timeout value.
            timer
                .start(TIMER_DELAYS_MS[current_timeout].millis())
                .unwrap();
        });
    }
}
