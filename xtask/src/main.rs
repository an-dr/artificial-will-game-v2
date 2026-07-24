//! Repository build orchestration, written as a normal Rust binary instead
//! of a shell script (AGENTS.md: no custom build scripts in this repo) --
//! run via `cargo xtask <command>` (aliased in `.cargo/config.toml`).
//!
//! `dist`: builds the `game` package (root `Cargo.toml`; its own `build.rs`
//! builds every `extensions/` workspace member as a side effect), then
//! assembles a self-contained `dist/` -- the game binary plus every
//! extension's `.wasm` in `dist/extensions/`, ready to copy anywhere and
//! run as-is (`src/paths.rs` looks for `extensions/` next to the exe
//! first, which is exactly this layout).

use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let command = std::env::args().nth(1);
    let result = match command.as_deref() {
        Some("dist") | None => dist(),
        Some(other) => Err(format!("unknown xtask command: {other}")),
    };
    if let Err(err) = result {
        eprintln!("fatal: {err}");
        std::process::exit(1);
    }
}

fn repo_root() -> PathBuf {
    // xtask's own manifest dir is always `<repo_root>/xtask`.
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask has a parent directory")
        .to_path_buf()
}

fn run_in(dir: &Path, program: &str, args: &[&str]) -> Result<(), String> {
    println!("==> {program} {} (in {})", args.join(" "), dir.display());
    let status = Command::new(program)
        .args(args)
        .current_dir(dir)
        .status()
        .map_err(|err| format!("failed to run {program}: {err}"))?;
    if !status.success() {
        return Err(format!("{program} {} failed", args.join(" ")));
    }
    Ok(())
}

fn dist() -> Result<(), String> {
    let root = repo_root();

    // Triggers `build.rs`, which builds every `extensions/` workspace
    // member as a side effect -- see this file's top doc comment.
    run_in(&root, "cargo", &["build", "--release"])?;

    let dist_dir = root.join("dist");
    let _ = std::fs::remove_dir_all(&dist_dir);
    let dist_extensions = dist_dir.join("extensions");
    std::fs::create_dir_all(&dist_extensions)
        .map_err(|err| format!("creating {}: {err}", dist_extensions.display()))?;

    let exe_name = if cfg!(windows) {
        "artificial-will.exe"
    } else {
        "artificial-will"
    };
    let exe_src = root.join("target/release").join(exe_name);
    let exe_dst = dist_dir.join(exe_name);
    std::fs::copy(&exe_src, &exe_dst)
        .map_err(|err| format!("copying {} to {}: {err}", exe_src.display(), exe_dst.display()))?;

    let ext_release = root.join("extensions/target/wasm32-wasip2/release");
    let entries = std::fs::read_dir(&ext_release)
        .map_err(|err| format!("reading {}: {err}", ext_release.display()))?;
    for entry in entries {
        let entry = entry.map_err(|err| format!("reading dir entry: {err}"))?;
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "wasm") {
            continue;
        }
        let file_name = path.file_name().expect("wasm file has a name");
        std::fs::copy(&path, dist_extensions.join(file_name))
            .map_err(|err| format!("copying {}: {err}", path.display()))?;
    }

    println!();
    println!("Distribution ready: {}", exe_dst.display());
    Ok(())
}
