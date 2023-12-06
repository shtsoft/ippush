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
  * udp as transport protocol
  * json output
  * less than 50 lines of code

### Installation

You can install ippush with `cargo` by running:

```console
user@host:~$ cargo install ippush
```

### Usage

To get a usage description just run the app without arguments:

```console
user@host:~$ ippush
```

## Contributing

If you want to contribute: [CONTRIBUTING](CONTRIBUTING.md).

### Security

For security-related issues see: [SECURITY](SECURITY.md).
