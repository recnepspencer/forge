use super::super::{
    adapters, admission, authoritative, check_artifact_with_read_receipt,
    declare_authoritative_meaning, execute_declared_adapter_parity,
    first_ship_authoritative_adapter_edge_registry, plan_read_compatibility,
    plan_read_compatibility_for_path, ArtifactSemanticVersion, CompatibilityAdapterCostClass,
    CompatibilityAdapterDigest, CompatibilityAdapterId, CompatibilityAdmissionBatch,
    CompatibilityAdmissionCounters, CompatibilityAdmissionPath, CompatibilityDecision,
    CompatibilityEdgeRegistry, CompatibilityFamilyKind, CompatibilityManifestIndex,
    CompatibilityReadIntent, CompatibilityRegistry, CompatibilityRejectionKind,
    CompatibilityRelation, DeclaredCompatibilityEdge, ReaderCapabilitySet,
};
use super::{adapter, native_edge, published_manifest_index, quarantined_artifact_for_family};

#[test]
fn compatibility_quarantined_artifact_checks_with_matching_receipt() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "authoritative");
    let mut batch = CompatibilityAdmissionBatch::new();
    let receipt = admission::plan_read_compatibility(
        &mut batch,
        &index,
        &CompatibilityEdgeRegistry::new(vec![native_edge(family_id.clone())]),
        &ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
        &CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1)),
        &artifact,
    )
    .expect("read should admit");
    let checked = admission::check_artifact_with_read_receipt(artifact, &receipt)
        .expect("receipt should check quarantined artifact");
    match checked.decision() {
        CompatibilityDecision::Admit(CompatibilityRelation::Native) => {}
        other => panic!("unexpected decision {other:?}"),
    }
}

#[test]
fn compatibility_authoritative_meaning_requires_checked_artifact_and_declaration() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = published_manifest_index(&snapshot);
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "authoritative");
    let mut batch = CompatibilityAdmissionBatch::new();
    let receipt = admission::plan_read_compatibility(
        &mut batch,
        &index,
        &CompatibilityEdgeRegistry::new(vec![native_edge(family_id.clone())]),
        &ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
        &CompatibilityReadIntent::new(family_id.clone(), ArtifactSemanticVersion::new(1)),
        &artifact,
    )
    .expect("read should admit");
    let checked = admission::check_artifact_with_read_receipt(artifact, &receipt)
        .expect("receipt should check artifact");
    let meaning = authoritative::declare_authoritative_meaning(
        family_id,
        ArtifactSemanticVersion::new(1),
        "commit-envelope-v1",
    );
    let (witness, report) = authoritative::admit_authoritative_meaning(
        batch.counters_mut(),
        &checked,
        &receipt,
        Some(&meaning),
    )
    .expect("declared native authoritative meaning should admit");
    assert_eq!(witness.family_id().as_str(), "commit_envelope");
    assert!(report.admitted_status());
    assert_eq!(
        batch
            .counters()
            .authoritative_partial_truth_rejection_count(),
        0
    );
}

#[test]
fn compatibility_authoritative_unknown_meaning_rejects_partial_truth() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = published_manifest_index(&snapshot);
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "authoritative");
    let mut batch = CompatibilityAdmissionBatch::new();
    let receipt = admission::plan_read_compatibility(
        &mut batch,
        &index,
        &CompatibilityEdgeRegistry::new(vec![native_edge(family_id.clone())]),
        &ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
        &CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1)),
        &artifact,
    )
    .expect("read should admit");
    let checked = admission::check_artifact_with_read_receipt(artifact, &receipt)
        .expect("receipt should check artifact");
    let rejection =
        authoritative::admit_authoritative_meaning(batch.counters_mut(), &checked, &receipt, None)
            .expect_err("unknown authoritative meaning must reject");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::AuthoritativePartialTruthRejected
    );
    assert_eq!(
        rejection.store_error_kind(),
        crate::StoreErrorKind::CompatibilityAuthoritativePartialTruthRejected
    );
    assert_eq!(
        batch
            .counters()
            .authoritative_partial_truth_rejection_count(),
        1
    );
}

#[test]
fn compatibility_adapter_required_authoritative_meaning_rejects_without_parity() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = published_manifest_index(&snapshot);
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "authoritative");
    let edge = DeclaredCompatibilityEdge::new(
        family_id.clone(),
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(1),
        CompatibilityRelation::AdapterRequired,
    )
    .with_adapter(adapter(CompatibilityAdapterCostClass::BoundedBatchLocal));
    let mut batch = CompatibilityAdmissionBatch::new();
    let receipt = admission::plan_read_compatibility_for_path(
        &mut batch,
        &index,
        &CompatibilityEdgeRegistry::new(vec![edge]),
        &ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
        &CompatibilityReadIntent::new(family_id.clone(), ArtifactSemanticVersion::new(1)),
        &artifact,
        CompatibilityAdmissionPath::BatchRead,
    )
    .expect("batch-local adapter can admit read receipt");
    let checked = admission::check_artifact_with_read_receipt(artifact, &receipt)
        .expect("receipt should check artifact");
    let meaning = authoritative::declare_authoritative_meaning(
        family_id,
        ArtifactSemanticVersion::new(1),
        "commit-envelope-v1",
    );
    let rejection = authoritative::admit_authoritative_meaning(
        batch.counters_mut(),
        &checked,
        &receipt,
        Some(&meaning),
    )
    .expect_err("adapter-required authority needs parity witness");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::AuthoritativePartialTruthRejected
    );
    assert_eq!(
        batch
            .counters()
            .authoritative_partial_truth_rejection_count(),
        1
    );
}

#[test]
fn compatibility_declared_adapter_parity_rejects_semantic_drift() {
    let edge_registry = adapters::first_ship_authoritative_adapter_edge_registry();
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let mut counters = CompatibilityAdmissionCounters::default();
    let rejection = adapters::execute_declared_adapter_parity(
        &mut counters,
        &edge_registry,
        &family_id,
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(2),
        &CompatibilityAdapterId::new("first_ship_commit_envelope_adapter"),
        &CompatibilityAdapterDigest::new("first_ship_commit_envelope_adapter_digest_v1"),
        br#"{"lane":"control","rows":[1,2]}"#,
        br#"{"lane":"adapted","rows":[1,3]}"#,
        2,
        1,
        1,
    )
    .expect_err("semantic drift between control and adapted lanes must reject parity");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::AdapterParityFailure
    );
    assert_eq!(counters.adapter_parity_failure_count(), 1);
    assert_eq!(counters.adapter_input_record_count(), 2);
    assert_eq!(counters.adapter_output_record_count(), 1);
    assert_eq!(counters.adapter_allocation_scope_count(), 1);
}
