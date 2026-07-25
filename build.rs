// Builds the `extensions/` workspace (a separate wasm32-wasip2 target, its
// own workspace/target dir -- see extensions/README.md) as part of a plain
// `cargo build`/`cargo run` of this package, so there is no second manual
// command for day-to-day iteration. Safe to recurse into cargo here
// specifically because it's a *different* workspace with its own target
// dir/lock -- unlike invoking cargo on your own package from its own
// build.rs, there is no shared-lock deadlock risk.

use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=extensions/Cargo.toml");
    println!("cargo:rerun-if-changed=extensions/Cargo.lock");
    println!("cargo:rerun-if-changed=extensions/level-one/Cargo.toml");
    println!("cargo:rerun-if-changed=extensions/level-one/src");
    println!("cargo:rerun-if-changed=extensions/level-one/assets");
    println!("cargo:rerun-if-changed=extensions/will/Cargo.toml");
    println!("cargo:rerun-if-changed=extensions/will/src");
    println!("cargo:rerun-if-changed=assets");

    let extensions_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("extensions");
    let status = Command::new("cargo")
        .args(["build", "--release", "--target", "wasm32-wasip2"])
        .current_dir(&extensions_dir)
        .status()
        .unwrap_or_else(|err| panic!("failed to run cargo for {}: {err}", extensions_dir.display()));
    assert!(
        status.success(),
        "building {} failed",
        extensions_dir.display()
    );
}
