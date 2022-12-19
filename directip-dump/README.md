# Iridium's SBD Direct-IP dump

Iridium's Direct-IP protocol of Short Burst Data communication system.

This crate is focused on parsing Direct-IP messages to extract information
from such binary message. It could be used, for instance, to identify if a
message is an MT type, show the IMEI, or dump the payload.

## Install

### MacOS (Apple Silicon)

``` shell
curl -o directip -L https://github.com/castelao/DirectIP/releases/latest/download/directip-dump-aarch64-apple-darwin
chmod +x directip
```

### MacOS (Intel)

``` shell
curl -o directip -L https://github.com/castelao/DirectIP/releases/latest/download/directip-dump-x86_64-apple-darwin
chmod +x directip
```

### Linux (arm)

``` shell
curl -o directip -L https://github.com/castelao/DirectIP/releases/latest/download/directip-dump-arm-unknown-linux-gnueabihf
chmod +x directip
```

### Linux (x86_64)

``` shell
curl -o directip -L https://github.com/castelao/DirectIP/releases/latest/download/directip-dump-x86_64-unknown-linux-musl
chmod +x directip
```

### Windows (x86_64)

``` shell
Invoke-WebRequest -Uri 'https://github.com/castelao/DirectIP/releases/latest/download/directip-dump-x86_64-pc-windows-msvc.exe' -OutFile directip
```

### From source code

Install the Rust compiler if you don't already have it. Follow the
instructions at https://www.rust-lang.org/tools/install

Compile it by running: `cargo build --release -p directip-dump`

## Options and customization

## Examples

## Minimum supported Rust version

Currently the minimum supported Rust version is 1.64.0

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