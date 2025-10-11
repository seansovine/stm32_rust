# Project Working Notes

Here we jot down notes and details about steps taken that will be useful
to remember later.

## Main steps we followed

So far this project is a mashup of the Knurling app template and Hiari's
Embedded Rustacean example repository, with our own additions and modifications.

We plan to document all steps from scratch, but for now here is an outline
and some notes on specific parts:

1. The basis of this project is a clone of the [app-template](https://github.com/knurling-rs/app-template) repo.

2. We followed a combination the steps in that repo and in Hiari's repo to setup the build toolchain.

3. We copied code from Hiari's LED blink and UART blog examples, and modified them to
   work with our STM32F4DISCOVERY board and with the app-template foundation code.

4. We set up communication over UART with minicom on our Linux PC. (Pretty straightforward.)

4. We experimented with some different parameters and states to change the example behavior.

Next we plan to do... a whole lot more. But those explain the state of this repository.

## Setting up the toolchain

We followed the instructions on and linked from the Embedded Rustacean GitHub
repository [here](https://github.com/theembeddedrustacean/learn-stm32f4-rs).

The main change for us was that we had previously built and installed OpenOCD
from source, so we didn't use the version in our distro (Ubuntu) package.

Another thing to note is that after following the steps linked
[here](https://docs.rust-embedded.org/book/intro/install/linux.html)
to add a udev rule for ST-Link, we had to restart our machine to get the
new rule to take effect, so that we can run OpenOCD without root priveleges.
This could probably be accomplished by just restarting some service(s).

## Getting the [app-template](https://github.com/knurling-rs/app-template) to build

For the most part we followed the instructions in the README exactly. However, there were
a couple steps we had to change.

__Add the HAL crate:__

We manually added the HAL crate to the `app-template` project, instead of using the board
support crate, which we might do later.

We specifically added this to our `Cargo.toml`:

```toml
[dependencies.stm32f4xx-hal]
version = "0.22.1"
features = ["stm32f407"]
```

And, we added a copy of the `memory.x` file from the HAL repo in this repo's root directory.

## STM32F4DISCOVERY board support crate

There is a [board support crate](https://github.com/stm32-rs/stm32f407g-disc) available for this
board that we might try out. It should provide some additional abstractions for the other devices
that come on the board.
