use super::*;

#[test]
fn preferences_round_trip() {
    let preferences = DisplayPreferences {
        width: 1920,
        height: 1080,
        fullscreen: true,
    };
    assert_eq!(
        DisplayPreferences::decode(&preferences.encode()),
        Some(preferences)
    );
}

#[test]
fn default_is_the_safe_windowed_fallback() {
    assert_eq!(
        DisplayPreferences::default(),
        DisplayPreferences {
            width: 800,
            height: 600,
            fullscreen: false,
        }
    );
}

#[test]
fn malformed_or_unsupported_preferences_are_rejected() {
    assert_eq!(DisplayPreferences::decode(&[]), None);

    let mut unsupported = DisplayPreferences::default().encode();
    unsupported[0] = 2;
    assert_eq!(DisplayPreferences::decode(&unsupported), None);

    let mut invalid_fullscreen = DisplayPreferences::default().encode();
    invalid_fullscreen[9] = 2;
    assert_eq!(DisplayPreferences::decode(&invalid_fullscreen), None);

    let mut zero_width = DisplayPreferences::default().encode();
    zero_width[1..5].copy_from_slice(&0_u32.to_le_bytes());
    assert_eq!(DisplayPreferences::decode(&zero_width), None);
}
