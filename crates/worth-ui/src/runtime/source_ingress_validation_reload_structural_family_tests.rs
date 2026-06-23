use std::collections::BTreeSet;

use super::source_ingress_authored_delta_test_support::{
    appearance_recipe_renamed_source_text, authored_delta_test_app, declaration_rows,
    mixed_content_and_appearance_source_text, prepare_validation_reload, runtime_for_source,
    semantic_fact_family_rows, semantic_fact_rows, source_text,
};
use crate::runtime::{
    WorthUiAppearanceRecipeId, WorthUiAuthoredDeclarationKind, WorthUiAuthoredDeltaChangePosture,
    WorthUiRuntimeFactFamily, WorthUiRuntimeFactId, WorthUiSemanticSliceId,
};

#[test]
fn appearance_recipe_rename_emits_real_appearance_recipe_facts() {
    let app = authored_delta_test_app();
    let runtime = runtime_for_source(&app, source_text("validation.surface.products.collection"));
    let prepared = prepare_validation_reload(
        &runtime,
        appearance_recipe_renamed_source_text("validation.surface.products.collection"),
    );
    let receipt = prepared
        .changed_fact_mapping_receipt()
        .expect("appearance recipe rename should emit changed-fact proof");

    assert_eq!(
        declaration_rows(receipt.authored_delta_summary()),
        BTreeSet::from([
            (
                WorthUiAuthoredDeclarationKind::Appearance,
                "ShopifyAdminTheme".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Removed,
            ),
            (
                WorthUiAuthoredDeclarationKind::Appearance,
                "ShopifyAdminThemeAlt".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Added,
            ),
        ])
    );
    assert_eq!(
        semantic_fact_rows(receipt),
        BTreeSet::from([
            (
                WorthUiSemanticSliceId::AppearanceRecipe,
                "appearance:ShopifyAdminTheme".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Removed,
                1,
            ),
            (
                WorthUiSemanticSliceId::AppearanceRecipe,
                "appearance:ShopifyAdminThemeAlt".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Added,
                1,
            ),
        ])
    );
    assert_eq!(
        semantic_fact_family_rows(receipt),
        BTreeSet::from([
            (
                WorthUiSemanticSliceId::AppearanceRecipe,
                "appearance:ShopifyAdminTheme".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Removed,
                vec![WorthUiRuntimeFactFamily::AppearanceRecipe],
            ),
            (
                WorthUiSemanticSliceId::AppearanceRecipe,
                "appearance:ShopifyAdminThemeAlt".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Added,
                vec![WorthUiRuntimeFactFamily::AppearanceRecipe],
            ),
        ])
    );
    assert!(receipt
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::appearance_recipe(
            &WorthUiAppearanceRecipeId::new("ShopifyAdminTheme").unwrap(),
        )));
    assert!(receipt
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::appearance_recipe(
            &WorthUiAppearanceRecipeId::new("ShopifyAdminThemeAlt").unwrap(),
        )));
}

#[test]
fn mixed_authored_save_carries_multiple_structural_fact_families_together() {
    let app = authored_delta_test_app();
    let runtime = runtime_for_source(&app, source_text("validation.surface.products.collection"));
    let prepared = prepare_validation_reload(&runtime, mixed_content_and_appearance_source_text());
    let receipt = prepared
        .changed_fact_mapping_receipt()
        .expect("mixed authored save should emit changed-fact proof");

    assert_eq!(
        declaration_rows(receipt.authored_delta_summary()),
        BTreeSet::from([
            (
                WorthUiAuthoredDeclarationKind::Appearance,
                "ShopifyAdminTheme".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Removed,
            ),
            (
                WorthUiAuthoredDeclarationKind::Appearance,
                "ShopifyAdminThemeAlt".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Added,
            ),
            (
                WorthUiAuthoredDeclarationKind::Content,
                "ProductsPage".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Changed,
            ),
        ])
    );
    assert_eq!(semantic_fact_rows(receipt).len(), 7);
    assert!(receipt
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::AppearanceRecipe));
    assert!(receipt
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::ContentMount));
    assert!(receipt
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::PageContentSlot));
    assert!(receipt
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::SurfaceMount));
    assert!(receipt
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::AuthoredMountComponentSelection,));
}
