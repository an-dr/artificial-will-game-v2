# artificial-will-game-v2

A from-scratch reimplementation of
[artificial-will-game](https://github.com/an-dr/artificial-will-game) — a
game about a robot named Will overcoming obstacles — built on
[bones](https://github.com/an-dr/bones), a native engine core with game
behavior shipped as WASM extensions, instead of the original's hand-rolled
C++/SDL2/EnTT engine.

See [docs/architecture.md](docs/architecture.md) for the reimplementation
plan (draft) and [AGENTS.md](AGENTS.md) for the AI-agent workflow this repo
uses ([an-dr/agents](https://github.com/an-dr/agents)).

## Repository layout

- `agents/` — the AI-agent workflow policy, a git submodule (root, same as
  `bones` embeds it in its own repo).
- `vendor/bones/` — the engine, a git submodule. We can and do patch it —
  changes belong upstream, committed inside that checkout and pushed
  deliberately, never silently drifting from `an-dr/bones`.
- `Cargo.toml` / `src/` — the game itself, package `game`, binary
  `artificial-will`. Embeds `bones` directly as path dependencies
  (`runner`, `game-core`), the same pattern `vendor/bones/embedding-demo`
  documents, rather than running bones' own generic `app` binary. Its own
  `[workspace]` excludes `vendor/bones` (see the comment in `Cargo.toml` —
  required because `vendor/bones/shared/bones-messages` is itself a
  self-isolated workspace nested inside this repo).
- `build.rs` — builds the `extensions/` workspace as part of `cargo build`,
  so there's no separate manual step for day-to-day iteration.
- `extensions/` — a separate Cargo workspace (different target: `wasm32-
  wasip2`): one crate per WASM extension (game logic), sharing one
  `target/` so every extension's `.wasm` lands in one predictable directory.
- `xtask/` — a small Rust program (not a shell script) that assembles a
  self-contained `dist/`: run via `cargo xtask dist` (aliased in
  `.cargo/config.toml`).

## Setup

```sh
git clone --recurse-submodules <this-repo-url>
# or, if already cloned:
git submodule update --init --recursive
```

Requires Rust (`rustup target add wasm32-wasip2`) and, for `vendor/bones`
itself, a C toolchain + CMake (it builds SDL3 from source) — see
[vendor/bones' own README](vendor/bones/README.md). If cmake can't find a
generator, set `CMAKE_GENERATOR=Ninja` (or point it at whatever compiler/
generator you have).

## Build & run

Plain `cargo` — no build scripts for day-to-day work:

```sh
cargo run --release
```

This builds `game` (embedding bones directly) *and*, via `build.rs`, every
extension in `extensions/` — one command. `src/paths.rs` finds them at
`extensions/target/wasm32-wasip2/release` relative to the exe (the dev-tree
layout); a shipped `dist/` build has `extensions/` sitting right next to
the binary instead, and that's checked first.

## Testing a self-contained build

`cargo run` is enough for day-to-day iteration — it never needs `dist/`.
Use `cargo xtask dist` when you actually need a standalone folder: handing
a build to someone else, or verifying the shipped-layout path resolution
itself (`extensions/` next to the exe, not the dev-tree fallback) actually
works, rather than trusting the dev-tree fallback path:

```sh
cargo xtask dist
```

A real Rust program (`xtask/`, not a script) that builds `game` and every
extension, then assembles `dist/artificial-will(.exe)` +
`dist/extensions/*.wasm` in one command — copy `dist/` anywhere and run it
as-is. This replaces manually rebuilding bones as an app and copying
`.wasm` files into some folder by hand every time you want to test that.

## Hello world

[extensions/hello](extensions/hello) is a minimal WASM extension proving the
engine + toolchain work end to end, before any real game logic exists — see
its own README for what it does.
