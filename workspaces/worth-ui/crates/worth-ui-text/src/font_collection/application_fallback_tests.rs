use std::sync::Arc;

use sha2::Digest;
use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiFontSlant, UiQualifiedFontFaceIdentity, UiTextOriginalRange,
};

use super::{
    application_selection_tests::{fallback, shape},
    application_test_world::{face, profile_collection_and_sources},
    UiApplicationFontPackDefinition,
};
use crate::{UiFontFamilyStack, UiTextCoverageDisposition, UiTextFaceRequest};

macro_rules! profile_font {
    ($name:literal) => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../profiles/worth-ui-global-text-v2/fonts/",
            $name
        )
    };
}

#[test]
pub(crate) fn khmer_shaping_syllable_falls_back_and_shapes_as_one_whole_cluster() {
    let (profile, sources) = profile_collection_and_sources();
    let definition = UiApplicationFontPackDefinition {
        name: Arc::from("partial-script fallback pack"),
        faces: Box::new([face(
            "Application Latin",
            sources["noto-sans-roman"].clone(),
            0,
            UiFontSlant::Upright,
        )]),
    };
    let (collection, receipt, _) = profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition)
        .unwrap();
    let collection = Arc::new(collection);
    let latin = receipt.family("Application Latin").unwrap();
    // KA + COENG + RO is one Khmer shaping syllable and one extended grapheme.
    let source = "\u{1780}\u{17D2}\u{179A}";
    let selected = fallback(
        &collection,
        UiFontFamilyStack::new(Box::new([latin])).unwrap(),
        UiTextFaceRequest::regular(),
        Box::new([]),
        source,
    );

    assert_eq!(selected.clusters().len(), 1);
    assert_eq!(
        selected.clusters()[0].original_range(),
        UiTextOriginalRange::from_text_mechanics(0, source.len() as u32).unwrap()
    );
    assert_eq!(
        selected.clusters()[0].face().unwrap(),
        profile_identity(include_bytes!(profile_font!("NotoSansKhmer-VF.ttf")), 0)
    );

    let shaped = shape(
        &collection,
        UiFontFamilyStack::new(Box::new([latin])).unwrap(),
        UiTextFaceRequest::regular(),
        Box::new([]),
        source,
    );
    assert_eq!(shaped.runs().len(), 1);
    assert_eq!(
        shaped.runs()[0].original_range(),
        selected.clusters()[0].original_range()
    );
    assert!(shaped.glyphs().iter().all(|glyph| {
        let range = glyph.original_range();
        range.start() < range.end() && range.end() <= source.len() as u32
    }));
}

#[test]
pub(super) fn authored_and_profile_fallback_use_exact_owned_faces_for_whole_clusters() {
    let (profile, sources) = profile_collection_and_sources();
    let definition = UiApplicationFontPackDefinition {
        name: Arc::from("cluster fallback pack"),
        faces: Box::new([
            face(
                "Application Latin",
                sources["noto-sans-math"].clone(),
                0,
                UiFontSlant::Upright,
            ),
            face(
                "Application CJK",
                sources["noto-sans-cjk-jp"].clone(),
                0,
                UiFontSlant::Upright,
            ),
            face(
                "Application Emoji",
                sources["noto-color-emoji"].clone(),
                0,
                UiFontSlant::Upright,
            ),
        ]),
    };
    let (collection, receipt, _) = profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition)
        .unwrap();
    let collection = Arc::new(collection);
    let latin = receipt.family("Application Latin").unwrap();
    let cjk = receipt.family("Application CJK").unwrap();
    let emoji = receipt.family("Application Emoji").unwrap();
    let selected = fallback(
        &collection,
        UiFontFamilyStack::new(Box::new([latin, cjk, emoji])).unwrap(),
        UiTextFaceRequest::regular(),
        Box::new([]),
        "\u{6F22}\u{1F469}\u{200D}\u{1F4BB}",
    );

    assert_eq!(selected.clusters().len(), 2);
    assert_eq!(
        selected.clusters()[0].face().unwrap(),
        application_face(&receipt, cjk)
    );
    assert_eq!(
        selected.clusters()[0].face().unwrap().font_bytes_digest(),
        font_digest(&sources["noto-sans-cjk-jp"])
    );
    assert_eq!(
        selected.clusters()[1].face().unwrap(),
        application_face(&receipt, emoji)
    );
    assert_eq!(
        selected.clusters()[1].face().unwrap().font_bytes_digest(),
        font_digest(&sources["noto-color-emoji"])
    );
    assert!(selected.clusters()[1].is_rgi_emoji());

    let profile_fallback = fallback(
        &collection,
        UiFontFamilyStack::new(Box::new([emoji])).unwrap(),
        UiTextFaceRequest::regular(),
        Box::new([]),
        "\u{0634}",
    );
    assert_eq!(
        profile_fallback.clusters()[0].face().unwrap(),
        profile_identity(include_bytes!(profile_font!("NotoSansArabic-VF.ttf")), 0)
    );
    assert_eq!(
        profile_fallback.clusters()[0].coverage(),
        UiTextCoverageDisposition::QualifiedFace
    );

    let profile_emoji = fallback(
        &collection,
        UiFontFamilyStack::new(Box::new([latin])).unwrap(),
        UiTextFaceRequest::regular(),
        Box::new([]),
        "\u{1F469}\u{200D}\u{1F4BB}",
    );
    assert_eq!(profile_emoji.clusters().len(), 1);
    assert_eq!(
        profile_emoji.clusters()[0].face().unwrap(),
        profile_identity(include_bytes!(profile_font!("NotoColorEmoji.ttf")), 0)
    );
    assert!(profile_emoji.clusters()[0].is_rgi_emoji());

    let last_resort = fallback(
        &collection,
        UiFontFamilyStack::new(Box::new([latin])).unwrap(),
        UiTextFaceRequest::regular(),
        Box::new([]),
        "\u{0378}",
    );
    assert_eq!(
        last_resort.clusters()[0].coverage(),
        UiTextCoverageDisposition::MissingCluster
    );
    assert_eq!(
        last_resort.clusters()[0].face().unwrap(),
        profile_identity(include_bytes!(profile_font!("LastResort-Regular.ttf")), 0)
    );
    assert_eq!(
        last_resort.clusters()[0]
            .attempted_collection_generation()
            .get(),
        2
    );
}

fn application_face(
    receipt: &super::UiQualifiedFontPackReceipt,
    family: worth_ui_host_contract::UiQualifiedFontFamilyIdentity,
) -> UiQualifiedFontFaceIdentity {
    receipt
        .faces()
        .iter()
        .find(|face| face.family() == family)
        .unwrap()
        .identity()
}

fn profile_identity(bytes: &[u8], face_index: u32) -> UiQualifiedFontFaceIdentity {
    UiQualifiedFontFaceIdentity::from_text_mechanics(font_digest(bytes), face_index)
}

fn font_digest(bytes: &[u8]) -> [u8; 32] {
    sha2::Sha256::digest(bytes).into()
}
