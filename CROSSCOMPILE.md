# Crosscompilation for ARMHF/ARM64

As armhf boards (like the raspberry pi) are becoming more and more common, 
cheaper and more powerful, using `fireguard` on an armhf/arm64 board is becoming
more convenient. This document explains briefly the changes necessary
for crosscompiling the project on Debian/Ubuntu (the developer's platform
of choice) for the armhf architecture (that supports RaspberryPi versions 2
to 3 inclusive and many other boards (like the ones from hardkernel)) and the
arm64 architecture like the Raspberry Pi 4 models.

# Local configuration

This guide assumes you are using [rustup](https://rustup.rs/) for managing
the toolchains and that you are running a fairly modern Debian or Ubuntu
host on `amd64` architecture. Cargo and rust commands do not require root,
`dpkg` and `apt` ones are to be invoked via `sudo` (recommended) or the root
user.

## 1: Add the architecture to rustup

Run the command

```
rustup target add armv7-unknown-linux-gnueabihf # raspberry < 4
rustup target add aarch64-unknown-linux-gnu # raspberry >= 4
```

to download the required Rust components for the target architecture

## 2: Configure cargo to support that target

Edit the file in `~/.cargo/config` (create it if it's not already
there) and add

```
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

so that the `cargo` command knows what linker to call

## 3: Add the target architecture to your host

Run the command

```
dpkg --add-architecture armhf
dpkg --add-architecture arm64
```

and update the list of packages with

```
apt update
```

## 4: Install the metapackage for the target architecture

Debian and Ubuntu luckily provide some `crossbuild-*`  metapackages that pull
all the required packages to support cargo in its job.

Run the command

```
apt install crossbuild-essential-armhf
apt install crossbuild-essential-arm64
```

this metapackage will _not_ be available unless you add the architecture
as per the step above.

## 5: You should now be all set

To test the crosscompilation setup you should be able to use the provided
`Makefile` and invoke

```
make raspberry_<arch>
```

to build the binaries in `release` mode. The `raspberry_<arch>_debug` make target is
also provided if you need debugging symbols in your binaries.

`<arch>` is the name of the architecture you are targeting.


# Binaries

You should be able to find the binaries for the target architecture in
the `target` directory as usual, under a directory named like the target 
architecture, in this case:

```
target/armv7-unknown-linux-gnueabihf/release/fireguard
target/aarch64-unknown-linux-gnu/release/fireguard
```

or `debug` if you built with the debug symbols enabled.
