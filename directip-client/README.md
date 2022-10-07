# Iridium's SBD Direct-IP client

Iridium's Direct-IP protocol of Short Burst Data communication system.

This crate is focused on transmitting Mobile Terminated messages, often used
to send commands to a remote station. For instance, it could be used to
communicate with an under water glider (a scientific autonomous robot).

## Installation

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
