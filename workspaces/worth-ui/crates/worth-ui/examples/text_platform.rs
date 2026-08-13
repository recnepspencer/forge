use std::sync::Arc;

use worth_ui::facade::app::{
    qualify_text_layout, UiApplicationFontFaceDefinition, UiApplicationFontLicenseRecord,
    UiApplicationFontPackDefinition, UiFontCollectionGeneration, UiFontFamilyStack, UiFontSlant,
    UiFontVariationCoordinate, UiGlobalFontCollection, UiOpenTypeFeature, UiTextAlignment,
    UiTextBaseDirection, UiTextFaceRequest, UiTextOriginalRange, UiTextOverflow,
    UiTextParagraphAdmissionInput, UiTextParagraphConstraints, UiTextParagraphConstraintsInput,
    UiTextProfileGeneration, UiTextScaleGeneration, UiTextStyle, UiTextStyleInput, UiTextStyleSpan,
    UiTextWrap,
};

const APP_FONT: &[u8] =
    include_bytes!("../../../profiles/worth-ui-global-text-v2/fonts/NotoSans-VF.ttf");

fn main() {
    let (fonts, app_sans) = application_fonts();
    let source: Arc<str> = Arc::from("Variable office — مرحبا — 👩🏽‍💻");
    let style = application_style(app_sans);
    let layout = qualify_text_layout(
        UiTextParagraphAdmissionInput {
            styles: Box::new([whole_span(&source, style)]),
            source,
            constraints: paragraph_constraints(),
            profile_generation: UiTextProfileGeneration::new(1).expect("profile generation"),
            font_collection_generation: fonts.generation(),
            text_scale_generation: UiTextScaleGeneration::new(1).expect("scale generation"),
        },
        fonts,
    )
    .expect("text qualification");
    println!(
        "qualified {} glyphs, {} lines, {} caret stops",
        layout.glyphs().len(),
        layout.lines().len(),
        layout.carets().len()
    );
}

fn application_fonts() -> (
    Arc<UiGlobalFontCollection>,
    worth_ui::facade::app::UiQualifiedFontFamilyIdentity,
) {
    let (profile, _) =
        UiGlobalFontCollection::admit_qualified_profile().expect("qualified embedded profile");
    let (fonts, pack, _) = profile
        .register_application_pack(
            UiFontCollectionGeneration::new(2).expect("successor generation"),
            UiApplicationFontPackDefinition {
                name: Arc::from("application typography"),
                faces: Box::new([UiApplicationFontFaceDefinition {
                    family: Arc::from("App Sans"),
                    bytes: Arc::from(APP_FONT),
                    face_index: 0,
                    weight: 400,
                    width_milli_percent: 100_000,
                    slant: UiFontSlant::Upright,
                    license: UiApplicationFontLicenseRecord {
                        identifier: Arc::from("OFL-1.1"),
                        notice: Arc::from("Copyright 2022 Google LLC"),
                    },
                }]),
            },
        )
        .expect("application font pack admission");
    (
        Arc::new(fonts),
        pack.family("App Sans").expect("admitted family receipt"),
    )
}

fn application_style(
    app_sans: worth_ui::facade::app::UiQualifiedFontFamilyIdentity,
) -> UiTextStyle {
    UiTextStyle::new(UiTextStyleInput {
        language: Arc::from("und"),
        font_size_millipoints: 16_000,
        letter_spacing_millipoints: 0,
        word_spacing_millipoints: 0,
        family_stack: UiFontFamilyStack::new(Box::new([app_sans])).expect("ordered family stack"),
        face_request: UiTextFaceRequest::new(600, 100_000, UiFontSlant::Upright)
            .expect("face request"),
        features: Box::new([UiOpenTypeFeature::new(*b"liga", 1).expect("feature")]),
        variations: Box::new([UiFontVariationCoordinate::new(*b"wght", 600_000).expect("axis")]),
    })
    .expect("qualified style request")
}

fn whole_span(source: &str, style: UiTextStyle) -> UiTextStyleSpan {
    UiTextStyleSpan::new(
        UiTextOriginalRange::from_text_mechanics(0, source.len() as u32)
            .expect("whole source range"),
        style,
    )
    .expect("nonempty style span")
}

fn paragraph_constraints() -> UiTextParagraphConstraints {
    UiTextParagraphConstraints::new(UiTextParagraphConstraintsInput {
        language: Arc::from("und"),
        base_direction: UiTextBaseDirection::Auto,
        wrap: UiTextWrap::UnicodeWord,
        alignment: UiTextAlignment::Start,
        overflow: UiTextOverflow::Clip,
        font_size_millipoints: 16_000,
        width_millipoints: 320_000,
        line_height_millipoints: 21_000,
        letter_spacing_millipoints: 0,
        word_spacing_millipoints: 0,
        tab_interval_millipoints: 64_000,
        maximum_lines: 4,
    })
    .expect("qualified paragraph constraints")
}
