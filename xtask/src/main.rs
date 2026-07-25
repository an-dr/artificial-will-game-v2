//! Repository build orchestration, written as a normal Rust binary instead
//! of a shell script (AGENTS.md: no custom build scripts in this repo) --
//! run via `cargo xtask <command>` (aliased in `.cargo/config.toml`).
//!
//! `dist`: builds the `game` package (root `Cargo.toml`; its own `build.rs`
//! builds every `extensions/` workspace member as a side effect), then
//! assembles a self-contained `dist/` -- the game binary plus each
//! extension's `.wasm` in its metadata-selected `extensions/core/` or
//! `extensions/levels/` directory, ready to copy anywhere and run as-is.

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

fn dist_group<'a>(package: &'a serde_json::Value, package_name: &str) -> Result<&'a str, String> {
    let group = package["metadata"]["artificial-will"]["dist-group"]
        .as_str()
        .ok_or_else(|| format!("extension package {package_name} has no dist-group"))?;
    if !matches!(group, "core" | "levels") {
        return Err(format!(
            "extension package {package_name} has unknown dist-group {group}"
        ));
    }
    Ok(group)
}

fn extension_artifacts(extensions_dir: &Path) -> Result<Vec<(String, String)>, String> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(extensions_dir)
        .output()
        .map_err(|err| format!("running cargo metadata: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "cargo metadata for extensions failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).map_err(|err| format!("parsing metadata: {err}"))?;
    let workspace_members = metadata["workspace_members"]
        .as_array()
        .ok_or("cargo metadata omitted workspace_members")?;
    let packages = metadata["packages"]
        .as_array()
        .ok_or("cargo metadata omitted packages")?;

    let mut artifacts = Vec::new();
    for package in packages {
        if !workspace_members.contains(&package["id"]) {
            continue;
        }
        let package_name = package["name"]
            .as_str()
            .ok_or("cargo metadata package omitted name")?;
        let targets = package["targets"]
            .as_array()
            .ok_or("cargo metadata package omitted targets")?;
        for target in targets {
            let crate_types = target["crate_types"]
                .as_array()
                .ok_or("cargo metadata target omitted crate_types")?;
            if !crate_types.iter().any(|kind| kind == "cdylib") {
                continue;
            }
            let group = dist_group(package, package_name)?;
            let name = target["name"]
                .as_str()
                .ok_or("cargo metadata target omitted name")?;
            artifacts.push((group.to_owned(), format!("{name}.wasm")));
        }
    }
    artifacts.sort();
    artifacts.dedup();
    if artifacts.is_empty() {
        return Err("extension workspace contains no cdylib targets".to_owned());
    }
    Ok(artifacts)
}

fn dist() -> Result<(), String> {
    let root = repo_root();

    // Triggers `build.rs`, which builds every `extensions/` workspace
    // member as a side effect -- see this file's top doc comment.
    run_in(&root, "cargo", &["build", "--release"])?;

    let dist_dir = root.join("dist");
    let _ = std::fs::remove_dir_all(&dist_dir);
    std::fs::create_dir_all(&dist_dir)
        .map_err(|err| format!("creating {}: {err}", dist_dir.display()))?;

    let exe_name = if cfg!(windows) {
        "artificial-will.exe"
    } else {
        "artificial-will"
    };
    let exe_src = root.join("target/release").join(exe_name);
    let exe_dst = dist_dir.join(exe_name);
    std::fs::copy(&exe_src, &exe_dst).map_err(|err| {
        format!(
            "copying {} to {}: {err}",
            exe_src.display(),
            exe_dst.display()
        )
    })?;

    let extensions_dir = root.join("extensions");
    let ext_release = extensions_dir.join("target/wasm32-wasip2/release");
    for (group, artifact_name) in extension_artifacts(&extensions_dir)? {
        let path = ext_release.join(&artifact_name);
        let group_dir = dist_dir.join("extensions").join(group);
        std::fs::create_dir_all(&group_dir)
            .map_err(|err| format!("creating {}: {err}", group_dir.display()))?;
        std::fs::copy(&path, group_dir.join(&artifact_name))
            .map_err(|err| format!("copying {}: {err}", path.display()))?;
    }

    println!();
    println!("Distribution ready: {}", exe_dst.display());
    Ok(())
}

#[cfg(test)]
mod tests;
