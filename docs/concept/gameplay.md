# Gameplay

The core loop is not defined yet; what exists is the hero's move set and a few proposed mechanics.

| Mechanic                                                       | State    | Notes                                                           |
| -------------------------------------------------------------- | -------- | --------------------------------------------------------------- |
| Walk, idle (4 directions)                                      | Animated | Classic Zelda movement                                          |
| Attack: arm blaster                                            | Animated | Wind-up, muzzle flash, beam, recover; normal "00" face kept     |
| Hurt, death                                                    | Animated | "XX" face when hurt; screen goes static, CRT line, off on death |
| Hold item                                                      | Animated | Zelda "item get" pose, happy face                               |
| Jump (4 directions)                                            | Animated | Crouch, rise, apex, fall, land; ground shadow shows height      |
| Spawn pads                                                     | Animated | Pad + light beam; spirits have appear/vanish animations         |
| Spirits cross water, robots need bridges                       | Proposed | Asymmetry for puzzles and story (Drowned District)              |
| Animal actions (bite, pounce, swipe, leap, hop, curl, flutter) | Animated | Whether animal spirits are hostile is undecided                 |

## Jump implementation note

The rise is baked into the frames, so the robot's logical position stays on the ground; disable the collisions a jump should clear (holes, low obstacles) during the airborne frames.

## Not decided

The robot's goal, enemies and combat role, progression, and whether the game is single-player or actually multiplayer.
