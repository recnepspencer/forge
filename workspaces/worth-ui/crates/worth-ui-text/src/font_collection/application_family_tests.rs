use std::sync::Arc;

use worth_ui_host_contract::{UiFontCollectionGeneration, UiFontSlant};

use super::{
    application_test_world::{
        face, layout, profile_collection_and_sources, selected_face, static_face_bytes,
    },
    UiApplicationFontPackDefinition,
};
use crate::{UiFontFamilyStack, UiTextFaceRequest};

#[test]
pub(super) fn ordered_application_families_and_face_attributes_select_owned_bytes() {
    let (profile, sources) = profile_collection_and_sources();
    let definition = UiApplicationFontPackDefinition {
        name: Arc::from("application typography"),
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
            face(
                "Styles",
                sources["noto-sans-roman"].clone(),
                0,
                UiFontSlant::Upright,
            ),
            face(
                "Styles",
                sources["noto-sans-italic"].clone(),
                0,
                UiFontSlant::Italic,
            ),
        ]),
    };
    let (collection, receipt, _) = profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition)
        .unwrap();
    let alpha = receipt.family("Alpha").unwrap();
    let beta = receipt.family("Beta").unwrap();
    let styles = receipt.family("Styles").unwrap();
    let alpha_face = receipt
        .faces()
        .iter()
        .find(|face| face.family() == alpha)
        .unwrap();
    let beta_face = receipt
        .faces()
        .iter()
        .find(|face| face.family() == beta)
        .unwrap();
    let styles_regular = receipt
        .faces()
        .iter()
        .find(|face| face.family() == styles && face.slant() == UiFontSlant::Upright)
        .unwrap();
    let styles_italic = receipt
        .faces()
        .iter()
        .find(|face| face.family() == styles && face.slant() == UiFontSlant::Italic)
        .unwrap();
    let collection = Arc::new(collection);
    assert_eq!(
        alpha_face.identity().font_bytes_digest(),
        styles_regular.identity().font_bytes_digest()
    );
    assert_ne!(alpha_face.identity(), styles_regular.identity());
    assert_eq!(
        selected_face(
            Arc::clone(&collection),
            UiFontFamilyStack::new(Box::new([alpha, beta])).unwrap(),
            UiTextFaceRequest::regular(),
        ),
        alpha_face.identity()
    );
    assert_eq!(
        selected_face(
            Arc::clone(&collection),
            UiFontFamilyStack::new(Box::new([beta, alpha])).unwrap(),
            UiTextFaceRequest::regular(),
        ),
        beta_face.identity()
    );
    assert_eq!(
        selected_face(
            Arc::clone(&collection),
            UiFontFamilyStack::new(Box::new([styles])).unwrap(),
            UiTextFaceRequest::new(400, 100_000, UiFontSlant::Italic).unwrap(),
        ),
        styles_italic.identity()
    );
    let qualified = layout(collection, styles, "office");
    let resource = qualified
        .artifact()
        .face_resource(styles_regular.identity())
        .unwrap();
    assert_eq!(resource.family(), styles);
    assert_eq!(resource.pack(), Some(receipt.identity()));
}

#[test]
pub(super) fn static_regular_bold_italic_and_oblique_faces_match_exact_requests() {
    let (profile, _) = profile_collection_and_sources();
    let base = include_bytes!("../../../worth-ui-host-native/assets/fonts/NotoSans-Regular.ttf");
    let styles = [
        (400, UiFontSlant::Upright),
        (700, UiFontSlant::Upright),
        (400, UiFontSlant::Italic),
        (400, UiFontSlant::Oblique),
    ];
    let definition = UiApplicationFontPackDefinition {
        name: Arc::from("static application styles"),
        faces: styles
            .map(|(weight, slant)| super::UiApplicationFontFaceDefinition {
                weight,
                ..face(
                    "Static Styles",
                    static_face_bytes(base, weight, slant),
                    0,
                    slant,
                )
            })
            .into(),
    };
    let (collection, receipt, _) = profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition)
        .unwrap();
    let family = receipt.family("Static Styles").unwrap();
    let stack = UiFontFamilyStack::new(Box::new([family])).unwrap();
    let collection = Arc::new(collection);
    for (weight, slant) in styles {
        let expected = receipt
            .faces()
            .iter()
            .find(|face| face.weight() == weight && face.slant() == slant)
            .unwrap();
        assert_eq!(
            selected_face(
                Arc::clone(&collection),
                stack.clone(),
                UiTextFaceRequest::new(weight, 100_000, slant).unwrap(),
            ),
            expected.identity()
        );
    }
}

#[test]
pub(super) fn same_named_families_in_distinct_packs_never_merge_authority() {
    let (profile, sources) = profile_collection_and_sources();
    let definition = |name: &'static str, bytes: Arc<[u8]>| UiApplicationFontPackDefinition {
        name: Arc::from(name),
        faces: Box::new([face("Shared Name", bytes, 0, UiFontSlant::Upright)]),
    };
    let (first, first_receipt, _) = profile
        .register_application_pack(
            UiFontCollectionGeneration::new(2).unwrap(),
            definition("first pack", sources["noto-sans-roman"].clone()),
        )
        .unwrap();
    let (second, second_receipt, _) = first
        .register_application_pack(
            UiFontCollectionGeneration::new(3).unwrap(),
            definition("second pack", sources["noto-sans-roman"].clone()),
        )
        .unwrap();
    let first_family = first_receipt.family("Shared Name").unwrap();
    let second_family = second_receipt.family("Shared Name").unwrap();
    assert_ne!(first_family, second_family);

    let collection = Arc::new(second);
    for (family, expected) in [
        (first_family, first_receipt.faces()[0].identity()),
        (second_family, second_receipt.faces()[0].identity()),
    ] {
        assert_eq!(
            selected_face(
                Arc::clone(&collection),
                UiFontFamilyStack::new(Box::new([family])).unwrap(),
                UiTextFaceRequest::regular(),
            ),
            expected,
        );
    }
}
