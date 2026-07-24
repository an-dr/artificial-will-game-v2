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
- `vendor/bones/` — the engine, a git submodule. We can and do patch it
  (see `vendor/bones/core/app/src/paths.rs`) — changes belong upstream,
  committed inside that checkout and pushed deliberately, never silently
  drifting from `an-dr/bones`.
- `extensions/` — our own Cargo workspace: one crate per WASM extension
  (game logic), sharing one `target/` so a single `cargo build` output is
  directly loadable — no per-extension build step or copy.
- `bones.toml` — dev-time engine config, read via `BONES_CONFIG` (below)
  instead of living next to the built exe.

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

Plain `cargo` — no build scripts. Two independent builds (native engine,
WASM extensions), one config file tying them together:

```sh
# 1. Build the engine (from the vendored bones checkout)
cargo build -p app --release --manifest-path vendor/bones/Cargo.toml

# 2. Build every extension (one shared workspace, one shared target/)
cargo build --release --target wasm32-wasip2 --manifest-path extensions/Cargo.toml

# 3. Run, pointed at our own bones.toml (see "Configuration" below)
BONES_CONFIG=$(pwd)/bones.toml ./vendor/bones/target/release/bones.exe
```

## Configuration

`bones.toml` at repo root sets `extensions_dir` to
`extensions/target/wasm32-wasip2/release` — the extensions workspace's own
cargo output, resolved relative to `bones.toml` itself once `BONES_CONFIG`
points at it (a small addition to `vendor/bones`, see its `app/README.md`
"Configuration" section). No file ever gets copied into place.

## Hello world

[extensions/hello](extensions/hello) is a minimal WASM extension proving the
engine + toolchain work end to end, before any real game logic exists — see
its own README for what it does.
