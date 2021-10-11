//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
// use vergen::{vergen, Config, TimestampKind, TimeZone, SemverKind, ShaKind};

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");

    // let mut config = Config::default();
    // // Generate all three date/time instructions
    // *config.build_mut().kind_mut() = TimestampKind::All;
    // // Change the date/time instructions to show `Local` time
    // *config.build_mut().timezone_mut() = TimeZone::Local;
    //
    // // Change the SHA output to the short variant
    // *config.git_mut().sha_kind_mut() = ShaKind::Short;
    // // Change the SEMVER output to the lightweight variant
    // *config.git_mut().semver_kind_mut() = SemverKind::Lightweight;
    // // Add a `-dirty` flag to the SEMVER output
    // *config.git_mut().semver_dirty_mut() = Some("-dirty");
}
