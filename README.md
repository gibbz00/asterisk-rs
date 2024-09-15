# asterisk-rs

An Asterisk Library in Rust

## `asterisk-rs-ari`

A crate for Asterisk REST Interface tooling

## `asterisk-rs-ami`

A crate for Asterisk Management Interface tooling

## `asterisk-rs-agi`

A crate for Asterisk Gateway Interface tooling

### Asterisk Docker Image

A `Dockerfile` is provided in this repository to run an Asterisk locally.

Build the container with:

```sh
# from repository root
cd docker
docker build --tag asterisk-rs .
```

To then run it with:

```sh
docker run -p 8088:8088/tcp -p 5060:5060/tcp -p 5060:5060/udp -p 16384-16394:16384-16394 asterisk-rs
```
