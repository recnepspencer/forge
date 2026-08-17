use super::super::{profile_inputs_from_repository, UiGlobalFontCollection};
use worth_ui_host_contract::UiFontCollectionGeneration;

#[test]
fn the_exact_profile_font_bytes_are_admitted_once_in_fallback_order() {
    let (collection, cost) = UiGlobalFontCollection::admit_profile(
        UiFontCollectionGeneration::new(1).unwrap(),
        profile_inputs_from_repository(),
    )
    .unwrap();
    assert_eq!(collection.face_count(), 30);
    assert_eq!(cost.faces_checked(), 30);
    assert_eq!(cost.shaper_data_built(), 30);
    assert!(cost.bytes_hashed() > 70_000_000);
}
