//! Resolves `extensions_dir` against wherever this binary actually is, so
//! the same exe behaves identically whether it's the one `cargo build`
//! just produced in `target/{debug,release}/` (dev) or the one `cargo
//! xtask dist` copied into `dist/` (shipped): a shipped build has a dedicated
//! `extensions/` scan root next to the exe; a dev build doesn't (extensions
//! live in the separate `extensions/` workspace next door, two directories
//! up from `target/{debug,release}/`), so we fall back to that known dev-tree
//! layout when the shipped root isn't there. `saves_dir`
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
    resolve_extensions_dir(&exe_dir())
}

fn resolve_extensions_dir(exe_dir: &Path) -> PathBuf {
    let shipped_extensions = exe_dir.join("extensions");
    if shipped_extensions.is_dir() {
        return shipped_extensions;
    }
    exe_dir.join("../../extensions/target/wasm32-wasip2/release")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shipped_discovery_is_confined_to_the_extensions_root() {
        let root =
            std::env::temp_dir().join(format!("artificial-will-paths-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("extensions/core")).unwrap();
        std::fs::create_dir_all(root.join("saves")).unwrap();

        assert_eq!(resolve_extensions_dir(&root), root.join("extensions"));

        std::fs::remove_dir_all(root).unwrap();
    }
}
