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

## Setup

```sh
git clone --recurse-submodules <this-repo-url>
# or, if already cloned:
git submodule update --init --recursive
```

Requires Rust (`rustup target add wasm32-wasip2`) and PowerShell 7+ (`pwsh`)
— see [bones' own README](bones/README.md) for its native-toolchain
requirements (SDL3 via CMake).

## Hello world

[extensions/hello](extensions/hello) is a minimal WASM extension proving the
engine + toolchain work end to end, before any real game logic exists:

```sh
cd extensions/hello
pwsh build.ps1
dist/bones
```
