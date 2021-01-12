all: debug

debug:
	cargo build

release:
	cargo build --release
