# ZK4LW - ZooKeeper Four Letter Word

TODO Description goes here

## Crates developed in this repository

* `zk4lw-client` - Rust client library for ZooKeeper ["Four Letter Word"](https://zookeeper.apache.org/doc/r3.5.7/zookeeperAdmin.html#sc_4lw) administration commands
* `zk4lw-rest` - Single-binary web-server that provides a RESTful API to execute 4lw commands against a ZooKeeper Server

## Supported ZooKeeper versions

* 3.4.x
* 3.5.x
* 3.6.x

## Development

### Prerequisites

* Rust & Cargo ([rustup](https://rustup.rs/) is recommended)
* [Docker Desktop](https://www.docker.com/products/docker-desktop), including [`docker-compose`](https://docs.docker.com/compose/) 

### Launch Zookeeper locally

You could use the provided `./compose.sh` for a "batteries included" setup:

```shell
↳ ./etc/compose/compose.sh
Description:
  Spins up and shuts down ZooKeeper, using Docker Compose.
  This is useful for local development/testing.

Usage:
  compose.sh <VERSION: 3.4|3.5|3.6> <TYPE: ensemble|standalone> <ACTION: up|down> [OPTION: attach|clean]

Options:
  attach    After launch, place Docker Compose logs in foreground (only for 'up' action)
  clean     After shutdown, delete any data directory (only for 'down' action)
```

For example, to spin up a ZooKeeper 3.5 ensemble:

```shell
↳ ./etc/compose/compose.sh 3.5 ensemble up
# ... do your work ...
↳ ./etc/compose/compose.sh 3.5 ensemble down
```

Another example, to spin up a ZooKeeper 3.6 standalone:

```shell
↳ ./etc/compose/compose.sh 3.6 standalone up
# ... do your work ...
↳ ./etc/compose/compose.sh 3.6 standalone down
```

And so on. You get the idea.

## License

[Apache 2.0](./LICENSE)

## Credits

Thank you to [Joe Wilm](https://github.com/jwilm) for the first inspiration for this project, the crate [zk-4lw](https://crates.io/crates/zk-4lw).