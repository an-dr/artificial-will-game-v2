use super::*;

#[test]
fn every_pack_embeds_each_file_its_manifest_names() {
    for pack in [&CAINOS_TOP_DOWN_BASIC, &CRAFTPIX_SLIMES] {
        assert_eq!(pack.missing_files().unwrap(), Vec::<String>::new());
    }
}

#[test]
fn every_slime_animation_cuts_from_its_upstream_sheet() {
    let pack = CRAFTPIX_SLIMES.pack().unwrap();
    for (id, sheet) in &pack.sheets {
        let png = CRAFTPIX_SLIMES.file(&sheet.image).unwrap();
        for name in sheet.animations.keys() {
            sheet
                .cut_strip(png, name, (0, 0))
                .unwrap_or_else(|error| panic!("{id}/{name}: {error}"));
        }
    }
}
