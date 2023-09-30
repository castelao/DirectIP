# Iridium's SBD Direct-IP communications

Iridium's Direct-IP protocol of Short Burst Data communication system.

This library understands and handles the Direct-IP protocol, thus providing
the core support for applications such as the one listed below.

## Applications

- [directip-client](https://crates.io/crates/directip-client): Compose and
  transmit MT messages, i.e. to remote station. For instance, it could be
  used to send a message to an underwater glider (a scientific autonomous
  robot).

- [directip-dump](https://crates.io/crates/directip-dump): Parse and extract
  components, such as the IMEI or message-id, from a Direct-IP binary
  transmission. This is an equivalent to ncdump but for Direct-IP messages.

- [directip-storage](https://crates.io/crates/directip-storage): A library
  that provides multiple backends to store Direct-IP messages. It can be
  used, for instance, to archive MT messages transmitted or MO messages
  received.

## Minimum supported Rust version

Currently the minimum supported Rust version is 1.60.0

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Alternatives

* [sbd-rs](https://crates.io/crates/sbd): Handles the Mobile-Originated side
  of the SBD protocol and provides a server to listen for incoming MO messages
  . It was visionary to use Rust in 2015! (@gadomski, 2015)
* [sbd_lib](https://crates.io/crates/sbd_lib): I believe it contains some
  elements of the sbd-rs crate, extending to MT messages while using serde
  to serialize and deserialize. Some interesting uses of the language
  beyond the basics. (@a1ien, 2018)
* [sbd.py](https://xed.ch/project/isbd/) This is a Python solution with a lot
  of information on the protocol and concepts. Very useful independent of the
  language you will use. (@cxed, 2016)
