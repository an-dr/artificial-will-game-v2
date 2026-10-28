//! Third-party asset packs, embedded through their `pack.json` manifests
//! under `assets/third-party/`. Game code asks a pack for an asset by id;
//! only the manifest knows where the file sits inside the upstream pack.

use sprite_sheet::EmbeddedPack;

macro_rules! embed_files {
    ($root:literal; $($path:literal),* $(,)?) => {
        &[$(($path, include_bytes!(concat!($root, $path)) as &[u8])),*]
    };
}

/// Cainos "Pixel Art Top Down - Basic": terrain tilesets.
pub static CAINOS_TOP_DOWN_BASIC: EmbeddedPack = EmbeddedPack {
    manifest: include_str!("../../../assets/third-party/cainos-pixel-art-top-down-basic/pack.json"),
    files: embed_files!(
        "../../../assets/third-party/cainos-pixel-art-top-down-basic/";
        "upstream/Texture/TX Tileset Grass.png",
    ),
};

/// CraftPix "Free Slime Mobs": the three slime enemies.
pub static CRAFTPIX_SLIMES: EmbeddedPack = EmbeddedPack {
    manifest: include_str!("../../../assets/third-party/craftpix-slime-mobs/pack.json"),
    files: embed_files!(
        "../../../assets/third-party/craftpix-slime-mobs/";
        "upstream/PNG/Slime1/With_shadow/Slime1_Idle_with_shadow.png",
        "upstream/PNG/Slime1/With_shadow/Slime1_Walk_with_shadow.png",
        "upstream/PNG/Slime1/With_shadow/Slime1_Attack_with_shadow.png",
        "upstream/PNG/Slime1/With_shadow/Slime1_Hurt_with_shadow.png",
        "upstream/PNG/Slime1/With_shadow/Slime1_Death_with_shadow.png",
        "upstream/PNG/Slime2/With_shadow/Slime2_Idle_with_shadow.png",
        "upstream/PNG/Slime2/With_shadow/Slime2_Walk_with_shadow.png",
        "upstream/PNG/Slime2/With_shadow/Slime2_Attack_with_shadow.png",
        "upstream/PNG/Slime2/With_shadow/Slime2_Hurt_with_shadow.png",
        "upstream/PNG/Slime2/With_shadow/Slime2_Death_with_shadow.png",
        "upstream/PNG/Slime3/With_shadow/Slime3_Idle_with_shadow.png",
        "upstream/PNG/Slime3/With_shadow/Slime3_Walk_with_shadow.png",
        "upstream/PNG/Slime3/With_shadow/Slime3_Attack_with_shadow.png",
        "upstream/PNG/Slime3/With_shadow/Slime3_Hurt_with_shadow.png",
        "upstream/PNG/Slime3/With_shadow/Slime3_Death_with_shadow.png",
    ),
};

#[cfg(test)]
mod tests;
