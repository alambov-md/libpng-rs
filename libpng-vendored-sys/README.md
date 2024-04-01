# libpng-vendored-sys
Cargo package for compiling [libpng](https://github.com/pnggroup/libpng) and vendoring it as **static** library. 

Main goal of the package is providing static library for linking with other C code, like versions of [Leptonica](http://www.leptonica.org/).
This package provides just rudimentary FFI bindings. More sophisticated bindings would be proveded in separate package. If you need to bind `libpng` with the Rust code directly, you should write your own bindings.

## Provided version
Compiles and vendors `libpng` with version `1.6.43` via [libpng-src](https://crates.io/crates/libpng-src).

## Currenlly supported OS and targets
Expected to work for:
* Linux: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu` (no cross-compilation supported yet)
* Windows: `x86_64-pc-windows-msvs`, `aarch644-pc-windows-msvs` (no cross-compilation supported yet)
* macOS: `x86_64-apple-darwin`, `aarch64-apple-darwin`
* iOS, including simulators (cross-compilation from macOS host): `x86_64-apple-ios`, `aarch64-apple-ios`, `aarch64-apple-ios-sim`

Tested before upload for all the targets, except `aarch64-unknown-linux-gnu` and `aarch64-pc-windows-msvs`.

## Dependenencies for hosts
See [libpng-src](https://crates.io/crates/libpng-src).

## TODO
* Support cross-compilation for Android;

## Authors
**Rust code and scripts:** Alexandr (Alex) Lambov <alex.lambov.md@gmail.com>, &copy; 2024

**libpng** -  see http://www.libpng.org/pub/png/libpng.html
