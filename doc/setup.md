# First run howto

The following howto specifies how to configure and run the project, in the
hope that it will be able to provide both a starting point for first time
users and a general overview of the architecture and spirit of the project.

## Rationale

The project aims at solving a relatively simple, common and yet surprisingly
tricky situation: a number of nodes, managed by different people across
the public internet, want to administer and share a vpn-line connection
with minimal hassle, reasonable security and ease of operation.

Let's assume for the sake of this example that there are four nodes belonging
to three different users:

- Alice, controlling a roaming laptop (L) and an home network raspberry pi (R)
- Bob, controlling a cloud server (C) on some $hosting_service
- Carol, controlling a baremetal host (B) in a basement with a great internet connection

All these users trust each other and want to create a mesh-like network
among their hosts (sometimes referred to as nodes), allowing them to
exchange information and services among their nodes with privacy and
security.

## Network topology and node setup

Since both L and R might be roaming via a non-public network, be behind a nat
or otherwise not have access to a public ip, the solution needs to work even when
the node only has "outgoing" access to the public internet.

Futhermore, the various nodes that take part in this network have been running for
quite some time and feature services and data that might not be intended to be
shared on this vpn-like network. For this reason, the choice of a single binary
running in a dockerize environment, only therefore exposing a subnet to the
physical host, minimizes intrusivity and makes operating the nodes easier.

## Network definition

In order to define their first network, the group of friends will first create
a repository on some publicly - or not - available git repository and initialize
the nodes definition file, aptly named `nodes.toml`. An example file follows,
with comments interspersed in.

First, the general network definition block:

```
repository = "avalon"
network = "10.123.123.0/24"
domain = "avalon.lan"
```

The above snippet defines the `avalon` network, having the internal
ip subnet `10.123.123.0` (they decide 254 hosts are plenty for the time
being and it's always possible to switch to a `/16` netmask in the future)
using the internal domeain `avalon.lan` for dns resolution. This part
is common and will usually be located on the top of the file since it's
driving the rest of the topology decisions.

While the `fireguard` binary provides a `peer` command to list,
add, remove and display a peer given a definition file, it is
intended for users that are already familiar with the usage of the
application, so the configuration file in this example will be
manually generated to explain the reasoning and decisions behind it.

Bob defines their node as

```
[peers.bob-cloud]
username = "bob"
peername = "cloud"
address = "10.123.123.183/24"
listen_port = 6666
public_key = "/UYfZPEWuJaZ1EmAc88n7TnZqwkO/HYoRh2iBrj98go="
allowed_ips = ["10.123.123.183/32"]
persistent_keepalive = 25
endpoint = "cloud.bob.net"
mtu = 1500
```

This block defines a node that will have the internal vpn address
of `10.123.123.183` and listen on port `6666` for incoming connections
from other peers on the public endpoint at `cloud.bob.net`. All
these parameters can be changed for the single node, but Bob must
take care that the chose ip address for the `cloud` node is not
repeated in the configuration. `fireguard` will actually
check this configuration for them, making sure that the ip - which
is randomly selected from the pool of ips that belong to the 
network defined in the common configuration above - does not repeat,
but two eyes are better than one :)

Next, Alice will define their two nodes

```
[peers.alice-laptop]
username = "alice"
peername = "laptop"
address = "10.123.123.111/24"
listen_port = 6666
public_key = "/UYfZPEWuJaZ1EmAc88n7TnZqwkO/HYoRh2iBrj98go="
allowed_ips = ["10.123.123.111/32"]
persistent_keepalive = 25
mtu = 1500

[peers.alice-raspberry]
username = "alice"
peername = "raspberry"
address = "10.123.123.240/24"
listen_port = 6666
public_key = "vS6JBz+GPStFTgerKD0Zz8cEZPOFdpABgGgAhB484wg="
allowed_ips = ["10.123.123.240/32"]
persistent_keepalive = 25
mtu = 1500
```

in a manner very similar to Bob's ones, the main difference
being that, since Alice's nodes do not have a publicly 
reachable ip address, the `endpoint` configuration option
is missing. This is not a problem, since wireguard allows
for a connection as long as at least one node has a public
ip address.

Finally, Carol defines their last node

```
[peers.carol-baremetal]
username = "carol"
peername = "baremetal"
address = "10.123.123.156/24"
listen_port = 6666
public_key = "+d/lKXMqw20+lcc6I+IesPWi9zTN+v4Vai9U4VTOkSY="
allowed_ips = ["10.123.123.156/32"]
persistent_keepalive = 25
endpoint = "baremetal.basement.carol.net"
mtu = 1500
```

This concludes the configuration file for the `avalon` network.
Once the configuration is completed and the various
decisions agreed upon, the next step is configuring the
repository and configuring the single hosts and nodes to
talk to each other.

### Security and trust considerations

There is a number of security and trust considerations to
keep in mind when using this software. While the information
in the configuration contains only public keys and public
dns names, this can be information enough that an attacker
might find the node endpoint worthy of consideration, especially
if the node exposes other services.

Furthermore, since the `fireguard` tool allows for automatic
update of the network topology (more on this later) a single
malicious user might connect to the network (by committing
to git its node definition) a malicious host, therefore
great care must be taken in allowing by whom, how and when
the main `nodes.toml` file is modified. The authors suggest
enabling branch protection on the repository end enforcing
a reasonable number of reviewers to sign off any merges
to this file, to minimize the opportunity for a single attacker.

## Installation on the hosts

In order to execute the `fireguard` binary and have the
nodes configuration actually mean something the Docker
engine needs to be installed on the node toghether with
the wireguard kernel module (unnecessary if running a recent
enough kernel where it is mainline) and its tools. Finally,
as the container will need to run in privileged mode to be
able to change the network configuration of the host and
expose the internal network to the nodes, root or privileged
access is necessary.

## Pulling the network repository

As a single node can be part of more than one Firguard network
the `clone` (to initialize) and `pull` commands are provided
inside the `repo` subcommand to handle the `nodes.toml` file.

Alice, Bob and Carol will therefore run the following commands
on each of their nodes, to create, update and manage the 
configuration and the wireguard instance.

### Cloning the repository for the first time

The command

```
fireguard repo -r https://path/to/the/avalon/repository clone
```

will access the repository and store its configuration, by default,
in the `/etc/fireguard/<network_name>` directory.

### Pulling a repository

To update the `avalon` repository issue the command

```
fireguard repo -r avalon pull
```

## Rendering the wireguard configuration

Once the repository has been cloned or updated, each user must generate the
wireguard configuration based on the `nodes.toml` file. This
is done via the `render` subcommand of the `wg` fireguard action.
Following the example Alice will run these two commands on their 
nodes:

```
root@laptop:~# fireguard wg -r avalon render -u alice -p laptop -P <private_key>
```

```
alice@raspberry:~# sudo fireguard wg -r avalon render -u alice -p raspberry -P <private_key>
```

This command is necessary to make sure the private key is only ever seen
and handled by Alice on their nodes, never being written down anywhere or
having to live in configuration files other than where strictly 
necessary.

The commands above, having generated the wireguard configuration for
the local node and allowing connections from the other nodes, 
render the nodes read for running the network proper.

## Running the wireguard docker container

Once all the users have configured their own hosts, they can then
issue the command that will pull, install and launch the
wireguard daemon inside a docker container, exposing the
network on the hosts. Following the Alice example the command

```
fireguard docker daemon -r avalon serve -u alice -p laptop
```

will hang on the internal fireguard process, waiting for
configuration and management events. Alice will be able to
check their side of the network is up and running with the
usual

```
wg show
```

command that will display the configuration of the network
and the hadnshake state with the other peers.

