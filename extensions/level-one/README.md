# level-one

The level-specific WASM component for the original Artificial Will level. It
publishes the embedded TMX and tileset to bones game-core, loads the box sprite,
spawns the three pushable boxes, and configures the level camera.

Character assets, input, spawning, and behavior remain independently owned by
the sibling [`will`](../will) component.
