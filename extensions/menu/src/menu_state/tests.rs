use super::*;

#[test]
fn starts_without_a_gameplay_session() {
    let state = MenuState::default();
    assert_eq!(state.screen(), Screen::Start);
    assert_eq!(state.active_level(), None);
}

#[test]
fn selecting_level_one_replaces_the_session_and_enters_gameplay() {
    let mut state = MenuState::default();
    state.open_level_selection();
    assert_eq!(
        state.select_level(Level::One),
        SessionRequest::Replace(Level::One)
    );
    assert_eq!(state.screen(), Screen::Gameplay);
    assert_eq!(state.active_level(), Some(Level::One));
    assert_eq!(Level::One.extension_name(), "level_one");
}

#[test]
fn cancelling_level_selection_returns_to_its_session_context() {
    let mut state = MenuState::default();
    state.open_level_selection();
    assert!(state.cancel_level_selection());
    assert_eq!(state.screen(), Screen::Start);

    state.select_level(Level::One);
    state.pause();
    state.open_level_selection();
    assert!(state.cancel_level_selection());
    assert_eq!(state.screen(), Screen::Pause);
}

#[test]
fn escape_style_pause_and_resume_only_apply_to_live_gameplay() {
    let mut state = MenuState::default();
    assert!(!state.pause());
    state.select_level(Level::One);
    assert!(state.pause());
    assert_eq!(state.screen(), Screen::Pause);
    assert!(!state.pause());
    assert!(state.resume());
    assert_eq!(state.screen(), Screen::Gameplay);
}

#[test]
fn settings_return_to_the_screen_that_opened_them() {
    let mut state = MenuState::default();
    assert!(state.open_settings());
    assert!(state.close_settings());
    assert_eq!(state.screen(), Screen::Start);

    state.select_level(Level::One);
    state.pause();
    assert!(state.open_settings());
    assert!(state.close_settings());
    assert_eq!(state.screen(), Screen::Pause);
}

#[test]
fn returning_home_stops_only_an_active_session() {
    let mut state = MenuState::default();
    assert_eq!(state.return_to_start(), None);
    state.select_level(Level::One);
    assert_eq!(state.return_to_start(), Some(SessionRequest::Stop));
    assert_eq!(state.screen(), Screen::Start);
    assert_eq!(state.active_level(), None);
}
