use std::sync::Arc;

use worth_ui_host_contract::{UiFontCollectionGeneration, UiFontSlant};

use super::{
    application_selection_tests::try_fallback,
    application_test_world::{face, profile_collection_and_sources},
    UiApplicationFontPackDefinition,
};
use crate::{UiFontFamilyStack, UiOpenTypeFeature, UiTextFaceRequest, UiTextFallbackDenial};

#[test]
pub(super) fn unsupported_explicit_feature_is_denied_before_shaping() {
    let (profile, sources) = profile_collection_and_sources();
    let (collection, receipt, _) = profile
        .register_application_pack(
            UiFontCollectionGeneration::new(2).unwrap(),
            UiApplicationFontPackDefinition {
                name: Arc::from("feature inventory"),
                faces: Box::new([face(
                    "Application",
                    Arc::clone(&sources["noto-sans-roman"]),
                    0,
                    UiFontSlant::Upright,
                )]),
            },
        )
        .unwrap();
    let family = receipt.family("Application").unwrap();
    let denial = match try_fallback(
        &Arc::new(collection),
        UiFontFamilyStack::new(Box::new([family])).unwrap(),
        UiTextFaceRequest::regular(),
        Box::new([UiOpenTypeFeature::new(*b"ZZZZ", 1).unwrap()]),
        "office",
    ) {
        Ok(_) => panic!("unsupported explicit feature reached shaping"),
        Err(denial) => denial,
    };

    assert_eq!(denial, UiTextFallbackDenial::UnsupportedOpenTypeFeature);
}

#[test]
pub(super) fn authored_latin_features_do_not_split_or_block_color_emoji_fallback() {
    let (profile, _) = profile_collection_and_sources();
    let emoji_family = crate::font_family::profile_family_identity("noto-color-emoji");
    let selected = try_fallback(
        &Arc::new(profile),
        UiFontFamilyStack::new(Box::new([emoji_family])).unwrap(),
        UiTextFaceRequest::regular(),
        Box::new([UiOpenTypeFeature::new(*b"liga", 1).unwrap()]),
        "\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}\u{200D}\u{1F466}",
    )
    .unwrap();

    assert_eq!(selected.clusters().len(), 1);
    assert!(selected.clusters()[0].is_rgi_emoji());
}
