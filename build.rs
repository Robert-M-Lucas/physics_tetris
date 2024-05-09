

// extern crate fs_extra;
// use fs_extra::dir::{CopyOptions};
use std::env;
use std::path::PathBuf;
use std::fs;
use fs_extra::dir::{copy, CopyOptions};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rustc-link-arg-bins=/STACK:100000000");
    let binding = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let target_dir = binding.parent().unwrap().parent().unwrap().parent().unwrap();
    let mut options = CopyOptions::new();
    options.overwrite = true;
    copy("static", &target_dir, &options).unwrap();
    let files = [
        // "SDL2.lib",
        "SDL2.dll",
        // "SDL2_gfx.lib",
        "SDL2_gfx.dll",
        // "SDL2_mixer.lib",
        "SDL2_mixer.dll",
    ];

    for f in files {
        fs::copy(f, target_dir.join(f)).unwrap();
    }
}