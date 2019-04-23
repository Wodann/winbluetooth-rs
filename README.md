# winbluetooth

**winbluetooth provides Rust bindings to the Windows SDK Bluetooth API.**

winbluetooth's code is currently under review to be included in [winapi](https://github.com/retep998/winapi-rs/pull/753). Upon approval, support for this repository and crate are likely to be discontinued in favour of the winapi repository.

## Usage

Add the following to your `cargo.toml`:

```toml
[target.'cfg(windows)'.dependencies]
winbluetooth = "0.1"
```

## No-std support

This crate currently requires the Rust standard library.

## Platform support

winbluetooth is guaranteed to build for the following platforms:

 * x86_64-pc-windows-msvc

## License

winbluetooth is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
 
 at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in winbluetooth by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

To contribute to winbluetooth, please see [CONTRIBUTING](CONTRIBUTING.md).
