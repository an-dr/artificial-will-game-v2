# hello

First WASM extension for Artificial Will's v2 engine, built against
[bones](../../vendor/bones) (vendored as a git submodule at
`vendor/bones`). Exercises the full contract (`vendor/bones/wit/core.wit`):
subscribes to `core/tick` in `init`, logs on every `init`, `on-tick`, and
`on-message`, and publishes a `will/received` envelope for every message it
gets.

Not the game itself — a smoke test proving the engine, the extension
toolchain, and the run loop all work end to end before any game logic
gets built on top. See [../../docs/architecture.md](../../docs/architecture.md)
for the planned shape of the actual game.

A member of the [extensions](..) workspace — see its README (and the repo
root README's "Build & run") for how it gets built and loaded. Building
this crate alone with `cargo build` (no `--target`) does not error — it
silently compiles a native `hello.dll`/`.so` instead, since a `cdylib`
crate has no other way to know which platform you meant. Always build via
the `extensions/` workspace with `--target wasm32-wasip2`.
