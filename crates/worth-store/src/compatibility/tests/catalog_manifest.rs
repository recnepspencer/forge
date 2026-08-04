use super::super::{
    admission, catalog, decoding, manifests, plan_read_compatibility, ArtifactCompatibilityWindow,
    ArtifactFamilyId, ArtifactFormatVersion, ArtifactSemanticVersion, CompatibilityAdmissionBatch,
    CompatibilityAuthorityClassification, CompatibilityEdgeRegistry, CompatibilityFamilyKind,
    CompatibilityManifestDeclaration, CompatibilityManifestDigest, CompatibilityManifestIndex,
    CompatibilityManifestPublicationLedger, CompatibilityReadIntent, CompatibilityRegistry,
    CompatibilityRejectionKind, CompatibilityRelation, DeclaredCompatibilityEdge,
    QuarantinedDecodedArtifact, ReaderCapabilitySet, FIRST_SHIP_COMPATIBILITY_FAMILIES,
    FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT,
};
use super::MILESTONE_12_COUNTER_NAMES;
use super::{
    native_edge, published_manifest_index, quarantined_artifact_for_family,
    quarantined_artifact_for_versions,
};

#[test]
fn compatibility_first_ship_registry_contains_required_catalog_families() {
    let snapshot = CompatibilityRegistry::first_ship();
    assert_eq!(
        snapshot.declarations().len(),
        FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT
    );
    for kind in catalog::FIRST_SHIP_COMPATIBILITY_FAMILIES {
        assert!(
            snapshot.get(kind).is_some(),
            "missing first-ship family {}",
            kind.label()
        );
    }
}

#[test]
fn compatibility_family_declarations_expose_enforcement_postures() {
    let snapshot = CompatibilityRegistry::first_ship();
    for declaration in snapshot.declarations() {
        assert_eq!(declaration.family_id().as_str(), declaration.kind().label());
        assert!(!declaration.restore_posture().is_empty());
        assert!(!declaration.rolling_posture().is_empty());
        assert!(!declaration.counter_family_id().is_empty());
        assert!(!declaration.certification_lane_id().is_empty());
        match declaration.manifest() {
            catalog::CompatibilityManifestDeclaration::Authoritative(_) => assert_eq!(
                declaration.authority_classification(),
                CompatibilityAuthorityClassification::Authoritative
            ),
            catalog::CompatibilityManifestDeclaration::Derived(_) => assert_eq!(
                declaration.authority_classification(),
                CompatibilityAuthorityClassification::Derived
            ),
        }
    }
}

#[test]
fn compatibility_registry_snapshots_are_deterministic_and_immutable() {
    let first = CompatibilityRegistry::first_ship();
    let second = CompatibilityRegistry::first_ship();
    assert_eq!(first, second);
    let mut labels: Vec<_> = first
        .declarations()
        .iter()
        .map(|declaration| declaration.kind().label())
        .collect();
    let observed = labels.clone();
    labels.sort();
    assert_eq!(observed, labels);
}

#[test]
fn compatibility_manifest_digest_identity_is_deterministic() {
    let family_id = ArtifactFamilyId::new("commit_envelope");
    let window = ArtifactCompatibilityWindow::native(1);
    let left =
        manifests::CompatibilityManifestDigest::compute(&family_id, &window, "authoritative");
    let right =
        manifests::CompatibilityManifestDigest::compute(&family_id, &window, "authoritative");
    assert_eq!(left, right);
}

#[test]
fn compatibility_relation_does_not_infer_from_numeric_ordering() {
    assert_eq!(
        CompatibilityRelation::from_declared_edge(None),
        CompatibilityRelation::Incompatible
    );
    let edge = DeclaredCompatibilityEdge::new(
        ArtifactFamilyId::new("commit_envelope"),
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(2),
        CompatibilityRelation::BackwardRead,
    );
    assert_eq!(
        CompatibilityRelation::from_declared_edge(Some(&edge)),
        CompatibilityRelation::BackwardRead
    );
}

#[test]
fn compatibility_quarantined_decoded_artifact_exposes_only_metadata() {
    let family_id = ArtifactFamilyId::new("commit_envelope");
    let digest = manifests::CompatibilityManifestDigest::compute(
        &family_id,
        &ArtifactCompatibilityWindow::native(1),
        "authoritative",
    );
    let artifact = decoding::QuarantinedDecodedArtifact::new(
        family_id,
        ArtifactFormatVersion::new(1),
        ArtifactSemanticVersion::new(1),
        digest,
        "structural-digest",
        "decode diagnostic",
    );
    assert_eq!(artifact.family_id().as_str(), "commit_envelope");
    assert_eq!(artifact.format_version().value(), 1);
    assert_eq!(artifact.semantic_version().value(), 1);
    assert_eq!(artifact.structural_digest(), "structural-digest");
    assert_eq!(artifact.diagnostic_context(), "decode diagnostic");
}

#[test]
fn milestone_12_phase_1_counter_contract_names_every_required_counter() {
    for counter in [
        "compatibility.admission.accepted_count",
        "compatibility.admission.rejected_count",
        "compatibility.manifest.index_rebuild_count",
        "compatibility.manifest.entries_visited",
        "compatibility.manifest.index_lookup_count",
        "compatibility.manifest.digest_check_count",
        "compatibility.manifest.publication_count",
        "compatibility.manifest.recovery_record_count",
        "compatibility.manifest.publication_gap_count",
        "compatibility.manifest.digest_mismatch_count",
        "compatibility.manifest.window_mismatch_count",
        "compatibility.receipt.reuse_rejection_count",
        "compatibility.receipt.reuse_hit_count",
        "compatibility.receipt.basis_mismatch_count",
        "compatibility.relation.recheck_count",
        "compatibility.edge.missing_rejection_count",
        "compatibility.index.row_scan_count",
        "compatibility.decode.malformed_frame_count",
        "compatibility.adapter.cost_class_count",
        "compatibility.adapter.inline_count",
        "compatibility.adapter.batch_count",
        "compatibility.adapter.maintenance_scheduled_count",
        "compatibility.adapter.parity_failure_count",
        "compatibility.adapter.input_record_count",
        "compatibility.adapter.output_record_count",
        "compatibility.adapter.allocation_scope_count",
        "compatibility.adapter.hot_path_rejection_count",
        "compatibility.adapter.maintenance_required_rejection_count",
        "compatibility.adapter.out_of_scope_rejection_count",
        "compatibility.admission.native_count",
        "compatibility.admission.forward_backward_count",
        "compatibility.authoritative.partial_truth_rejection_count",
        "compatibility.derived.reuse_incompatibility_count",
        "compatibility.derived.rebuild_incompatibility_count",
        "compatibility.derived.rebuild_required_count",
        "compatibility.derived.invalidation_count",
        "compatibility.derived.stale_version_rejection_count",
        "compatibility.derived.rebuild_debt_count",
        "compatibility.maintenance.rebuild_admission_count",
        "compatibility.maintenance.rebuild_rejection_count",
        "compatibility.derived.lane_plan_count",
        "compatibility.derived.lane_reuse_count",
        "compatibility.derived.lane_invalidation_count",
        "compatibility.derived.lane_rejection_count",
        "compatibility.derived.snapshot_reuse_count",
        "compatibility.derived.delta_reuse_count",
        "compatibility.derived.layout_basis_rejection_count",
        "compatibility.derived.bulk_resume_rejection_count",
        "compatibility.derived.maintenance_summary_rebuild_count",
        "compatibility.tier.non_authority_preserved_count",
        "compatibility.tier.manifest_rejection_count",
        "compatibility.maintenance.lane_mismatch_rejection_count",
        "compatibility.rolling.window_admission_count",
        "compatibility.rolling.window_rejection_count",
        "compatibility.rolling.multi_writer_rejection_count",
        "compatibility.rolling.mixed_version_skew_count",
        "compatibility.restore.out_of_scope_scan_count",
        "compatibility.restore.accept_count",
        "compatibility.restore.rejection_count",
        "compatibility.restore.publication_conflict_rejection_count",
        "compatibility.disaster_recovery.truth_window_count",
        "compatibility.disaster_recovery.derived_window_count",
    ] {
        assert!(
            MILESTONE_12_COUNTER_NAMES.contains(&counter),
            "missing counter {counter}"
        );
    }
}

#[test]
fn compatibility_manifest_index_rebuild_is_manifest_bounded() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
    assert_eq!(index.entries().count(), snapshot.declarations().len());
    assert_eq!(index.rebuild_counters().manifest_index_rebuild_count(), 1);
    assert_eq!(
        index.rebuild_counters().manifest_entries_visited(),
        snapshot.declarations().len() as u64
    );
    assert_eq!(index.rebuild_counters().artifact_row_scan_count(), 0);
}

#[test]
fn compatibility_manifest_publication_records_are_append_only() {
    let snapshot = CompatibilityRegistry::first_ship();
    let declaration = snapshot
        .get(CompatibilityFamilyKind::CommitEnvelope)
        .expect("commit envelope family exists");
    let mut ledger = CompatibilityManifestPublicationLedger::new();
    let first = ledger.publish_declaration(declaration);
    let second = ledger.publish_declaration(declaration);
    assert_eq!(ledger.records().len(), 2);
    assert_eq!(first.record().publication_sequence(), 1);
    assert_eq!(second.record().publication_sequence(), 2);
    assert_eq!(
        first.record().manifest_digest(),
        second.record().manifest_digest()
    );
    assert_ne!(first.frontier().identity(), second.frontier().identity());
}

#[test]
fn compatibility_recovered_manifest_index_is_publication_bounded() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = published_manifest_index(&snapshot);
    assert_eq!(index.entries().count(), snapshot.declarations().len());
    assert_eq!(
        index.rebuild_counters().manifest_publication_count(),
        snapshot.declarations().len() as u64
    );
    assert_eq!(
        index.rebuild_counters().manifest_recovery_record_count(),
        snapshot.declarations().len() as u64
    );
    assert_eq!(index.rebuild_counters().artifact_row_scan_count(), 0);
}

#[test]
fn compatibility_recovered_manifest_gap_rejects_declared_family() {
    let snapshot = CompatibilityRegistry::first_ship();
    let recovered = CompatibilityManifestPublicationLedger::new().recover();
    let index = CompatibilityManifestIndex::rebuild_from_recovered_manifests(&snapshot, &recovered);
    let mut batch = CompatibilityAdmissionBatch::new();
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "authoritative");
    let rejection = admission::plan_read_compatibility(
        &mut batch,
        &index,
        &CompatibilityEdgeRegistry::new(vec![native_edge(family_id.clone())]),
        &ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
        &CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1)),
        &artifact,
    )
    .expect_err("declared family without recovered manifest publication should reject");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::MissingManifestPublication
    );
    assert_eq!(batch.counters().manifest_publication_gap_count(), 1);
}

#[test]
fn compatibility_recovered_manifest_digest_drift_rejects() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = published_manifest_index(&snapshot);
    let mut batch = CompatibilityAdmissionBatch::new();
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact = quarantined_artifact_for_versions(family_id.clone(), 1, 1, 2, "authoritative");
    let rejection = admission::plan_read_compatibility(
        &mut batch,
        &index,
        &CompatibilityEdgeRegistry::new(vec![native_edge(family_id.clone())]),
        &ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
        &CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1)),
        &artifact,
    )
    .expect_err("recovered manifest digest drift should reject");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::RecoveredManifestDigestMismatch
    );
    assert_eq!(batch.counters().manifest_digest_mismatch_count(), 1);
}

#[test]
fn compatibility_recovered_manifest_window_drift_rejects() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = published_manifest_index(&snapshot);
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
    .expect_err("recovered manifest window drift should reject");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::RecoveredManifestWindowMismatch
    );
    assert_eq!(batch.counters().manifest_window_mismatch_count(), 1);
}
