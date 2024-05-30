//! Helper Cargo package for compiling [libpng](https://github.com/pnggroup/libpng) into a static C library.
//!
//! Meant to be used as build dependency for dufferent `-sys` or `-vendored` packages.
//! Does not provide directly usable **libpng** functionality or bindings.
//!
//! Expected to work for:
//! - Linux: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu` (no cross-compilation supported yet)
//! - Windows: `x86_64-pc-windows-msvc`, `aarch644-pc-windows-msvc` (no cross-compilation supported yet)
//! - macOS: `x86_64-apple-darwin`, `aarch64-apple-darwin`
//! - iOS, including simulators (cross-compilation from macOS host): `x86_64-apple-ios`, `aarch64-apple-ios`, `aarch64-apple-ios-sim`
//! - Android (cross-compilation from Linux, macOS or Windows hosts): `armv7-linux-androideabi`, `aarch64-linux-android`,
//! `i686-linux-android`, `x86_64-linux-android`

use std::{
    env::consts::{ARCH as HOST_ARCH, OS as HOST_OS},
    error::Error,
    ffi::OsString,
    fs::{self, copy, create_dir, create_dir_all, remove_dir_all},
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
    vec::Vec,
};

/// Version of the **libpng** library
pub const LIBPNG_VERSION: &str = "1.6.43";

/// Represents result of complete building.
pub struct Artifacts {
    /// Artifacts root directory, see [build_all_artifacts](build_all_artifacts) for explanantion.
    pub root_dir: PathBuf,
    /// C headers directory, see [build_all_artifacts](build_all_artifacts) for explanantion.
    pub include_dir: PathBuf,
    /// Library search directory, see [build_all_artifacts](build_all_artifacts) for explanantion.
    pub lib_dir: PathBuf,
    /// Library name for linker.
    pub link_name: String,
}

/// Returns the path to the source directory without any modifications.
///
/// Use it to generate bindings to the **libpng** if needed.
/// The directory does not contain 'pnglibconf.h', generated at build time.
pub fn source_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("libpng")
}

/// Builds all artifacts and aggregates library and include headers in a directory.
/// Would create working directory if missing.
/// Would remove previous content of 'build/' and 'libpng/' subdirectories if not empty (see below).
///
/// # Example
/// ```ignore
/// // 'build.rs' of an another crate
/// use std::{env::var, path::PathBuf};
///
/// use libpng_src::build_artifact;
///
/// fn main() {
///     let target = var("TARGET").unwrap();
///     let out_dir = var("OUT_DIR").map(PathBuf::from).unwrap();
///
///     let artifact_info = build_artifact(&target, &out_dir)
///         .unwrap();
///
///     println!("cargo:rustc-link-search=native={}", artifact_info.lib_dir.to_string_lossy());
///     println!("cargo:rustc-link-lib=static={}", artifact_info.link_name);
/// }
/// ```
///
/// # Example with bindgen
/// ```ignore
/// use std::{env::var, path::PathBuf};
/// // 'build.rs' of an another crate
///
/// use bindgen;
///
/// use libpng_src::build_artifact;
///
/// fn main() {
///     let target = var("TARGET").unwrap();
///     let out_dir = var("OUT_DIR").map(PathBuf::from).unwrap();
///
///     let artifact_info = build_artifact(&target, &out_dir)
///         .unwrap();
///
///     println!("cargo:rustc-link-search=native={}", artifact_info.lib_dir.to_string_lossy());
///     println!("cargo:rustc-link-lib=static={}", artifact_info.link_name);
///
///     let main_header_path = artifact_info.include_dir.join("png.h");
///
///     bindgen::builder()
///         .header(main_header_path.to_string_lossy())
///         .allowlist_file(main_header_path.to_string_lossy())
///         .generate()
///         .unwrap()
///         .write_to_file(out_dir.join("bindings.rs"))
///         .unwrap()
/// }
/// ```
///
/// # File structure
/// ```text
/// working_directory/
///     |->build/  ... Temporary build directory - do not use directly.
///     └->libpng/ ... Artifact root directory.
///         |->include/ ... C include headers - generate FFI bindings.
///         └->lib/ ... Static library - add to link search path.
/// ```
pub fn build_artifact(target_str: &str, working_dir: &Path) -> Result<Artifacts, Box<dyn Error>> {
    let build_dir = working_dir.join("build");

    let library_path = compile_lib(target_str, &build_dir)?;
    let library_filename = library_path
        .file_name()
        .map(|os| os.to_string_lossy())
        .map(String::from)
        .unwrap();

    let root_dir = working_dir.join("libpng");

    if root_dir.exists() {
        remove_dir_all(&root_dir)?;
    }

    create_dir_all(&root_dir)?;

    let include_dir = root_dir.join("include");

    create_dir(&include_dir)?;
    copy(source_path().join("png.h"), include_dir.join("png.h"))?;
    copy(
        source_path().join("pngconf.h"),
        include_dir.join("pngconf.h"),
    )?;
    copy(
        build_dir.join("pnglibconf.h"),
        include_dir.join("pnglibconf.h"),
    )?;

    let lib_dir = root_dir.join("lib");

    create_dir_all(&lib_dir)?;
    copy(library_path, lib_dir.join(&library_filename))?;
    // Cleanup
    remove_dir_all(build_dir)
        .unwrap_or_else(|_| println!("'libpng-src' cannot clean build directoey"));

    Ok(Artifacts {
        root_dir,
        include_dir,
        lib_dir,
        link_name: link_name(library_filename, target_str),
    })
}

/// Statically compiles **libpng** library and returns the path to the compiled artifact.
/// Should be used when include headers are not needed.
/// Would create working directory if missing, would remove its previous content if not empty.
/// # Usage Example
/// ```ignore
/// /// 'build.rs' of a consumer crate
/// use std::{env::var, fs::copy, path::PathBuf};
///
/// use libpng_src;
///
/// fn main() {
///     let target = var("TARGET").unwrap();
///     let out_dir = var("OUT_DIR").map(PathBuf::from).unwrap();
///     
///     let lib_path = libpng_src::compile_lib(&target, &out_dir).unwrap();
///
///     println!("cargo:rustc-link-search=native={}", lib_path.parent().unwrap().to_string_lossy());
///     #[cfg(not(target_os = "windows"))]
///     println!("cargo:rustc-link-lib=static=png16");
///     #[cfg(target_os = "windows")]
///     println!("cargo:rustc-link-lib=static=png16_static");
/// }
/// ```
pub fn compile_lib(target_str: &str, working_dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    if !allowed_targets_for_host().contains(&target_str) {
        return Err(format!(
            "Unsupported target: {target_str}, for host OS: {HOST_OS}, arch: {HOST_ARCH}"
        )
        .into());
    }

    if working_dir.exists() {
        fs::remove_dir_all(working_dir)?;
    }
    fs::create_dir_all(working_dir)?;

    let source_path = source_path();

    let mut cmake_args = cmake_options(target_str)?;
    cmake_args.push(source_path.into_os_string());

    execute("cmake", &cmake_args, working_dir)?;
    execute(
        "cmake",
        &["--build", ".", "--config", "Release"].map(OsString::from),
        working_dir,
    )?;

    artifact_path(working_dir)
}

trait TryIntoVecOsString<T, E> {
    type Error;

    fn try_into_os_string(self) -> Result<Vec<OsString>, Self::Error>;
}

impl TryIntoVecOsString<Vec<&str>, Box<dyn Error>> for Vec<&str> {
    type Error = Box<dyn Error>;

    fn try_into_os_string(self) -> Result<Vec<OsString>, Self::Error> {
        let mut result_vec: Vec<OsString> = Vec::new();

        for str_e in self {
            result_vec.push(OsString::from_str(str_e)?);
        }

        Ok(result_vec)
    }
}

fn allowed_targets_for_host() -> Vec<&'static str> {
    match (HOST_OS, HOST_ARCH) {
        ("macos", _) => [
            vec![
                "aarch64-apple-darwin",
                "x86_64-apple-darwin",
                "aarch64-apple-ios",
                "aarch64-apple-ios-sim",
                "x86_64-apple-ios",
            ],
            androd_targets(),
        ]
        .concat(),
        ("linux", "x86_64") => [vec!["x86_64-unknown-linux-gnu"], androd_targets()].concat(),
        ("linux", "aarch64") => vec!["aarch64-unknown-linux-gnu"],
        ("windows", "x86_64") => [vec!["x86_64-pc-windows-msvc"], androd_targets()].concat(),
        ("windows", "aarch64") => vec!["aarch64-pc-windows-msvc"],
        _ => vec![],
    }
}

fn androd_targets() -> Vec<&'static str> {
    vec![
        "aarch64-linux-android",
        "armv7-linux-androideabi",
        "x86_64-linux-android",
        "i686-linux-android",
    ]
}

fn cmake_options(target_str: &str) -> Result<Vec<OsString>, Box<dyn Error>> {
    let mut options = common_cmake_options();
    options.append(&mut target_specific_cmake_options(target_str)?);

    Ok(options)
}

fn common_cmake_options() -> Vec<OsString> {
    vec![
        OsString::from("-DPNG_SHARED=OFF"),
        OsString::from("-DPNG_TESTS=OFF"),
    ]
}

fn target_specific_cmake_options(target_str: &str) -> Result<Vec<OsString>, Box<dyn Error>> {
    if target_str.contains("apple") {
        return apple_specific_cmake_options(target_str);
    }

    if target_str.contains("android") {
        return androdid_specific_cmake_options(target_str, HOST_OS);
    }

    if target_str.contains("windows") {
        return windows_specific_cmake_options();
    }

    // Linux
    Ok(vec![])
}

fn apple_specific_cmake_options(target_str: &str) -> Result<Vec<OsString>, Box<dyn Error>> {
    let rust_arch = target_str.split('-').next().unwrap();

    let cmake_arch = match rust_arch {
        "aarch64" => Ok("arm64"),
        "x86_64" => Ok("x86_64"),
        _ => Err(format!(
            "Unsupported target: {target_str}, for host OS: {HOST_OS}, arch: {HOST_ARCH}"
        )),
    }?;

    let arch_param_sting = format!("-DCMAKE_OSX_ARCHITECTURES={cmake_arch}");

    let mut param_vec = vec![arch_param_sting.as_str(), "-DPNG_FRAMEWORK=OFF"];

    // Compile for iOS, not macOS
    if target_str.contains("ios") {
        param_vec.push("-DCMAKE_SYSTEM_NAME=iOS");
    }

    // Compile for iOS sim
    if target_str == "aarch64-apple-ios-sim" || target_str == "x86_64-apple-ios" {
        param_vec.push("-DCMAKE_OSX_SYSROOT=iphonesimulator");
    }

    param_vec.try_into_os_string()
}

fn androdid_specific_cmake_options(
    target_str: &str,
    host_os_str: &str,
) -> Result<Vec<OsString>, Box<dyn Error>> {
    let build_arch = match target_str {
        "armv7-linux-androideabi" => Ok("armeabi-v7a"),
        "aarch64-linux-android" => Ok("arm64-v8a"),
        "i686-linux-android" => Ok("x86"),
        "x86_64-linux-android" => Ok("x86_64"),
        _ => Err(format!(
            "Unsupported target: {target_str}, for host OS: {HOST_OS}, arch: {HOST_ARCH}"
        )),
    }?;

    let arch_param_string = format!("-DCMAKE_ANDROID_ARCH_ABI={build_arch}");

    let mut param_vec = vec!["-DCMAKE_SYSTEM_NAME=Android", arch_param_string.as_str()];

    if host_os_str == "windows" {
        param_vec.push("-DCMAKE_TRY_COMPILE_TARGET_TYPE=STATIC_LIBRARY");
    }

    param_vec.try_into_os_string()
}

fn windows_specific_cmake_options() -> Result<Vec<OsString>, Box<dyn Error>> {
    let zlib_include_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("win-zlib-include");
    let zlib_lib_path = zlib_include_path.join("zlib.lib");

    let mut include_param = OsString::from("-DZLIB_INCLUDE_DIR=");
    include_param.push(zlib_include_path);

    let mut lib_param = OsString::from("-DZLIB_LIBRARY=");
    lib_param.push(zlib_lib_path);

    Ok(vec![include_param, lib_param])
}

fn execute(command: &str, args: &[OsString], cwd: &Path) -> Result<(), Box<dyn Error>> {
    let output = Command::new(command).current_dir(cwd).args(args).output()?;

    if !output.status.success() {
        Err(format!(
            "Command '{}' failed with status code {}\nError: {}",
            command,
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stderr)
        ))?;
    }

    let args_vec: Vec<&str> = args
        .iter()
        .map(|a| a.to_str().unwrap_or("!error!"))
        .collect();

    println!("Executed '{} {}' successfully", command, args_vec.join(" "));
    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

fn artifact_path(working_dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let filename = match HOST_OS {
        "windows" => "Release\\libpng16_static.lib",
        _ => "libpng16.a",
    };

    let artifact_path = working_dir.join(filename);

    if !artifact_path.exists() {
        return Err(format!("Artifact not found at path: {}", artifact_path.display()).into());
    }

    Ok(artifact_path)
}

fn link_name(file_name: String, target_str: &str) -> String {
    let mut file_name = file_name.split('.').next().unwrap();

    if !target_str.contains("windows") {
        file_name = file_name.trim_start_matches("lib");
    }

    file_name.to_string()
}

#[cfg(test)]
mod tests;
