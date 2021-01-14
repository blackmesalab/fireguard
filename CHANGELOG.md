# 📰 Changelog 📰

- [Version 0.0.5](#version-0.0.5)
- [Version 0.0.4](#version-0.0.4)
- [Version 0.0.3](#version-0.0.3)
- [Version 0.0.2](#version-0.0.2)
- [Version 0.0.1](#version-0.0.1)

## Version 0.0.5
* c37d8c1 Update changelog for v0.0.5 ce63fe6 Release version 0.0.5 
* 7fbd004 Add docker multi arch build directives to Makefile 
* 7670b55 Move package installation to Fireguard startup to let it happen on the actual host and not during docker buildx, where apt causes many failures inside docker qemu. 
* fb305f0 Release version 0.0.4 4da6bb6 Fix

## Version 0.0.4
* 2c55318 Release version 0.0.4
* 4da6bb6 Fix #11: implement a single command to replace entrypoint.sh
* e683f0a Add docker inception command
* fcbcdc9 Add PID store functionalities
* c39f12e Still not working but better implemented DNS service discovery management
* 3cd4294 Add Makefile target to build Fireguard Docker image

## Version 0.0.3
* Release v0.0.3. Fix CI and allow travis to push on github releases.     

## Version 0.0.2
* Release v0.0.2. Fix CI
* Add initial CONTRIBUTING guide
* Add CHANGELOG
* Fix #10: allow logging to be controlled via command line

## Version 0.0.1
* Release version 0.0.1
* Update crosscompile instructions
* Merge pull request #15 from uovobw/add-armhf-crosscompile-information
* Merge pull request #13 from uovobw/support-endpointless-nodes
* Merge pull request #16 from blackmesalab/use_sys_wg
* Add makefile targets and documentation required to enable crosscompilation to raspberry pi boards
* Make the `Endpoint` value optional in the configuration for nodes that are part of the network but do not have  a public address that can be reached by other nodes.
* A bit more accurate README
* Add (not working) Dns / service discovery command
* Make UI more consistent by asking for the repository also in the repo command.
* Add docker x86-64 build
* Make wg up / down functional
* Add initial wg status command
* Add boringtun + wg-quick up and down management
* Better shell command handling with multiple options for stdin, env, etc..
* Add trivial makefile
* Fix typo in git pull command that ran `gi pull`
* Add port to endpoint in wireguard config and change config file name to the repository name.
* Another try fixing the repo clone
* Fix bug where clone is creating the wrong repo name
* Fix #3. Fix config issues and add support for fwmark and table config directives.
* Let's just build stable for now
* Do not check for docker before parsing command line
* Build on both stable and nightly Rust. Add initial README
* Cleanup Cargo.toml. Add vendored openssl to allow Travis to build
* Fix Travis IRC notification. Move to Focal Fossa
* Install libssl-dev inside Travis
* Fix travis configuration syntax and typos
* Add travis CI
* Bundle template in code without external dependencies
* Fix wireguard configuration rendering
* Add initial (broken) rendering of Wireguard config. Add prechecks infrastructure for commands
* Fix help message typo
* Refactor to use Tokio async runtime and allow using asyncio based concurrency.
* Add IP pool and command to add and remove peers with IPAM functionalities
* Initial Wireguard structure handling implementation.
* Basic command structure with example commands handling trust repo git management.
* Initial commit
