//! Our own composition root, embedding bones directly as path dependencies
//! (`vendor/bones/docs/design/modules.md`'s "Embedding bones") rather than
//! running bones' own generic `app` binary + `bones.toml` -- we know
//! exactly which modules this game needs (renderer and game-core; no
//! audio yet) and bake that in as code, not runtime config. `runner` and
//! `game-core` below are `Cargo.toml` path dependencies; no `use` or
//! `extern crate` is needed to reach `runner::Engine` or
//! `game_core::GameCore` -- every dependency has been addressable by its
//! crate name directly since the 2018 edition.

mod paths;

fn main() {
    if let Err(err) = run() {
        eprintln!("fatal: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    runner::Engine::new()
        .window("Artificial Will", 800, 600)
        .extensions_dir(paths::extensions_dir())
        .startup_extension("menu")
        .extension_controller("menu")
        .renderer()
        .module(game_core::GameCore::new())
        .run()
        .map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests;
