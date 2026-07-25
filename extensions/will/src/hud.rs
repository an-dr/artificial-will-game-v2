use game_messages::PlayerStats;
use game_ui::{DrawCommand, Rect};

use crate::combat_state::EXPERIENCE_PER_LEVEL;

const HUD_LAYER: u8 = 235;
const HUD_TEXT_LAYER: u8 = 237;
const PANEL: (u8, u8, u8, u8) = (8, 14, 24, 224);
const BORDER: (u8, u8, u8, u8) = (55, 91, 112, 255);
const LIFE: (u8, u8, u8, u8) = (245, 92, 102, 255);
const LEVEL: (u8, u8, u8, u8) = (104, 218, 232, 255);
const EXPERIENCE: (u8, u8, u8, u8) = (123, 214, 126, 255);
const COINS: (u8, u8, u8, u8) = (248, 197, 76, 255);

const fn rect(x: i32, y: i32, width: u32, height: u32) -> Rect {
    Rect {
        x,
        y,
        width,
        height,
    }
}

pub fn commands(stats: PlayerStats) -> Vec<DrawCommand> {
    let level_floor = stats
        .level
        .saturating_sub(1)
        .saturating_mul(EXPERIENCE_PER_LEVEL);
    let experience_in_level = stats
        .experience
        .saturating_sub(level_floor)
        .min(EXPERIENCE_PER_LEVEL);
    let progress_width = experience_in_level * 58 / EXPERIENCE_PER_LEVEL;

    vec![
        DrawCommand::rectangle(rect(14, 14, 438, 54), true, PANEL, HUD_LAYER),
        DrawCommand::rectangle(rect(14, 14, 438, 54), false, BORDER, HUD_LAYER + 1),
        DrawCommand::text(
            format!("LIVES {}", stats.lives),
            28,
            29,
            18,
            LIFE,
            HUD_TEXT_LAYER,
        ),
        DrawCommand::text(
            format!("LV {}", stats.level),
            126,
            29,
            18,
            LEVEL,
            HUD_TEXT_LAYER,
        ),
        DrawCommand::text(
            format!("XP {}", stats.experience),
            202,
            29,
            18,
            EXPERIENCE,
            HUD_TEXT_LAYER,
        ),
        DrawCommand::rectangle(rect(260, 37, 58, 8), true, (29, 49, 47, 255), HUD_LAYER + 1),
        DrawCommand::rectangle(
            rect(260, 37, progress_width, 8),
            true,
            EXPERIENCE,
            HUD_TEXT_LAYER,
        ),
        DrawCommand::text(
            format!("COINS {}", stats.coins),
            330,
            29,
            18,
            COINS,
            HUD_TEXT_LAYER,
        ),
    ]
}

#[cfg(test)]
mod tests;
