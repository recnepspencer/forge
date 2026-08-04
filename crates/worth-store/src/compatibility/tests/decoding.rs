use super::super::{
    admission, decoding, plan_read_compatibility_for_path, ArtifactSemanticVersion,
    CompatibilityAdapterCostClass, CompatibilityAdmissionBatch, CompatibilityAdmissionPath,
    CompatibilityEdgeRegistry, CompatibilityFamilyKind, CompatibilityManifestIndex,
    CompatibilityReadIntent, CompatibilityRegistry, CompatibilityRejectionKind,
    CompatibilityRelation, DeclaredCompatibilityEdge, RawArtifactBytes, ReaderCapabilitySet,
};
use super::{adapter, frame_header, quarantined_artifact_for_family};

#[test]
fn compatibility_empty_raw_artifact_rejects_before_quarantine() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let mut batch = CompatibilityAdmissionBatch::new();
    let rejection = decoding::decode_artifact_to_quarantine(
        &mut batch,
        RawArtifactBytes::new(family_id.clone(), vec![]),
        frame_header(family_id, 1, 1, "authoritative", 0),
    )
    .expect_err("empty frame should reject");
    assert_eq!(rejection.kind(), CompatibilityRejectionKind::MalformedFrame);
    assert_eq!(batch.counters().malformed_frame_count(), 1);
}

#[test]
fn compatibility_truncated_raw_artifact_rejects_before_quarantine() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let mut batch = CompatibilityAdmissionBatch::new();
    let rejection = decoding::decode_artifact_to_quarantine(
        &mut batch,
        RawArtifactBytes::new(family_id.clone(), vec![1, 2]),
        frame_header(family_id, 1, 1, "authoritative", 3),
    )
    .expect_err("truncated frame should reject");
    assert_eq!(rejection.kind(), CompatibilityRejectionKind::TruncatedFrame);
    assert_eq!(batch.counters().malformed_frame_count(), 1);
}

#[test]
fn compatibility_overlong_raw_artifact_rejects_before_quarantine() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let mut batch = CompatibilityAdmissionBatch::new();
    let rejection = decoding::decode_artifact_to_quarantine(
        &mut batch,
        RawArtifactBytes::new(family_id.clone(), vec![1, 2, 3, 4]),
        frame_header(family_id, 1, 1, "authoritative", 3),
    )
    .expect_err("overlong frame should reject");
    assert_eq!(rejection.kind(), CompatibilityRejectionKind::MalformedFrame);
    assert_eq!(batch.counters().malformed_frame_count(), 1);
}

#[test]
fn compatibility_valid_frame_produces_quarantined_metadata_only() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let mut batch = CompatibilityAdmissionBatch::new();
    let artifact = decoding::decode_artifact_to_quarantine(
        &mut batch,
        RawArtifactBytes::new(family_id.clone(), vec![1, 2, 3]),
        frame_header(family_id, 1, 1, "authoritative", 3),
    )
    .expect("valid frame should quarantine");
    assert_eq!(artifact.family_id().as_str(), "commit_envelope");
    assert_eq!(artifact.format_version().value(), 1);
    assert_eq!(artifact.semantic_version().value(), 1);
    assert!(!artifact.structural_digest().is_empty());
}

#[test]
fn compatibility_hot_read_rejects_batch_local_adapter_edge() {
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
    let rejection = admission::plan_read_compatibility_for_path(
        &mut batch,
        &index,
        &CompatibilityEdgeRegistry::new(vec![edge]),
        &ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
        &CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1)),
        &artifact,
        CompatibilityAdmissionPath::HotRead,
    )
    .expect_err("batch adapter should reject hot read");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::AdapterHotPathRejected
    );
    assert_eq!(batch.counters().adapter_hot_path_rejection_count(), 1);
}
