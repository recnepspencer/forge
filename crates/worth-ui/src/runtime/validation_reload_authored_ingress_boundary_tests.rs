use super::source_ingress_authored_delta_test_support::{
    authored_delta_test_app, observed_authored_edit, runtime_for_source, source_text,
    validation_reload_request,
};
use super::source_ingress_authored_package_test_support::{
    observed_authored_edit_for_modules, packaged_source_modules, runtime_for_modules,
    validation_reload_request_for_modules,
};

#[test]
fn validation_reload_request_adapter_matches_canonical_observed_edit_ingress() {
    let app = authored_delta_test_app();
    let baseline_source = source_text("validation.surface.products.collection");
    let changed_source = source_text("validation.surface.orders.collection");
    let runtime_from_request = runtime_for_source(&app, baseline_source.clone());
    let runtime_from_submission = runtime_for_source(&app, baseline_source);

    let adapter_prepared = runtime_from_request.prepare_validation_reload(
        app.capabilities(),
        validation_reload_request(changed_source.clone()),
    );
    let canonical_submission = runtime_from_submission
        .observe_authored_edit(app.capabilities(), observed_authored_edit(changed_source))
        .expect("observed edit should lower through the canonical source-ingress seam");
    let canonical_prepared = runtime_from_submission
        .prepare_validation_reload_from_authored_submission(
            canonical_submission
                .into_source_authored_submission()
                .expect("source-authored observed edit should carry authored ingress proof"),
        );

    assert!(adapter_prepared
        .evidence()
        .used_validation_request_adapter());
    assert!(!canonical_prepared
        .evidence()
        .used_validation_request_adapter());
    assert_eq!(
        adapter_prepared.evidence().status(),
        canonical_prepared.evidence().status()
    );
    assert_eq!(
        adapter_prepared.evidence().authored_delta_digest(),
        canonical_prepared.evidence().authored_delta_digest()
    );
    assert_eq!(
        adapter_prepared.evidence().source_revision_digest(),
        canonical_prepared.evidence().source_revision_digest()
    );
    assert_eq!(
        adapter_prepared.evidence().ordering_receipt_digest(),
        canonical_prepared.evidence().ordering_receipt_digest()
    );
    assert_eq!(
        adapter_prepared.evidence().candidate_artifact_digest(),
        canonical_prepared.evidence().candidate_artifact_digest()
    );
    assert_eq!(
        adapter_prepared.evidence().candidate_plan_digest(),
        canonical_prepared.evidence().candidate_plan_digest()
    );
    assert_eq!(
        adapter_prepared.evidence().observed_modules(),
        canonical_prepared.evidence().observed_modules()
    );
    assert_eq!(
        adapter_prepared.evidence().parsed_modules(),
        canonical_prepared.evidence().parsed_modules()
    );
    assert_eq!(
        adapter_prepared
            .evidence()
            .authored_declarations_inspected(),
        canonical_prepared
            .evidence()
            .authored_declarations_inspected()
    );
    assert_eq!(
        adapter_prepared.evidence().authored_declarations_touched(),
        canonical_prepared
            .evidence()
            .authored_declarations_touched()
    );
    assert_eq!(
        adapter_prepared.evidence().semantic_slices_emitted(),
        canonical_prepared.evidence().semantic_slices_emitted()
    );
    assert_eq!(
        adapter_prepared.evidence().changed_facts(),
        canonical_prepared.evidence().changed_facts()
    );
}

#[test]
fn multi_module_validation_reload_request_still_lowers_through_the_request_adapter_only() {
    let app = authored_delta_test_app();
    let baseline_modules = packaged_source_modules("validation.surface.products.collection");
    let changed_modules = packaged_source_modules("validation.surface.orders.collection");
    let runtime_from_request = runtime_for_modules(&app, baseline_modules.clone());
    let runtime_from_submission = runtime_for_modules(&app, baseline_modules);

    let adapter_prepared = runtime_from_request.prepare_validation_reload(
        app.capabilities(),
        validation_reload_request_for_modules(changed_modules.clone()),
    );
    let canonical_prepared = runtime_from_submission
        .prepare_validation_reload_from_authored_submission(
            runtime_from_submission
                .observe_authored_edit(
                    app.capabilities(),
                    observed_authored_edit_for_modules(changed_modules),
                )
                .expect("observed multi-module package should lower through source ingress")
                .into_source_authored_submission()
                .expect("source-authored package should carry authored ingress proof"),
        );

    assert!(adapter_prepared
        .evidence()
        .used_validation_request_adapter());
    assert!(!canonical_prepared
        .evidence()
        .used_validation_request_adapter());
    assert_eq!(
        adapter_prepared.evidence().authored_delta_digest(),
        canonical_prepared.evidence().authored_delta_digest()
    );
    assert_eq!(
        adapter_prepared.evidence().observed_modules(),
        canonical_prepared.evidence().observed_modules()
    );
    assert_eq!(
        adapter_prepared.evidence().changed_facts(),
        canonical_prepared.evidence().changed_facts()
    );
}
