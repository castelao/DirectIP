# Iridium's SBD Direct-IP communications

Iridium's Direct-IP protocol of Short Burst Data communication system.

This crate is focused on transmitting Mobile Terminated messages, often used
to send commands to a remote station. For instance, it could be used to
communicate with an under water glider (a scientific autonomous robot).


# API reference

- from-file: Optional argument. When used, it is expected a path to a file
  instead of the payload itself, such as:
  ```text
  echo "Hello world" > ./my_command.txt
  directip-client ... --from-file ./my_command.txt
  ```

- encoding:
  - ascii: Default option. Expects a valid UTF-8.
  - binary: Transmit as it is.

# Examples

Note that the examples below expect a server running local. If that's not
your case, you can add '--dry-run' to test it.

## ASCII payload

Giving the payload as an ASCII argument:

```shell,no_run
directip-client --msg-id=987 --server 127.0.0.1:10800 --imei 012345678901234 "Hello World"
```

or chain it with another command and pass the payload through stdin:
```text
echo "Hello World" | directip-client --msg-id=987 --server 127.0.0.1:10800 --imei 012345678901234
```

## Binary payload

Chain it with another command to pass a binary payload using stdin:
```text
head -c 8 /dev/urandom | directip-client --msg-id=987 --server 127.0.0.1:10800 --imei 012345678901234
```

## HEX payload

WIP

## Minimum supported Rust version

Currently the minimum supported Rust version is 1.57.0

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
