use std::sync::Arc;

use worth_ui_host_contract::{UiFontCollectionGeneration, UiFontSlant};

use super::{
    application_selection_tests::shape,
    application_test_world::{face, profile_collection_and_sources},
    UiApplicationFontPackDefinition,
};
use crate::{UiFontFamilyStack, UiQualifiedTextLayout, UiTextFaceRequest};

#[test]
pub(crate) fn variable_and_color_glyph_ink_is_derived_from_the_selected_font_instance() {
    super::ink_bounds::color_tests::transparent_and_porter_duff_layers_have_exact_nonzero_bounds();
    super::ink_bounds::bitmap::transparent_and_bordered_bitmap_alpha_has_exact_support();
    let (profile, sources) = profile_collection_and_sources();
    let definition = UiApplicationFontPackDefinition {
        name: Arc::from("ink-metric variable application family"),
        faces: Box::new([face(
            "Variable Ink",
            sources["noto-sans-roman"].clone(),
            0,
            UiFontSlant::Upright,
        )]),
    };
    let (collection, receipt, _) = profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition)
        .unwrap();
    let family = receipt.family("Variable Ink").unwrap();
    let stack = UiFontFamilyStack::new(Box::new([family])).unwrap();
    let collection = Arc::new(collection);
    let light = shape(
        &collection,
        stack.clone(),
        UiTextFaceRequest::new(100, 100_000, UiFontSlant::Upright).unwrap(),
        Box::new([]),
        "W",
    );
    let heavy = shape(
        &collection,
        stack,
        UiTextFaceRequest::new(900, 100_000, UiFontSlant::Upright).unwrap(),
        Box::new([]),
        "W",
    );
    assert_eq!(light.runs()[0].face(), heavy.runs()[0].face());
    let light = UiQualifiedTextLayout::layout(light).unwrap();
    let heavy = UiQualifiedTextLayout::layout(heavy).unwrap();
    assert_ne!(
        light.positioned_glyphs()[0].ink_bounds(),
        heavy.positioned_glyphs()[0].ink_bounds(),
        "the selected variation instance must move the real outline bounds"
    );

    let emoji = crate::layout::tests::layout("\u{1f600}", 96_000, 1);
    assert!(emoji.positioned_glyphs().iter().any(|glyph| {
        let bounds = glyph.ink_bounds();
        bounds.width_millipoints() > 0 && bounds.height_millipoints() > 0
    }));
    assert!(emoji.view().ink_bounds().width_millipoints() > 0);
    assert!(emoji.view().ink_bounds().height_millipoints() > 0);
}
