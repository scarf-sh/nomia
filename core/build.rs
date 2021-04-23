use anyhow::Result;
use std::fs::remove_file;
use std::process::Command;

fn main() -> Result<()> {
    // re-run if the grammar changes
    println!("cargo:rerun-if-changed=name.cf");

    // Generate C code from the grammar
    let status = Command::new("bnfc")
        .args(&["--outputdir=gen", "--c", "name.cf"])
        .status()?;

    assert!(status.success());

    remove_file("gen/Test.c")?;
    remove_file("gen/Skeleton.c")?;
    remove_file("gen/Skeleton.h")?;

    // Build the C code for the parser
    let status = Command::new("make").current_dir("gen").status()?;
    assert!(status.success());

    Ok(())
}
