# 🔥 Fireguard 🔥
[![Travis](https://img.shields.io/travis/blackmesalab/fireguard?style=for-the-badge)](https://travis-ci.org/github/blackmesalab/fireguard)
[![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](https://github.com/crisidev/qrsync/blob/master/LICENSE)

- [Wireguard trust network](#wireguard-trust-network)
- [Install](#install)
- [Rust version](#rust-version)
- [License](#license)

### Wireguard Trust Network

### Install
Travis-CI releases [binaries](https://github.com/blackmesalab/fireguard/releases) for various architectures when a new tag is pushed:
* x84-64 Linux GNU
* x86-64 Linux Musl
* x86-64 Darwin
* x86-64 Windows
* aarch64 Linux GNU
* aarch64 Linux Musl
* arm Linux GNU
* armv7 Linux GNU

Alternatively you can install the latest tag directly from git:
```sh
❯❯❯ cargo install --git https://github.com/blackmesalab/fireguard --branch main
```

### Rust version
Fireguard has been tested with Rustc >= 1.42, both stable and nightly.

### License
See [LICENSE](https://github.com/blackmesalab/fireguard/blob/master/LICENSE) file.
