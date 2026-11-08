//! Resolves `extensions_dir` against wherever this binary actually is, so
//! the same exe behaves identically whether it's the one `cargo build`
//! just produced in `target/{debug,release}/` (dev) or the one `cargo
//! xtask dist` copied into `dist/` (shipped): a shipped build has a dedicated
//! `extensions/` scan root next to the exe; a dev build doesn't (extensions
//! live in the separate `extensions/` workspace next door, two directories
//! up from `target/{debug,release}/`), so we fall back to that known dev-tree
//! layout when the shipped root isn't there. `saves_dir`
//! needs no equivalent here -- `bones_kernel::Kernel`'s own default already
//! resolves a relative path against the exe's directory the same way.

use std::path::{Path, PathBuf};

fn exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(Path::to_path_buf))
        .expect("running executable has a parent directory")
}

/// Points `kernel` at this build's extensions. A shipped build scans its
/// `extensions/` folder. The dev tree is Cargo's output directory, whose
/// `deps/` holds a second copy of every `.wasm`; bones scans recursively, so
/// there each top-level component is cataloged by name instead.
pub fn with_extensions(kernel: bones_kernel::Kernel) -> bones_kernel::Kernel {
    match resolve_extensions_dir(&exe_dir()) {
        Extensions::Shipped(dir) => kernel.extensions_dir(dir),
        Extensions::Dev(dir) => catalog_dir(kernel, &dir),
    }
}

/// Catalogs every `<name>.wasm` directly inside `dir`, not its subfolders.
pub fn catalog_dir(mut kernel: bones_kernel::Kernel, dir: &Path) -> bones_kernel::Kernel {
    let entries = std::fs::read_dir(dir).into_iter().flatten().flatten();
    for path in entries.map(|entry| entry.path()) {
        if path.extension().is_some_and(|ext| ext == "wasm") {
            if let Some(name) = path.file_stem().and_then(|stem| stem.to_str()) {
                kernel = kernel.catalog_extension(name.to_owned(), path.clone());
            }
        }
    }
    kernel
}

#[derive(Debug, PartialEq)]
enum Extensions {
    Shipped(PathBuf),
    Dev(PathBuf),
}

fn resolve_extensions_dir(exe_dir: &Path) -> Extensions {
    let shipped_extensions = exe_dir.join("extensions");
    if shipped_extensions.is_dir() {
        return Extensions::Shipped(shipped_extensions);
    }
    Extensions::Dev(exe_dir.join("../../extensions/target/wasm32-wasip2/release"))
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

        assert_eq!(
            resolve_extensions_dir(&root),
            Extensions::Shipped(root.join("extensions"))
        );

        std::fs::remove_dir_all(root).unwrap();
    }
}
