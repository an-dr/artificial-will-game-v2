# Menu and level lifecycle

The game uses bones' runtime-managed extension activation
([ADR-024](../../vendor/bones/docs/adr/ADR-024-runtime-managed-extension-activation.md))
to keep navigation in WASM while loading only the selected gameplay content.

## Ownership

| Owner | Responsibility |
| --- | --- |
| `menu.wasm` | Screen state, navigation, display preferences, and lifecycle requests |
| bones extension manager | Recursive catalog, startup allow-list, load/unload/reload, and lifecycle results |
| `will.wasm` | Character resources, entity, input, and cleanup |
| `level_one.wasm` | Level resources, entities, camera, and cleanup |
| native `game-core` | Simulation, rendering, pause, and world reset |

The menu is the only startup extension. Gameplay extensions are cataloged but
remain uninstantiated until a level is selected.

## Screens

```mermaid
stateDiagram-v2
    [*] --> Start
    Start --> Settings
    Start --> LevelSelection: Start
    LevelSelection --> Gameplay: Level One
    Gameplay --> Pause: Escape
    Pause --> Gameplay: Resume
    Pause --> Settings
    Pause --> LevelSelection
    Pause --> Start: Main Menu
    Settings --> Start: Back from startup
    Settings --> Pause: Back from pause
```

Start provides Start, Settings, and Quit. Pause provides Resume, Settings,
Level Selection, Main Menu, and Quit. UI interaction supports mouse and
keyboard through bones' egui input layer.

## Session transitions

Starting a level loads the selected level extension followed by `will`, then
unpauses `game-core`. Changing levels pauses simulation, unloads `will` and the
current level, resets the native world, and loads the new pair. Returning to
the start screen performs the same unload and reset without loading a
replacement.

`shutdown` lets each gameplay extension publish cleanup for the resources and
entities it created. The world reset is a defensive session boundary, not a
replacement for extension cleanup.

Esc pauses native simulation and character behavior without unloading either
gameplay extension, preserving the exact session for Resume. The menu keeps
receiving frame ticks and publishing its immediate-mode UI while gameplay is
paused.

## Display preferences

The settings screen lists the display modes reported by bones, falling back to
800×600 if none are available. It applies resolution and fullscreen/windowed
mode immediately through `gfx/set-display`.

`menu.wasm` stores a versioned preference record through bones persistence.
Missing, corrupt, or unsupported saved values fall back safely and are
rewritten after the user selects a valid setting.

## Distribution

```text
dist/
├── artificial-will[.exe]
├── core/
│   ├── menu.wasm
│   └── will.wasm
└── levels/
    └── level_one.wasm
```

Cargo package metadata assigns each component its distribution group. The
packager rejects missing or unknown groups rather than silently flattening an
artifact.
