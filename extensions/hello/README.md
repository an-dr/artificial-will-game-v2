# hello

First WASM extension for Artificial Will's v2 engine, built against the
`bones` engine (vendored as a git submodule at repo root). Exercises the
full contract (`bones/wit/core.wit`): subscribes to `core/tick` in `init`,
logs on every `init`, `on-tick`, and `on-message`, and publishes a
`will/received` envelope for every message it gets.

Not the game itself — a smoke test proving the engine, the extension
toolchain, and the run loop all work end to end before any game logic
gets built on top. See [../../docs/architecture.md](../../docs/architecture.md)
for the planned shape of the actual game.

## Build

```sh
pwsh build.ps1
```

Requires PowerShell 7+ (`pwsh`) — cross-platform, so this is the only build
script needed on any OS. It runs `rustup target add wasm32-wasip2` (safe to
repeat), builds this extension, then builds the `bones` engine from the
submodule and assembles a runnable `dist/` next to this README —
`dist/bones(.exe)` with `dist/extensions/hello.wasm` already in place.

Building directly with `cargo build` (no `--target`) does not error — it
silently compiles a native `hello.dll`/`.so` instead, since this crate has no
other way to know which platform you meant. Always go through the script.

## Run

```sh
dist/bones
```
