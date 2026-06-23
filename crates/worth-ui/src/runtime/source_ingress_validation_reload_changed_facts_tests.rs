use std::collections::BTreeSet;

use super::source_ingress_authored_delta_test_support::{
    authored_delta_test_app, declaration_rows, layout_changed_source_text,
    layout_gap_changed_source_text, layout_padding_changed_source_text, prepare_validation_reload,
    reordered_source_text, runtime_for_source, semantic_fact_family_rows, semantic_fact_rows,
    shell_reassigned_source_text, source_text,
};
use crate::runtime::{
    WorthUiAuthoredDeclarationKind, WorthUiAuthoredDeltaChangePosture, WorthUiRuntimeFactFamily,
    WorthUiRuntimeFactId, WorthUiSemanticSliceId, WorthUiValidationReloadStatus,
};

#[test]
fn validation_reload_receipt_emits_exact_changed_facts_for_content_repoint() {
    let app = authored_delta_test_app();
    let runtime = runtime_for_source(&app, source_text("validation.surface.products.collection"));
    let prepared = prepare_validation_reload(
        &runtime,
        source_text("validation.surface.orders.collection"),
    );
    let receipt = prepared
        .changed_fact_mapping_receipt()
        .expect("changed source should emit changed-fact proof");

    assert_eq!(
        prepared.evidence().status(),
        WorthUiValidationReloadStatus::ReadyForFrameBoundary
    );
    assert_eq!(
        semantic_fact_rows(receipt),
        BTreeSet::from([
            (
                WorthUiSemanticSliceId::AuthoredMountComponentSelection,
                "surface:validation.surface.orders.collection".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Changed,
                1,
            ),
            (
                WorthUiSemanticSliceId::AuthoredMountComponentSelection,
                "surface:validation.surface.products.collection".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Changed,
                1,
            ),
            (
                WorthUiSemanticSliceId::ContentSlotAssignment,
                "page-slot:ProductsPage:collection".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Changed,
                2,
            ),
            (
                WorthUiSemanticSliceId::SurfaceMountTarget,
                "surface:validation.surface.orders.collection".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Changed,
                1,
            ),
            (
                WorthUiSemanticSliceId::SurfaceMountTarget,
                "surface:validation.surface.products.collection".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Changed,
                1,
            ),
        ])
    );
    assert_eq!(
        semantic_fact_family_rows(receipt),
        BTreeSet::from([
            (
                WorthUiSemanticSliceId::AuthoredMountComponentSelection,
                "surface:validation.surface.orders.collection".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Changed,
                vec![WorthUiRuntimeFactFamily::AuthoredMountComponentSelection],
            ),
            (
                WorthUiSemanticSliceId::AuthoredMountComponentSelection,
                "surface:validation.surface.products.collection".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Changed,
                vec![WorthUiRuntimeFactFamily::AuthoredMountComponentSelection],
            ),
            (
                WorthUiSemanticSliceId::ContentSlotAssignment,
                "page-slot:ProductsPage:collection".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Changed,
                vec![
                    WorthUiRuntimeFactFamily::ContentMount,
                    WorthUiRuntimeFactFamily::PageContentSlot,
                ],
            ),
            (
                WorthUiSemanticSliceId::SurfaceMountTarget,
                "surface:validation.surface.orders.collection".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Changed,
                vec![WorthUiRuntimeFactFamily::SurfaceMount],
            ),
            (
                WorthUiSemanticSliceId::SurfaceMountTarget,
                "surface:validation.surface.products.collection".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Changed,
                vec![WorthUiRuntimeFactFamily::SurfaceMount],
            ),
        ])
    );
    assert_eq!(receipt.changed_facts(), prepared.evidence().changed_facts());
    assert!(receipt
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::content_mount(
            "ProductsPage.collection",
        )));
    assert!(receipt
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::PageContentSlot));
    assert!(receipt
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::surface_mount_raw(
            "validation.surface.orders.collection",
        )));
    assert!(receipt.changed_facts().contains_exact(
        &WorthUiRuntimeFactId::authored_mount_component_selection(
            "validation.surface.products.collection",
        ),
    ));
}

#[test]
fn equivalent_validation_reload_receipt_preserves_zero_changed_facts() {
    let app = authored_delta_test_app();
    let runtime = runtime_for_source(&app, source_text("validation.surface.products.collection"));
    let prepared = prepare_validation_reload(
        &runtime,
        reordered_source_text("validation.surface.products.collection"),
    );
    let receipt = prepared
        .changed_fact_mapping_receipt()
        .expect("equivalent source should still expose authored-delta proof");

    assert_eq!(
        prepared.evidence().status(),
        WorthUiValidationReloadStatus::EquivalentNoOp
    );
    assert!(receipt.rows().is_empty());
    assert!(receipt.changed_facts().is_empty());
    assert!(receipt
        .authored_delta_summary()
        .semantic_slice_rows()
        .is_empty());
    assert!(prepared.evidence().changed_facts().is_empty());
}

#[test]
fn layout_topology_edit_emits_only_layout_topology_fact() {
    let app = authored_delta_test_app();
    let runtime = runtime_for_source(&app, source_text("validation.surface.products.collection"));
    let prepared = prepare_validation_reload(
        &runtime,
        layout_changed_source_text("validation.surface.products.collection"),
    );
    let receipt = prepared
        .changed_fact_mapping_receipt()
        .expect("layout edit should emit changed-fact proof");

    assert_eq!(
        semantic_fact_rows(receipt),
        BTreeSet::from([(
            WorthUiSemanticSliceId::LayoutTopology,
            "page:ProductsPage".to_owned(),
            WorthUiAuthoredDeltaChangePosture::Changed,
            1,
        ),])
    );
    assert_eq!(
        semantic_fact_family_rows(receipt),
        BTreeSet::from([(
            WorthUiSemanticSliceId::LayoutTopology,
            "page:ProductsPage".to_owned(),
            WorthUiAuthoredDeltaChangePosture::Changed,
            vec![WorthUiRuntimeFactFamily::LayoutTopology],
        ),])
    );
    assert!(receipt
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::layout_topology("ProductsPage")));
    assert!(!receipt
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::layout_gap("ProductsPage")));
    assert!(!receipt
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::layout_padding("ProductsPage")));
}

#[test]
fn layout_gap_edit_emits_only_layout_gap_fact() {
    let app = authored_delta_test_app();
    let runtime = runtime_for_source(&app, source_text("validation.surface.products.collection"));
    let prepared = prepare_validation_reload(
        &runtime,
        layout_gap_changed_source_text("validation.surface.products.collection"),
    );
    let receipt = prepared
        .changed_fact_mapping_receipt()
        .expect("layout gap edit should emit changed-fact proof");

    assert_eq!(
        semantic_fact_rows(receipt),
        BTreeSet::from([(
            WorthUiSemanticSliceId::LayoutGapRule,
            "page:ProductsPage".to_owned(),
            WorthUiAuthoredDeltaChangePosture::Changed,
            1,
        )])
    );
    assert_eq!(
        semantic_fact_family_rows(receipt),
        BTreeSet::from([(
            WorthUiSemanticSliceId::LayoutGapRule,
            "page:ProductsPage".to_owned(),
            WorthUiAuthoredDeltaChangePosture::Changed,
            vec![WorthUiRuntimeFactFamily::LayoutGap],
        )])
    );
    assert!(receipt
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::layout_gap("ProductsPage")));
    assert!(!receipt
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::layout_topology("ProductsPage")));
    assert!(receipt
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::layout_gap("ProductsPage")));
    assert!(!receipt
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::layout_padding("ProductsPage")));
}

#[test]
fn layout_padding_edit_emits_only_layout_padding_fact() {
    let app = authored_delta_test_app();
    let runtime = runtime_for_source(&app, source_text("validation.surface.products.collection"));
    let prepared = prepare_validation_reload(
        &runtime,
        layout_padding_changed_source_text("validation.surface.products.collection"),
    );
    let receipt = prepared
        .changed_fact_mapping_receipt()
        .expect("layout padding edit should emit changed-fact proof");

    assert_eq!(
        semantic_fact_rows(receipt),
        BTreeSet::from([(
            WorthUiSemanticSliceId::LayoutPaddingRule,
            "page:ProductsPage".to_owned(),
            WorthUiAuthoredDeltaChangePosture::Changed,
            1,
        )])
    );
    assert_eq!(
        semantic_fact_family_rows(receipt),
        BTreeSet::from([(
            WorthUiSemanticSliceId::LayoutPaddingRule,
            "page:ProductsPage".to_owned(),
            WorthUiAuthoredDeltaChangePosture::Changed,
            vec![WorthUiRuntimeFactFamily::LayoutPadding],
        )])
    );
    assert!(!receipt
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::layout_topology("ProductsPage")));
    assert!(!receipt
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::layout_gap("ProductsPage")));
    assert!(receipt
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::layout_padding("ProductsPage")));
}

#[test]
fn workspace_shell_edit_emits_shell_slot_assignment_fact() {
    let app = authored_delta_test_app();
    let runtime = runtime_for_source(&app, source_text("validation.surface.products.collection"));
    let prepared = prepare_validation_reload(&runtime, shell_reassigned_source_text());
    let receipt = prepared
        .changed_fact_mapping_receipt()
        .expect("shell edit should emit changed-fact proof");

    assert_eq!(
        declaration_rows(receipt.authored_delta_summary()),
        BTreeSet::from([(
            WorthUiAuthoredDeclarationKind::Workspace,
            "AdminWorkspace".to_owned(),
            WorthUiAuthoredDeltaChangePosture::Changed,
        )])
    );
    assert_eq!(
        semantic_fact_rows(receipt),
        BTreeSet::from([(
            WorthUiSemanticSliceId::ShellSlotAssignment,
            "workspace:AdminWorkspace".to_owned(),
            WorthUiAuthoredDeltaChangePosture::Changed,
            1,
        )])
    );
    assert_eq!(
        semantic_fact_family_rows(receipt),
        BTreeSet::from([(
            WorthUiSemanticSliceId::ShellSlotAssignment,
            "workspace:AdminWorkspace".to_owned(),
            WorthUiAuthoredDeltaChangePosture::Changed,
            vec![WorthUiRuntimeFactFamily::ShellSlotAssignment],
        )])
    );
    assert!(receipt
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::shell_slot_assignment(
            "AdminWorkspace",
        )));
}

#[test]
fn distinct_semantic_edits_produce_distinct_authored_delta_digests() {
    let app = authored_delta_test_app();
    let runtime = runtime_for_source(&app, source_text("validation.surface.products.collection"));
    let content_receipt = prepare_validation_reload(
        &runtime,
        source_text("validation.surface.orders.collection"),
    )
    .changed_fact_mapping_receipt()
    .expect("content edit should emit changed-fact proof")
    .clone();
    let shell_receipt = prepare_validation_reload(&runtime, shell_reassigned_source_text())
        .changed_fact_mapping_receipt()
        .expect("shell edit should emit changed-fact proof")
        .clone();

    assert_ne!(
        content_receipt.authored_delta_summary().digest(),
        shell_receipt.authored_delta_summary().digest()
    );
    assert_ne!(content_receipt.rows(), shell_receipt.rows());
}

#[test]
fn replay_of_the_same_source_edit_produces_the_same_authored_delta_digest() {
    let app = authored_delta_test_app();
    let runtime = runtime_for_source(&app, source_text("validation.surface.products.collection"));
    let first = prepare_validation_reload(
        &runtime,
        source_text("validation.surface.orders.collection"),
    );
    let second = prepare_validation_reload(
        &runtime,
        source_text("validation.surface.orders.collection"),
    );

    let first_digest = first
        .changed_fact_mapping_receipt()
        .expect("replayed source edit should emit changed-fact proof")
        .authored_delta_summary()
        .digest();
    let second_digest = second
        .changed_fact_mapping_receipt()
        .expect("replayed source edit should emit changed-fact proof")
        .authored_delta_summary()
        .digest();

    assert_eq!(first_digest, second_digest);
}
