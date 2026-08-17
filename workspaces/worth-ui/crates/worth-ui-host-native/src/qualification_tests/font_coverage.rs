#[test]
fn body_default_asset_covers_every_qualified_printable_basic_latin_scalar() {
    let face = rustybuzz::Face::from_slice(super::WORTH_UI_BODY_DEFAULT_FONT, 0)
        .expect("qualified BodyDefault face parses");
    for scalar in 0x20_u32..=0x7e {
        let character = char::from_u32(scalar).expect("Basic Latin scalar");
        assert!(
            face.glyph_index(character).is_some(),
            "BodyDefault omits U+{scalar:04X}"
        );
    }
}
