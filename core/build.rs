use anyhow::Result;
use bindgen;
use cc;
use std::env;
use std::fs::remove_file;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<()> {
    // re-run if the grammar changes
    println!("cargo:rerun-if-changed=name.cf");
    println!("cargo:rerun-if-changed=gen/Makefile");
    println!("cargo:rerun-if-changed=gen/bnfc.h");

    // Generate C code from the grammar
    let status = Command::new("bnfc")
        .args(&["--outputdir=gen", "--c", "name.cf"])
        .status()?;

    assert!(status.success());

    remove_file("gen/Test.c")?;
    remove_file("gen/Skeleton.c")?;
    remove_file("gen/Skeleton.h")?;

    // Generate the C from the bison and flex source.
    let status = Command::new("make").current_dir("gen").status()?;
    assert!(status.success());

    cc::Build::new()
        .include("gen")
        .file("gen/Absyn.c")
        .file("gen/Buffer.c")
        .file("gen/Lexer.c")
        .file("gen/Parser.c")
        .file("gen/Printer.c")
        .compile("parser");

    let bindings = bindgen::Builder::default()
        .header("gen/bnfc.h")
        .use_core()
        .ctypes_prefix("libc")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}
