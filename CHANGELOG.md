# 📰 Changelog 📰

- [Version 0.0.1](#version-0.0.1)

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
