# STM32F4DISCOVERY Rust

A project to get started with embedded Rust on the
[STM32F4DISCOVERY](https://www.st.com/en/evaluation-tools/stm32f4discovery.html)
board. See [project notes](doc/ProjectNotes.md) for more information on the steps
we've taken so far and some ideas we have for further projects.

## UART to Linux PC example

In [`uart.rs`](src/projects/uart.rs) we have an example program of that sends text to a
PC through the UART peripheral, which we have connected using a USB to TTL UART adapter,
as in the photo.

Once this project is setup, you can run it with:

```shell
# in one terminal; your device may vary
minicom -D /dev/ttyUSB0

# in another terminal, from this directory
cargo run --bin uart
```

<p align="center" margin="20px">
	<img src="https://github.com/seansovine/page_images/blob/main/photos/STM32F4DISCOVERY%20-%20UART%20-%202025-10-10.jpg?raw=true" alt="drawing" width="400" style="padding-top: 10px; padding-bottom: 10px"/>
</p>

## RTIC framework LED blink example

In [`rtic.rs`](src/projects/rtic.rs) we have an example to test out interrupt-driven programming
using the [RTIC](https://rtic.rs/1/book/en/) framework. It uses a timer interrupt to blink
an LED while cycling through two different rates periodically. To get started with this we
followed the tutorial available
[here](https://dev.to/theembeddedrustacean/stm32f4-embedded-rust-at-the-hal-the-rtic-framework-1j9i).

## ADC DMA example

In [`rtic-adc-dma.rs`](src/projects/rtic-adc-dma.rs) we have an example of reading analog signals
from two ADC channels and then getting the data using DMA through an interrupt. This is based on
the Embedded Rustacean tutorial project [here](https://blog.theembeddedrustacean.com/stm32f4-embedded-rust-at-the-hal-dma-controllers),
adapted for our hardware, and also uses the RTIC framework.

<p align="center" margin="20px">
	<img src="https://github.com/seansovine/page_images/blob/74fdc0d2807d75516bbe7a1a50879712b04a9356/photos/STM32F4DISCOVERY%20-%20ADC%20potentiometer%20op%20amp%20-%202025-12-27.jpg?raw=true" alt="drawing" width="400" style="padding-top: 10px; padding-bottom: 10px"/>
</p>

To test out the ADC reading we have a trimmer pot powered by the board rails connected to the ADC
through a Microchip MCP6002 op amp configured as a unity buffer / voltage follower, to ensure that
too much current isn't driven into the ADC pins. The MCP6002 was chosen because it has low supply
power requirements and can operate rail-to-rail with single input supply power from the board rails.

This also uses the USB UART connection, to send the ADC readings back to a PC for debugging.

## Licenses and credits

To get this project started we've relied on this
[learn-stm32f4-rs](https://github.com/theembeddedrustacean/learn-stm32f4-rs)
project from The Embedded Rustacean and the
[app-template](https://github.com/knurling-rs/app-template)
repository from Knurling. Some of the code in this repository is adapted from those two
sources; we've tried to indicated in comments code that closely follows their examples.
Both come with MIT licenses, and we're using their code here in this project for purely
educational purposes. Much thanks to the authors of those projects for providing such
great learning resources.

This project is released under the MIT license.
