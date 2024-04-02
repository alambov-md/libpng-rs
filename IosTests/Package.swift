// swift-tools-version: 5.7.1
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

#if arch(arm64)
let librarySearchPath = "../target/aarch64-apple-ios-sim/debug"
#else
let librarySearchPath = "../target/x86_64-apple-ios/debug"
#endif

let package = Package(
    name: "IosTests",
    platforms: [.iOS(.v15)],
    products: [
        .library(
            name: "Binding",
            targets: ["Binding"]),
    ],
    targets: [
        .target(
            name: "Binding",
            linkerSettings: [
                .unsafeFlags(["-L", librarySearchPath]),
                .linkedLibrary("mobile_test_helper")]),
        .target(
            name: "ImageProvider",
            resources: [.copy("Resources/test_image.png")]),
        .testTarget(
            name: "IosTests",
            dependencies: ["Binding", "ImageProvider"]),
    ]
)
