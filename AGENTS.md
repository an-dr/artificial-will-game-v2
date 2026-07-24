# Agent Context

Notes for AI agents working on this repo that cannot be deduced from the code alone.

## Primary instructions

- Use `agents/AGENTS.md` as the base instruction
- Use `AGENTS.md` in the repo root and in the subfolders as scoped extensions of the base rules
- Priority (later entries extend or overwrite earlier ones):
  1. `REPO/agents/AGENTS.md` — base
  2. `REPO/AGENTS.md` — this file
  3. `REPO/**/AGENTS.md` — any subdirectory AGENTS.md, chained by depth

## Project

This repo is a from-scratch reimplementation ("v2") of
[artificial-will-game](https://github.com/an-dr/artificial-will-game): a
2D game about a robot named Will overcoming obstacles. The original is a
C++/SDL2/EnTT engine; v2 is built on [bones](https://github.com/an-dr/bones)
instead — a native engine core (windows, input, rendering, audio, a message
bus) with game behavior shipped as WASM extensions.

## Conventions

- `agents/` (root) and `vendor/bones/` are git submodules. `vendor/` is
  where a vendored code dependency belongs — the same convention `bones`
  itself uses for `vendor/pubsub-bus`, while `agents` (workflow tooling,
  not code) stays at root, again matching how `bones` embeds it. `bones`
  itself vendors nested submodules (`vendor/bones/agents`,
  `vendor/bones/vendor/pubsub-bus`); run
  `git submodule update --init --recursive` after cloning.
- We *can* patch `vendor/bones` when it makes integration better (we own
  both repos) — commit those changes inside the `vendor/bones` checkout
  itself, on its own branch/history, never folded into a v2 commit. A
  bumped submodule pointer in v2 is a separate, deliberate commit.
- Game logic lives under `extensions/` as WASM components implementing
  `vendor/bones/wit/core.wit`, all members of one Cargo workspace
  (`extensions/Cargo.toml`) so `cargo build --release --target
  wasm32-wasip2` from there is the only build step — no scripts, no copy.
  See `docs/architecture.md` for the planned module breakdown and
  `extensions/hello/` for the reference extension shape.
- No custom build scripts (PowerShell or otherwise) in this repo. Plain
  `cargo build`/`cargo run`, tied together by `bones.toml` +
  `BONES_CONFIG` (see root `README.md`).
- Docs capture behavior and boundaries, not code, matching bones' own doc
  policy (`vendor/bones/docs/index.md`): `docs/architecture.md` stays at
  system altitude and should not need an update for an average refactor.
