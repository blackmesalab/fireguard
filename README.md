# 🔥 Fireguard 🔥 - A Wireguard based trust network
[![Travis](https://img.shields.io/travis/blackmesalab/fireguard?style=for-the-badge)](https://travis-ci.org/github/blackmesalab/fireguard)
[![Docker](https://img.shields.io/docker/v/blackmesalab/fireguard?sort=semver&style=for-the-badge)](https://hub.docker.com/r/blackmesalab/fireguard/)
[![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](https://github.com/crisidev/qrsync/blob/master/LICENSE)

- [Install](#install)
  - [Rust version](#rust-version)
- [Docker](#docker)
- [Software](#software)
- [License](#license)

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

Sooner or later is will be on [Crates.io](https://crates.io).

#### Rust version
Fireguard has been tested with Rustc >= 1.42, both stable and nightly.

### Docker
Fireguard is meant to run inside a privileged container with network=host. 

Currently we are building and x86-64 image on [Docker Hub](https://hub.docker.com/r/blackmesalab/fireguard/).

To run it:

```sh
❯❯❯ docker run --privileged --net=host --rm -ti blackmesalab/fireguard \
        https://github.com/blackmesalb/novanet \
        novanet bigo 8FLd3fxj2MqJEoxRy5Y0FA5Bu81bp20rG7kuojcAYmA= 
```

Where the arguments are the trust repository, its name on disk, the user, the name of the new node / peer and finally the Wireguard private key.

### Software
All this awesomness is possible thanks to:
* [Wireguard](https://www.wireguard.com/)
* [Rust](https://www.rust-lang.org/)
* [Borintun](https://github.com/cloudflare/boringtun)
* [Git](https://git-scm.com/)
* [DNSMasq](http://www.thekelleys.org.uk/dnsmasq/doc.html)
* Plenty of others..

### License
See [LICENSE](https://github.com/blackmesalab/fireguard/blob/master/LICENSE) file.
