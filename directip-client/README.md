# Iridium's SBD Direct-IP client

Iridium's Direct-IP protocol of Short Burst Data communication system.

This crate is focused on transmitting Mobile Terminated messages, often used
to send commands to a remote station. For instance, it could be used to
communicate with an under water glider (a scientific autonomous robot).

## Install

### MacOS (Apple Silicon)

``` shell
curl -o directip -L https://github.com/castelao/DirectIP/releases/latest/download/directip-client-aarch64-apple-darwin
chmod +x directip
```

### MacOS (Intel)

``` shell
curl -o directip -L https://github.com/castelao/DirectIP/releases/latest/download/directip-client-x86_64-apple-darwin
chmod +x directip
```

### Linux (arm)

``` shell
curl -o directip -L https://github.com/castelao/DirectIP/releases/latest/download/directip-client-arm-unknown-linux-gnueabihf
chmod +x directip
```

### Linux (x86_64)

``` shell
curl -o directip -L https://github.com/castelao/DirectIP/releases/latest/download/directip-client-x86_64-unknown-linux-musl
chmod +x directip
```

### Windows (x86_64)

``` shell
Invoke-WebRequest -Uri 'https://github.com/castelao/DirectIP/releases/latest/download/directip-client-x86_64-pc-windows-msvc.exe' -OutFile directip
```

### From source code

Install the Rust compiler if you don't already have it. Follow the
instructions at https://www.rust-lang.org/tools/install

Compile it by running: `cargo build --release -p directip-client`

## Options and customization

- dry-run: Create a message and show in the screen without trying to
           transmit it.

- encoding:
  - ascii: Default option. Expects a valid UTF-8.
  - binary: Transmit as it is.
  - hex: Not fully functional yet.

- from-file: When used, it is expected a path to a file instead of the
             payload itself.

## Examples

Note that the examples below expect a server running local. If that's not
your case, you can add `--dry-run` to test it.

### ASCII payload

Giving the payload as an argument:

```shell,no_run
directip-client --msg-id=987 --server 127.0.0.1:10800 --imei 012345678901234 "Hello World"
```

or chain it with another command and pass the payload through stdin:
```text
echo "Hello World" | directip-client --msg-id=987 --server 127.0.0.1:10800 --imei 012345678901234
```

or load it from a file:
```text
echo "Hello world" > ./my_command.txt

directip-client --server 127.0.0.1:10800 \
    --msg-id=987 \
    --imei 012345678901234 \
    --from-file ./my_command.txt
```

### Binary payload

Chain it with another command to pass a binary payload using stdin:
```text
head -c 8 /dev/urandom | directip-client --server 127.0.0.1:10800 --msg-id=987 --imei 012345678901234
```

or load it from a file:
```text
head -c 8 /dev/urandom > ./my_command.txt

directip-client --server 127.0.0.1:10800 \
    --msg-id=987 \
    --imei 012345678901234 \
    --encoding=binary \
    --from-file ./my_command.txt
```

### HEX payload

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
