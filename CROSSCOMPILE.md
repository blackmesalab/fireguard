# 🤯 Crosscompile 🤯

- [Automatic crosscompile (Linux / Mac)](#automatic-crosscompile-(linux-/-mac))
  - [Binaries](#binaries)
- [Manual Debian based process for arm32 / arm64](#manual-debian-based-process-for-arm32-/-arm64)
  - [Local configuration](#local-configuration)
    - [1: Add the architecture to rustup](#1:-add-the-architecture-to-rustup)
    - [2: Configure cargo to support that target](#2:-configure-cargo-to-support-that-target)
    - [3: Add the target architecture to your host](#3:-add-the-target-architecture-to-your-host)
    - [4: Install the metapackage for the target architecture](#4:-install-the-metapackage-for-the-target-architecture)
    - [5: You should now be all set](#5:-you-should-now-be-all-set)

# Automatic crosscompile (Linux / Mac)
[Cross](https://github.com/rust-embedded/cross) can crosscompile your Rust project for any of the targets without having
to install dependencies and libraries. It works for GLIBC and Musl Linux.

```sh
❯❯❯ cargo install cross
❯❯❯ cross build --target armv7-unknown-linux-gnueabihf --release # raspberry < 3b, rasbian 32bit
❯❯❯ cross build --target aarch-unknown-linux-gnu --release # raspberry >= 3b, 64bit
```

## Binaries

You should be able to find the binaries for the target architecture in
the `target` directory as usual, under a directory named like the target 
architecture, in this case:

```
target/armv7-unknown-linux-gnueabihf/release/fireguard
target/aarch64-unknown-linux-gnu/release/fireguard
```

or `debug` if you built with the debug symbols enabled.

# Manual Debian based process for arm32 / arm64

As armhf boards (like the raspberry pi) are becoming more and more common, 
cheaper and more powerful, using `fireguard` on an armhf/arm64 board is becoming
more convenient. This document explains briefly the changes necessary
for crosscompiling the project on Debian/Ubuntu (the developer's platform
of choice) for the armhf architecture (that supports RaspberryPi versions 2
to 3 inclusive and many other boards (like the ones from hardkernel)) and the
arm64 architecture like the Raspberry Pi 4 models.

## Local configuration

This guide assumes you are using [rustup](https://rustup.rs/) for managing
the toolchains and that you are running a fairly modern Debian or Ubuntu
host on `amd64` architecture. Cargo and rust commands do not require root,
`dpkg` and `apt` ones are to be invoked via `sudo` (recommended) or the root
user.

### 1: Add the architecture to rustup

Run the command

```sh
❯❯❯ rustup target add armv7-unknown-linux-gnueabihf # raspberry < 3b, rasbian 32bit
❯❯❯ rustup target add aarch64-unknown-linux-gnu # raspberry >= 3b, 64bit
```

to download the required Rust components for the target architecture

### 2: Configure cargo to support that target

Edit the file in `~/.cargo/config` (create it if it's not already
there) and add

```sh
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

so that the `cargo` command knows what linker to call

### 3: Add the target architecture to your host

Run the command

```sh
❯❯❯ dpkg --add-architecture armhf
❯❯❯ dpkg --add-architecture arm64
```

and update the list of packages with

```sh
❯❯❯ apt update
```

### 4: Install the metapackage for the target architecture

Debian and Ubuntu luckily provide some `crossbuild-*`  metapackages that pull
all the required packages to support cargo in its job.

Run the command

```
apt install crossbuild-essential-armhf
apt install crossbuild-essential-arm64
```

this metapackage will _not_ be available unless you add the architecture
as per the step above.

### 5: You should now be all set

To test the crosscompilation setup you should be able to use the provided
`Makefile` and invoke

```
make raspberry_<arch>
```

to build the binaries in `release` mode. The `raspberry_<arch>_debug` make target is
also provided if you need debugging symbols in your binaries.

`<arch>` is the name of the architecture you are targeting.

### Binaries
Same as the automatic method.
