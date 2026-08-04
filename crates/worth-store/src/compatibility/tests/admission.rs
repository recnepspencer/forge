use super::super::{
    admission, plan_read_compatibility, plan_write_compatibility, ArtifactFamilyId,
    ArtifactSemanticVersion, CompatibilityAdmissionBatch, CompatibilityEdgeRegistry,
    CompatibilityFamilyKind, CompatibilityManifestIndex, CompatibilityReadIntent,
    CompatibilityRegistry, CompatibilityRejectionKind, CompatibilityRelation,
    CompatibilityWriteIntent, DeclaredCompatibilityEdge, ReaderCapabilitySet, WriterCapabilitySet,
};
use super::Milestone12AdmissionReport;
use super::{
    native_edge, published_manifest_ledger, quarantined_artifact_for_family,
    quarantined_artifact_for_versions,
};

#[test]
fn compatibility_read_admission_rejects_undeclared_family() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
    let mut batch = CompatibilityAdmissionBatch::new();
    let family_id = ArtifactFamilyId::new("future_family");
    let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "authoritative");
    let rejection = admission::plan_read_compatibility(
        &mut batch,
        &index,
        &CompatibilityEdgeRegistry::new(vec![native_edge(family_id.clone())]),
        &ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
        &CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1)),
        &artifact,
    )
    .expect_err("undeclared family should reject");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::UndeclaredFamily
    );
    assert_eq!(
        rejection.store_error_kind(),
        crate::StoreErrorKind::CompatibilityArtifactFamilyUndeclared
    );
    assert!(rejection.reason().contains("undeclared"));
    assert_eq!(batch.counters().rejected_count(), 1);
}

#[test]
fn compatibility_read_admission_rejects_unsupported_format() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
    let mut batch = CompatibilityAdmissionBatch::new();
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact = quarantined_artifact_for_versions(family_id.clone(), 2, 1, 1, "authoritative");
    let rejection = admission::plan_read_compatibility(
        &mut batch,
        &index,
        &CompatibilityEdgeRegistry::new(vec![native_edge(family_id.clone())]),
        &ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
        &CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1)),
        &artifact,
    )
    .expect_err("unsupported format should reject");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::UnsupportedFormatVersion
    );
    assert!(rejection.reason().contains("manifest window"));
}

#[test]
fn compatibility_read_admission_rejects_unsupported_semantic_version() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
    let mut batch = CompatibilityAdmissionBatch::new();
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact = quarantined_artifact_for_versions(family_id.clone(), 1, 2, 1, "authoritative");
    let rejection = admission::plan_read_compatibility(
        &mut batch,
        &index,
        &CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
            family_id.clone(),
            ArtifactSemanticVersion::new(2),
            ArtifactSemanticVersion::new(1),
            CompatibilityRelation::BackwardRead,
        )]),
        &ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
        &CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1)),
        &artifact,
    )
    .expect_err("unsupported semantic version should reject");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::UnsupportedSemanticVersion
    );
    assert!(rejection.reason().contains("manifest window"));
}

#[test]
fn compatibility_missing_edge_rejects_even_for_adjacent_versions() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
    let mut batch = CompatibilityAdmissionBatch::new();
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "authoritative");
    let rejection = admission::plan_read_compatibility(
        &mut batch,
        &index,
        &CompatibilityEdgeRegistry::default(),
        &ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(2)]),
        &CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(2)),
        &artifact,
    )
    .expect_err("numeric adjacency without edge should reject");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::MissingCompatibilityEdge
    );
    assert_eq!(batch.counters().rejected_count(), 1);
    assert_eq!(batch.counters().edge_missing_rejection_count(), 1);
    assert!(rejection.reason().contains("edge is missing"));
}

#[test]
fn compatibility_declared_edge_admits_read_and_write_distinct_receipts() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "authoritative");
    let edges = CompatibilityEdgeRegistry::new(vec![native_edge(family_id.clone())]);
    let mut batch = CompatibilityAdmissionBatch::new();
    let read = admission::plan_read_compatibility(
        &mut batch,
        &index,
        &edges,
        &ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
        &CompatibilityReadIntent::new(family_id.clone(), ArtifactSemanticVersion::new(1)),
        &artifact,
    )
    .expect("declared native read edge should admit");
    let write = admission::plan_write_compatibility(
        &mut batch,
        &index,
        &edges,
        &WriterCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
        &CompatibilityWriteIntent::new(family_id, ArtifactSemanticVersion::new(1)),
        &artifact,
    )
    .expect("declared native write edge should admit");
    assert_eq!(read.receipt().relation(), CompatibilityRelation::Native);
    assert_eq!(write.receipt().relation(), CompatibilityRelation::Native);
}

#[test]
fn compatibility_receipt_reuse_avoids_manifest_and_relation_rechecks() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "authoritative");
    let edges = CompatibilityEdgeRegistry::new(vec![native_edge(family_id.clone())]);
    let mut batch = CompatibilityAdmissionBatch::new();
    let reader = ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
    let intent = CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1));
    let _ =
        admission::plan_read_compatibility(&mut batch, &index, &edges, &reader, &intent, &artifact)
            .expect("first admission should succeed");
    let _ =
        admission::plan_read_compatibility(&mut batch, &index, &edges, &reader, &intent, &artifact)
            .expect("second admission should reuse receipt");
    assert_eq!(batch.counters().relation_recheck_count(), 1);
    assert_eq!(batch.counters().manifest_index_lookup_count(), 1);
    assert_eq!(batch.counters().receipt_reuse_hit_count(), 1);
    assert_eq!(batch.counters().accepted_count(), 2);
    assert_eq!(batch.counters().artifact_row_scan_count(), 0);
}

#[test]
fn compatibility_receipt_reuse_rejects_after_manifest_frontier_changes() {
    let snapshot = CompatibilityRegistry::first_ship();
    let mut ledger = published_manifest_ledger(&snapshot);
    let first_index =
        CompatibilityManifestIndex::rebuild_from_recovered_manifests(&snapshot, &ledger.recover());
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let declaration = snapshot
        .get(CompatibilityFamilyKind::CommitEnvelope)
        .expect("commit envelope declaration exists");
    let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "authoritative");
    let edges = CompatibilityEdgeRegistry::new(vec![native_edge(family_id.clone())]);
    let mut batch = CompatibilityAdmissionBatch::new();
    let reader = ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
    let intent = CompatibilityReadIntent::new(family_id.clone(), ArtifactSemanticVersion::new(1));
    let _ = admission::plan_read_compatibility(
        &mut batch,
        &first_index,
        &edges,
        &reader,
        &intent,
        &artifact,
    )
    .expect("first frontier should admit");
    ledger.publish_declaration(declaration);
    let second_index =
        CompatibilityManifestIndex::rebuild_from_recovered_manifests(&snapshot, &ledger.recover());
    let rejection = admission::plan_read_compatibility(
        &mut batch,
        &second_index,
        &edges,
        &reader,
        &intent,
        &artifact,
    )
    .expect_err("changed manifest frontier should invalidate batch receipt reuse");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::ReceiptBasisMismatch
    );
    assert_eq!(batch.counters().receipt_basis_mismatch_count(), 1);
    assert_eq!(batch.counters().receipt_reuse_rejection_count(), 1);
}

#[test]
fn milestone_12_admission_report_projects_counter_surface() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "authoritative");
    let edges = CompatibilityEdgeRegistry::new(vec![native_edge(family_id.clone())]);
    let mut batch = CompatibilityAdmissionBatch::new();
    let reader = ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
    let intent = CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1));
    let _ =
        admission::plan_read_compatibility(&mut batch, &index, &edges, &reader, &intent, &artifact)
            .expect("first admission should succeed");
    let _ =
        admission::plan_read_compatibility(&mut batch, &index, &edges, &reader, &intent, &artifact)
            .expect("second admission should reuse receipt");
    let report = crate::Milestone12AdmissionReport::from_admission_counters(batch.counters());
    assert_eq!(report.accepted_count, 2);
    assert_eq!(report.rejected_count, 0);
    assert_eq!(report.relation_recheck_count, 1);
    assert_eq!(report.edge_missing_rejection_count, 0);
    assert_eq!(report.receipt_reuse_count, 1);
    assert_eq!(report.artifact_row_scan_count, 0);
    assert_eq!(report.admitted_native_count, 1);
    assert_eq!(report.restore_accept_count, 0);
    assert_eq!(report.restore_out_of_scope_scan_count, 0);
}
