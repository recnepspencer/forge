use super::source_ingress_authored_delta_test_support::{
    authored_delta_test_app, changed_source_provider, declaration_rows, observed_authored_edit,
    page_added_source_text, runtime_binding_added_source_text, runtime_for_source, semantic_rows,
    source_text,
};
use super::source_ingress_authored_package_test_support::{
    observed_authored_edit_for_modules, packaged_source_modules, runtime_for_modules,
};
use crate::facade::WorthUi;
use crate::runtime::source_ingress_test_support::{
    empty_artifact, runtime_from_artifact, rust_import_provider,
};
use crate::runtime::{
    WorthUiAuthoredDeclarationKind, WorthUiAuthoredDeltaChangePosture, WorthUiObservedAuthoredEdit,
    WorthUiSemanticSliceId, WorthUiSourceAuthoredCandidateSubmissionDenial,
};

#[test]
fn file_authored_candidate_submission_emits_canonical_authored_delta_summary() {
    let app = authored_delta_test_app();
    let runtime = runtime_for_source(&app, source_text("validation.surface.products.collection"));
    let submission = runtime
        .observe_authored_edit(
            app.capabilities(),
            WorthUiObservedAuthoredEdit::from_source_provider(changed_source_provider())
                .expect("changed source provider is a real observed edit"),
        )
        .expect("file-authored submission lowers from the runtime-owned observed-edit seam");
    let authored_delta = submission
        .authored_delta_summary()
        .expect("file-authored submission emits authored delta");

    assert_eq!(authored_delta.counters().observed_modules(), 1);
    assert_eq!(authored_delta.counters().parsed_modules(), 1);
    assert_eq!(authored_delta.counters().authored_declarations_touched(), 1);
    assert_eq!(
        declaration_rows(authored_delta),
        std::collections::BTreeSet::from([(
            WorthUiAuthoredDeclarationKind::Content,
            "ProductsPage".to_owned(),
            WorthUiAuthoredDeltaChangePosture::Changed,
        )])
    );
    assert_eq!(
        semantic_rows(authored_delta),
        std::collections::BTreeSet::from([
            (
                WorthUiSemanticSliceId::AuthoredMountComponentSelection,
                "surface:validation.surface.orders.collection".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Changed,
            ),
            (
                WorthUiSemanticSliceId::AuthoredMountComponentSelection,
                "surface:validation.surface.products.collection".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Changed,
            ),
            (
                WorthUiSemanticSliceId::ContentSlotAssignment,
                "page-slot:ProductsPage:collection".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Changed,
            ),
            (
                WorthUiSemanticSliceId::SurfaceMountTarget,
                "surface:validation.surface.orders.collection".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Changed,
            ),
            (
                WorthUiSemanticSliceId::SurfaceMountTarget,
                "surface:validation.surface.products.collection".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Changed,
            ),
        ])
    );
}

#[test]
fn rust_authored_candidate_submission_does_not_mint_authored_delta_summary() {
    let app = WorthUi::app().freeze();
    let submission = runtime_from_artifact(empty_artifact())
        .observe_authored_edit(
            app.capabilities(),
            WorthUiObservedAuthoredEdit::from_source_provider(rust_import_provider())
                .expect("rust-authored provider is still a real observed edit"),
        )
        .expect("rust-authored submission lowers");

    assert!(submission.authored_delta_summary().is_none());
    assert_eq!(submission.counters().authored_declarations_inspected(), 0);
    assert_eq!(submission.counters().semantic_slices_emitted(), 0);
    assert_eq!(
        submission.into_source_authored_submission(),
        Err(WorthUiSourceAuthoredCandidateSubmissionDenial::MissingAuthoredDeltaProof)
    );
}

#[test]
fn multi_module_file_authored_submission_reports_package_breadth_without_widening_touch_scope() {
    let app = authored_delta_test_app();
    let runtime = runtime_for_modules(
        &app,
        packaged_source_modules("validation.surface.products.collection"),
    );
    let submission = runtime
        .observe_authored_edit(
            app.capabilities(),
            observed_authored_edit_for_modules(packaged_source_modules(
                "validation.surface.orders.collection",
            )),
        )
        .expect("multi-module observed edit should lower from the runtime-owned seam");
    let authored_delta = submission
        .authored_delta_summary()
        .expect("file-authored package emits authored delta");

    assert_eq!(authored_delta.counters().observed_modules(), 2);
    assert_eq!(authored_delta.counters().parsed_modules(), 2);
    assert_eq!(authored_delta.counters().authored_declarations_touched(), 1);
    assert_eq!(
        declaration_rows(authored_delta),
        std::collections::BTreeSet::from([(
            WorthUiAuthoredDeclarationKind::Content,
            "ProductsPage".to_owned(),
            WorthUiAuthoredDeltaChangePosture::Changed,
        )])
    );
    assert!(semantic_rows(authored_delta)
        .iter()
        .all(|(_, _, posture)| { *posture == WorthUiAuthoredDeltaChangePosture::Changed }));
}

#[test]
fn page_addition_emits_page_template_instance_and_binding_semantic_rows() {
    let app = authored_delta_test_app();
    let runtime = runtime_for_source(&app, source_text("validation.surface.products.collection"));
    let submission = runtime
        .observe_authored_edit(
            app.capabilities(),
            observed_authored_edit(page_added_source_text(
                "validation.surface.products.collection",
            )),
        )
        .expect("page addition should lower from the runtime-owned observed-edit seam");
    let authored_delta = submission
        .authored_delta_summary()
        .expect("page addition should preserve authored delta proof");
    let page_rows = semantic_rows(authored_delta)
        .into_iter()
        .filter(|(slice_id, subject, _)| {
            subject == "page:OrdersPage"
                && matches!(
                    slice_id,
                    WorthUiSemanticSliceId::PageTemplateDeclaration
                        | WorthUiSemanticSliceId::PageInstanceDeclaration
                        | WorthUiSemanticSliceId::PageTemplateBinding
                )
        })
        .collect::<std::collections::BTreeSet<_>>();

    assert!(
        declaration_rows(authored_delta).is_superset(&std::collections::BTreeSet::from([
            (
                WorthUiAuthoredDeclarationKind::Page,
                "OrdersPage".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Added,
            ),
            (
                WorthUiAuthoredDeclarationKind::Layout,
                "OrdersPage".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Added,
            ),
            (
                WorthUiAuthoredDeclarationKind::Content,
                "OrdersPage".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Added,
            ),
        ]))
    );
    assert_eq!(
        page_rows,
        std::collections::BTreeSet::from([
            (
                WorthUiSemanticSliceId::PageInstanceDeclaration,
                "page:OrdersPage".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Added,
            ),
            (
                WorthUiSemanticSliceId::PageTemplateBinding,
                "page:OrdersPage".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Added,
            ),
            (
                WorthUiSemanticSliceId::PageTemplateDeclaration,
                "page:OrdersPage".to_owned(),
                WorthUiAuthoredDeltaChangePosture::Added,
            ),
        ])
    );
}

#[test]
fn runtime_binding_addition_emits_runtime_binding_semantic_rows() {
    let app = authored_delta_test_app();
    let runtime = runtime_for_source(&app, source_text("validation.surface.products.collection"));
    let submission = runtime
        .observe_authored_edit(
            app.capabilities(),
            observed_authored_edit(runtime_binding_added_source_text(
                "validation.surface.products.collection",
            )),
        )
        .expect("binding addition should lower from the runtime-owned observed-edit seam");
    let authored_delta = submission
        .authored_delta_summary()
        .expect("binding addition should preserve authored delta proof");

    assert!(
        declaration_rows(authored_delta).is_superset(&std::collections::BTreeSet::from([(
            WorthUiAuthoredDeclarationKind::RuntimeBinding,
            "workspace.view_binding.selection".to_owned(),
            WorthUiAuthoredDeltaChangePosture::Added,
        )]))
    );
    assert!(
        semantic_rows(authored_delta).is_superset(&std::collections::BTreeSet::from([(
            WorthUiSemanticSliceId::AuthoredQueryBindingShape,
            "binding:workspace.view_binding.selection".to_owned(),
            WorthUiAuthoredDeltaChangePosture::Added,
        )]))
    );
}
