# ippush

[![crates.io][crates-badge]][crates-url]
[![GPL licensed][license-badge]][license-url]
[![CI][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/ippush.svg
[crates-url]: https://crates.io/crates/ippush
[license-badge]: https://img.shields.io/badge/license-GPL-blue.svg
[license-url]: ./Cargo.toml
[actions-badge]: https://github.com/shtsoft/ippush/actions/workflows/ci.yaml/badge.svg
[actions-url]: https://github.com/shtsoft/ippush/actions/workflows/ci.yaml

A **simple** remote procdure returning the callers IP address and a timestamp.

- simplicity:
  * UDP as transport protocol
  * JSON output
  * less than 100 lines of code

### Installation

You can install ippush with `cargo` by running:

```console
user@host:~$ cargo install ippush
```

### Usage

To run the app you have to call the binary with an appropriate IP (e.g. 1.2.3.4) and port (e.g. 5678) as arguments:

```console
user@host:~$ ippush 1.2.3.4 5678
```

Then you can call the remote procedure by connecting to 1.2.3.4:5678 via UDP and sending arbitrary data.

## Contributing

If you want to contribute: [CONTRIBUTING](CONTRIBUTING.md).

### Security

For security-related issues see: [SECURITY](SECURITY.md).
