# Project Working Notes

Notes on work we've done so far, work we'd like to do, and some details that we jot
down so that we can remember them later.

## Steps we've taken so far

1. Carried out the steps to setup and build the code from Knurling template and run it on the board.

2. Imported the blinking LED project from The Embedded Rustacean blog
   and made small changes to build and run it from the app-template project environment.

3. Imported the Embedded Rustacean UART project with modifcations for our board and an external
   UART to USB adapter.

## Possible next steps

We will continue learning and doing more projects.
This definitely includes following the Embedded Rustacean STM32
[blog series](https://blog.theembeddedrustacean.com/series/stm32f4-embedded-rust-hal).
This is a great resource for getting started. See also the accompanying GitHub
[repository](https://github.com/theembeddedrustacean/learn-stm32f4-rs)
with instructions on setting up an embedded Rust toolchain for your device.
Similarly to the situation with the [The Embedded Rust Book](https://docs.rust-embedded.org/book/),
the example project template there has been deprecated, so we had to adapt the instructions
to use the Knurling app-template project. See [project notes](doc/ProjectNotes.md) on this
and other steps.

We will also continue to look more into the various tools in the embedded Rust
ecosystem, to get more deeply familiar with them, and will make more example projects for
trying them out.

### Audio DSP following Reay's book:

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

## More details

So far this project is a mashup of the Knurling app template and Hiari's
Embedded Rustacean example repository, with our own additions and modifications.

We plan to document all steps from scratch, but for now here is an outline
and some notes on specific parts:

1. The basis of this project is a clone of the [app-template](https://github.com/knurling-rs/app-template) repo.

2. We followed a combination the steps in that repo and in Hiari's repo to setup the build toolchain.

3. We copied code from Hiari's LED blink and UART blog examples, and modified them to
   work with our STM32F4DISCOVERY board and with the app-template foundation code.

4. We set up communication over UART with minicom on our Linux PC. (Pretty straightforward.)

5. We experimented with some different parameters and states to change the example behavior.

Next we plan to do... a whole lot more. But those explain the state of this repository.

### Setting up the toolchain

We followed the instructions on and linked from the Embedded Rustacean GitHub
repository [here](https://github.com/theembeddedrustacean/learn-stm32f4-rs).

The main change for us was that we had previously built and installed OpenOCD
from source, so we didn't use the version in our distro (Ubuntu) package.

Another thing to note is that after following the steps linked
[here](https://docs.rust-embedded.org/book/intro/install/linux.html)
to add a udev rule for ST-Link, we had to restart our machine to get the
new rule to take effect, so that we can run OpenOCD without root priveleges.
This could probably be accomplished by just restarting some service(s).

### Getting the [app-template](https://github.com/knurling-rs/app-template) to build

For the most part we followed the instructions in the README exactly. However, there were
a couple steps we had to change.

**Add the HAL crate:**

We manually added the HAL crate to the `app-template` project, instead of using the board
support crate, which we might do later.

We specifically added this to our `Cargo.toml`:

```toml
[dependencies.stm32f4xx-hal]
version = "0.22.1"
features = ["stm32f407"]
```

And, we added a copy of the `memory.x` file from the HAL repo in this repo's root directory.

### STM32F4DISCOVERY board support crate

There is a [board support crate](https://github.com/stm32-rs/stm32f407g-disc) available for this
board that we might try out. It should provide some additional abstractions for the other devices
that come on the board.
