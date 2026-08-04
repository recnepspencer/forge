use super::super::{
    admission, plan_read_compatibility, plan_read_compatibility_for_path, plan_write_compatibility,
    ArtifactSemanticVersion, CompatibilityAdapterCostClass, CompatibilityAdmissionBatch,
    CompatibilityAdmissionPath, CompatibilityEdgeRegistry, CompatibilityFamilyKind,
    CompatibilityManifestIndex, CompatibilityReadAdmissionOutcome, CompatibilityReadIntent,
    CompatibilityRegistry, CompatibilityRejectionKind, CompatibilityRelation,
    CompatibilityWriteAdmissionOutcome, CompatibilityWriteIntent, DeclaredCompatibilityEdge,
    ReaderCapabilitySet, WriterCapabilitySet,
};
use super::{adapter, native_edge, quarantined_artifact_for_family};

#[test]
fn compatibility_batch_read_admits_batch_local_adapter_edge() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
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
        &CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1)),
        &artifact,
        CompatibilityAdmissionPath::BatchRead,
    )
    .expect("batch adapter should admit batch read");
    assert_eq!(
        receipt.receipt().relation(),
        CompatibilityRelation::AdapterRequired
    );
    assert_eq!(batch.counters().admitted_adapter_count(), 1);
    assert_eq!(batch.counters().adapter_cost_class_count(), 1);
    assert_eq!(batch.counters().adapter_batch_count(), 1);
}

#[test]
fn compatibility_declared_incompatible_edge_rejects_read() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "authoritative");
    let edge = DeclaredCompatibilityEdge::new(
        family_id.clone(),
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(1),
        CompatibilityRelation::Incompatible,
    )
    .with_adapter(adapter(CompatibilityAdapterCostClass::BoundedBatchLocal));
    let mut batch = CompatibilityAdmissionBatch::new();
    let rejection = admission::plan_read_compatibility_for_path(
        &mut batch,
        &index,
        &CompatibilityEdgeRegistry::new(vec![edge]),
        &ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
        &CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1)),
        &artifact,
        CompatibilityAdmissionPath::BatchRead,
    )
    .expect_err("declared incompatible relation must not produce a receipt");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::DeclaredIncompatibleRelation
    );
    assert_eq!(batch.counters().rejected_count(), 1);
    assert_eq!(batch.counters().admitted_adapter_count(), 0);
}

#[test]
fn compatibility_out_of_scope_adapter_rejects_and_counts() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "authoritative");
    let edge = DeclaredCompatibilityEdge::new(
        family_id.clone(),
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(1),
        CompatibilityRelation::AdapterRequired,
    )
    .with_adapter(adapter(CompatibilityAdapterCostClass::OutOfScope));
    let mut batch = CompatibilityAdmissionBatch::new();
    let rejection = admission::plan_read_compatibility_for_path(
        &mut batch,
        &index,
        &CompatibilityEdgeRegistry::new(vec![edge]),
        &ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
        &CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1)),
        &artifact,
        CompatibilityAdmissionPath::MaintenanceScheduled,
    )
    .expect_err("out-of-scope adapter must reject even outside the hot path");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::AdapterOutOfScope
    );
    assert_eq!(batch.counters().adapter_out_of_scope_rejection_count(), 1);
}

#[test]
fn compatibility_admission_outcomes_report_without_exposing_proofs() {
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
    .expect("native edge should admit");
    let outcome = CompatibilityReadAdmissionOutcome::accepted(&receipt, batch.counters());
    assert!(outcome.is_accepted());
    assert_eq!(outcome.family_id().as_str(), "commit_envelope");
    assert_eq!(outcome.relation(), Some(CompatibilityRelation::Native));
    assert_eq!(outcome.rejection_kind(), None);
    assert_eq!(outcome.counters().accepted_count(), 1);
}

#[test]
fn compatibility_write_outcome_reports_rejection_posture() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "authoritative");
    let mut batch = CompatibilityAdmissionBatch::new();
    let rejection = admission::plan_write_compatibility(
        &mut batch,
        &index,
        &CompatibilityEdgeRegistry::default(),
        &WriterCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
        &CompatibilityWriteIntent::new(family_id, ArtifactSemanticVersion::new(1)),
        &artifact,
    )
    .expect_err("missing edge should reject write");
    let outcome =
        CompatibilityWriteAdmissionOutcome::rejected(&artifact, &rejection, batch.counters());
    assert!(!outcome.is_accepted());
    assert_eq!(outcome.relation(), None);
    assert_eq!(
        outcome.rejection_kind(),
        Some(CompatibilityRejectionKind::MissingCompatibilityEdge)
    );
    assert_eq!(outcome.counters().rejected_count(), 1);
}
