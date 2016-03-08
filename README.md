# tus-rust

[![Build Status](https://travis-ci.org/Rio/tus-rust.svg?branch=master)](https://travis-ci.org/Rio/tus-rust)

Rust implementation of the tus resumable upload protocol. http://tus.io

## Build

`cargo build --release`

## Test

Currently testing is just starting a server and running the tests.  
Ideally the tests should start the server or mimic a server.

`cargo run` and in a different terminal `cargo test`.

## TODO
Protocol:
 - [x] Implement [Core](http://tus.io/protocols/resumable-upload.html#core-protocol) Protocol
  - [x] Head
  - [x] Patch
  - [x] Options
 - [ ] Implement [Creation](http://tus.io/protocols/resumable-upload.html#creation) Extension
 - [ ] Implement [Expiration](http://tus.io/protocols/resumable-upload.html#expiration) Extension
 - [ ] Implement [Checksum](http://tus.io/protocols/resumable-upload.html#checksum) Extension
 - [ ] Implement [Termination](http://tus.io/protocols/resumable-upload.html#termination) Extension
 - [ ] Implement [Concatenation](http://tus.io/protocols/resumable-upload.html#concatenation) Extension

Binaries:
 - [x] Implement basic server
 - [ ] Implement basic commandline client
 - [ ] Implement pluggable backend protocols for server starting with [ZeroMQ](http://zeromq.org)
