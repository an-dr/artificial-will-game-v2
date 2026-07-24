//! Resolves `extensions_dir` against wherever this binary actually is, so
//! the same exe behaves identically whether it's the one `cargo build`
//! just produced in `target/{debug,release}/` (dev) or the one `cargo
//! xtask dist` copied into `dist/` (shipped): a shipped build has
//! `extensions/` sitting right next to the exe; a dev build doesn't
//! (extensions live in the separate `extensions/` workspace next door, two
//! directories up from `target/{debug,release}/`), so we fall back to that
//! known dev-tree layout when the shipped one isn't there. `saves_dir`
//! needs no equivalent here -- `runner::Engine`'s own default already
//! resolves a relative path against the exe's directory the same way.

use std::path::{Path, PathBuf};

fn exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(Path::to_path_buf))
        .expect("running executable has a parent directory")
}

pub fn extensions_dir() -> PathBuf {
    let exe_dir = exe_dir();
    let shipped = exe_dir.join("extensions");
    if shipped.is_dir() {
        return shipped;
    }
    exe_dir.join("../../extensions/target/wasm32-wasip2/release")
}
