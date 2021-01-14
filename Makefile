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

docker_prepare:
	cross build --target x86_64-unknown-linux-gnu --release
	cross build --target aarch64-unknown-linux-gnu --release
	cross build --target armv7-unknown-linux-gnueabihf --release
	mkdir -p docker/linux/amd64
	mkdir -p docker/linux/arm64
	mkdir -p docker/linux/arm/v7
	cp target/x86_64-unknown-linux-gnu/release/fireguard docker/linux/amd64/fireguard
	cp target/aarch64-unknown-linux-gnu/release/fireguard docker/linux/amd64/fireguard
	cp target/armv7-unknown-linux-musleabihf/release/fireguard docker/linux/arm/v7
	$(eval VERSION=$(shell target/x86_64-unknown-linux-gnu/release/fireguard --version | sed 's#fireguard ##g'))
docker_build: docker_prepare
	docker buildx create --use --name=qemu || echo "Already exist"
	docker buildx inspect --bootstrap
	docker buildx build --platform linux/arm64,linux/arm/v7 -t blackmesalab/fireguard:latest docker 
	docker buildx build --platform linux/arm64,linux/arm/v7 -t blackmesalab/fireguard:$(VERSION) docker 
	docker buildx build --load --platform linux/amd64 -t blackmesalab/fireguard:latest docker 
	docker buildx build --load --platform linux/amd64 -t blackmesalab/fireguard:$(VERSION) docker 
docker_push: docker_prepare
	docker buildx create --use --name=qemu || echo "Already exist"
	docker buildx inspect --bootstrap
	docker buildx build --push --platform linux/amd64,linux/arm64,linux/arm/v7 -t blackmesalab/fireguard:latest docker 
	docker buildx build --push --platform linux/amd64,linux/arm64,linux/arm/v7 -t blackmesalab/fireguard:$(VERSION) docker 
