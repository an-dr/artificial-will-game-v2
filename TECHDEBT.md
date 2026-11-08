# Technical debt

Known compromises, why each was made, and the change that removes it.

## Sprite sheets are re-cut at load time

**Where:** `extensions/sprite-sheet` (`SpriteSheet::cut_strip`), called by
`extensions/will/src/character.rs` and `extensions/level-two`.

**Why:** bones plays an animation only from frames laid out left to right
starting at the image origin (`Sprite::frame_w` and
`SpritePresentation::frames_per_row`). William's sheet keeps every animation
on its own row of one PNG, and the format allows arbitrary frame rectangles.
Each extension therefore decodes the sheet, copies every animation's frames
into a fresh strip, re-encodes it as PNG, and loads that strip as a separate
sprite. The cost is CPU and memory at load, and one bones sprite id per
animation instead of one per sheet.

**Clean fix (bones upgrade):**

1. Let a presentation name its frames by source rectangle: either a list of
   `[x, y, w, h]` per frame, or a `(origin_x, origin_y)` offset added to the
   existing grid maths. A sheet is then loaded once with `LoadSprite`.
2. Move `extensions/sprite-sheet` to `vendor/bones/shared/sprite-sheet`. Its
   parsing and `AssetPack` manifest types move unchanged. `presentation`
   starts emitting rectangles, and `cut_strip` plus the `png` dependency are
   deleted.
3. In the game, load each sheet once, drop the per-animation sprite id tables
   (`FIRST_SPRITE_ID` in `will`, the `SLIME_*_SPRITE_IDS` in `level-two`), and
   remove the `cut_strip` calls.

## Feet are aligned by shifting pixels

**Where:** `FEET_TARGET` in `extensions/will/src/character.rs` and
`SpriteSheet::shift_feet_to`.

**Why:** bones draws a sprite centred on its entity
(`game-core/src/game_core.rs`, `dst_x = x - draw_w / 2`) and has no anchor or
origin. The sheet says the feet are at `origin_feet` (48, 78) in each 96 px
frame. To put them on the bottom edge of Will's collider, the cutting step
shifts every frame's content up by 2 px. This only works because the strip is
re-cut anyway, and it silently clips art that would move outside the frame.

**Clean fix (bones upgrade):** add a sprite origin (pivot) to
`SpritePresentation` (frame pixels, default the frame centre) and draw at
`entity - origin * scale`. `SpriteSheet::presentation` then passes
`origin_feet` through, and `FEET_TARGET` and `shift_feet_to` are deleted. Land
this together with the source-rectangle change above, because both remove the
cutting step.

## Tilesets are extruded at load time

**Where:** `extensions/sprite-sheet` (`extrude_tiles`), used by `extensions/level-three`.

**What:** at non-integer window scales bones samples a texel outside a tile's source rectangle, so thin dark lines show along tile edges whose atlas neighbour is transparent or a different tile. Level three re-lays its tileset with 2 px of each tile's own edge pixels around it, and its `.tmx` declares the matching `margin="2" spacing="4"`.

**Remove when:** bones samples tiles without bleeding (for example by insetting source rects or snapping draws to integer scale). Then drop the call and the `.tmx` attributes.
