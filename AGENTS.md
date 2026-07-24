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

- `bones/` and `agents/` are git submodules — do not edit their contents
  directly; changes belong upstream. `bones` itself vendors its own copy of
  `agents` (`bones/agents`) plus `bones/vendor/pubsub-bus` as nested
  submodules; run `git submodule update --init --recursive` after cloning.
- Game logic lives under `extensions/` as WASM components implementing
  `bones/wit/core.wit` — see `docs/architecture.md` for the planned module
  breakdown and `extensions/hello/` for the reference extension shape.
- Docs capture behavior and boundaries, not code, matching bones' own doc
  policy (`bones/docs/index.md`): `docs/architecture.md` stays at system
  altitude and should not need an update for an average refactor.
