use std::sync::Arc;

use worth_ui_host_contract::{UiFontCollectionGeneration, UiFontSlant, UiTextOriginalRange};

use super::{
    application_test_world::{constraints, face, profile_collection_and_sources},
    UiApplicationFontPackDefinition,
};
use crate::{
    qualify_text_layout, UiFontFamilyStack, UiQualifiedTextLayoutRequest, UiTextFaceRequest,
    UiTextFallbackDenial, UiTextParagraphAdmissionDenial, UiTextParagraphAdmissionInput,
    UiTextProfileGeneration, UiTextQualificationDenial, UiTextScaleGeneration, UiTextStyle,
    UiTextStyleInput, UiTextStyleSpan,
};

#[test]
pub(crate) fn retired_collection_denies_fresh_work_but_reconstructs_its_exact_qualified_layout() {
    let (profile, sources) = profile_collection_and_sources();
    let definition = pack(
        "generation two",
        Arc::clone(&sources["noto-sans-roman"]),
        UiFontSlant::Upright,
    );
    let (generation_two, receipt, _) = profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition)
        .unwrap();
    let family = receipt.family("Application").unwrap();
    let generation_two = Arc::new(generation_two);
    let input = paragraph(generation_two.generation(), family);
    let layout = qualify_text_layout(input.clone(), Arc::clone(&generation_two)).unwrap();
    let identity = layout.identity();
    let source = Arc::clone(layout.reconstruction_source().unwrap());
    let selected = layout.view().logical_runs()[0].face();
    let original_bytes = Arc::clone(layout.artifact().face_resource(selected).unwrap().bytes());

    let _generation_three = generation_two
        .replace_application_pack(
            receipt.identity(),
            UiFontCollectionGeneration::new(3).unwrap(),
            pack(
                "generation three",
                Arc::clone(&sources["noto-sans-italic"]),
                UiFontSlant::Italic,
            ),
        )
        .unwrap();
    assert!(matches!(
        qualify_text_layout(input, Arc::clone(&generation_two)),
        Err(UiTextQualificationDenial::Admission(
            UiTextParagraphAdmissionDenial::StaleFontCollectionGeneration
        ))
    ));

    let mut invalid = paragraph(generation_two.generation(), family);
    invalid.styles = Box::new([]);
    assert!(matches!(
        qualify_text_layout(invalid, Arc::clone(&generation_two)),
        Err(UiTextQualificationDenial::Admission(
            UiTextParagraphAdmissionDenial::StaleFontCollectionGeneration
        ))
    ));

    drop(layout);
    let rebuilt = source.reconstruct().unwrap();
    assert_eq!(rebuilt.identity(), identity);
    assert_eq!(rebuilt.view().logical_runs()[0].face(), selected);
    assert!(Arc::ptr_eq(
        rebuilt.artifact().face_resource(selected).unwrap().bytes(),
        &original_bytes
    ));
    assert!(source.matches_font_collection(rebuilt.pinned_font_collection()));
}

#[test]
pub(super) fn retired_collection_denies_pack_admission_before_inspecting_candidate_bytes() {
    let (profile, sources) = profile_collection_and_sources();
    let (generation_two, receipt, _) = profile
        .register_application_pack(
            UiFontCollectionGeneration::new(2).unwrap(),
            pack(
                "generation two",
                Arc::clone(&sources["noto-sans-roman"]),
                UiFontSlant::Upright,
            ),
        )
        .unwrap();
    let _generation_three = generation_two
        .remove_application_pack(
            receipt.identity(),
            UiFontCollectionGeneration::new(3).unwrap(),
        )
        .unwrap();

    let malformed_candidate = UiApplicationFontPackDefinition {
        name: Arc::from("must not be inspected"),
        faces: Box::new([]),
    };
    assert!(matches!(
        generation_two.register_application_pack(
            UiFontCollectionGeneration::new(3).unwrap(),
            malformed_candidate,
        ),
        Err(super::UiFontCollectionAdmissionDenial::StaleCollectionGeneration)
    ));
}

#[test]
pub(crate) fn same_numbered_collections_cannot_substitute_for_the_request_owner() {
    let (left_profile, sources) = profile_collection_and_sources();
    let (right_profile, _) = profile_collection_and_sources();
    let (left, left_receipt, _) = left_profile
        .register_application_pack(
            UiFontCollectionGeneration::new(2).unwrap(),
            pack(
                "left lineage",
                Arc::clone(&sources["noto-sans-roman"]),
                UiFontSlant::Upright,
            ),
        )
        .unwrap();
    let (right, _, _) = right_profile
        .register_application_pack(
            UiFontCollectionGeneration::new(2).unwrap(),
            pack(
                "right lineage",
                Arc::clone(&sources["noto-sans-italic"]),
                UiFontSlant::Italic,
            ),
        )
        .unwrap();
    let left = Arc::new(left);
    let right = Arc::new(right);
    let input = paragraph(
        left.generation(),
        left_receipt.family("Application").unwrap(),
    );
    let left_request = UiQualifiedTextLayoutRequest::new(input.clone(), Arc::clone(&left));
    let right_request = UiQualifiedTextLayoutRequest::new(input, Arc::clone(&right));

    assert_ne!(left_request.identity(), right_request.identity());
    let layout = left_request.qualify().unwrap();
    assert!(matches!(
        right_request.qualify(),
        Err(UiTextQualificationDenial::Fallback(
            UiTextFallbackDenial::ForeignFontFamily
        ))
    ));
    let source = layout.reconstruction_source().unwrap();
    assert!(source.matches_font_collection(&left));
    assert!(!source.matches_font_collection(&right));
}

fn pack(name: &str, bytes: Arc<[u8]>, slant: UiFontSlant) -> UiApplicationFontPackDefinition {
    UiApplicationFontPackDefinition {
        name: Arc::from(name),
        faces: Box::new([face("Application", bytes, 0, slant)]),
    }
}

fn paragraph(
    generation: UiFontCollectionGeneration,
    family: worth_ui_host_contract::UiQualifiedFontFamilyIdentity,
) -> UiTextParagraphAdmissionInput {
    let source: Arc<str> = Arc::from("WORTH reconstruction");
    let constraints = constraints();
    let style = UiTextStyle::new(UiTextStyleInput {
        language: Arc::from("und"),
        font_size_millipoints: constraints.font_size_millipoints(),
        letter_spacing_millipoints: constraints.letter_spacing_millipoints(),
        word_spacing_millipoints: constraints.word_spacing_millipoints(),
        family_stack: UiFontFamilyStack::new(Box::new([family])).unwrap(),
        face_request: UiTextFaceRequest::regular(),
        features: Box::new([]),
        variations: Box::new([]),
    })
    .unwrap();
    let range = UiTextOriginalRange::from_text_mechanics(0, source.len() as u32).unwrap();
    UiTextParagraphAdmissionInput {
        source,
        constraints,
        profile_generation: UiTextProfileGeneration::new(1).unwrap(),
        font_collection_generation: generation,
        text_scale_generation: UiTextScaleGeneration::new(1).unwrap(),
        styles: Box::new([UiTextStyleSpan::new(range, style).unwrap()]),
    }
}
