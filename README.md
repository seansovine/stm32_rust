# STM32F4DISCOVERY Rust Project

This is a project to get started with embedded Rust on the
[STM32F4DISCOVERY](https://www.st.com/en/evaluation-tools/stm32f4discovery.html)
board.

<p align="center" margin="20px">
	<img src="https://github.com/seansovine/page_images/blob/main/photos/STM32F%24DISCOVERY%20-%202025-09-14.jpg?raw=true" alt="drawing" width="400" style="padding-top: 10px; padding-bottom: 10px"/>
</p>

## Steps we've taken so far

1. Carried out the steps to setup and build the code from Knurling template and run it on the board.

2. Imported the blinking LED project from The Embedded Rustacean blog
	and made small changes to build and run it from the app-template project environment.

See [project notes](doc/ProjectNotes.md) for more details on the steps we took to get these working.

## Next steps

We will definitely continue to follow the Embedded Rustacean
[STM32 blog series](https://blog.theembeddedrustacean.com/series/stm32f4-embedded-rust-hal).
This is a great resource for getting started. See also the accompanying
[GitHub repository](https://github.com/theembeddedrustacean/learn-stm32f4-rs)
with instructions on setting up an embedded Rust toolchain for your device.
Similarly to [The Embedded Rust Book](https://docs.rust-embedded.org/book/),
the example project there has been deprecated, so we had to adapt the instructions
to use the Knurling app-template project.

We will also continue to look more into the various tools in the embedded Rust
ecosystem, to get more deeply familiar with them, and will make more example projects for
trying them out.

__Audio DSP following Reay's book:__

Now that we have the basic workflow set up for developing for this board in Rust,
it would be nice to take on a larger project. I've been looking at
the book _Digital Signal Processing Using the ARM Cortex M4_ by Donald Reay
for a while. The board we are using is one of the development boards he uses
in this book. So one interesting project would be to implement the things he
does in that book using embedded Rust, instead of C with the Kiel MDK environment
that he uses.

Some challenges of this would be:

1. Finding the equivalent Rust APIs to the ones he uses in his code, including CMSIS-DSP.

2. Possibly using a different I2S audio codec board, as the one he uses is out of production now.

## Licenses and credits

To get this project started we've relied on this [learn-stm32f4-rs](https://github.com/theembeddedrustacean/learn-stm32f4-rs)
project from The Embedded Rustacean and the [app-template](https://github.com/knurling-rs/app-template)
repository from Knurling. Most of the code currently in this repository
is a mashup of those two. Both come with MIT licenses, and we're using their
code here in this project for purely educational purposes.
