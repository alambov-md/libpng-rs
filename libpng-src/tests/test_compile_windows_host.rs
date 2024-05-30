#![cfg(target_os = "windows")]

mod helpers;
use helpers::{test_artifact_build, test_compile};

#[cfg(target_arch = "x86_64")]
#[test]
fn test_compile_x86_64() {
    test_compile("x86_64-pc-windows-msvc")
}

#[cfg(target_arch = "aarch64")]
#[test]
fn test_compile_aarch64() {
    test_compile("aarch64-pc-windows-msvc")
}

#[test]
fn test_compile_android_armv7() {
    test_compile("armv7-linux-androideabi")
}

#[test]
fn test_compile_android_aarch64() {
    test_compile("aarch64-linux-android")
}

#[test]
fn test_compile_android_x86() {
    test_compile("i686-linux-android")
}

#[test]
fn test_compile_android_x86_64() {
    test_compile("x86_64-linux-android")
}

#[cfg(target_arch = "x86_64")]
#[test]
fn test_build_and_bindgen_x86_64() {
    test_artifact_build("x86_64-pc-windows-msvc")
}
