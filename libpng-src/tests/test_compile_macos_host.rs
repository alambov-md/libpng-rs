#![cfg(target_os = "macos")]

mod helpers;
use helpers::{test_artifact_build, test_compile};

#[test]
fn test_compile_macos_intel() {
    test_compile("x86_64-apple-darwin")
}

#[test]
fn test_compile_macos_arm() {
    test_compile("aarch64-apple-darwin")
}

#[test]
fn test_compile_ios_arm() {
    test_compile("aarch64-apple-ios")
}

#[test]
fn test_compile_ios_arm_sim() {
    test_compile("aarch64-apple-ios-sim")
}

#[test]
fn test_compile_ios_intel_sim() {
    test_compile("x86_64-apple-ios")
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

#[test]
fn test_build_and_bindgen_macos_intel() {
    test_artifact_build("x86_64-apple-darwin")
}

#[test]
fn test_build_and_bindgen_macos_arm() {
    test_artifact_build("aarch64-apple-darwin")
}

#[test]
fn test_build_and_bindgen_ios_arm() {
    test_artifact_build("aarch64-apple-ios")
}

#[test]
fn test_build_and_bindgen_ios_arm_sim() {
    test_artifact_build("aarch64-apple-ios-sim")
}

#[test]
fn test_build_and_bindgen_ios_intel_sim() {
    test_artifact_build("x86_64-apple-ios")
}

#[test]
fn test_build_and_bindgen_android_armv7() {
    test_artifact_build("armv7-linux-androideabi")
}

#[test]
fn test_build_and_bindgen_android_aarch64() {
    test_artifact_build("aarch64-linux-android")
}

#[test]
fn test_build_and_bindgen_android_x86() {
    test_artifact_build("i686-linux-android")
}

#[test]
fn test_build_and_bindgen_android_x86_64() {
    test_artifact_build("x86_64-linux-android")
}
