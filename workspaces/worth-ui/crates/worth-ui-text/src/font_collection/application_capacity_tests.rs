use std::sync::Arc;

use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiFontSlant, UiTextOriginalRange, UiTextProfileGeneration,
    UiTextScaleGeneration,
};

use super::application_test_world::{constraints, face, profile_collection_and_sources};
use super::{UiApplicationFontPackDefinition, UiGlobalFontCollection};
use crate::{
    qualify_text_layout, UiFontFamilyStack, UiTextFaceRequest, UiTextParagraphAdmissionDenial,
    UiTextParagraphAdmissionInput, UiTextQualificationDenial, UiTextStyle, UiTextStyleInput,
    UiTextStyleSpan,
};

#[test]
pub(crate) fn application_gsub_expansion_bound_is_carried_into_pre_shape_reservation() {
    let (profile, sources) = profile_collection_and_sources();
    let (collection, receipt, _) = profile
        .register_application_pack(
            UiFontCollectionGeneration::new(2).unwrap(),
            UiApplicationFontPackDefinition {
                name: Arc::from("bengali-capacity"),
                faces: Box::new([face(
                    "capacity-bengali",
                    Arc::clone(&sources["noto-sans-bengali"]),
                    0,
                    UiFontSlant::Upright,
                )]),
            },
        )
        .unwrap();
    let derived = receipt.faces()[0].max_glyphs_per_input_byte();
    assert_eq!(derived, 19);
    assert_eq!(collection.maximum_glyph_expansion_per_input_byte(), 19);

    let fonts = Arc::new(collection);
    let source: Arc<str> = "a"
        .repeat(crate::UiGlobalTextProfile::MAX_GLYPHS / usize::try_from(derived).unwrap() + 1)
        .into();
    let family = receipt.families()[0].identity();
    let style = UiTextStyle::new(UiTextStyleInput {
        language: Arc::from("bn"),
        font_size_millipoints: 14_000,
        letter_spacing_millipoints: 0,
        word_spacing_millipoints: 0,
        family_stack: UiFontFamilyStack::new(Box::new([family])).unwrap(),
        face_request: UiTextFaceRequest::regular(),
        features: Box::new([]),
        variations: Box::new([]),
    })
    .unwrap();
    let denial = match qualify_text_layout(
        UiTextParagraphAdmissionInput {
            source: Arc::clone(&source),
            constraints: constraints(),
            profile_generation: UiTextProfileGeneration::new(1).unwrap(),
            font_collection_generation: fonts.generation(),
            text_scale_generation: UiTextScaleGeneration::new(1).unwrap(),
            styles: Box::new([UiTextStyleSpan::new(
                UiTextOriginalRange::from_text_mechanics(0, source.len() as u32).unwrap(),
                style,
            )
            .unwrap()]),
        },
        fonts,
    ) {
        Ok(_) => panic!("collection-derived expansion overflow was admitted"),
        Err(denial) => denial,
    };
    assert_eq!(
        denial,
        UiTextQualificationDenial::Admission(
            UiTextParagraphAdmissionDenial::DerivedCapacityExceeded
        )
    );
}

#[test]
fn qualified_profile_retains_its_declared_minimum_expansion_bound() {
    let (profile, _) = UiGlobalFontCollection::admit_qualified_profile().unwrap();
    assert_eq!(
        profile.maximum_glyph_expansion_per_input_byte(),
        crate::UiGlobalTextProfile::MAX_GLYPH_EXPANSION_PER_INPUT_BYTE
    );
}
