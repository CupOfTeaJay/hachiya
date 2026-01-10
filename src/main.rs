/*
    Fuyu
    Copyright (c) 2026-2026 whoami™ LLC

    TODO: document.
*/

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use dylib::DynamicLibrary;
use glob::glob;

fn collect(mods: &Path) -> Vec<PathBuf> {
    println!("collecting mods");
    let mut dylibs: Vec<PathBuf> = Vec::new();
    let binding = mods.join("target/debug/*.so");
    let pattern: &str = binding.to_str().unwrap();
    for item in glob(pattern).expect("glob failed") {
        match item {
            Ok(dylib) => dylibs.push(dylib),
            Err(e) => println!("{:?}", e),
        }
    }
    dylibs
}

fn compile(mods: &Path) {
    println!("compiling mods");
    let output = Command::new("cargo")
        .args([
            "build",
            "--manifest-path",
            mods.join("Cargo.toml").to_str().unwrap(),
        ])
        .output()
        .unwrap();
    // println!("status: {}", output.status);
    let _ = io::stdout().write_all(&output.stdout);
    let _ = io::stderr().write_all(&output.stderr);
}

fn main() {
    let mods: &Path = Path::new("mods");
    compile(mods);
    for dylib in collect(mods) {
        let library = DynamicLibrary::open(Some(&dylib)).unwrap();
        let hello: fn() = unsafe {
            std::mem::transmute(library.symbol::<usize>("hello").unwrap())
        };
        hello();
    }
}
