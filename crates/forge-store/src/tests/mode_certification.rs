use crate::{
    BasisFreeCheckpoint, CheckpointAuthorityReport, DerivedDurableCheckpointKind,
    EmbeddedCheckpointClassification, ExternalRuntimeCheckpointEnvelope, ForgeStoreBuilder,
    Milestone2CertificationBundle, NoContainedCommits, ObservedModeFailure,
};

use super::harness::{
    certification::{
        assertions::{assert_all_equal, assert_rejection_payloads_present},
        core::{AssertionClass, CanonicalRow, CertificationSuite, LaneResult, RejectionRow},
        requirements::{evaluate_completeness, OPERATING_MODE_CONTRACT_PARITY_TEST},
    },
    fixtures::runtime::{create_entity_commit, latest_envelope, runtime_with_demo_schema},
    scenarios::modes::mode_contract_parity,
};

fn milestone_2_suite() -> CertificationSuite<String, String> {
    let scenario = mode_contract_parity();
    let _bundle = Milestone2CertificationBundle::new(
        scenario.durable_lane.clone(),
        scenario.embedded_lane.clone(),
        scenario.absent_lane.clone(),
        scenario.checkpoint_authority_report.clone(),
        &[],
    );

    let mut embedded = ForgeStoreBuilder::new()
        .in_memory()
        .embedded_mode()
        .build()
        .unwrap();
    let error = embedded
        .persist_external_checkpoint_unchecked(ExternalRuntimeCheckpointEnvelope::new(
            "checkpoint-authoritative",
            "embedded-runtime",
            EmbeddedCheckpointClassification::AuthoritativeCommitBundle,
        ))
        .unwrap_err();

    let mut embedded_runtime = runtime_with_demo_schema();
    create_entity_commit(&mut embedded_runtime, "alpha");
    let envelope = latest_envelope(&embedded_runtime);
    embedded
        .persist_external_commit(crate::ExternalRuntimeCommitEnvelope::new(
            "embedded-runtime",
            envelope,
        ))
        .unwrap();
    let embedded_lane = embedded.milestone_2_lane_evidence();

    let durable = ForgeStoreBuilder::new()
        .in_memory()
        .durable_mode(runtime_with_demo_schema())
        .build()
        .unwrap();
    let durable_lane = durable.milestone_2_lane_evidence();
    let absent_lane =
        crate::AbsentRuntimeWitness::new(runtime_with_demo_schema()).milestone_2_lane_evidence();
    let checkpoint_authority_report = CheckpointAuthorityReport::from_checkpoint(
        embedded_lane.artifact_digest.clone(),
        embedded_lane.artifact_digest.clone(),
        &embedded
            .persist_external_checkpoint(
                embedded
                    .admit_external_checkpoint(BasisFreeCheckpoint::<
                        DerivedDurableCheckpointKind,
                        NoContainedCommits,
                    >::new("checkpoint-derived", "embedded-runtime"))
                    .unwrap(),
            )
            .unwrap(),
    );
    let failure_bundle = Milestone2CertificationBundle::new(
        durable_lane,
        embedded_lane,
        absent_lane,
        checkpoint_authority_report,
        &[ObservedModeFailure::from_error(&error)],
    );

    CertificationSuite::new(OPERATING_MODE_CONTRACT_PARITY_TEST.suite_name)
        .with_canonical_row(CanonicalRow::new(
            "mode_contract_parity",
            vec![
                LaneResult::new("durable", scenario.durable_lane.artifact_digest),
                LaneResult::new("embedded", scenario.embedded_lane.artifact_digest),
            ],
            &[AssertionClass::Equality, AssertionClass::ExactCounter],
        ))
        .with_rejection_row(RejectionRow::new(
            "typed_mode_failure",
            vec![LaneResult::new("embedded", failure_bundle.failure_digest)],
            &[AssertionClass::TypedFailure, AssertionClass::ExactCounter],
        ))
}

#[test]
fn milestone_2_certification_bundle_proves_mode_contract_parity() {
    let scenario = mode_contract_parity();
    let bundle = Milestone2CertificationBundle::new(
        scenario.durable_lane.clone(),
        scenario.embedded_lane.clone(),
        scenario.absent_lane.clone(),
        scenario.checkpoint_authority_report.clone(),
        &[],
    );

    assert_eq!(
        scenario.durable_lane.artifact_digest,
        scenario.embedded_lane.artifact_digest
    );
    assert!(bundle.mode_contract_matrix.durable_embedded_artifact_parity);
    assert!(bundle.mode_contract_matrix.absent_mode_is_no_store);
    assert!(bundle.mode_contract_matrix.zero_forbidden_cross_mode_work);
    assert!(!bundle.artifact_digest.is_empty());
    assert!(!bundle.diagnostics_digest.is_empty());
    assert!(!bundle.failure_digest.is_empty());
    assert_eq!(
        bundle
            .checkpoint_authority_report
            .authoritative_artifact_digest_before,
        bundle
            .checkpoint_authority_report
            .authoritative_artifact_digest_after
    );
    assert!(!bundle.canonical_json().is_empty());
    assert_eq!(
        bundle
            .counter_snapshot
            .absent_lane
            .absent_mode_selection_count,
        1
    );
    assert_eq!(
        bundle
            .counter_snapshot
            .absent_lane
            .absent_mode_store_touch_count,
        0
    );

    let suite = milestone_2_suite();
    assert_all_equal(&suite.canonical_rows()[0]);
    let completeness = evaluate_completeness(&suite, &OPERATING_MODE_CONTRACT_PARITY_TEST);
    assert!(completeness.missing_rows().is_empty());
    assert!(completeness.missing_assertion_classes().is_empty());
}

#[test]
fn milestone_2_certification_bundle_captures_typed_mode_failures() {
    let suite = milestone_2_suite();
    assert_rejection_payloads_present(&suite.rejection_rows()[0]);
}
