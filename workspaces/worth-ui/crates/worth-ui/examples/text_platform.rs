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
use worth_ui::facade::declaration::{
    ComponentSemanticTextContract, ComponentSemanticTextSpanContract, ThemeTokenId,
};

const APP_FONT: &[u8] =
    include_bytes!("../../../profiles/worth-ui-global-text-v2/fonts/NotoSans-VF.ttf");

fn main() {
    let (fonts, app_sans, app_display) = application_fonts();
    let source: Arc<str> = Arc::from("Variable office — مرحبا — 👩🏽‍💻");
    let arabic_start = source.find("مرحبا").expect("Arabic source range") as u32;
    let emoji_start = source.find('👩').expect("emoji source range") as u32;
    let latin_style = application_style("en", Box::new([app_display, app_sans]), 700, *b"liga");
    let arabic_style = application_style("ar", Box::new([app_sans, app_display]), 500, *b"kern");
    let emoji_style = application_style("und", Box::new([app_sans, app_display]), 600, *b"liga");
    let paint = mixed_paint_contract(
        arabic_start,
        emoji_start,
        source.len() as u32,
        [&latin_style, &arabic_style, &emoji_style],
    );
    let layout = qualify_text_layout(
        UiTextParagraphAdmissionInput {
            styles: Box::new([
                span(0, arabic_start, latin_style),
                span(arabic_start, emoji_start, arabic_style),
                span(emoji_start, source.len() as u32, emoji_style),
            ]),
            source,
            constraints: paragraph_constraints(),
            profile_generation: UiTextProfileGeneration::new(1).expect("profile generation"),
            font_collection_generation: fonts.generation(),
            text_scale_generation: UiTextScaleGeneration::new(1).expect("scale generation"),
        },
        fonts,
    )
    .expect("text qualification");
    let first_line = layout.lines().first().expect("qualified line");
    println!(
        "qualified {} glyphs, {} lines, {} caret stops, {} paint spans; first line logical={}x{} ink={}x{} millipoints",
        layout.glyphs().len(),
        layout.lines().len(),
        layout.carets().len(),
        paint.scalar_spans().len(),
        first_line.logical_bounds().width_millipoints(),
        first_line.logical_bounds().height_millipoints(),
        first_line.ink_bounds().width_millipoints(),
        first_line.ink_bounds().height_millipoints(),
    );
}

fn mixed_paint_contract(
    arabic_start: u32,
    emoji_start: u32,
    end: u32,
    styles: [&UiTextStyle; 3],
) -> ComponentSemanticTextContract {
    let primary = ThemeTokenId::new("text.primary").expect("primary paint token");
    let accent = ThemeTokenId::new("text.accent").expect("accent paint token");
    ComponentSemanticTextContract::spanned(
        primary.clone(),
        0,
        [
            paint_span(0, arabic_start, primary.clone(), styles[0].clone()),
            paint_span(arabic_start, emoji_start, accent, styles[1].clone()),
            paint_span(emoji_start, end, primary, styles[2].clone()),
        ],
    )
    .expect("contiguous original-range paint spans")
}

fn paint_span(
    start: u32,
    end: u32,
    token: ThemeTokenId,
    style: UiTextStyle,
) -> ComponentSemanticTextSpanContract {
    ComponentSemanticTextSpanContract::new(range(start, end), token, style)
        .expect("nonempty paint span")
}

fn application_fonts() -> (
    Arc<UiGlobalFontCollection>,
    worth_ui::facade::app::UiQualifiedFontFamilyIdentity,
    worth_ui::facade::app::UiQualifiedFontFamilyIdentity,
) {
    let (profile, _) =
        UiGlobalFontCollection::admit_qualified_profile().expect("qualified embedded profile");
    let (fonts, pack, _) = profile
        .register_application_pack(
            UiFontCollectionGeneration::new(2).expect("successor generation"),
            UiApplicationFontPackDefinition {
                name: Arc::from("application typography"),
                faces: Box::new([
                    application_face("App Sans"),
                    application_face("App Display"),
                ]),
            },
        )
        .expect("application font pack admission");
    (
        Arc::new(fonts),
        pack.family("App Sans").expect("admitted family receipt"),
        pack.family("App Display")
            .expect("second admitted family receipt"),
    )
}

fn application_face(family: &'static str) -> UiApplicationFontFaceDefinition {
    UiApplicationFontFaceDefinition {
        family: Arc::from(family),
        bytes: Arc::from(APP_FONT),
        face_index: 0,
        weight: 400,
        width_milli_percent: 100_000,
        slant: UiFontSlant::Upright,
        license: UiApplicationFontLicenseRecord {
            identifier: Arc::from("OFL-1.1"),
            notice: Arc::from("Copyright 2022 Google LLC"),
        },
    }
}

fn application_style(
    language: &'static str,
    families: Box<[worth_ui::facade::app::UiQualifiedFontFamilyIdentity]>,
    weight: u16,
    feature: [u8; 4],
) -> UiTextStyle {
    UiTextStyle::new(UiTextStyleInput {
        language: Arc::from(language),
        font_size_millipoints: 16_000,
        letter_spacing_millipoints: 0,
        word_spacing_millipoints: 0,
        family_stack: UiFontFamilyStack::new(families).expect("ordered family stack"),
        face_request: UiTextFaceRequest::new(weight, 100_000, UiFontSlant::Upright)
            .expect("face request"),
        features: Box::new([UiOpenTypeFeature::new(feature, 1).expect("feature")]),
        variations: Box::new([
            UiFontVariationCoordinate::new(*b"wght", i32::from(weight) * 1_000).expect("axis"),
        ]),
    })
    .expect("qualified style request")
}

fn span(start: u32, end: u32, style: UiTextStyle) -> UiTextStyleSpan {
    UiTextStyleSpan::new(range(start, end), style).expect("nonempty style span")
}

fn range(start: u32, end: u32) -> UiTextOriginalRange {
    UiTextOriginalRange::from_text_mechanics(start, end).expect("source range")
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
