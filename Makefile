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

docker_build:
	cross build --target x86_64-unknown-linux-musl --release
	cp target/x86_64-unknown-linux-musl/release/fireguard docker/
	$(eval VERSION=$(shell target/x86_64-unknown-linux-musl/release/fireguard --version | sed 's#fireguard ##g'))
	docker build -t blackmesalab/fireguard:latest docker
	docker tag blackmesalab/fireguard:latest blackmesalab/fireguard:$(VERSION)
