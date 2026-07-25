# menu

The persistent navigation WASM component for Artificial Will. It owns startup,
pause, settings, and level-selection state while gameplay extensions are
loaded and unloaded around it.

Display preferences use a small versioned binary record. Invalid or unsupported
saved data falls back to 800×600 windowed mode.

Menus work with the mouse or egui's Tab/Enter keyboard navigation. Enter opens
level selection from startup; Escape opens and resumes the pause menu.

Build from the parent extension workspace:

```sh
cargo build --release --target wasm32-wasip2
```
