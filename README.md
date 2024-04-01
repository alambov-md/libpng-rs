# libpng-rs
Cargo packages suite for 'libpng' library usage in Rust.

## Content
* [libpng-src](libpng-src/README.md) - Helper package for compiling libpng into a static library.
* [libpng-vendored-sys](libpng-vendored-sys/README.md) - `-sys` package for vendoring `libpng` as static library.

## Currenlly supported OS and targets
Expected to work for:
* Linux: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu` (no cross-compilation supported yet)
* Windows: `x86_64-pc-windows-msvs`, `aarch644-pc-windows-msvs` (no cross-compilation supported yet)
* macOS: `x86_64-apple-darwin`, `aarch64-apple-darwin`
* iOS, including simulators (cross-compilation from macOS host): `x86_64-apple-ios`, `aarch64-apple-ios`, `aarch64-apple-ios-sim`

## Authors
**Rust code and scripts:** Alexandr (Alex) Lambov <alex.lambov.md@gmail.com>, &copy; 2024

**libpng** -  see http://www.libpng.org/pub/png/libpng.html