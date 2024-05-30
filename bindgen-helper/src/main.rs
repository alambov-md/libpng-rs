use std::{env::temp_dir, fs::write, path::PathBuf};

use guess_host_triple::guess_host_triple;
use libpng_src::build_artifact;

const MANUAL_BEGINNING: &str = "//! Cargo package for compiling [libpng](https://github.com/pnggroup/libpng) and vendoring it as **static** library.
//!
//! Main goal of the package is providing static library for linking with other C code, like versions of [Leptonica](http://www.leptonica.org/).
//! It provides just rudimentary FFI bindings. More sophisticated bindings would be provided in a separate crate.
//! If you need to bind **libpng** with the Rust code directly, you should write your own bindings.
//!
//! Does not translate C macros from **libpng**.
//!
//! Rust FFI bindings not documented. For information on underlying C functions and constants is provided here:
//! https://github.com/pnggroup/libpng/blob/libpng16/libpng-manual.txt

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use libc::{time_t, tm, FILE};

#[cfg(feature = \"link-libz\")]
#[allow(unused_imports)]
// Used for linking only
use libz_sys;

mod c_macro_helpers;
pub use c_macro_helpers::*;

";

fn main() {
    let target = guess_host_triple().expect("Cannot detect this hosts target");

    let working_dir = temp_dir().join("bindgen-helper");

    let artifact_info = build_artifact(target, &working_dir).unwrap();
    let png_h_path = artifact_info.include_dir.join("png.h");

    let dest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("libpng-vendored-sys")
        .join("src")
        .join("lib.rs");

    let bindings = bindgen::builder()
        .header(png_h_path.to_string_lossy())
        .allowlist_file(png_h_path.to_string_lossy())
        .blocklist_item("FILE")
        .blocklist_item("time_t")
        .blocklist_item("tm")
        .blocklist_item("fpos_t")
        .blocklist_item("__sFILE")
        .blocklist_item("__sFILEX")
        .blocklist_item("__sbuf")
        .blocklist_item("__darwin_time_t")
        .blocklist_item("__darwin_off_t")
        .blocklist_item("__int64_t")
        .generate()
        .unwrap()
        .to_string();

    write(dest_path, format!("{MANUAL_BEGINNING}{bindings}")).unwrap();
}
