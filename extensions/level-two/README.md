# level-two

The overgrown-ruins level component for Artificial Will. It publishes an
embedded grass map with deliberate broken-stone patches, spawns a dense but
traversable mix of illustrated fixed boulders and rock piles, adds six animated
idle slimes, and configures the shared game-core camera.

The slimes are passive scenery with colliders: they loop their idle animations
while stationary, but have no input or tick behavior, pursuit, attacks, damage,
or combat AI. Will remains in the sibling `will` component.
