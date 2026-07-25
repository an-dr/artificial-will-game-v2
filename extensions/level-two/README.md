# level-two

The stone-field level component for Artificial Will. It publishes an embedded
stone TMX map, spawns a dense but traversable field of fixed rock obstacles,
adds six animated idle slimes, and configures the shared game-core camera.

The slimes are passive scenery with colliders: they loop their idle animations
while stationary, but have no input or tick behavior, pursuit, attacks, damage,
or combat AI. Will remains in the sibling `will` component.
