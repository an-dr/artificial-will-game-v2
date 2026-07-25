use super::*;

#[test]
fn empty_or_invalid_modes_fall_back_safely() {
    assert_eq!(
        normalize_resolutions(vec![(0, 600), (800, 0)]),
        vec![(800, 600)]
    );
}

#[test]
fn modes_are_sorted_and_deduplicated() {
    assert_eq!(
        normalize_resolutions(vec![(1920, 1080), (800, 600), (1920, 1080)]),
        vec![(800, 600), (1920, 1080)]
    );
}

#[test]
fn long_mode_lists_keep_a_bounded_range_with_both_extremes() {
    let options = normalize_resolutions(vec![
        (640, 480),
        (800, 600),
        (1024, 768),
        (1280, 720),
        (1280, 800),
        (1366, 768),
        (1600, 900),
        (1920, 1080),
    ]);
    assert_eq!(options.len(), MAX_RESOLUTIONS);
    assert_eq!(options.first(), Some(&(640, 480)));
    assert_eq!(options.last(), Some(&(1920, 1080)));
}
