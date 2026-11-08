//! Where everything in White Harbour stands: pure data, no host calls, so
//! the placement rules are testable on the native target.

/// Map size in 64 px tiles; must match `assets/level-three.tmx`.
pub const MAP_TILES: (u32, u32) = (24, 18);
pub const TILE: f32 = 64.0;
/// Props are exported at the pack's native pixel size and drawn at 2x, the
/// same scale as the 64 px tiles (see the pack's README and demo map).
pub const PROP_SCALE: u32 = 2;

/// A horizontal run of all-sea tiles, as `(tile x, tile y, length)`.
/// Generated together with the `.tmx`, so the two agree tile for tile.
pub const SEA_RUNS: &[(u32, u32, u32)] = &[
    (18, 8, 6),
    (17, 9, 7),
    (16, 10, 8),
    (15, 11, 9),
    (14, 12, 10),
    (13, 13, 11),
    (0, 14, 24),
    (0, 15, 24),
    (0, 16, 24),
    (0, 17, 24),
];

/// An invisible fixed wall, centre and half extents in world pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wall {
    pub x: f32,
    pub y: f32,
    pub half_w: f32,
    pub half_h: f32,
}

/// The sea runs plus the map's north, west, and east land edges.
pub fn walls() -> Vec<Wall> {
    let (w, h) = (MAP_TILES.0 as f32 * TILE, MAP_TILES.1 as f32 * TILE);
    let edge = 16.0;
    let mut walls: Vec<Wall> = SEA_RUNS
        .iter()
        .map(|&(tx, ty, len)| Wall {
            x: (tx as f32 + len as f32 / 2.0) * TILE,
            y: (ty as f32 + 0.5) * TILE,
            half_w: len as f32 * TILE / 2.0,
            half_h: TILE / 2.0,
        })
        .collect();
    walls.push(Wall {
        x: w / 2.0,
        y: -edge,
        half_w: w / 2.0,
        half_h: edge,
    });
    walls.push(Wall {
        x: -edge,
        y: h / 2.0,
        half_w: edge,
        half_h: h / 2.0,
    });
    walls.push(Wall {
        x: w + edge,
        y: h / 2.0,
        half_w: edge,
        half_h: h / 2.0,
    });
    walls
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropKind {
    House1,
    House2,
    House3,
    Tree1,
    Tree2,
    Lighthouse,
    Pier,
    Boat,
    FlowerPot,
    Rubble,
}

pub const PROP_KINDS: [PropKind; 10] = [
    PropKind::House1,
    PropKind::House2,
    PropKind::House3,
    PropKind::Tree1,
    PropKind::Tree2,
    PropKind::Lighthouse,
    PropKind::Pier,
    PropKind::Boat,
    PropKind::FlowerPot,
    PropKind::Rubble,
];

pub struct PropArt {
    /// Name in `white_harbour_props.json` and `props/<name>.png`; read by
    /// the test that checks sizes and anchors against the atlas.
    #[cfg_attr(not(test), allow(dead_code))]
    pub name: &'static str,
    /// Native image size.
    pub size: (u32, u32),
    /// Ground contact point in native image pixels.
    pub anchor: (u32, u32),
    /// Collider half extents in world pixels, centred just above the
    /// anchor; `None` for scenery Will never touches.
    pub footprint: Option<(f32, f32)>,
}

impl PropKind {
    pub const fn art(self) -> PropArt {
        // Sizes and anchors copied from white_harbour_props.json; the test
        // module checks them against the file.
        let (name, size, anchor, footprint) = match self {
            Self::House1 => ("house_1", (98, 76), (48, 68), Some((88.0, 22.0))),
            Self::House2 => ("house_2", (82, 68), (40, 60), Some((72.0, 22.0))),
            Self::House3 => ("house_3", (106, 80), (52, 72), Some((96.0, 22.0))),
            Self::Tree1 => ("tree_1", (56, 70), (28, 60), Some((14.0, 10.0))),
            Self::Tree2 => ("tree_2", (50, 66), (24, 58), Some((14.0, 10.0))),
            Self::Lighthouse => ("lighthouse_1", (102, 190), (50, 168), Some((44.0, 24.0))),
            Self::Pier => ("pier_1", (42, 132), (20, 0), None),
            Self::Boat => ("boat_1", (66, 48), (32, 38), None),
            Self::FlowerPot => ("decor_1", (18, 22), (8, 20), Some((10.0, 6.0))),
            Self::Rubble => ("rubble_1", (20, 18), (10, 10), None),
        };
        PropArt {
            name,
            size,
            anchor,
            footprint,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Prop {
    pub kind: PropKind,
    /// World point the prop's anchor stands on.
    pub ground: (f32, f32),
}

impl Prop {
    const fn new(kind: PropKind, x: f32, y: f32) -> Self {
        Self {
            kind,
            ground: (x, y),
        }
    }

    /// Bones centres a sprite on its entity, so the sprite entity sits where
    /// the scaled image's centre falls once its anchor is on `ground`.
    pub fn sprite_centre(self) -> (f32, f32) {
        let art = self.kind.art();
        let s = PROP_SCALE as f32;
        (
            self.ground.0 + (art.size.0 as f32 / 2.0 - art.anchor.0 as f32) * s,
            self.ground.1 + (art.size.1 as f32 / 2.0 - art.anchor.1 as f32) * s,
        )
    }

    /// The collider's centre and half extents, if the prop blocks Will.
    pub fn collider(self) -> Option<Wall> {
        self.kind.art().footprint.map(|(half_w, half_h)| Wall {
            x: self.ground.0,
            y: self.ground.1 - half_h,
            half_w,
            half_h,
        })
    }
}

/// Promenade plaza: tiles 4..11 x 4..10, Will spawns in its middle.
pub const PROPS: &[Prop] = &[
    // Row of white houses north of the plaza.
    Prop::new(PropKind::House1, 300.0, 250.0),
    Prop::new(PropKind::House3, 600.0, 250.0),
    Prop::new(PropKind::House2, 900.0, 300.0),
    // Lighthouse on the eastern headland, at the end of the promenade path.
    Prop::new(PropKind::Lighthouse, 1250.0, 420.0),
    // Greenery.
    Prop::new(PropKind::Tree1, 100.0, 300.0),
    Prop::new(PropKind::Tree2, 120.0, 560.0),
    Prop::new(PropKind::Tree1, 1100.0, 180.0),
    Prop::new(PropKind::Tree2, 1450.0, 200.0),
    Prop::new(PropKind::Tree1, 820.0, 620.0),
    // Flower pots at the plaza corners.
    Prop::new(PropKind::FlowerPot, 280.0, 300.0),
    Prop::new(PropKind::FlowerPot, 690.0, 300.0),
    Prop::new(PropKind::FlowerPot, 280.0, 640.0),
    Prop::new(PropKind::FlowerPot, 690.0, 640.0),
    // Rubble from the old world.
    Prop::new(PropKind::Rubble, 180.0, 720.0),
    Prop::new(PropKind::Rubble, 210.0, 740.0),
    Prop::new(PropKind::Rubble, 1000.0, 720.0),
    // The pier reaches south into the sea from the beach; a boat is moored
    // beside it and another lies off the eastern shore.
    Prop::new(PropKind::Pier, 480.0, 880.0),
    Prop::new(PropKind::Boat, 600.0, 1040.0),
    Prop::new(PropKind::Boat, 1350.0, 760.0),
];

/// The spawn pad's centre on the beach; spirits arrive here.
pub const SPAWN_PAD: (f32, f32) = (900.0, 480.0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cast {
    Scrap,
    Radio,
    Merchant,
    Gardener,
    SpiritElder,
    SpiritChild,
    SpiritFox,
    SpiritDeer,
    SpiritPartridge,
    SpiritHare,
}

impl Cast {
    /// Robots are solid residents; spirits are pass-through visitors.
    pub const fn solid(self) -> bool {
        matches!(
            self,
            Self::Scrap | Self::Radio | Self::Merchant | Self::Gardener
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Character {
    pub cast: Cast,
    /// Entity centre; the feet land `CHARACTER_HALF_H` below it.
    pub x: f32,
    pub y: f32,
    pub animation: &'static str,
}

pub const CHARACTER_HALF_W: f32 = 18.0;
pub const CHARACTER_HALF_H: f32 = 16.0;

const fn character(cast: Cast, x: f32, y: f32, animation: &'static str) -> Character {
    Character {
        cast,
        x,
        y,
        animation,
    }
}

pub const CHARACTERS: &[Character] = &[
    // Robot residents.
    character(Cast::Merchant, 360.0, 360.0, "idle_down"),
    character(Cast::Gardener, 640.0, 560.0, "idle_left"),
    character(Cast::Radio, 1150.0, 470.0, "idle_down"),
    character(Cast::Scrap, 250.0, 760.0, "idle_right"),
    // Spirit visitors.
    character(Cast::SpiritElder, 900.0, 440.0, "idle_down"),
    character(Cast::SpiritChild, 980.0, 520.0, "idle_left"),
    character(Cast::SpiritFox, 760.0, 800.0, "idle_right"),
    character(Cast::SpiritDeer, 120.0, 860.0, "idle_right"),
    character(Cast::SpiritPartridge, 1000.0, 260.0, "idle_down"),
    character(Cast::SpiritHare, 560.0, 760.0, "idle_down"),
];

#[cfg(test)]
mod tests;
