# level-two

The stone-field level component for Artificial Will. It publishes an embedded
stone TMX map, spawns a dense but traversable field of fixed rock obstacles,
and configures the shared game-core camera.

The component owns only level terrain and rocks. Will remains in the sibling
`will` component, and slime inhabitants are deliberately separate from the
rock-field setup.
