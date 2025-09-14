# Project Working Notes

Here we jot down notes and details about steps taken that will be useful
to remember later.

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
