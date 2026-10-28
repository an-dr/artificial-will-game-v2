# sprite-sheet

A library, not an extension, that reads sprite-sheet documents and
third-party asset-pack manifests.

A sheet is one PNG plus JSON: `image`, `frame_size`, an optional
`origin_feet`, and `animations`. Each animation has `fps`, `loop`, and its
frames, either as source rectangles (`rects: [[x, y, w, h], ...]`) or as the
`row` + `frames` shorthand for a uniform grid. `assets/william/tv_robot_sheet.json`
is the reference document.

An `AssetPack` is the `pack.json` we write next to a third-party pack's
untouched `upstream/` folder. It records the pack's name, author, source, and
license, and maps asset ids to images (`images`) and to sheets in the format
above (`sheets`). `EmbeddedPack` pairs a manifest with the bytes an extension
compiles in.

This crate is meant to move into `vendor/bones/shared/` once bones draws
source rectangles natively. Until then `cut_strip` re-packs an animation into
the left-to-right strip bones plays; see `TECHDEBT.md` at the repository root.
