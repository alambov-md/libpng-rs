use std::{env::var, path::PathBuf};

use libpng_src::build_artifact;

fn main() {
    let target = var("TARGET").unwrap();
    let out_dir = var("OUT_DIR").map(PathBuf::from).unwrap();

    let artifact_info = build_artifact(&target, &out_dir).unwrap();

    println!(
        "cargo:rustc-link-search=native={}",
        artifact_info.lib_dir.to_string_lossy()
    );
    println!("cargo:rustc-link-lib=static={}", artifact_info.link_name);
}
