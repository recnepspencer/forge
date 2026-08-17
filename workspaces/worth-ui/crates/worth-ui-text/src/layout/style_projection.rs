use worth_ui_host_contract::{
    UiQualifiedTextFeatureRecord, UiQualifiedTextStyleInput, UiQualifiedTextStyleRecord,
    UiQualifiedTextVariationRecord,
};

use crate::UiShapedTextParagraph;

pub(super) fn records(shaped: &UiShapedTextParagraph) -> Box<[UiQualifiedTextStyleRecord]> {
    shaped
        .styles()
        .iter()
        .map(|span| {
            let style = span.style();
            UiQualifiedTextStyleRecord::from_text_mechanics(UiQualifiedTextStyleInput {
                original_range: span.original_range(),
                language: style.language().into(),
                font_size_millipoints: style.font_size_millipoints(),
                letter_spacing_millipoints: style.letter_spacing_millipoints(),
                word_spacing_millipoints: style.word_spacing_millipoints(),
                family_stack: style.family_stack().families().into(),
                weight: style.face_request().weight(),
                width_milli_percent: style.face_request().width_milli_percent(),
                slant: style.face_request().slant(),
                features: style
                    .features()
                    .iter()
                    .map(|feature| {
                        UiQualifiedTextFeatureRecord::from_text_mechanics(
                            feature.tag(),
                            feature.value(),
                        )
                    })
                    .collect(),
                variations: style
                    .variations()
                    .iter()
                    .map(|variation| {
                        UiQualifiedTextVariationRecord::from_text_mechanics(
                            variation.axis(),
                            variation.value_milli(),
                        )
                    })
                    .collect(),
            })
        })
        .collect()
}
