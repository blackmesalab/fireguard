all: debug

cross: raspberry

debug:
	cargo build

release:
	cargo build --release

raspberry_debug:
	cargo build --target=armv7-unknown-linux-gnueabihf

raspberry:
	cargo build --release --target=armv7-unknown-linux-gnueabihf
