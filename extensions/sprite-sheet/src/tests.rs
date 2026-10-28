use super::*;

const WILLIAM_JSON: &str = include_str!("../../../assets/william/tv_robot_sheet.json");
const WILLIAM_PNG: &[u8] = include_bytes!("../../../assets/william/tv_robot_sheet.png");

/// A 4x2 RGBA image whose pixel (x, y) is `[x, y, 0, 255]`.
fn gradient_png() -> Vec<u8> {
    let pixels: Vec<u8> = (0..2u8)
        .flat_map(|y| (0..4u8).flat_map(move |x| [x, y, 0, 255]))
        .collect();
    let mut out = Vec::new();
    let mut encoder = png::Encoder::new(&mut out, 4, 2);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap();
    writer.finish().unwrap();
    out
}

fn decode(png_bytes: &[u8]) -> (u32, u32, Vec<u8>) {
    let decoder = png::Decoder::new(std::io::Cursor::new(png_bytes));
    let mut reader = decoder.read_info().unwrap();
    let mut buffer = vec![0; reader.output_buffer_size().unwrap()];
    let info = reader.next_frame(&mut buffer).unwrap();
    (
        info.width,
        info.height,
        buffer[..info.buffer_size()].to_vec(),
    )
}

fn tiny_sheet(animation: &str) -> SpriteSheet {
    SpriteSheet::parse(&format!(
        r#"{{"image":"g.png","frame_size":[2,1],"origin_feet":[1,1],
            "animations":{{"a":{animation}}}}}"#
    ))
    .unwrap()
}

#[test]
fn parses_the_william_sheet_as_shipped() {
    let sheet = SpriteSheet::parse(WILLIAM_JSON).unwrap();
    assert_eq!(sheet.image, "tv_robot_sheet.png");
    assert_eq!(sheet.frame_size, [96, 96]);
    assert_eq!(sheet.origin_feet, Some([48, 78]));
    let walk = sheet.animation("walk_left").unwrap();
    assert!(walk.looping);
    assert_eq!(sheet.rects("walk_left").unwrap()[1], [96, 576, 96, 96]);
}

#[test]
fn row_shorthand_expands_to_the_same_rects_as_explicit_ones() {
    let sheet = SpriteSheet::parse(WILLIAM_JSON).unwrap();
    let explicit = sheet.rects("attack_up").unwrap();
    let shorthand = SpriteSheet::parse(
        r#"{"image":"x","frame_size":[96,96],
            "animations":{"attack_up":{"row":9,"frames":4,"fps":12}}}"#,
    )
    .unwrap();
    assert_eq!(shorthand.rects("attack_up").unwrap(), explicit);
}

#[test]
fn an_animation_without_frames_or_a_name_is_an_error() {
    let sheet = tiny_sheet(r#"{"fps":1}"#);
    assert!(matches!(sheet.rects("a"), Err(SheetError::NoFrames(_))));
    assert!(matches!(
        sheet.rects("b"),
        Err(SheetError::UnknownAnimation(_))
    ));
}

#[test]
fn cut_strip_packs_frames_left_to_right_from_any_source_position() {
    let sheet = tiny_sheet(r#"{"fps":1,"rects":[[2,1,2,1],[0,0,2,1]]}"#);
    let strip = sheet.cut_strip(&gradient_png(), "a", (0, 0)).unwrap();
    let (w, h, pixels) = decode(&strip);
    assert_eq!((w, h), (4, 1));
    let xy: Vec<(u8, u8)> = pixels.chunks(4).map(|p| (p[0], p[1])).collect();
    assert_eq!(xy, [(2, 1), (3, 1), (0, 0), (1, 0)]);
}

#[test]
fn cut_strip_shift_moves_content_and_clears_what_it_uncovers() {
    let sheet = tiny_sheet(r#"{"fps":1,"rects":[[0,0,2,1]]}"#);
    let strip = sheet.cut_strip(&gradient_png(), "a", (1, 0)).unwrap();
    let (_, _, pixels) = decode(&strip);
    assert_eq!(&pixels[..4], &[0, 0, 0, 0]);
    assert_eq!(&pixels[4..], &[0, 0, 0, 255]);
}

#[test]
fn cut_strip_rejects_rects_outside_the_image() {
    let sheet = tiny_sheet(r#"{"fps":1,"rects":[[3,0,2,1]]}"#);
    assert!(matches!(
        sheet.cut_strip(&gradient_png(), "a", (0, 0)),
        Err(SheetError::RectOutOfBounds { .. })
    ));
}

#[test]
fn every_william_animation_cuts_from_the_shipped_png() {
    let sheet = SpriteSheet::parse(WILLIAM_JSON).unwrap();
    for name in sheet.animations.keys() {
        let strip = sheet.cut_strip(WILLIAM_PNG, name, (0, 0)).unwrap();
        let (w, h, _) = decode(&strip);
        assert_eq!((w, h), (96 * sheet.rects(name).unwrap().len() as u32, 96));
    }
}

#[test]
fn feet_shift_targets_the_requested_point() {
    let sheet = SpriteSheet::parse(WILLIAM_JSON).unwrap();
    assert_eq!(sheet.shift_feet_to((48, 62)), (0, -16));
}

#[test]
fn presentation_describes_the_cut_strip() {
    let sheet = SpriteSheet::parse(WILLIAM_JSON).unwrap();
    let presentation = sheet.presentation("attack_down", 7, [48, 48]).unwrap();
    assert_eq!(presentation.sprite.sprite_id, 7);
    assert_eq!(presentation.sprite.frame_count, 4);
    assert_eq!(presentation.frames_per_row, 4);
    assert_eq!((presentation.draw_w, presentation.draw_h), (48, 48));
    assert!(!presentation.looping);
    assert!((presentation.sprite.frame_duration - 1.0 / 12.0).abs() < 1e-6);
}

#[test]
fn asset_pack_resolves_images_and_sheets_by_id() {
    let pack = AssetPack::parse(
        r#"{"name":"P","author":"A","source":"https://x","license":"upstream/L.txt",
            "images":{"tile":"upstream/t.png"},
            "sheets":{"mob":{"image":"upstream/m.png","frame_size":[8,8],
                "animations":{"idle":{"row":0,"frames":2,"fps":4,"loop":true}}}}}"#,
    )
    .unwrap();
    assert_eq!(pack.image("tile").unwrap(), "upstream/t.png");
    assert_eq!(pack.sheet("mob").unwrap().rects("idle").unwrap().len(), 2);
    assert!(matches!(
        pack.image("mob"),
        Err(SheetError::UnknownAsset(_))
    ));
    let files: Vec<&str> = pack.files().collect();
    assert_eq!(files, ["upstream/t.png", "upstream/m.png"]);
}
