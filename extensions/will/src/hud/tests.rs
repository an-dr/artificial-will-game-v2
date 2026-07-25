use super::*;

#[test]
fn hud_is_a_compact_screen_space_status_strip() {
    let commands = commands(PlayerStats {
        lives: 2,
        experience: 4,
        level: 2,
        coins: 7,
    });

    assert_eq!(commands.len(), 8);
    assert!(commands.iter().any(|command| {
        matches!(
            command,
            DrawCommand::Text { text, .. } if text == "LIVES 2"
        )
    }));
    assert!(commands
        .iter()
        .any(|command| { matches!(command, DrawCommand::Text { text, .. } if text == "LV 2") }));
    assert!(commands
        .iter()
        .any(|command| { matches!(command, DrawCommand::Text { text, .. } if text == "XP 4") }));
    assert!(commands.iter().any(|command| {
        matches!(
            command,
            DrawCommand::Text { text, .. } if text == "COINS 7"
        )
    }));
}

#[test]
fn experience_bar_tracks_progress_inside_the_current_level() {
    let commands = commands(PlayerStats {
        lives: 3,
        experience: 5,
        level: 2,
        coins: 0,
    });
    assert!(commands.iter().any(|command| {
        matches!(
            command,
            DrawCommand::Rectangle {
                bounds: Rect {
                    x: 260,
                    y: 37,
                    width: 38,
                    height: 8,
                },
                color,
                ..
            } if *color == EXPERIENCE
        )
    }));
}
