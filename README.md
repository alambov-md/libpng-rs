# libpng-rs
Cargo packages suite for 'libpng' library usage in Rust.

## Goal
Main goal of this workspace is educational. It describes a way how to replicate cross-platrform Rust library with C++ dependency for all major mobile and desktop OS in a testable manner.
Rust **libping** wrapper library is build and tested automatically for Linux/Windows/macOS/iOS/Android on each PR both with unit and integration tests. For mobile testing mobile simulators are used. C++ code compilation, FFI and linking issues are addressed.
For the test pipeline see [rust.yml](github/workflows/rust.yml)

## Content
Main packages in the workspace are:
* [libpng-src](libpng-src/README.md) - Helper package for compiling libpng into a static library.
* [libpng-vendored-sys](libpng-vendored-sys/README.md) - `-sys` package for vendoring **libpng** as static library.

## Currenlly supported OS and targets
Expected to work for:
* Linux: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu` (no cross-compilation supported yet)
* Windows: `x86_64-pc-windows-msvc`, `aarch644-pc-windows-msvc` (no cross-compilation supported yet)
* macOS: `x86_64-apple-darwin`, `aarch64-apple-darwin`
* iOS, including simulators (cross-compilation from macOS host): `x86_64-apple-ios`, `aarch64-apple-ios`, `aarch64-apple-ios-sim`
* Android (cross-compilation from Linux, macOS or Windows hosts): `armv7-linux-androideabi`, `aarch64-linux-android`, `i686-linux-android`, `x86_64-linux-android`

## Authors
**Rust code and scripts:** Alexandr (Alex) Lambov <alex.lambov.md@gmail.com>, &copy; 2024

**libpng** -  see http://www.libpng.org/pub/png/libpng.html