all: debug

debug:
	cargo build

release:
	cargo build --release

raspberry_armhf_debug:
	cargo build --target=armv7-unknown-linux-gnueabihf

raspberry_armhf:
	cargo build --release --target=armv7-unknown-linux-gnueabihf

raspberry_aarch64_debug:
	cargo build --target=aarch64-unknown-linux-gnu

raspberry_aarch64:
	cargo build --release --target=aarch64-unknown-linux-gnu 
