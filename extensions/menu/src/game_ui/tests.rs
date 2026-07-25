use super::*;
use game_ui::Selection;

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
        let layout = build_layout(screen, preferences, &resolutions);
        assert_eq!(layout.buttons.len(), expected);
        assert!(layout.buttons.iter().all(|button| button.bounds.x > 0
            && button.bounds.y > 0
            && button.bounds.x + button.bounds.width as i32 <= SCREEN_WIDTH as i32
            && button.bounds.y + button.bounds.height as i32 <= SCREEN_HEIGHT as i32));
    }
}

#[test]
fn hit_testing_converts_physical_window_pixels_to_logical_space() {
    let layout = build_layout(Screen::Start, DisplayPreferences::default(), &[(800, 600)]);
    let first = &layout.buttons[0];
    let logical_x = first.bounds.x as f32 + 5.0;
    let logical_y = first.bounds.y as f32 + 5.0;
    assert_eq!(
        layout.hit_test(canvas(), logical_x * 2.0, logical_y * 2.0, (1600, 1200)),
        Some((0, START))
    );
    assert_eq!(layout.hit_test(canvas(), 0.0, 0.0, (1600, 1200)), None);
}

#[test]
fn keyboard_selection_wraps_in_both_directions() {
    let mut selection = Selection::default();
    selection.move_by(3, -1);
    assert_eq!(selection.index(), 2);
    selection.move_by(3, 1);
    assert_eq!(selection.index(), 0);
    selection.move_by(0, 1);
    assert_eq!(selection.index(), 0);
}
