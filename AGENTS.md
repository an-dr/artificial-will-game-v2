# Agent Context

Notes for AI agents working on this repo that cannot be deduced from the code alone.

## Primary instructions

- Use the base agents policy ([an-dr/agents](https://github.com/an-dr/agents)),
  installed globally rather than vendored here, as the base instruction
- Use `AGENTS.md` in the repo root and in the subfolders as scoped extensions of the base rules
- Priority (later entries extend or overwrite earlier ones):
  1. the base agents policy
  2. `REPO/AGENTS.md` — this file
  3. `REPO/**/AGENTS.md` — any subdirectory AGENTS.md, chained by depth

## Game concept (read first)

[docs/concept/](docs/concept/README.md) is the game's concept document and the
most important document in this repository: features, levels, art, and
architecture grow from it. Read it before designing or building anything.
When a request or the code disagrees with it, say so instead of silently
diverging; open questions in it are the author's to decide, not ours.

## Project

This repo is a from-scratch reimplementation ("v2") of
[artificial-will-game](https://github.com/an-dr/artificial-will-game): a
2D game about a robot named Will overcoming obstacles. The original is a
C++/SDL2/EnTT engine; v2 is built on [bones](https://github.com/an-dr/bones)
instead — a native engine core (windows, input, rendering, audio, a message
bus) with game behavior shipped as WASM extensions.

## Conventions

- `vendor/bones/` is the only git submodule; `vendor/` is where a vendored
  code dependency belongs, the same convention `bones` uses for its own
  `vendor/pubsub-bus`. Run `git submodule update --init --recursive` after
  cloning.
- We *can* patch `vendor/bones` when it makes integration better (we own
  both repos) — commit those changes inside the `vendor/bones` checkout
  itself, on its own branch/history, never folded into a v2 commit. A
  bumped submodule pointer in v2 is a separate, deliberate commit.
- The game (`Cargo.toml` + `src/` at repo root, package `game`, binary
  `artificial-will`) embeds the bones kernel as one path dependency,
  `bones-kernel` (features `wasm`, `devtools`) — the
  `vendor/bones/examples/embedding/custom-engine` pattern — rather than
  running the stock `bones` binary + `bones.toml`. Its `[workspace]` must
  `exclude = ["vendor/bones"]`: `vendor/bones/crates/bones-messages`
  self-isolates as its own workspace (built for host and WASM guests), and
  Cargo treats an un-excluded ancestor workspace as a conflict ("multiple
  workspace roots found").
- Game logic lives under `extensions/` as WASM components built on
  `bones-wasm-sdk` (`vendor/bones/crates/bones-wasm-sdk`; the `game-ui`
  feature for the menu and HUD), all members of one separate Cargo workspace
  (`extensions/Cargo.toml`, target `wasm32-wasip2`) so every extension's
  `.wasm` lands in one predictable directory. Prefer the SDK's typed
  `bus::publish(&msg)` / `bus::subscribe::<M>()` over raw topic strings.
- No custom build scripts (PowerShell or otherwise) in this repo. Plain
  `cargo build`/`cargo run` is enough for day-to-day iteration — builds the
  game *and* extensions (root `build.rs` builds the `extensions/` workspace
  as a side effect), no `dist/` needed. `cargo xtask dist` (a real Rust
  program, `xtask/`) is only for when a standalone folder is actually
  needed — handing a build to someone, or testing the shipped-layout path
  resolution itself rather than the dev-tree fallback. Not a release/
  installer pipeline (bones has no recommendation there either — see
  ADR-011/`design/modules.md`: packaging is entirely the embedder's
  concern); reach for `cargo-dist` or similar if that's ever needed. See
  root `README.md`.
- Docs capture behavior and boundaries, not code, matching bones' own doc
  policy (`vendor/bones/docs/index.md`): `docs/architecture.md` stays at
  system altitude and should not need an update for an average refactor.
