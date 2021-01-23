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

test:
	cargo test

coverage:
ifeq ("", $(shell which cargo-tarpaulin))
$(error "No cargo tarpaulin found in $(PATH), please install with cargo install cargo-tarpaulin if you want to run coverage")
endif
	cargo tarpaulin


tag: debug
	$(eval VERSION=$(shell target/debug/fireguard --version | sed 's#fireguard ##g'))
	$(eval CHANGELOG=$(shell git log $(shell git describe --tags --abbrev=0)..HEAD --oneline))
	@echo The changelog from the previous tag to $(VERSION) is:$(CHANGELOG)
	@echo "######################### Start $(VERSION) changelog ###################" >> CHANGELOG.md
	@echo $(CHANGELOG) >> CHANGELOG.md
	@echo "######################### End $(VERSION) changelog ###################" >> CHANGELOG.md
	vim CHANGELOG.md
	git add CHANGELOG.md
	git commit CHANGELOG.md
	git tag v$(VERSION)

docker_prepare:
	cross build --target x86_64-unknown-linux-gnu --release
	cross build --target aarch64-unknown-linux-gnu --release
	cross build --target armv7-unknown-linux-gnueabihf --release
	mkdir -p docker/linux/amd64
	mkdir -p docker/linux/arm64
	mkdir -p docker/linux/arm/v7
	cp target/x86_64-unknown-linux-gnu/release/fireguard docker/linux/amd64/fireguard
	cp target/aarch64-unknown-linux-gnu/release/fireguard docker/linux/arm64/fireguard
	cp target/armv7-unknown-linux-musleabihf/release/fireguard docker/linux/arm/v7

docker_build: docker_prepare
	$(eval VERSION=$(shell target/x86_64-unknown-linux-gnu/release/fireguard --version | sed 's#fireguard ##g'))
	docker buildx create --use --name=qemu || echo "Already exist"
	docker buildx inspect --bootstrap
	docker buildx build --platform linux/arm64,linux/arm/v7 -t blackmesalab/fireguard:latest docker 
	docker buildx build --platform linux/arm64,linux/arm/v7 -t blackmesalab/fireguard:$(VERSION) docker 
	docker buildx build --load --platform linux/amd64 -t blackmesalab/fireguard:latest docker 
	docker buildx build --load --platform linux/amd64 -t blackmesalab/fireguard:$(VERSION) docker 

docker_push: docker_prepare
	$(eval VERSION=$(shell target/x86_64-unknown-linux-gnu/release/fireguard --version | sed 's#fireguard ##g'))
	docker buildx create --use --name=qemu || echo "Already exist"
	docker buildx inspect --bootstrap
	docker buildx build --push --platform linux/amd64,linux/arm64,linux/arm/v7 -t blackmesalab/fireguard:latest docker 
	docker buildx build --push --platform linux/amd64,linux/arm64,linux/arm/v7 -t blackmesalab/fireguard:$(VERSION) docker 
