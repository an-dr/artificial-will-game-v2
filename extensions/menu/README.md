# menu

The persistent navigation WASM component for Artificial Will. It owns startup,
pause, game-over, settings, and level-selection state while gameplay
extensions are loaded and unloaded around it.

Display preferences use a small versioned binary record. Invalid or unsupported
saved data falls back to 800×600 windowed mode.

Menus are rendered by the game renderer as screen-space rectangles and text;
they do not use the app UI module. Use the mouse or arrow/WASD keys to select
items, Enter or Space to activate them, and Escape to open or resume the pause
menu. Layout, selection, gfx command generation, and scaled hit-testing come
from bones' theme-free guest-side `game-ui` crate; this extension owns the
Artificial Will theme, labels, and screen flow.

Build from the parent extension workspace:

```sh
cargo build --release --target wasm32-wasip2
```
