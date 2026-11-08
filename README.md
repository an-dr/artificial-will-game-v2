# artificial-will-game-v2

[![AI Level 3](https://ai-level.dev/badge/standard/3.svg)](https://ai-level.dev/level-3)

A 2D top-down game about Will, a TV-head robot, fighting his way through grass fields and overgrown ruins and wandering a peaceful harbour town.

![Will on the white-stone plaza of White Harbour, with the Merchant and Gardener robots, flower pots, houses, and trees around him](docs/README/white-harbour.png)

## Why

This is a from-scratch reimplementation of [artificial-will-game](https://github.com/an-dr/artificial-will-game). The original runs on a hand-rolled C++/SDL2/EnTT engine; v2 runs on [bones](https://github.com/an-dr/bones), a native engine core that loads all game behavior as WASM extensions.

## Installation

```sh
git clone --recurse-submodules https://github.com/an-dr/artificial-will-game-v2
cd artificial-will-game-v2
rustup target add wasm32-wasip2
```

If the repository is already cloned, fetch the submodules with `git submodule update --init --recursive`.

Bones builds SDL3 from source, so it also needs a C toolchain and CMake; see [the bones README](vendor/bones/README.md). If CMake cannot find a generator, use the platform's compiler environment and set `CMAKE_GENERATOR=Ninja`, or point it at another installed generator.

## Quickstart

```sh
cargo run --release
```

One command builds the game and, through [build.rs](build.rs), every extension in `extensions/`. Pick **Start** in the menu to play.

## Controls

- Mouse or arrow keys/WASD navigate menus; Enter or Space activates the selected item.
- WASD or arrow keys move Will at 160 pixels per second on each axis.
- Space swings once in the facing direction and hits only the nearest box or slime in that lane.
- Escape pauses play; press it again to resume.

## Features

- Will has idle, walk, attack, and hurt animations for each of four directions, all from one sprite sheet. He keeps moving during an attack, but his facing stays fixed until its four frames finish.
- Three lives per session, a short invulnerability window after damage, and a HUD with lives, level, total XP, level progress, and coins.
- Level One keeps the original grass map with pushable boxes; each box breaks in one hit and drops a fixed coin reward.
- Level Two is an overgrown ruin with broken-stone ground, rock obstacles, and six slimes. Slimes chase Will inside their awareness radius, stop short of contact, and deal damage only when their visible attack reaches its strike frame. The wind-up can be escaped or interrupted; touching a slime alone is harmless.
- Slimes take two hits and give one XP each; every three XP raises Will's level. Progression resets when the level session restarts or changes.
- White Harbour is a coastal level with no enemies: sand, a promenade plaza, sea, white houses, a lighthouse, a pier, and an animated spawn pad. Four robot residents (Merchant, Gardener, Radio, Scrap) stand in Will's way; six translucent spirit visitors, people and animals, let him walk through them.
- Losing the last life pauses the world on a game-over screen; Enter returns to the main screen.

## Configuration

The game reads two environment variables, both for inspecting or automating it:

- `BONES_OFFSCREEN=1` renders into memory with no visible window.
- `BONES_DEVTOOLS_PORT=<port>` opens the bones devtools channel on `127.0.0.1:<port>`: recorded messages, logs, `GET /screenshot`, `GET /frame`, pausing and stepping, and `POST /publish` to inject input. See [the devtools README](vendor/bones/crates/bones-kernel/bones-module-devtools/README.md).

Display preferences (fullscreen, resolution) are set in the in-game Settings menu and saved between runs.

## Documentation

- [docs/concept/](docs/concept/README.md): the game concept — pitch, world and lore, gameplay, and characters. The game grows from it; read it first.
- [docs/architecture.md](docs/architecture.md): system boundaries, extension responsibilities, and assets and packaging.
- [docs/design/](docs/design/): design notes, starting with the menu and level lifecycle.
- [TECHDEBT.md](TECHDEBT.md): known compromises, such as sprite-sheet re-cutting, and the bones upgrades that remove them.

## Repository map

- `src/` and `Cargo.toml`: the game binary `artificial-will`. It embeds the bones 4 kernel as one path dependency (`bones-kernel`), following `vendor/bones/examples/embedding/custom-engine`, instead of running the stock `bones` binary. Its `[workspace]` excludes `vendor/bones`, because `vendor/bones/crates/bones-messages` is a self-isolated workspace nested inside this repository.
- `extensions/`: a separate Cargo workspace targeting `wasm32-wasip2`. It holds the `menu`, `level-one`, `level-two`, `level-three`, and `will` components plus three shared libraries: `game-messages`, `sprite-sheet` (sprite-sheet and asset-pack JSON), and `game-assets` (the embedded third-party packs). See [extensions/README.md](extensions/README.md).
- `assets/`: our own art in `william/`, `box.png`, and `robot_game_assets/` (procedural placeholder biomes and characters, used by `level-three`), and licensed packs in `third-party/<pack>/`. Each pack sits byte-for-byte under `upstream/` with our `pack.json` beside it, which records author, source, and license and gives every asset the game uses a stable id. Add or swap third-party art by editing the manifest, not game code.
- `xtask/`: a Rust program that assembles a self-contained `dist/`.
- `vendor/bones/`: the engine, a git submodule. Patches to it are committed inside that checkout and pushed upstream deliberately.

## Development

Run the game's tests and the extensions' tests separately, because the extensions are their own workspace:

```sh
cargo test
cd extensions
cargo test
```

`cargo run` finds the extensions at `extensions/target/wasm32-wasip2/release`, relative to the executable. For a standalone folder, to hand a build to someone or to test the shipped layout, run:

```sh
cargo xtask dist
```

It builds the game and every extension, then writes `dist/artificial-will(.exe)` with `dist/extensions/core/` (`menu.wasm`, `will.wasm`) and `dist/extensions/levels/` (`level_one.wasm`, `level_two.wasm`, `level_three.wasm`). The executable checks that grouped extension directory first. Copy `dist/` anywhere and run it as is.

## License

GNU General Public License v3.0; see [LICENSE](LICENSE). Third-party art keeps its own license, recorded in each `assets/third-party/<pack>/pack.json`.

## Status

In active development.
