use super::*;

#[test]
fn every_menu_screen_builds_centered_game_buttons() {
    let preferences = DisplayPreferences::default();
    let resolutions = [(800, 600), (1920, 1080)];
    for (screen, expected) in [
        (Screen::Start, 3),
        (Screen::LevelSelection, 2),
        (Screen::Pause, 5),
        (Screen::Settings, 4),
    ] {
        let buttons = build_buttons(screen, preferences, &resolutions);
        assert_eq!(buttons.len(), expected);
        assert!(buttons.iter().all(|button| button.x > 0
            && button.y > 0
            && button.x + button.width as i32 <= SCREEN_WIDTH
            && button.y + button.height as i32 <= SCREEN_HEIGHT));
    }
}

#[test]
fn hit_testing_converts_physical_window_pixels_to_logical_space() {
    let buttons = build_buttons(Screen::Start, DisplayPreferences::default(), &[(800, 600)]);
    let first = &buttons[0];
    let logical_x = first.x as f32 + 5.0;
    let logical_y = first.y as f32 + 5.0;
    assert_eq!(
        find_button_at(&buttons, logical_x * 2.0, logical_y * 2.0, (1600, 1200)),
        Some(0)
    );
    assert_eq!(find_button_at(&buttons, 0.0, 0.0, (1600, 1200)), None);
}

#[test]
fn keyboard_selection_wraps_in_both_directions() {
    assert_eq!(move_selection(0, 3, -1), 2);
    assert_eq!(move_selection(2, 3, 1), 0);
    assert_eq!(move_selection(0, 0, 1), 0);
}
