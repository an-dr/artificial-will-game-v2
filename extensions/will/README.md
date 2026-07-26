# will

The character-specific WASM component for Artificial Will. It owns input
interpretation and Will's behavior while publishing typed commands to the
reusable bones host modules. Character asset and entity bindings are isolated
in `src/character.rs`; the level components own terrain, obstacles, and camera
setup. Will uses the frictionless retro body kind: held input remains immediate
with no carried momentum, while fixed level geometry can still push him out of
overlap.

Controls: move with WASD or the arrow keys. Movement preserves the original
160-pixel-per-second speed on each axis, including unnormalized diagonals.
Press Space to play Will's facing-dependent attack once; movement continues
during the attack while its facing remains fixed. A swing targets one nearby
object in its forward lane. Will owns three session lives, brief damage
invulnerability, coins, XP, and the derived level (one level per three XP).
Its compact game-rendered HUD stays below menu overlays. Damage blinks Will's
own sprite red while preserving knockback; confirmed enemy hits rely on the
target's Hurt animation instead of a detached marker.

Build from the parent extension workspace:

```sh
cargo build --release --target wasm32-wasip2
```
