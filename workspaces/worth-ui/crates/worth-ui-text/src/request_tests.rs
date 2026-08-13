use std::sync::Arc;

use worth_ui_host_contract::{UiFontSlant, UiTextOriginalRange};

use crate::{
    UiFontFamilyStack, UiFontVariationCoordinate, UiGlobalFontCollection, UiOpenTypeFeature,
    UiQualifiedTextLayoutRequest, UiTextAlignment, UiTextBaseDirection, UiTextFaceRequest,
    UiTextOverflow, UiTextParagraphAdmissionInput, UiTextParagraphConstraints,
    UiTextParagraphConstraintsInput, UiTextProfileGeneration, UiTextScaleGeneration, UiTextStyle,
    UiTextStyleInput, UiTextStyleSpan, UiTextWrap,
};

#[test]
fn feature_and_variation_domains_cannot_alias_style_or_request_identity() {
    let feature = style(
        Box::new([UiOpenTypeFeature::new(*b"wght", 700).unwrap()]),
        Box::new([]),
    );
    let variation = style(
        Box::new([]),
        Box::new([UiFontVariationCoordinate::new(*b"wght", 700).unwrap()]),
    );
    assert_ne!(feature.identity_digest(), variation.identity_digest());

    let (fonts, _) = UiGlobalFontCollection::admit_qualified_profile().unwrap();
    let fonts = Arc::new(fonts);
    let feature_request = UiQualifiedTextLayoutRequest::new(input(feature, &fonts), fonts.clone());
    let variation_request = UiQualifiedTextLayoutRequest::new(input(variation, &fonts), fonts);
    assert_ne!(feature_request.identity(), variation_request.identity());
}

fn input(style: UiTextStyle, fonts: &UiGlobalFontCollection) -> UiTextParagraphAdmissionInput {
    let source: Arc<str> = Arc::from("office");
    UiTextParagraphAdmissionInput {
        source: source.clone(),
        constraints: constraints(),
        profile_generation: UiTextProfileGeneration::new(1).unwrap(),
        font_collection_generation: fonts.generation(),
        text_scale_generation: UiTextScaleGeneration::new(1).unwrap(),
        styles: Box::new([UiTextStyleSpan::new(
            UiTextOriginalRange::from_text_mechanics(0, source.len() as u32).unwrap(),
            style,
        )
        .unwrap()]),
    }
}

fn constraints() -> UiTextParagraphConstraints {
    UiTextParagraphConstraints::new(UiTextParagraphConstraintsInput {
        language: Arc::from("und"),
        base_direction: UiTextBaseDirection::Auto,
        wrap: UiTextWrap::UnicodeWord,
        alignment: UiTextAlignment::Start,
        overflow: UiTextOverflow::Clip,
        font_size_millipoints: 14_000,
        width_millipoints: 100_000,
        line_height_millipoints: 18_000,
        letter_spacing_millipoints: 0,
        word_spacing_millipoints: 0,
        tab_interval_millipoints: 56_000,
        maximum_lines: 2,
    })
    .unwrap()
}

fn style(
    features: Box<[UiOpenTypeFeature]>,
    variations: Box<[UiFontVariationCoordinate]>,
) -> UiTextStyle {
    UiTextStyle::new(UiTextStyleInput {
        language: Arc::from("und"),
        font_size_millipoints: 14_000,
        letter_spacing_millipoints: 0,
        word_spacing_millipoints: 0,
        family_stack: UiFontFamilyStack::profile_sans(),
        face_request: UiTextFaceRequest::new(400, 100_000, UiFontSlant::Upright).unwrap(),
        features,
        variations,
    })
    .unwrap()
}
