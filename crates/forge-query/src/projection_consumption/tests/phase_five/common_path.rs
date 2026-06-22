use super::super::super::{
    CompletedProjectionFactConsumption, DeferredProjectionConsumptionReason,
    ProjectMaterializedFacts, ProjectionConsumptionDenialReason, ProjectionConsumptionSource,
    ProjectionConsumptionWarningKind, ProjectionFactConsumptionAttempt,
};
use super::super::phase_four::support::{
    authorized_projection, query_context_execution_preview, read_result, read_result_shape,
    write_receipt, write_receipt_without_source_references,
};

#[test]
fn read_result_common_path_consumes_projection_facts_through_receipt() {
    let result = read_result();
    let result_shape = read_result_shape();
    let authorized_projection = authorized_projection(
        "query:test",
        result_shape.digest().as_str(),
        &["profile.display_name", "metrics.priority"],
    );

    let attempt = result
        .consume_projection_facts(
            &result_shape,
            &authorized_projection,
            ProjectMaterializedFacts::declare()
                .entity_identities()
                .display_field_path(
                    crate::projection_consumption::projection_fact_field_path_from_segments([
                        "profile",
                        "display_name",
                    ]),
                ),
        )
        .unwrap();

    match attempt {
        ProjectionFactConsumptionAttempt::Admitted(completed) => {
            assert_common_path_completion(&completed, 2, 4);
        }
        other => panic!("expected admitted common-path read consumption, got {other:?}"),
    }
}

#[test]
fn write_receipt_common_path_consumes_projection_facts_through_receipt() {
    let receipt = write_receipt();
    let authorized_projection =
        authorized_projection("query:test", "result-shape:test", &["identity.id"]);

    let attempt = receipt
        .consume_projection_facts(
            "result-shape:test",
            &authorized_projection,
            ProjectMaterializedFacts::declare()
                .target_identity()
                .source_references(),
        )
        .unwrap();

    match attempt {
        ProjectionFactConsumptionAttempt::Admitted(completed) => {
            assert_common_path_completion(&completed, 2, 3);
        }
        other => panic!("expected admitted common-path write consumption, got {other:?}"),
    }
}

#[test]
fn query_context_common_path_preserves_warning_bearing_admission() {
    let execution = query_context_execution_preview();
    let authorized_projection =
        authorized_projection("query:test", "result-shape:test", &["profile.display_name"]);

    let attempt = execution
        .consume_projection_facts(
            &authorized_projection,
            ProjectMaterializedFacts::declare().display_field_path(
                crate::projection_consumption::projection_fact_field_path_from_segments([
                    "profile",
                    "display_name",
                ]),
            ),
        )
        .unwrap();

    match attempt {
        ProjectionFactConsumptionAttempt::AdmittedWithWarnings(completed, warnings) => {
            assert_eq!(
                warnings.warning_kinds(),
                [ProjectionConsumptionWarningKind::PreviewDerivedContext]
            );
            assert_common_path_completion(&completed, 1, 1);
            assert_eq!(
                completed.receipt().warning_kinds(),
                [ProjectionConsumptionWarningKind::PreviewDerivedContext]
            );
            assert_eq!(
                completed.projection_consumption_envelope().warning_kinds(),
                [ProjectionConsumptionWarningKind::PreviewDerivedContext]
            );
        }
        other => {
            panic!("expected warning-bearing common-path query-context consumption, got {other:?}")
        }
    }
}

#[test]
fn completed_common_path_exposes_receipt_first_inspection_helpers() {
    let result = read_result();
    let result_shape = read_result_shape();
    let authorized_projection = authorized_projection(
        "query:test",
        result_shape.digest().as_str(),
        &["profile.display_name", "metrics.priority"],
    );

    let attempt = result
        .consume_projection_facts(
            &result_shape,
            &authorized_projection,
            ProjectMaterializedFacts::declare()
                .entity_identities()
                .display_field_path(
                    crate::projection_consumption::projection_fact_field_path_from_segments([
                        "profile",
                        "display_name",
                    ]),
                ),
        )
        .unwrap();

    let completed = attempt.completed().expect("read path should be admitted");
    assert_eq!(
        completed.source_family(),
        completed.receipt().source_family()
    );
    assert_eq!(
        completed.source_identity(),
        completed.receipt().source_identity()
    );
    assert_eq!(
        completed.support_posture(),
        completed.receipt().support_posture()
    );
    assert_eq!(
        completed.warning_kinds(),
        completed.receipt().warning_kinds()
    );
    assert_eq!(
        completed.admitted_fact_family_count(),
        completed.receipt().admitted_fact_family_count()
    );
    assert_eq!(
        completed.extracted_fact_count(),
        completed.receipt().extracted_fact_count()
    );
    assert_eq!(
        completed.authority_reopen_count(),
        completed.receipt().authority_reopen_count()
    );
    assert_eq!(
        completed.deferred_neighbors(),
        completed.receipt().deferred_neighbors()
    );
    assert_eq!(
        completed.transition_rules().rules_digest(),
        completed.receipt().transition_rules().rules_digest()
    );
}

#[test]
fn attempt_inspection_helpers_preserve_typed_non_admitted_postures() {
    let result = read_result();
    let result_shape = read_result_shape();
    let read_authorized_projection = authorized_projection(
        "query:test",
        result_shape.digest().as_str(),
        &["metrics.priority"],
    );
    let denied = result
        .consume_projection_facts(
            &result_shape,
            &read_authorized_projection,
            ProjectMaterializedFacts::declare().display_field_path(
                crate::projection_consumption::projection_fact_field_path_from_segments([
                    "profile",
                    "display_name",
                ]),
            ),
        )
        .unwrap();
    assert!(denied.completed().is_none());
    assert!(denied.warnings().is_none());
    assert!(denied.deferred().is_none());
    assert!(denied.source_mismatch().is_none());
    assert_eq!(
        denied.denied().expect("typed denial").reason(),
        &ProjectionConsumptionDenialReason::FactFamilyNotVisible {
            field_key: "profile.display_name".to_string(),
        }
    );

    let write_authorized_projection =
        authorized_projection("query:test", "result-shape:test", &["identity.id"]);
    let deferred = write_receipt_without_source_references()
        .consume_projection_facts(
            "result-shape:test",
            &write_authorized_projection,
            ProjectMaterializedFacts::declare().source_references(),
        )
        .unwrap();
    assert!(deferred.completed().is_none());
    assert_eq!(
        deferred.deferred().expect("typed deferred").reason(),
        &DeferredProjectionConsumptionReason::WriteReceiptContractBindingPending
    );

    let mismatch = write_receipt()
        .consume_projection_facts(
            "result-shape:test",
            &write_authorized_projection,
            ProjectMaterializedFacts::declare().display_field_path(
                crate::projection_consumption::projection_fact_field_path_from_segments([
                    "profile",
                    "display_name",
                ]),
            ),
        )
        .unwrap();
    assert!(mismatch.completed().is_none());
    assert_eq!(
        mismatch
            .source_mismatch()
            .expect("typed source mismatch")
            .source_family(),
        super::super::super::ProjectionSourceFamily::QueryWriteReceipt
    );
}

#[test]
fn common_path_read_result_returns_typed_denial_without_constructing_later_phase_artifacts() {
    let result = read_result();
    let result_shape = read_result_shape();
    let authorized_projection = authorized_projection(
        "query:test",
        result_shape.digest().as_str(),
        &["metrics.priority"],
    );

    let attempt = result
        .consume_projection_facts(
            &result_shape,
            &authorized_projection,
            ProjectMaterializedFacts::declare().display_field_path(
                crate::projection_consumption::projection_fact_field_path_from_segments([
                    "profile",
                    "display_name",
                ]),
            ),
        )
        .unwrap();

    match attempt {
        ProjectionFactConsumptionAttempt::Denied(denied) => {
            assert_eq!(
                denied.reason(),
                &ProjectionConsumptionDenialReason::FactFamilyNotVisible {
                    field_key: "profile.display_name".to_string(),
                }
            );
        }
        other => panic!("expected typed denial on the common path, got {other:?}"),
    }
}

#[test]
fn common_path_write_receipt_returns_typed_deferred_without_constructing_later_phase_artifacts() {
    let receipt = write_receipt();
    let authorized_projection =
        authorized_projection("query:test", "result-shape:test", &["identity.id"]);

    let attempt = receipt
        .consume_projection_facts(
            "result-shape:test",
            &authorized_projection,
            ProjectMaterializedFacts::declare().display_field_path(
                crate::projection_consumption::projection_fact_field_path_from_segments([
                    "profile",
                    "display_name",
                ]),
            ),
        )
        .unwrap();

    match attempt {
        ProjectionFactConsumptionAttempt::SourceMismatch(mismatch) => {
            assert_eq!(
                mismatch.source_family(),
                super::super::super::ProjectionSourceFamily::QueryWriteReceipt
            );
        }
        other => panic!("expected typed source mismatch on the common path, got {other:?}"),
    }
}

#[test]
fn common_path_write_receipt_returns_typed_deferred_when_requested_evidence_is_not_carried() {
    let receipt = write_receipt_without_source_references();
    let authorized_projection =
        authorized_projection("query:test", "result-shape:test", &["identity.id"]);

    let attempt = receipt
        .consume_projection_facts(
            "result-shape:test",
            &authorized_projection,
            ProjectMaterializedFacts::declare().source_references(),
        )
        .unwrap();

    match attempt {
        ProjectionFactConsumptionAttempt::Deferred(deferred) => {
            assert_eq!(
                deferred.reason(),
                &DeferredProjectionConsumptionReason::WriteReceiptContractBindingPending
            );
        }
        other => panic!("expected typed deferred posture on the common path, got {other:?}"),
    }
}

#[test]
fn source_surfaces_expose_support_discovery_helpers() {
    let read_shape = read_result_shape();
    let read_support = read_result()
        .receipt()
        .discover_projection_fact_consumption_support(&read_shape);
    assert_eq!(
        read_support.source_family(),
        ProjectionConsumptionSource::from_read_receipt(read_result().receipt(), &read_shape)
            .family()
    );

    let write_support = write_receipt().discover_projection_fact_consumption_support();
    assert_eq!(
        write_support.source_family(),
        ProjectionConsumptionSource::from_write_receipt(&write_receipt()).family()
    );

    let query_context_support =
        query_context_execution_preview().discover_projection_fact_consumption_support();
    assert_eq!(
        query_context_support.source_family(),
        ProjectionConsumptionSource::from_query_context_execution(
            &query_context_execution_preview()
        )
        .family()
    );
}

fn assert_common_path_completion(
    completed: &CompletedProjectionFactConsumption,
    expected_admitted_fact_families: usize,
    expected_extracted_fact_count: usize,
) {
    assert_eq!(
        completed.receipt().declaration_digest(),
        completed.declaration().declaration_digest()
    );
    assert_eq!(
        completed.receipt().contract_digest(),
        completed.contract().contract_digest()
    );
    assert_eq!(
        completed.receipt().fact_set_digest(),
        completed.facts().fact_set_digest()
    );
    assert_eq!(
        completed.receipt().admitted_fact_family_count(),
        expected_admitted_fact_families
    );
    assert_eq!(
        completed.receipt().extracted_fact_count(),
        expected_extracted_fact_count
    );
    assert_eq!(completed.receipt().authority_reopen_count(), 0);
    assert_eq!(
        completed
            .projection_consumption_envelope()
            .sources()
            .receipt_digest(),
        completed.receipt().receipt_digest()
    );
}
