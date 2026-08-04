use super::super::{
    decoding, derived, manifests, ArtifactCompatibilityWindow, ArtifactFamilyId,
    ArtifactFormatVersion, ArtifactSemanticVersion, BackupCompatibilityManifest,
    CompatibilityAdapterCostClass, CompatibilityAdapterDigest, CompatibilityAdapterId,
    CompatibilityAdmissionCounters, CompatibilityAdmissionPath, CompatibilityAdmissionReceipt,
    CompatibilityArtifactFrameHeader, CompatibilityFamilyKind, CompatibilityManifestDigest,
    CompatibilityManifestIndex, CompatibilityManifestPublicationLedger, CompatibilityRegistry,
    CompatibilityRegistrySnapshot, CompatibilityRejectionKind, CompatibilityRelation,
    DeclaredCompatibilityAdapter, DeclaredCompatibilityEdge, DerivedBasisCompatibilityInput,
    DerivedCompatibilityLaneRegistry, DerivedCompatibilityReusePlan, DerivedFamilyDeclaration,
    Milestone12CertificationLaneInput, Milestone12CertificationLaneKind,
    Milestone12CertificationLaneOutcome, Milestone12CertificationLaneStatus,
    QuarantinedDecodedArtifact, ReadCompatibilityReceipt,
};
use super::{
    Milestone12ComplexityPathStatus, Milestone12ComplexitySurface, Milestone12VersionSkewReport,
};

pub(super) fn native_edge(family_id: ArtifactFamilyId) -> DeclaredCompatibilityEdge {
    DeclaredCompatibilityEdge::new(
        family_id,
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(1),
        CompatibilityRelation::Native,
    )
}

pub(super) fn backup_manifest_for_family(
    family_id: ArtifactFamilyId,
    version: u32,
) -> BackupCompatibilityManifest {
    let window = ArtifactCompatibilityWindow::native(version);
    let digest = manifests::CompatibilityManifestDigest::compute(&family_id, &window, "backup");
    BackupCompatibilityManifest::new(family_id, window, digest)
}

pub(super) fn published_manifest_ledger(
    snapshot: &CompatibilityRegistrySnapshot,
) -> CompatibilityManifestPublicationLedger {
    let mut ledger = CompatibilityManifestPublicationLedger::new();
    for declaration in snapshot.declarations() {
        ledger.publish_declaration(declaration);
    }
    ledger
}

pub(super) fn published_manifest_index(
    snapshot: &CompatibilityRegistrySnapshot,
) -> CompatibilityManifestIndex {
    let ledger = published_manifest_ledger(snapshot);
    CompatibilityManifestIndex::rebuild_from_recovered_manifests(snapshot, &ledger.recover())
}

pub(super) fn quarantined_artifact_for_family(
    family_id: ArtifactFamilyId,
    version: u32,
    authority_label: &str,
) -> QuarantinedDecodedArtifact {
    quarantined_artifact_for_versions(family_id, version, version, version, authority_label)
}

pub(super) fn quarantined_artifact_for_versions(
    family_id: ArtifactFamilyId,
    format_version: u32,
    semantic_version: u32,
    digest_version: u32,
    authority_label: &str,
) -> QuarantinedDecodedArtifact {
    let digest = manifests::CompatibilityManifestDigest::compute(
        &family_id,
        &ArtifactCompatibilityWindow::native(digest_version),
        authority_label,
    );
    decoding::QuarantinedDecodedArtifact::new(
        family_id,
        ArtifactFormatVersion::new(format_version),
        ArtifactSemanticVersion::new(semantic_version),
        digest,
        "structural-digest",
        "decode diagnostic",
    )
}

pub(super) fn frame_header(
    family_id: ArtifactFamilyId,
    format_version: u32,
    semantic_version: u32,
    authority_label: &str,
    declared_payload_len: usize,
) -> CompatibilityArtifactFrameHeader {
    let digest = manifests::CompatibilityManifestDigest::compute(
        &family_id,
        &ArtifactCompatibilityWindow::native(1),
        authority_label,
    );
    CompatibilityArtifactFrameHeader::new(
        family_id,
        ArtifactFormatVersion::new(format_version),
        ArtifactSemanticVersion::new(semantic_version),
        digest,
        declared_payload_len,
    )
}

pub(super) fn adapter(cost_class: CompatibilityAdapterCostClass) -> DeclaredCompatibilityAdapter {
    DeclaredCompatibilityAdapter::new(
        CompatibilityAdapterId::new("adapter"),
        CompatibilityAdapterDigest::new("digest"),
        cost_class,
    )
}

pub(super) fn derived_family_declaration(
    snapshot: &CompatibilityRegistrySnapshot,
    kind: CompatibilityFamilyKind,
) -> DerivedFamilyDeclaration {
    DerivedFamilyDeclaration::new(
        snapshot
            .get(kind)
            .expect("expected first-ship derived family")
            .clone(),
    )
}

pub(super) fn synthetic_read_receipt(
    artifact: &QuarantinedDecodedArtifact,
    target_semantic_version: ArtifactSemanticVersion,
    relation: CompatibilityRelation,
) -> ReadCompatibilityReceipt {
    ReadCompatibilityReceipt::new(CompatibilityAdmissionReceipt::new(
        artifact.family_id().clone(),
        artifact.manifest_digest().clone(),
        "test-registry",
        "test-frontier",
        artifact.semantic_version(),
        target_semantic_version,
        CompatibilityAdmissionPath::HotRead,
        relation,
    ))
}

pub(super) fn derived_rebuild_plan_for_test() -> DerivedCompatibilityReusePlan {
    let snapshot = CompatibilityRegistry::first_ship();
    let family_id = CompatibilityFamilyKind::SnapshotRecord.family_id();
    let artifact = quarantined_artifact_for_family(family_id, 1, "derived");
    let receipt = synthetic_read_receipt(
        &artifact,
        ArtifactSemanticVersion::new(1),
        CompatibilityRelation::BackwardRead,
    );
    let declaration =
        derived_family_declaration(&snapshot, CompatibilityFamilyKind::SnapshotRecord);
    let mut counters = CompatibilityAdmissionCounters::default();
    derived::plan_exact_derived_reuse(&mut counters, &declaration, &artifact, &receipt)
        .expect("non-native receipt should require rebuild")
}

pub(super) fn milestone_12_certification_input() -> Milestone12CertificationLaneInput {
    Milestone12CertificationLaneInput::new(
        CompatibilityFamilyKind::CommitEnvelope.family_id(),
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(1),
        Some(CompatibilityRelation::Native),
        None,
    )
}

pub(super) fn milestone_12_certification_outcomes() -> Vec<Milestone12CertificationLaneOutcome> {
    milestone_12_certification_outcomes_with_counterless_lane(None)
}

pub(super) fn milestone_12_certification_outcomes_with_zero_counter_lane(
) -> Vec<Milestone12CertificationLaneOutcome> {
    milestone_12_certification_outcomes_with_counterless_lane(Some(
        Milestone12CertificationLaneKind::CatalogCompleteness,
    ))
}

fn milestone_12_certification_outcomes_with_counterless_lane(
    counterless_lane: Option<Milestone12CertificationLaneKind>,
) -> Vec<Milestone12CertificationLaneOutcome> {
    Milestone12CertificationLaneKind::mandatory_phase_5a()
        .iter()
        .copied()
        .map(|kind| {
            let mut counters = CompatibilityAdmissionCounters::default();
            if counterless_lane != Some(kind) {
                counters.record_relation_recheck();
            }
            match kind {
                Milestone12CertificationLaneKind::CatalogCompleteness
                | Milestone12CertificationLaneKind::DisasterRecoveryTruthWindow
                | Milestone12CertificationLaneKind::DisasterRecoveryDerivedWindow => {
                    Milestone12CertificationLaneOutcome::non_admitted(
                        kind,
                        milestone_12_certification_input(),
                        Milestone12CertificationLaneStatus::EvidenceOnly,
                        &counters,
                    )
                }
                Milestone12CertificationLaneKind::AuthoritativeNativeRead
                | Milestone12CertificationLaneKind::AuthoritativeForwardRead
                | Milestone12CertificationLaneKind::AuthoritativeBackwardRead
                | Milestone12CertificationLaneKind::DerivedSnapshotReuseAccepted
                | Milestone12CertificationLaneKind::MaintenanceSummaryRebuildAdmitted
                | Milestone12CertificationLaneKind::TierManifestNonAuthorityPreserved
                | Milestone12CertificationLaneKind::RollingTwoCapabilityAdmitted
                | Milestone12CertificationLaneKind::AdapterParityAdmitted
                | Milestone12CertificationLaneKind::RestoreScopedBackupAdmitted => {
                    Milestone12CertificationLaneOutcome::accepted(
                        kind,
                        milestone_12_certification_input(),
                        CompatibilityRelation::Native,
                        &counters,
                    )
                }
                Milestone12CertificationLaneKind::AuthoritativeMissingEdgeRejected
                | Milestone12CertificationLaneKind::RollingMissingEdgeRejected
                | Milestone12CertificationLaneKind::RestoreMissingEdgeRejected => {
                    counters.record_edge_missing_rejection();
                    Milestone12CertificationLaneOutcome::rejected(
                        kind,
                        milestone_12_certification_input(),
                        CompatibilityRejectionKind::MissingCompatibilityEdge,
                        &counters,
                    )
                }
                Milestone12CertificationLaneKind::RollingMultiWriterRejected => {
                    Milestone12CertificationLaneOutcome::rejected(
                        kind,
                        milestone_12_certification_input(),
                        CompatibilityRejectionKind::RollingMultiWriterRejected,
                        &counters,
                    )
                }
                Milestone12CertificationLaneKind::RestoreOutOfScopeRejected => {
                    Milestone12CertificationLaneOutcome::rejected(
                        kind,
                        milestone_12_certification_input(),
                        CompatibilityRejectionKind::RestoreOutOfScopeScanRejected,
                        &counters,
                    )
                }
                Milestone12CertificationLaneKind::RestorePublicationConflictRejected => {
                    Milestone12CertificationLaneOutcome::rejected(
                        kind,
                        milestone_12_certification_input(),
                        CompatibilityRejectionKind::RestorePublicationConflictRejected,
                        &counters,
                    )
                }
                Milestone12CertificationLaneKind::AuthoritativeIncompatibleEdgeRejected
                | Milestone12CertificationLaneKind::DerivedLayoutBasisRejected
                | Milestone12CertificationLaneKind::DerivedBulkResumeRejected
                | Milestone12CertificationLaneKind::RollingAdapterEdgeRejected => {
                    Milestone12CertificationLaneOutcome::rejected(
                        kind,
                        milestone_12_certification_input(),
                        CompatibilityRejectionKind::UnsupportedSemanticVersion,
                        &counters,
                    )
                }
                Milestone12CertificationLaneKind::AdapterParityDigestRejected => {
                    Milestone12CertificationLaneOutcome::rejected(
                        kind,
                        milestone_12_certification_input(),
                        CompatibilityRejectionKind::AdapterParityFailure,
                        &counters,
                    )
                }
            }
        })
        .collect()
}

pub(super) fn milestone_12_version_skew_report() -> Milestone12VersionSkewReport {
    Milestone12VersionSkewReport {
        mixed_version_store_lane_count: 1,
        mixed_version_replica_lane_count: 1,
        rolling_upgrade_skew_rejection_count: 1,
    }
}

pub(super) fn milestone_12_complexity_surface() -> Milestone12ComplexitySurface {
    Milestone12ComplexitySurface {
        relation_recheck: Milestone12ComplexityPathStatus::verified("bounded relation recheck"),
        index_lookup: Milestone12ComplexityPathStatus::verified("manifest index lookup"),
        adapter_cost: Milestone12ComplexityPathStatus::verified("declared adapter cost class"),
        restore_scan: Milestone12ComplexityPathStatus::verified("backup-scope scan bound"),
    }
}

pub(super) fn derived_lane_fixture(
    family_kind: CompatibilityFamilyKind,
    relation: CompatibilityRelation,
    format_version: u32,
    semantic_version: u32,
) -> (
    DerivedBasisCompatibilityInput,
    QuarantinedDecodedArtifact,
    ReadCompatibilityReceipt,
) {
    let snapshot = CompatibilityRegistry::first_ship();
    let lane_snapshot =
        DerivedCompatibilityLaneRegistry::from_compatibility_snapshot(&snapshot).snapshot();
    let lane = lane_snapshot
        .get_by_family_kind(family_kind)
        .expect("expected derived lane")
        .clone();
    let derived_family = derived_family_declaration(&snapshot, family_kind);
    let artifact = quarantined_artifact_for_versions(
        family_kind.family_id(),
        format_version,
        semantic_version,
        1,
        "derived",
    );
    let receipt = synthetic_read_receipt(&artifact, ArtifactSemanticVersion::new(1), relation);
    (
        DerivedBasisCompatibilityInput::new(
            lane,
            derived_family,
            ArtifactCompatibilityWindow::native(1),
        ),
        artifact,
        receipt,
    )
}
