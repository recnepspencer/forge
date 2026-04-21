use crate::{
    ArtifactSemanticVersion, CompatibilityAdapterDigest, CompatibilityAdapterId,
    CompatibilityAuthoritativeAdapterRequest, CompatibilityFamilyKind, CompatibilityRelation,
    ForgeStoreBuilder, Milestone12CertificationLaneKind, Milestone12CertificationRunner,
    StoreErrorKind,
};

use super::harness::fixtures::runtime::{create_entity, latest_envelope, runtime_with_demo_schema};

#[test]
fn authoritative_adapter_executes_through_public_store_path() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope).unwrap();

    let outcome = store
        .execute_compatibility_authoritative_adapter(adapter_request(
            CompatibilityAdapterDigest::new("first_ship_commit_envelope_adapter_digest_v1"),
        ))
        .unwrap();

    assert_eq!(
        outcome.family_kind(),
        CompatibilityFamilyKind::CommitEnvelope
    );
    assert_eq!(outcome.relation(), CompatibilityRelation::AdapterRequired);
    assert_eq!(outcome.control_lane_digest(), outcome.adapted_lane_digest());
    assert_eq!(
        outcome.parity_witness().adapter_id().as_str(),
        "first_ship_commit_envelope_adapter"
    );
    assert_eq!(
        outcome.parity_witness().adapter_digest().as_str(),
        "first_ship_commit_envelope_adapter_digest_v1"
    );
    assert_eq!(outcome.admission_report().admitted_adapter_count, 1);
    assert_eq!(outcome.admission_report().adapter_cost_class_count, 1);
    assert_eq!(outcome.admission_report().adapter_batch_count, 1);
    assert_eq!(outcome.admission_report().adapter_parity_failure_count, 0);
    assert!(outcome.admission_report().adapter_input_record_count > 0);
    assert!(outcome.admission_report().adapter_output_record_count > 0);
    assert_eq!(outcome.admission_report().adapter_allocation_scope_count, 1);
}

#[test]
fn authoritative_adapter_rejects_digest_drift() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope).unwrap();

    let error = store
        .execute_compatibility_authoritative_adapter(adapter_request(
            CompatibilityAdapterDigest::new("drifted-adapter-digest"),
        ))
        .unwrap_err();

    assert_eq!(
        error.kind(),
        &StoreErrorKind::CompatibilityAdapterParityFailure
    );
}

#[test]
fn authoritative_adapter_matches_certification_lane_evidence() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope).unwrap();

    let outcome = store
        .execute_compatibility_authoritative_adapter(adapter_request(
            CompatibilityAdapterDigest::new("first_ship_commit_envelope_adapter_digest_v1"),
        ))
        .unwrap();
    let certification = Milestone12CertificationRunner::first_ship().run().unwrap();
    let admitted_lane = certification
        .evidence_bundle()
        .lane_outcomes()
        .iter()
        .find(|lane| lane.lane_kind() == Milestone12CertificationLaneKind::AdapterParityAdmitted)
        .unwrap();
    let rejected_lane = certification
        .evidence_bundle()
        .lane_outcomes()
        .iter()
        .find(|lane| {
            lane.lane_kind() == Milestone12CertificationLaneKind::AdapterParityDigestRejected
        })
        .unwrap();

    assert_eq!(outcome.relation(), admitted_lane.relation().unwrap());
    assert_eq!(
        rejected_lane.rejection_kind(),
        Some(crate::CompatibilityRejectionKind::AdapterParityFailure)
    );
    assert!(!certification
        .diagnostics()
        .runtime_gap_labels()
        .contains(&"adapter_execution_deferred"));
}

fn adapter_request(
    adapter_digest: CompatibilityAdapterDigest,
) -> CompatibilityAuthoritativeAdapterRequest {
    CompatibilityAuthoritativeAdapterRequest::new(
        CompatibilityFamilyKind::CommitEnvelope,
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(2),
        CompatibilityAdapterId::new("first_ship_commit_envelope_adapter"),
        adapter_digest,
    )
}
