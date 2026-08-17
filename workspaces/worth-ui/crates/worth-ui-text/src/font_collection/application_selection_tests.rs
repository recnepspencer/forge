use std::sync::Arc;

use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiFontSlant, UiQualifiedFontFamilyIdentity, UiTextOriginalRange,
    UiTextProfileGeneration, UiTextScaleGeneration,
};

use super::{
    application_test_world::{constraints, face, profile_collection_and_sources},
    UiApplicationFontFaceDefinition, UiApplicationFontPackDefinition,
};
use crate::{
    UiAdmittedTextParagraph, UiAnalyzedTextParagraph, UiFallbackTextParagraph, UiFontFamilyStack,
    UiGlobalFontCollection, UiOpenTypeFeature, UiShapedTextParagraph, UiTextFaceRequest,
    UiTextParagraphAdmissionInput, UiTextStyle, UiTextStyleInput, UiTextStyleSpan,
};

#[test]
pub(super) fn pack_and_face_selection_ignore_registration_and_definition_order() {
    let (profile, sources) = profile_collection_and_sources();
    let (reverse_profile, _) = profile_collection_and_sources();
    let definition = |reverse| {
        let mut faces = vec![
            face(
                "Alpha",
                sources["noto-sans-roman"].clone(),
                0,
                UiFontSlant::Upright,
            ),
            face(
                "Beta",
                sources["noto-sans-italic"].clone(),
                0,
                UiFontSlant::Italic,
            ),
        ];
        if reverse {
            faces.reverse();
        }
        UiApplicationFontPackDefinition {
            name: Arc::from("order-independent-pack"),
            faces: faces.into_boxed_slice(),
        }
    };
    let (forward, forward_receipt, _) = profile
        .register_application_pack(
            UiFontCollectionGeneration::new(2).unwrap(),
            definition(false),
        )
        .unwrap();
    let (reverse, reverse_receipt, _) = reverse_profile
        .register_application_pack(
            UiFontCollectionGeneration::new(2).unwrap(),
            definition(true),
        )
        .unwrap();

    assert_eq!(forward_receipt, reverse_receipt);
    let forward = Arc::new(forward);
    let reverse = Arc::new(reverse);
    let stack = UiFontFamilyStack::new(Box::new([
        forward_receipt.family("Beta").unwrap(),
        forward_receipt.family("Alpha").unwrap(),
    ]))
    .unwrap();
    assert_eq!(
        selected_faces(
            &forward,
            stack.clone(),
            UiTextFaceRequest::regular(),
            "office"
        ),
        selected_faces(&reverse, stack, UiTextFaceRequest::regular(), "office")
    );
}

#[test]
pub(super) fn authored_application_family_stacks_are_selected_independently_per_span() {
    let (profile, sources) = profile_collection_and_sources();
    let definition = UiApplicationFontPackDefinition {
        name: Arc::from("span application families"),
        faces: Box::new([
            face(
                "Alpha",
                sources["noto-sans-roman"].clone(),
                0,
                UiFontSlant::Upright,
            ),
            face(
                "Beta",
                sources["noto-sans-italic"].clone(),
                0,
                UiFontSlant::Italic,
            ),
        ]),
    };
    let (collection, receipt, _) = profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition)
        .unwrap();
    let collection = Arc::new(collection);
    let source = "office italic";
    let alpha = receipt.family("Alpha").unwrap();
    let beta = receipt.family("Beta").unwrap();
    let styles = Box::new([
        UiTextStyleSpan::new(
            UiTextOriginalRange::from_text_mechanics(0, 7).unwrap(),
            style(
                UiFontFamilyStack::new(Box::new([alpha])).unwrap(),
                UiTextFaceRequest::regular(),
                Box::new([UiOpenTypeFeature::new(*b"liga", 1).unwrap()]),
            ),
        )
        .unwrap(),
        UiTextStyleSpan::new(
            UiTextOriginalRange::from_text_mechanics(7, source.len() as u32).unwrap(),
            style(
                UiFontFamilyStack::new(Box::new([beta])).unwrap(),
                UiTextFaceRequest::new(400, 100_000, UiFontSlant::Italic).unwrap(),
                Box::new([]),
            ),
        )
        .unwrap(),
    ]);
    let (admitted, _) = UiAdmittedTextParagraph::admit(UiTextParagraphAdmissionInput {
        source: Arc::from(source),
        constraints: constraints(),
        profile_generation: UiTextProfileGeneration::new(1).unwrap(),
        font_collection_generation: collection.generation(),
        text_scale_generation: UiTextScaleGeneration::new(1).unwrap(),
        styles,
    })
    .unwrap();
    let fallback = UiFallbackTextParagraph::select(
        UiAnalyzedTextParagraph::analyze(admitted),
        Arc::clone(&collection),
    )
    .unwrap();
    assert_eq!(
        fallback.clusters()[0].face().unwrap(),
        face_for(&receipt, alpha)
    );
    assert_eq!(
        fallback.clusters()[7].face().unwrap(),
        face_for(&receipt, beta)
    );
}

#[test]
pub(super) fn family_fallback_never_uses_a_worse_face_to_skip_a_later_family() {
    let (profile, sources) = profile_collection_and_sources();
    let arabic = sources["noto-sans-arabic"].clone();
    let definition = UiApplicationFontPackDefinition {
        name: Arc::from("family before face fallback"),
        faces: Box::new([
            face(
                "First family",
                sources["noto-sans-roman"].clone(),
                0,
                UiFontSlant::Upright,
            ),
            UiApplicationFontFaceDefinition {
                weight: 700,
                ..face("First family", arabic.clone(), 0, UiFontSlant::Upright)
            },
            face("Second family", arabic, 0, UiFontSlant::Upright),
        ]),
    };
    let (collection, receipt, _) = profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition)
        .unwrap();
    let first = receipt.family("First family").unwrap();
    let second = receipt.family("Second family").unwrap();
    let expected = receipt
        .faces()
        .iter()
        .find(|face| face.family() == second && face.weight() == 400)
        .unwrap()
        .identity();

    let selected = selected_faces(
        &Arc::new(collection),
        UiFontFamilyStack::new(Box::new([first, second])).unwrap(),
        UiTextFaceRequest::regular(),
        "\u{0634}",
    );
    assert_eq!(&*selected, &[expected]);
}

fn face_for(
    receipt: &super::UiQualifiedFontPackReceipt,
    family: UiQualifiedFontFamilyIdentity,
) -> worth_ui_host_contract::UiQualifiedFontFaceIdentity {
    receipt
        .faces()
        .iter()
        .find(|face| face.family() == family)
        .unwrap()
        .identity()
}

pub(super) fn selected_faces(
    collection: &Arc<UiGlobalFontCollection>,
    stack: UiFontFamilyStack,
    request: UiTextFaceRequest,
    source: &str,
) -> Box<[worth_ui_host_contract::UiQualifiedFontFaceIdentity]> {
    fallback(collection, stack, request, Box::new([]), source)
        .clusters()
        .iter()
        .filter_map(|cluster| cluster.face())
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

pub(super) fn shape(
    collection: &Arc<UiGlobalFontCollection>,
    stack: UiFontFamilyStack,
    request: UiTextFaceRequest,
    features: Box<[UiOpenTypeFeature]>,
    source: &str,
) -> UiShapedTextParagraph {
    UiShapedTextParagraph::shape(fallback(collection, stack, request, features, source)).unwrap()
}

pub(super) fn fallback(
    collection: &Arc<UiGlobalFontCollection>,
    stack: UiFontFamilyStack,
    request: UiTextFaceRequest,
    features: Box<[UiOpenTypeFeature]>,
    source: &str,
) -> UiFallbackTextParagraph {
    try_fallback(collection, stack, request, features, source)
        .unwrap_or_else(|denial| panic!("fallback failed for {source:?}: {denial:?}"))
}

pub(super) fn try_fallback(
    collection: &Arc<UiGlobalFontCollection>,
    stack: UiFontFamilyStack,
    request: UiTextFaceRequest,
    features: Box<[UiOpenTypeFeature]>,
    source: &str,
) -> Result<UiFallbackTextParagraph, crate::UiTextFallbackDenial> {
    let style = style(stack, request, features);
    let styles = Box::new([UiTextStyleSpan::new(
        UiTextOriginalRange::from_text_mechanics(0, source.len() as u32).unwrap(),
        style,
    )
    .unwrap()]);
    let (admitted, _) = UiAdmittedTextParagraph::admit(UiTextParagraphAdmissionInput {
        source: Arc::from(source),
        constraints: constraints(),
        profile_generation: UiTextProfileGeneration::new(1).unwrap(),
        font_collection_generation: collection.generation(),
        text_scale_generation: UiTextScaleGeneration::new(1).unwrap(),
        styles,
    })
    .unwrap();
    UiFallbackTextParagraph::select(
        UiAnalyzedTextParagraph::analyze(admitted),
        Arc::clone(collection),
    )
}

fn style(
    stack: UiFontFamilyStack,
    request: UiTextFaceRequest,
    features: Box<[UiOpenTypeFeature]>,
) -> UiTextStyle {
    UiTextStyle::new(UiTextStyleInput {
        language: Arc::from("und"),
        font_size_millipoints: 14_000,
        letter_spacing_millipoints: 0,
        word_spacing_millipoints: 0,
        family_stack: stack,
        face_request: request,
        features,
        variations: Box::new([]),
    })
    .unwrap()
}

pub(super) fn advance(shaped: &UiShapedTextParagraph) -> i32 {
    shaped
        .glyphs()
        .iter()
        .map(|glyph| glyph.x_advance_font_units())
        .sum()
}
