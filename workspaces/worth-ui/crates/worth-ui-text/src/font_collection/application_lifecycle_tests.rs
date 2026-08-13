use std::sync::Arc;

use worth_ui_host_contract::{UiFontCollectionGeneration, UiFontSlant};

use super::{
    application_selection_tests::selected_faces,
    application_test_world::{face, profile_collection_and_sources},
    UiApplicationFontPackDefinition,
};
use crate::{UiFontFamilyStack, UiTextFaceRequest};

#[test]
pub(super) fn multiple_packs_never_make_registration_order_a_selector() {
    let (profile, sources) = profile_collection_and_sources();
    let (reverse_profile, _) = profile_collection_and_sources();
    let alpha = || UiApplicationFontPackDefinition {
        name: Arc::from("alpha pack"),
        faces: Box::new([face(
            "Alpha",
            sources["noto-sans-roman"].clone(),
            0,
            UiFontSlant::Upright,
        )]),
    };
    let beta = || UiApplicationFontPackDefinition {
        name: Arc::from("beta pack"),
        faces: Box::new([face(
            "Beta",
            sources["noto-sans-italic"].clone(),
            0,
            UiFontSlant::Italic,
        )]),
    };

    let (alpha_first, alpha_early, _) = profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), alpha())
        .unwrap();
    let (alpha_then_beta, beta_late, _) = alpha_first
        .register_application_pack(UiFontCollectionGeneration::new(3).unwrap(), beta())
        .unwrap();
    let (beta_first, beta_early, _) = reverse_profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), beta())
        .unwrap();
    let (beta_then_alpha, alpha_late, _) = beta_first
        .register_application_pack(UiFontCollectionGeneration::new(3).unwrap(), alpha())
        .unwrap();

    assert_eq!(alpha_early.identity(), alpha_late.identity());
    assert_eq!(beta_early.identity(), beta_late.identity());
    let alpha_family = alpha_early.family("Alpha").unwrap();
    let beta_family = beta_early.family("Beta").unwrap();
    let stack = UiFontFamilyStack::new(Box::new([alpha_family, beta_family])).unwrap();
    let alpha_then_beta = Arc::new(alpha_then_beta);
    let beta_then_alpha = Arc::new(beta_then_alpha);
    assert_eq!(
        selected_faces(
            &alpha_then_beta,
            stack.clone(),
            UiTextFaceRequest::regular(),
            "office",
        ),
        selected_faces(
            &beta_then_alpha,
            stack.clone(),
            UiTextFaceRequest::regular(),
            "office",
        )
    );

    assert_eq!(alpha_then_beta.generation().get(), 3);
}
