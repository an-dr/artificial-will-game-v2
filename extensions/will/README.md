# will

The character-specific WASM component for Artificial Will. It owns input
interpretation and Will's behavior while publishing typed commands to the
reusable bones host modules. Character asset and entity bindings are isolated
in `src/character.rs`; [`level-one`](../level-one) owns the level's TMX, boxes,
and camera setup.

Controls: move with WASD or the arrow keys. Movement preserves the original
160-pixel-per-second speed on each axis, including unnormalized diagonals.
Press Space to play Will's facing-dependent attack once; movement continues
during the attack while its facing remains fixed.

Build from the parent extension workspace:

```sh
cargo build --release --target wasm32-wasip2
```
