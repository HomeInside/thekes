# Thekes :open_book:

[![Release](https://github.com/HomeInside/thekes/actions/workflows/release.yml/badge.svg)](https://github.com/HomeInside/thekes/actions/workflows/release.yml)
[![GPUI Kit](https://img.shields.io/badge/UI-GPUI%20Kit-0A7EA4)](https://gpui-kit.com/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux-lightgrey)](#installation)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](README.md#license)

A very simple, native desktop application, written in [Rust](https://www.rust-lang.org) using [GPUi-Kit](https://gpui-kit.com/), for managing your books.

**Thekes** comes from [θήκη](https://en.wikipedia.org/wiki/Library#Etymology) ('théke', 'cupboard, box'),'the place where books are kept'.


## Installation

Download the latest Windows, or Gnu/Linux build from [Releases](https://github.com/HomeInside/thekes/releases). Checksums are included.

Gnu/Linux releases currently target x86_64 glibc-based distributions (glibc 2.35 or newer) and require a compatible system graphics driver.

The portable Windows ZIP and Gnu/Linux tarball are updated manually from [Releases](https://github.com/HomeInside/thekes/releases).


## Development

### Requirements

 - [rustup](https://rustup.rs) to install Rust :wink:
 - [Rust](https://www.rust-lang.org) **1.97.0** and above

### Recommended tools
 - [just](https://github.com/casey/just) **1.55** and above

### Clone the repo and make it yours:

```sh
$ git clone git@github.com:HomeInside/thekes.git
```

### Build

```sh
$ cd thekes
```

```sh
$ cargo build --release
```


## Pre-Built Binaries

Pre-built binaries for Gnu/Linux, and Windows can be found on the [releases page](https://github.com/HomeInside/thekes/releases).


## Tutorials

 - We are working on it... :smile:


## Thanks To:

- [GPUi-Kit](https://gpui-kit.com/)
- [BookOrbit](https://bookorbit.app/)
- [Panda Reader](https://github.com/yuhangch/panda-reader)
- [RustGrid](https://github.com/yisier/rustgrid)


## License

**thekes** is licensed under either of the following licenses, at your option:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
