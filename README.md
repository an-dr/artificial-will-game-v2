# artificial-will-game-v2

A from-scratch reimplementation of
[artificial-will-game](https://github.com/an-dr/artificial-will-game) — a
game about a robot named Will overcoming obstacles — built on
[bones](https://github.com/an-dr/bones), a native engine core with game
behavior shipped as WASM extensions, instead of the original's hand-rolled
C++/SDL2/EnTT engine.

See [docs/architecture.md](docs/architecture.md) for the implemented system
boundaries and [AGENTS.md](AGENTS.md) for the AI-agent workflow this repo uses
([an-dr/agents](https://github.com/an-dr/agents)).

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
  wasip2`): independent menu, level, and character WASM components sharing one
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
generator, use the platform's compiler environment and set
`CMAKE_GENERATOR=Ninja` (or point it at another installed generator).

## Build & run

Plain `cargo` — no separate extension or asset-copy command:

```sh
cargo run --release
```

This builds `game` (embedding bones directly) *and*, via `build.rs`, every
extension in `extensions/` — one command. `src/paths.rs` finds them at
`extensions/target/wasm32-wasip2/release` relative to the exe (the dev-tree
layout); a shipped `dist/` build has grouped `extensions/core/` and
`extensions/levels/` directories next to the binary instead, and that
dedicated extension root is checked first.

## Controls

- Use the mouse or the arrow/WASD keys to navigate menus. Enter or Space
  activates the selected item.
- Press Escape during play to pause; press it again to resume.
- Move with WASD or the arrow keys at 160 pixels per second on each axis.
- Press Space for one facing-dependent melee swing. A swing affects only the
  nearest box or slime in its forward lane.

Will keeps moving during an attack, but facing remains fixed until its two
frames finish. Will starts each session with three lives and a short damage
invulnerability window. The game HUD shows lives, level, total XP, current
level progress, and coins; progression resets when the level session restarts
or changes. Walk and idle animations select down, up, or side sheets; the
right-facing side sheet is mirrored.

Level One preserves the original grass map and pushable boxes. Each box breaks
in one hit and contains a deterministic coin reward. Level Two is an overgrown
ruin with mixed grass and broken-stone ground, illustrated fixed rock
obstacles, and six animated slimes. Nearby slimes pursue Will, deal contact
damage, take two hits, and award one XP on defeat. Every three XP increases
Will's displayed level. Losing the last life transactionally restarts the
current level.

## Testing a self-contained build

`cargo run` is enough for day-to-day iteration — it never needs `dist/`.
Use `cargo xtask dist` when you actually need a standalone folder: handing
a build to someone else, or verifying the shipped-layout path resolution
itself (`extensions/` next to the exe, not the dev-tree fallback)
actually works, rather than trusting the dev-tree fallback path:

```sh
cargo xtask dist
```

A real Rust program (`xtask/`, not a script) that builds `game` and every
current extension target, then assembles `dist/artificial-will(.exe)`,
`dist/extensions/core/*.wasm`, and `dist/extensions/levels/*.wasm` in one
command — copy `dist/` anywhere and run it as-is. The current contents are
`extensions/core/menu.wasm`, `extensions/core/will.wasm`,
`extensions/levels/level_one.wasm`, and `extensions/levels/level_two.wasm`. This
replaces manually rebuilding bones as an app and copying `.wasm` files into
place.
