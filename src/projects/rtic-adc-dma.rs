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
    use core::fmt::Write;
    use stm32f4xx_hal::{
        adc::{
            Adc,
            config::{AdcConfig, Clock, Dma, Resolution, SampleTime, Scan, Sequence},
        },
        dma::{PeripheralToMemory, Stream0, StreamsTuple, Transfer, config::DmaConfig},
        gpio::{self, Output, PushPull},
        pac::{ADC1, DMA2, TIM2, USART1},
        prelude::*,
        serial::{Tx, config::Config},
        timer::{CounterHz, Event, Flag},
    };

    // Alias to simplify type name; borrowed from Hiari.
    type DMATransfer =
        Transfer<Stream0<DMA2>, 0, Adc<ADC1>, PeripheralToMemory, &'static mut [u16; 2]>;

    // Resources shared between tasks
    #[shared]
    struct Shared {
        transfer: DMATransfer,
    }

    // Local resources to specific tasks (cannot be shared)
    #[local]
    struct Local {
        led: gpio::PD13<Output<PushPull>>,
        uart_tx: Tx<USART1>,
        buffer: Option<&'static mut [u16; 2]>,
        timer: CounterHz<TIM2>,
    }

    #[init(local = [first_buffer: [u16; 2] = [0; 2],second_buffer: [u16; 2] = [0; 2]])]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        // Borrow peripherals handle.
        let dp = ctx.device;

        // Get system clock peripheral.
        let rcc = dp.RCC.constrain();

        // Configure clock peripheral as Hiari does in his example.
        let clocks = rcc
            .cfgr
            // Use external (to chip) high speed oscillator. (Datasheet pg 24.)
            .use_hse(8.MHz())
            // HSE is input to PLL to increase frequency. (Datasheet pg 24.)
            .sysclk(84.MHz())
            // Clock signal to AHB bus. (Internet.)
            .hclk(84.MHz())
            // Special 48Hz PLL-generated clock. (Why needed?)
            .require_pll48clk()
            // Sets PCLK2 = HCLK / 4.
            .pclk2(21.MHz())
            .freeze();

        // See this page on STM32 clocks:
        //  https://www.learningaboutelectronics.com/Articles/SYSCLK-HCLK-PCLK1-PCLK2-clock-STM32F4xx.php

        let gpioa = dp.GPIOA.split();
        let mic1 = gpioa.pa1.into_analog();
        let mic2 = gpioa.pa2.into_analog();

        // Configure ADC peripheral following Hiari. He says:
        //  Configure ADC for sequence conversion with interrtups.
        let adc_config = AdcConfig::default()
            .dma(Dma::Continuous)
            .scan(Scan::Enabled)
            .resolution(Resolution::Ten)
            .clock(Clock::Pclk2_div_8);

        let mut adc = Adc::adc1(dp.ADC1, true, adc_config);
        adc.configure_channel(&mic1, Sequence::One, SampleTime::Cycles_480);
        adc.configure_channel(&mic2, Sequence::Two, SampleTime::Cycles_480);

        // Configure orange LED for simple debugging.
        let gpiod = dp.GPIOD.split();
        let mut led = gpiod.pd13.into_push_pull_output();
        led.set_high();

        // Setup UART transmit pin via multiplexer config.
        let gpiob = dp.GPIOB.split();
        // Pin configuration type is inferred from use below.
        let tx_pin = gpiob.pb6.into_alternate();

        // Configure USART/UART peripheral with chosen pin.
        let uart_tx: Tx<USART1> = dp
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

        // Setup DMA following Hiari.
        let dma = StreamsTuple::new(dp.DMA2);
        let dma_config = DmaConfig::default()
            .transfer_complete_interrupt(true)
            .memory_increment(true)
            .double_buffer(false);

        let transfer = Transfer::init_peripheral_to_memory(
            dma.0,
            adc,
            ctx.local.first_buffer,
            None,
            dma_config,
        );

        // How often to trigger ADC transfer start.
        //  Should be as fast as UART can send,
        //  if we've calculated correctly.
        const ADC_TIMER_RATE_HZ: u32 = 1000;

        // Setup timer.
        let mut timer = dp.TIM2.counter_hz(&clocks);
        timer.listen(Event::Update);
        timer.start(ADC_TIMER_RATE_HZ.Hz()).unwrap();

        (
            Shared { transfer },
            Local {
                led,
                uart_tx,
                buffer: Some(ctx.local.second_buffer),
                timer,
            },
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

    // Based on Hiari's example.
    #[task(binds = TIM2, shared = [transfer], local = [led, timer])]
    fn adc_start(mut ctx: adc_start::Context) {
        ctx.local.led.toggle();
        ctx.shared.transfer.lock(|transfer| {
            transfer.start(|adc| {
                adc.start_conversion();
            });
        });
        ctx.local.timer.clear_flags(Flag::Update);
    }

    // Based on Hiari's example.
    #[task(binds = DMA2_STREAM0, shared = [transfer], local = [uart_tx, buffer])]
    fn dma(ctx: dma::Context) {
        let mut shared = ctx.shared;
        let local = ctx.local;

        let buffer = shared.transfer.lock(|transfer| {
            let (buffer, _) = transfer
                .next_transfer(local.buffer.take().unwrap())
                .unwrap();
            buffer
        });

        let mic1 = buffer[0];
        let mic2 = buffer[1];

        // From Hiari: After this RHS buffer is dropped and returned to pool.
        *local.buffer = Some(buffer);

        // Send data to PC; each message is 14 bytes.
        writeln!(local.uart_tx, "{:05} -- {:05}\r", mic1, mic2).unwrap();
    }
}
