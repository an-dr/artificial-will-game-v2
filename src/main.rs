//! Our own composition root, embedding the bones kernel directly rather
//! than running the stock `bones` binary with a `bones.toml`: we know which
//! modules this game needs and bake them in as code.

mod paths;

fn main() {
    if let Err(err) = run() {
        eprintln!("fatal: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let kernel = bones_kernel::Kernel::new().window("Artificial Will", 800, 600);
    paths::with_extensions(kernel)
        // Only the menu starts; it loads levels and Will on demand. Also
        // required: without it the window opens blank, so a missing or
        // failing menu.wasm fails the build instead.
        .startup_extension("menu")
        .required_extension("menu")
        .extension_controller("menu")
        .renderer()
        .module(bones_kernel::game_core::GameCore::new())
        // BONES_OFFSCREEN renders into memory (headless screenshots);
        // BONES_DEVTOOLS_PORT opens the devtools channel for inspection.
        .offscreen_from_env("BONES_OFFSCREEN")
        .devtools_from_env("BONES_DEVTOOLS_PORT")
        .run()
        .map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests;
