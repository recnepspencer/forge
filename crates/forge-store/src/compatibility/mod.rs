#![allow(dead_code)]

mod admission;
mod authoritative;
mod catalog;
mod certification;
mod certification_runner;
mod decoding;
mod derived;
mod evidence;
mod manifests;
mod restore;
mod rolling;

pub use admission::{
    BackwardReadCompatibilityWitness, CompatibilityAdapterCostClass, CompatibilityAdapterDigest,
    CompatibilityAdapterId, CompatibilityAdapterParityWitness, CompatibilityAdmissionBatch,
    CompatibilityAdmissionCounters, CompatibilityAdmissionPath, CompatibilityAdmissionPlan,
    CompatibilityAdmissionReceipt, CompatibilityBatchScope, CompatibilityDecision,
    CompatibilityEdgeProof, CompatibilityEdgeRegistry, CompatibilityManifestIndex,
    CompatibilityManifestIndexEntry, CompatibilityReadAdmissionOutcome, CompatibilityReadIntent,
    CompatibilityRejection, CompatibilityRejectionKind, CompatibilityRelation,
    CompatibilityWriteAdmissionOutcome, CompatibilityWriteIntent, DeclaredCompatibilityAdapter,
    DeclaredCompatibilityEdge, DerivedReuseCompatibilityReceipt, ForwardReadCompatibilityWitness,
    ReadCompatibilityReceipt, ReaderCapabilitySet, RestoreCompatibilityReceipt,
    RollingWindowCompatibilityReceipt, SemanticMeaningPreservationWitness, UpgradeAdmissionWitness,
    WriteCompatibilityReceipt, WriterCapabilitySet,
};
#[allow(unused_imports)]
pub use authoritative::{
    AuthoritativeAdmissionReport, AuthoritativeCompatibilityWitness,
    AuthoritativeMeaningDeclaration, AuthoritativePartialTruthRejection,
    AuthoritativeUnknownMeaning, BackwardAuthoritativeReadPlan, ForwardAuthoritativeReadPlan,
    UnsupportedAuthoritativeVersion,
};
pub use catalog::{
    AuthoritativeFamilyDeclaration, CompatibilityAuthorityClassification,
    CompatibilityFamilyDeclaration, CompatibilityFamilyKind, CompatibilityRegistry,
    CompatibilityRegistrySnapshot, DerivedFamilyDeclaration, FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT,
};
#[allow(unused_imports)]
pub use certification::{
    Milestone12CertificationLaneId, Milestone12CertificationLaneInput,
    Milestone12CertificationLaneKind, Milestone12CertificationLaneOutcome,
    Milestone12CertificationLaneRejection, Milestone12CertificationLaneStatus,
    Milestone12CertificationRunSummary, Milestone12CompatibilityMatrix,
    Milestone12CompatibilityMatrixEntry, Milestone12CompatibilityMatrixStatus,
};
pub use certification_runner::{
    Milestone12ArtifactFormatEvolutionCertification, Milestone12CertificationDiagnostics,
    Milestone12CertificationDigestSet, Milestone12CertificationFixture,
    Milestone12CertificationRunner, Milestone12CertificationScenario,
};
pub use decoding::{
    CompatibilityArtifactFrameHeader, CompatibilityCheckedArtifact, FramedArtifactRecord,
    QuarantinedDecodedArtifact, RawArtifactBytes, SemanticArtifactView,
};
pub use derived::{
    BulkResumeCompatibilityPlan, BulkResumeCompatibilityRejection, BulkResumeInterpretation,
    CompatibilityMaintenanceAdmissionWitness, CompatibilityMaintenanceLaneAdmission,
    CompatibilityMaintenanceLaneRejection, CompatibilityMaintenanceLaneRequirement,
    CompatibilityRebuildDebt, DerivedBasisCompatibilityInput, DerivedBasisCompatibilityPlan,
    DerivedBasisCompatibilityPosture, DerivedCompatibilityLane,
    DerivedCompatibilityLaneDeclaration, DerivedCompatibilityLaneKind,
    DerivedCompatibilityLaneRegistry, DerivedCompatibilityLaneSnapshot,
    DerivedCompatibilityReusePlan, DerivedCompatibilityReuseWitness, DerivedCompatibilityWitness,
    DerivedInvalidationPlan, DerivedInvalidationReason, DerivedLaneCompatibilityPlan,
    DerivedLaneCompatibilityPosture, DerivedLaneInvalidation, DerivedLaneRebuildRequirement,
    DerivedLaneRejection, DerivedLaneReuseAdmission, DerivedRebuildCompatibilityPlan,
    DerivedRebuildRequirement, DerivedReusePosture, RetainedAuthorityCompatibilityWitness,
    StaleDerivedVersionRejection, TierCompatibilityNonAuthorityPosture,
    TierManifestCompatibilityPlan, TierManifestCompatibilityRejection,
};
pub use evidence::{
    ArtifactFamilyCompatibilityIndex, ArtifactFamilyVersionSummary,
    CompatibilityAdapterCostClassReport, CompatibilityAdapterCostSummary,
    CompatibilityAdmissionReceiptSummary, CompatibilityAuditPlan, CompatibilityAuditSummary,
    CompatibilityAuditUnit, CompatibilityBatchScopeReport, CompatibilityManifestSummary,
    CompatibilityRebuildSummary, DerivedInvalidationSummary, Milestone12Phase1Evidence,
    ReaderWriterSkewSummary, RestoreCompatibilityBreadthBudget, RestoreVersionSummary,
};
#[allow(unused_imports)]
pub use manifests::{
    ArtifactCompatibilityWindow, ArtifactFamilyId, ArtifactFormatVersion, ArtifactSemanticVersion,
    AuthoritativeCompatibilityManifest, CompatibilityManifestDigest, CompatibilityManifestFrontier,
    CompatibilityManifestPublicationLedger, CompatibilityManifestPublicationReceipt,
    CompatibilityManifestPublicationRecord, CompatibilityManifestPublicationUnit,
    CompatibilityManifestRecoveryPlan, CompatibilityRecoveredManifestIndex,
    DerivedCompatibilityManifest, ManifestDigestMismatch, ManifestPublicationGap,
    ManifestPublicationWitness, ManifestRecoverySummary,
};
#[allow(unused_imports)]
pub use restore::{
    BackupCompatibilityManifest, DisasterRecoveryCompatibilityClass,
    DisasterRecoveryCompatibilityPlan, DisasterRecoveryCompatibilityWindow, RestoreBackupScope,
    RestoreCompatibilityPlan, RestoreCompatibilityTarget, RestorePublicationConflictKind,
    RestorePublicationConflictSet, RestorePublicationConflictUnit, RestorePublicationWitness,
    RestoreVersionRejection,
};
pub use rolling::{
    MaintenanceCompatibilityPosture, MixedVersionPostureKind, MixedVersionStorePosture,
    ReplicaCompatibilityPosture, RollingCapabilityWindow, RollingUpgradeAdmissionPlan,
    RollingUpgradePolicy, RollingUpgradeRejection, RollingUpgradeWindow, UpgradeSkewRejection,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence::{
        Milestone12AdmissionReport, Milestone12CertificationEvidenceBundle,
        Milestone12ComplexityPathStatus, Milestone12ComplexitySurface, Milestone12CounterContract,
        Milestone12CounterContractViolation, Milestone12VersionSkewReport,
        MILESTONE_12_ADMISSION_REPORT_COUNTER_FIELD_NAMES, MILESTONE_12_COUNTER_NAMES,
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
            "compatibility.adapter.hot_path_rejection_count",
            "compatibility.adapter.maintenance_required_rejection_count",
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
        let index =
            CompatibilityManifestIndex::rebuild_from_recovered_manifests(&snapshot, &recovered);
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
        let artifact =
            quarantined_artifact_for_versions(family_id.clone(), 1, 1, 2, "authoritative");
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
        let artifact =
            quarantined_artifact_for_versions(family_id.clone(), 2, 1, 1, "authoritative");
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
        let artifact =
            quarantined_artifact_for_versions(family_id.clone(), 2, 1, 1, "authoritative");
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
        let artifact =
            quarantined_artifact_for_versions(family_id.clone(), 1, 2, 1, "authoritative");
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
        let reader =
            ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
        let intent = CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1));
        let _ = admission::plan_read_compatibility(
            &mut batch, &index, &edges, &reader, &intent, &artifact,
        )
        .expect("first admission should succeed");
        let _ = admission::plan_read_compatibility(
            &mut batch, &index, &edges, &reader, &intent, &artifact,
        )
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
        let first_index = CompatibilityManifestIndex::rebuild_from_recovered_manifests(
            &snapshot,
            &ledger.recover(),
        );
        let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let declaration = snapshot
            .get(CompatibilityFamilyKind::CommitEnvelope)
            .expect("commit envelope declaration exists");
        let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "authoritative");
        let edges = CompatibilityEdgeRegistry::new(vec![native_edge(family_id.clone())]);
        let mut batch = CompatibilityAdmissionBatch::new();
        let reader =
            ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
        let intent =
            CompatibilityReadIntent::new(family_id.clone(), ArtifactSemanticVersion::new(1));
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
        let second_index = CompatibilityManifestIndex::rebuild_from_recovered_manifests(
            &snapshot,
            &ledger.recover(),
        );
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
        let reader =
            ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
        let intent = CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1));
        let _ = admission::plan_read_compatibility(
            &mut batch, &index, &edges, &reader, &intent, &artifact,
        )
        .expect("first admission should succeed");
        let _ = admission::plan_read_compatibility(
            &mut batch, &index, &edges, &reader, &intent, &artifact,
        )
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
        let rejection = authoritative::admit_authoritative_meaning(
            batch.counters_mut(),
            &checked,
            &receipt,
            None,
        )
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
    fn compatibility_native_read_receipt_admits_exact_derived_reuse() {
        let snapshot = CompatibilityRegistry::first_ship();
        let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
        let family_id = CompatibilityFamilyKind::SnapshotRecord.family_id();
        let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "derived");
        let mut batch = CompatibilityAdmissionBatch::new();
        let receipt = admission::plan_read_compatibility(
            &mut batch,
            &index,
            &CompatibilityEdgeRegistry::new(vec![native_edge(family_id.clone())]),
            &ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
            &CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1)),
            &artifact,
        )
        .expect("native derived read should admit");
        let declaration =
            derived_family_declaration(&snapshot, CompatibilityFamilyKind::SnapshotRecord);
        let plan = derived::plan_exact_derived_reuse(
            batch.counters_mut(),
            &declaration,
            &artifact,
            &receipt,
        )
        .expect("native receipt should admit exact derived reuse");
        assert_eq!(plan.posture(), DerivedReusePosture::ReuseAdmitted);
        assert!(plan.reuse_receipt().is_some());

        let checked = admission::check_artifact_with_read_receipt(artifact, &receipt)
            .expect("native receipt should check artifact");
        let witness = derived::admit_checked_derived_reuse(checked, &plan)
            .expect("checked native artifact should produce reuse witness");
        assert_eq!(witness.family_id().as_str(), "snapshot_record");
    }

    #[test]
    fn compatibility_non_native_read_receipt_requires_derived_rebuild() {
        let snapshot = CompatibilityRegistry::first_ship();
        let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
        let family_id = CompatibilityFamilyKind::SnapshotRecord.family_id();
        let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "derived");
        let mut batch = CompatibilityAdmissionBatch::new();
        let edge = DeclaredCompatibilityEdge::new(
            family_id.clone(),
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(1),
            CompatibilityRelation::BackwardRead,
        );
        let receipt = admission::plan_read_compatibility(
            &mut batch,
            &index,
            &CompatibilityEdgeRegistry::new(vec![edge]),
            &ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
            &CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1)),
            &artifact,
        )
        .expect("declared non-native read should admit read");
        let declaration =
            derived_family_declaration(&snapshot, CompatibilityFamilyKind::SnapshotRecord);
        let plan = derived::plan_exact_derived_reuse(
            batch.counters_mut(),
            &declaration,
            &artifact,
            &receipt,
        )
        .expect("non-native read should become rebuild plan");
        assert_eq!(plan.posture(), DerivedReusePosture::RebuildRequired);
        assert!(plan.reuse_receipt().is_none());
        assert_eq!(batch.counters().derived_rebuild_required_count(), 1);

        let requirement = DerivedRebuildRequirement::from_reuse_plan(
            &plan,
            ArtifactCompatibilityWindow::native(1),
        )
        .expect("rebuild plan should produce a rebuild requirement");
        assert_eq!(requirement.family_id().as_str(), "snapshot_record");
    }

    #[test]
    fn compatibility_mismatched_receipt_rejects_derived_reuse() {
        let snapshot = CompatibilityRegistry::first_ship();
        let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
        let receipt_family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let artifact_family_id = CompatibilityFamilyKind::SnapshotRecord.family_id();
        let receipt_artifact =
            quarantined_artifact_for_family(receipt_family_id.clone(), 1, "authoritative");
        let derived_artifact =
            quarantined_artifact_for_family(artifact_family_id.clone(), 1, "derived");
        let mut batch = CompatibilityAdmissionBatch::new();
        let receipt = admission::plan_read_compatibility(
            &mut batch,
            &index,
            &CompatibilityEdgeRegistry::new(vec![native_edge(receipt_family_id.clone())]),
            &ReaderCapabilitySet::new(
                receipt_family_id.clone(),
                vec![ArtifactSemanticVersion::new(1)],
            ),
            &CompatibilityReadIntent::new(receipt_family_id, ArtifactSemanticVersion::new(1)),
            &receipt_artifact,
        )
        .expect("receipt source should admit");
        let declaration =
            derived_family_declaration(&snapshot, CompatibilityFamilyKind::SnapshotRecord);
        let rejection = derived::plan_exact_derived_reuse(
            batch.counters_mut(),
            &declaration,
            &derived_artifact,
            &receipt,
        )
        .expect_err("mismatched receipt must reject derived reuse");
        assert_eq!(
            rejection.kind(),
            CompatibilityRejectionKind::DerivedReuseIncompatible
        );
        assert_eq!(
            rejection.store_error_kind(),
            crate::StoreErrorKind::CompatibilityDerivedReuseIncompatible
        );
        assert_eq!(batch.counters().derived_reuse_incompatibility_count(), 1);
    }

    #[test]
    fn compatibility_authoritative_receipt_cannot_admit_derived_reuse() {
        let snapshot = CompatibilityRegistry::first_ship();
        let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
        let authoritative_family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let artifact =
            quarantined_artifact_for_family(authoritative_family_id.clone(), 1, "authoritative");
        let mut batch = CompatibilityAdmissionBatch::new();
        let receipt = admission::plan_read_compatibility(
            &mut batch,
            &index,
            &CompatibilityEdgeRegistry::new(vec![native_edge(authoritative_family_id.clone())]),
            &ReaderCapabilitySet::new(
                authoritative_family_id.clone(),
                vec![ArtifactSemanticVersion::new(1)],
            ),
            &CompatibilityReadIntent::new(authoritative_family_id, ArtifactSemanticVersion::new(1)),
            &artifact,
        )
        .expect("authoritative read should admit");
        let declaration =
            derived_family_declaration(&snapshot, CompatibilityFamilyKind::SnapshotRecord);
        let rejection = derived::plan_exact_derived_reuse(
            batch.counters_mut(),
            &declaration,
            &artifact,
            &receipt,
        )
        .expect_err("authoritative artifact cannot satisfy a derived family declaration");
        assert_eq!(
            rejection.kind(),
            CompatibilityRejectionKind::DerivedReuseIncompatible
        );
    }

    #[test]
    fn compatibility_derived_basis_format_drift_invalidates_without_runtime_rebuild() {
        let snapshot = CompatibilityRegistry::first_ship();
        let family_id = CompatibilityFamilyKind::SnapshotRecord.family_id();
        let artifact = quarantined_artifact_for_versions(family_id.clone(), 2, 1, 1, "derived");
        let receipt = synthetic_read_receipt(
            &artifact,
            ArtifactSemanticVersion::new(1),
            CompatibilityRelation::Native,
        );
        let declaration =
            derived_family_declaration(&snapshot, CompatibilityFamilyKind::SnapshotRecord);
        let mut counters = CompatibilityAdmissionCounters::default();
        let plan = derived::plan_derived_basis_compatibility(
            &mut counters,
            &declaration,
            &artifact,
            &receipt,
            ArtifactCompatibilityWindow::native(1),
        )
        .expect("format drift should produce an invalidation plan");
        assert_eq!(
            plan.posture(),
            DerivedBasisCompatibilityPosture::InvalidateAndRebuild
        );
        assert_eq!(
            plan.invalidation().expect("invalidation").reason_code(),
            DerivedInvalidationReason::FormatWindowMismatch
        );
        assert!(plan.rebuild_requirement().is_some());
        assert_eq!(counters.derived_invalidation_count(), 1);
        assert_eq!(counters.derived_rebuild_required_count(), 1);
    }

    #[test]
    fn compatibility_derived_basis_non_native_relation_invalidates() {
        let snapshot = CompatibilityRegistry::first_ship();
        let family_id = CompatibilityFamilyKind::SnapshotRecord.family_id();
        let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "derived");
        let receipt = synthetic_read_receipt(
            &artifact,
            ArtifactSemanticVersion::new(1),
            CompatibilityRelation::ForwardRead,
        );
        let declaration =
            derived_family_declaration(&snapshot, CompatibilityFamilyKind::SnapshotRecord);
        let mut counters = CompatibilityAdmissionCounters::default();
        let plan = derived::plan_derived_basis_compatibility(
            &mut counters,
            &declaration,
            &artifact,
            &receipt,
            ArtifactCompatibilityWindow::native(1),
        )
        .expect("non-native read should force derived rebuild");
        assert_eq!(
            plan.invalidation().expect("invalidation").reason_code(),
            DerivedInvalidationReason::NonNativeReadRelation
        );
    }

    #[test]
    fn compatibility_derived_rebuild_requires_retained_authority() {
        let requirement = DerivedRebuildRequirement::from_reuse_plan(
            &derived_rebuild_plan_for_test(),
            ArtifactCompatibilityWindow::native(1),
        )
        .expect("rebuild requirement");
        let mut counters = CompatibilityAdmissionCounters::default();
        let rejection =
            derived::admit_derived_rebuild_maintenance(&mut counters, &requirement, None, None)
                .expect_err("retained authority is required before rebuild admission");
        assert_eq!(
            rejection.kind(),
            CompatibilityRejectionKind::DerivedStaleVersion
        );
        assert_eq!(counters.derived_stale_version_rejection_count(), 1);
    }

    #[test]
    fn compatibility_derived_rebuild_requires_maintenance_admission() {
        let requirement = DerivedRebuildRequirement::from_reuse_plan(
            &derived_rebuild_plan_for_test(),
            ArtifactCompatibilityWindow::native(1),
        )
        .expect("rebuild requirement");
        let mut counters = CompatibilityAdmissionCounters::default();
        let authority =
            derived::prove_retained_authority_for_derived_rebuild(requirement.family_id().clone());
        let rejection = derived::admit_derived_rebuild_maintenance(
            &mut counters,
            &requirement,
            Some(&authority),
            None,
        )
        .expect_err("maintenance admission is required before rebuild planning");
        assert_eq!(
            rejection.kind(),
            CompatibilityRejectionKind::DerivedRebuildAdmissionRejected
        );
        assert_eq!(
            counters.maintenance_compatibility_rebuild_rejection_count(),
            1
        );
    }

    #[test]
    fn compatibility_derived_rebuild_admission_and_debt_are_counted() {
        let requirement = DerivedRebuildRequirement::from_reuse_plan(
            &derived_rebuild_plan_for_test(),
            ArtifactCompatibilityWindow::native(1),
        )
        .expect("rebuild requirement");
        let mut counters = CompatibilityAdmissionCounters::default();
        let debt = derived::defer_derived_rebuild(&mut counters, &requirement, 3);
        assert_eq!(debt.debt_record_count(), 3);
        assert_eq!(counters.derived_rebuild_debt_count(), 3);

        let authority =
            derived::prove_retained_authority_for_derived_rebuild(requirement.family_id().clone());
        let maintenance = derived::prove_maintenance_admission_for_derived_rebuild(
            &mut counters,
            requirement.family_id().clone(),
            "m11-derived-rebuild-lane",
        );
        let rebuild = derived::admit_derived_rebuild_maintenance(
            &mut counters,
            &requirement,
            Some(&authority),
            Some(&maintenance),
        )
        .expect("matching authority and maintenance proofs should admit rebuild plan");
        assert_eq!(rebuild.family_id(), requirement.family_id());
        assert_eq!(rebuild.maintenance_lane_id(), "m11-derived-rebuild-lane");
        assert_eq!(
            counters.maintenance_compatibility_rebuild_admission_count(),
            1
        );
    }

    #[test]
    fn compatibility_derived_lane_registry_covers_every_derived_family_once() {
        let snapshot = CompatibilityRegistry::first_ship();
        let lanes =
            DerivedCompatibilityLaneRegistry::from_compatibility_snapshot(&snapshot).snapshot();
        let derived_family_count = snapshot
            .declarations()
            .iter()
            .filter(|declaration| {
                declaration.authority_classification()
                    == CompatibilityAuthorityClassification::Derived
            })
            .count();
        assert_eq!(lanes.declarations().len(), derived_family_count);
        for declaration in snapshot.declarations().iter().filter(|declaration| {
            declaration.authority_classification() == CompatibilityAuthorityClassification::Derived
        }) {
            assert!(
                lanes.get_by_family_kind(declaration.kind()).is_some(),
                "missing derived lane for {:?}",
                declaration.kind()
            );
        }
    }

    #[test]
    fn compatibility_derived_lane_snapshot_is_deterministic() {
        let snapshot = CompatibilityRegistry::first_ship();
        let first =
            DerivedCompatibilityLaneRegistry::from_compatibility_snapshot(&snapshot).snapshot();
        let second =
            DerivedCompatibilityLaneRegistry::from_compatibility_snapshot(&snapshot).snapshot();
        let first_lanes: Vec<_> = first
            .declarations()
            .iter()
            .map(|declaration| declaration.lane_kind())
            .collect();
        let second_lanes: Vec<_> = second
            .declarations()
            .iter()
            .map(|declaration| declaration.lane_kind())
            .collect();
        assert_eq!(first_lanes, second_lanes);
    }

    #[test]
    fn compatibility_snapshot_lane_admits_exact_native_reuse() {
        let (input, artifact, receipt) = derived_lane_fixture(
            CompatibilityFamilyKind::SnapshotRecord,
            CompatibilityRelation::Native,
            1,
            1,
        );
        let mut counters = CompatibilityAdmissionCounters::default();
        let plan =
            derived::plan_derived_lane_compatibility(&mut counters, &input, &artifact, &receipt)
                .expect("native snapshot lane should admit exact reuse");
        assert_eq!(
            plan.lane_kind(),
            DerivedCompatibilityLaneKind::SnapshotReuse
        );
        assert_eq!(
            plan.posture(),
            DerivedLaneCompatibilityPosture::ReuseAdmitted
        );
        assert_eq!(counters.derived_lane_reuse_count(), 1);
        assert_eq!(counters.derived_snapshot_reuse_count(), 1);
    }

    #[test]
    fn compatibility_delta_lane_admits_exact_native_reuse() {
        let (input, artifact, receipt) = derived_lane_fixture(
            CompatibilityFamilyKind::DeltaRecord,
            CompatibilityRelation::Native,
            1,
            1,
        );
        let mut counters = CompatibilityAdmissionCounters::default();
        let plan =
            derived::plan_derived_lane_compatibility(&mut counters, &input, &artifact, &receipt)
                .expect("native delta lane should admit exact reuse");
        assert_eq!(
            plan.lane_kind(),
            DerivedCompatibilityLaneKind::BranchDeltaReuse
        );
        assert_eq!(counters.derived_delta_reuse_count(), 1);
    }

    #[test]
    fn compatibility_layout_lane_rejects_basis_drift() {
        let (input, artifact, receipt) = derived_lane_fixture(
            CompatibilityFamilyKind::Milestone6LayoutBlockChunkRecord,
            CompatibilityRelation::Native,
            2,
            1,
        );
        let mut counters = CompatibilityAdmissionCounters::default();
        let rejection =
            derived::plan_derived_lane_compatibility(&mut counters, &input, &artifact, &receipt)
                .expect_err("layout lane must reject basis drift");
        assert_eq!(
            rejection.kind(),
            CompatibilityRejectionKind::DerivedBasisIncompatible
        );
        assert_eq!(counters.derived_layout_basis_rejection_count(), 1);
    }

    #[test]
    fn compatibility_bulk_resume_lane_rejects_changed_interpretation() {
        let (input, artifact, receipt) = derived_lane_fixture(
            CompatibilityFamilyKind::Milestone9BulkRecord,
            CompatibilityRelation::ForwardRead,
            1,
            1,
        );
        let mut counters = CompatibilityAdmissionCounters::default();
        let rejection =
            derived::plan_derived_lane_compatibility(&mut counters, &input, &artifact, &receipt)
                .expect_err("bulk resume must reject changed interpretation");
        assert_eq!(
            rejection.kind(),
            CompatibilityRejectionKind::BulkResumeCompatibilityRejected
        );
        assert_eq!(counters.derived_bulk_resume_rejection_count(), 1);
    }

    #[test]
    fn compatibility_tier_manifest_preserves_non_authority() {
        let (input, artifact, receipt) = derived_lane_fixture(
            CompatibilityFamilyKind::Milestone13TieringRecord,
            CompatibilityRelation::Native,
            1,
            1,
        );
        let mut counters = CompatibilityAdmissionCounters::default();
        let plan =
            derived::plan_derived_lane_compatibility(&mut counters, &input, &artifact, &receipt)
                .expect("native tier manifest should admit only placement support");
        assert_eq!(
            plan.tier_manifest().expect("tier plan").posture(),
            TierCompatibilityNonAuthorityPosture::PlacementSupportOnly
        );
        assert_eq!(counters.tier_non_authority_preserved_count(), 1);
    }

    #[test]
    fn compatibility_tier_manifest_skew_rejects_without_authority() {
        let (input, artifact, receipt) = derived_lane_fixture(
            CompatibilityFamilyKind::Milestone13TieringRecord,
            CompatibilityRelation::ForwardRead,
            1,
            1,
        );
        let mut counters = CompatibilityAdmissionCounters::default();
        let rejection =
            derived::plan_derived_lane_compatibility(&mut counters, &input, &artifact, &receipt)
                .expect_err("tier manifest drift should reject");
        assert_eq!(
            rejection.kind(),
            CompatibilityRejectionKind::TierManifestCompatibilityRejected
        );
        assert_eq!(counters.tier_manifest_rejection_count(), 1);
    }

    #[test]
    fn compatibility_maintenance_lane_requires_matching_work_class() {
        let family_id = CompatibilityFamilyKind::Milestone11MaintenanceRecord.family_id();
        let requirement = CompatibilityMaintenanceLaneRequirement::new(
            family_id.clone(),
            "certification.derived.lane.maintenance_summary_support",
            "DerivedFamilyRebuild",
        );
        let wrong_requirement = CompatibilityMaintenanceLaneRequirement::new(
            family_id,
            "certification.derived.lane.maintenance_summary_support",
            "MaintenanceAudit",
        );
        let mut counters = CompatibilityAdmissionCounters::default();
        let admission = derived::prove_compatibility_maintenance_lane_admission(
            &mut counters,
            &requirement,
            "m11-derived-rebuild-lane",
        );
        let rejection = derived::require_matching_maintenance_lane(
            &mut counters,
            &wrong_requirement,
            &admission,
        )
        .expect_err("maintenance work class mismatch should reject");
        assert_eq!(
            rejection.kind(),
            CompatibilityRejectionKind::MaintenanceLaneMismatch
        );
        assert_eq!(counters.maintenance_lane_mismatch_rejection_count(), 1);
    }

    #[test]
    fn compatibility_derived_lane_counters_project_to_milestone_12_report() {
        let (input, artifact, receipt) = derived_lane_fixture(
            CompatibilityFamilyKind::SnapshotRecord,
            CompatibilityRelation::Native,
            1,
            1,
        );
        let mut counters = CompatibilityAdmissionCounters::default();
        let _ =
            derived::plan_derived_lane_compatibility(&mut counters, &input, &artifact, &receipt)
                .expect("snapshot lane should admit");
        let report = crate::Milestone12AdmissionReport::from_admission_counters(&counters);
        assert_eq!(report.derived_lane_plan_count, 1);
        assert_eq!(report.derived_lane_reuse_count, 1);
        assert_eq!(report.derived_snapshot_reuse_count, 1);
    }

    #[test]
    fn compatibility_rolling_two_capability_window_admits() {
        let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let window = RollingUpgradeWindow::new(
            family_id.clone(),
            ArtifactCompatibilityWindow::new(
                ArtifactFormatVersion::new(1),
                ArtifactFormatVersion::new(2),
                ArtifactSemanticVersion::new(1),
                ArtifactSemanticVersion::new(2),
            ),
        );
        let reader =
            ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
        let writer =
            WriterCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(2)]);
        let edge_registry = CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
            family_id,
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(2),
            CompatibilityRelation::ForwardRead,
        )]);
        let mut counters = CompatibilityAdmissionCounters::default();
        let plan = rolling::plan_first_ship_rolling_upgrade(
            &mut counters,
            &edge_registry,
            &window,
            &[reader],
            &[writer],
        )
        .expect("one reader plus one writer inside declared window should admit");
        assert_eq!(plan.policy(), RollingUpgradePolicy::FirstShipTwoCapability);
        assert_eq!(plan.relation(), CompatibilityRelation::ForwardRead);
        assert_eq!(
            plan.store_posture().posture(),
            &MixedVersionPostureKind::AdmittedTwoCapabilityWindow
        );
        assert_eq!(counters.relation_recheck_count(), 1);
        assert_eq!(counters.rolling_window_admission_count(), 1);
    }

    #[test]
    fn compatibility_rolling_multi_writer_rejects() {
        let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let window =
            RollingUpgradeWindow::new(family_id.clone(), ArtifactCompatibilityWindow::native(1));
        let reader =
            ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
        let first_writer =
            WriterCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
        let second_writer =
            WriterCapabilitySet::new(family_id, vec![ArtifactSemanticVersion::new(1)]);
        let edge_registry = CompatibilityEdgeRegistry::new(Vec::new());
        let mut counters = CompatibilityAdmissionCounters::default();
        let rejection = rolling::plan_first_ship_rolling_upgrade(
            &mut counters,
            &edge_registry,
            &window,
            &[reader],
            &[first_writer, second_writer],
        )
        .expect_err("multi-writer first-ship rolling window should reject");
        assert_eq!(
            rejection.kind(),
            CompatibilityRejectionKind::RollingMultiWriterRejected
        );
        assert_eq!(counters.rolling_multi_writer_rejection_count(), 1);
    }

    #[test]
    fn compatibility_rolling_skew_outside_window_rejects() {
        let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let window =
            RollingUpgradeWindow::new(family_id.clone(), ArtifactCompatibilityWindow::native(1));
        let reader =
            ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
        let writer =
            WriterCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(2)]);
        let edge_registry = CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
            family_id,
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(2),
            CompatibilityRelation::ForwardRead,
        )]);
        let mut counters = CompatibilityAdmissionCounters::default();
        let rejection = rolling::plan_first_ship_rolling_upgrade(
            &mut counters,
            &edge_registry,
            &window,
            &[reader],
            &[writer],
        )
        .expect_err("writer outside rolling semantic window should reject");
        assert_eq!(
            rejection.kind(),
            CompatibilityRejectionKind::MixedVersionSkewRejected
        );
        assert_eq!(counters.mixed_version_skew_count(), 1);
        assert_eq!(counters.rolling_window_rejection_count(), 1);
    }

    #[test]
    fn compatibility_rolling_counters_project_to_milestone_12_report() {
        let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let window =
            RollingUpgradeWindow::new(family_id.clone(), ArtifactCompatibilityWindow::native(1));
        let reader =
            ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
        let writer =
            WriterCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
        let edge_registry = CompatibilityEdgeRegistry::new(vec![native_edge(family_id)]);
        let mut counters = CompatibilityAdmissionCounters::default();
        let plan = rolling::plan_first_ship_rolling_upgrade(
            &mut counters,
            &edge_registry,
            &window,
            &[reader],
            &[writer],
        )
        .expect("rolling window should admit");
        assert_eq!(plan.relation(), CompatibilityRelation::Native);
        let report = crate::Milestone12AdmissionReport::from_admission_counters(&counters);
        assert_eq!(report.rolling_window_admission_count, 1);
        assert_eq!(report.rolling_window_rejection_count, 0);
    }

    #[test]
    fn compatibility_rolling_missing_edge_rejects_numeric_proximity() {
        let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let window = RollingUpgradeWindow::new(
            family_id.clone(),
            ArtifactCompatibilityWindow::new(
                ArtifactFormatVersion::new(1),
                ArtifactFormatVersion::new(2),
                ArtifactSemanticVersion::new(1),
                ArtifactSemanticVersion::new(2),
            ),
        );
        let reader =
            ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
        let writer = WriterCapabilitySet::new(family_id, vec![ArtifactSemanticVersion::new(2)]);
        let edge_registry = CompatibilityEdgeRegistry::new(Vec::new());
        let mut counters = CompatibilityAdmissionCounters::default();
        let rejection = rolling::plan_first_ship_rolling_upgrade(
            &mut counters,
            &edge_registry,
            &window,
            &[reader],
            &[writer],
        )
        .expect_err("numeric proximity without a declared edge must reject");
        assert_eq!(
            rejection.kind(),
            CompatibilityRejectionKind::MissingCompatibilityEdge
        );
        assert_eq!(counters.relation_recheck_count(), 1);
        assert_eq!(counters.edge_missing_rejection_count(), 1);
        assert_eq!(counters.rolling_window_rejection_count(), 1);
    }

    #[test]
    fn compatibility_rolling_single_set_multi_version_rejects() {
        let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let window = RollingUpgradeWindow::new(
            family_id.clone(),
            ArtifactCompatibilityWindow::new(
                ArtifactFormatVersion::new(1),
                ArtifactFormatVersion::new(2),
                ArtifactSemanticVersion::new(1),
                ArtifactSemanticVersion::new(2),
            ),
        );
        let reader = ReaderCapabilitySet::new(
            family_id.clone(),
            vec![
                ArtifactSemanticVersion::new(1),
                ArtifactSemanticVersion::new(2),
            ],
        );
        let writer = WriterCapabilitySet::new(family_id, vec![ArtifactSemanticVersion::new(2)]);
        let edge_registry = CompatibilityEdgeRegistry::new(Vec::new());
        let mut counters = CompatibilityAdmissionCounters::default();
        let rejection = rolling::plan_first_ship_rolling_upgrade(
            &mut counters,
            &edge_registry,
            &window,
            &[reader],
            &[writer],
        )
        .expect_err("a single capability set cannot hide multiple semantic versions");
        assert_eq!(
            rejection.kind(),
            CompatibilityRejectionKind::RollingWindowRejected
        );
        assert_eq!(counters.relation_recheck_count(), 0);
        assert_eq!(counters.rolling_window_rejection_count(), 1);
    }

    #[test]
    fn compatibility_rolling_adapter_edge_rejects_without_execution() {
        let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let window = RollingUpgradeWindow::new(
            family_id.clone(),
            ArtifactCompatibilityWindow::new(
                ArtifactFormatVersion::new(1),
                ArtifactFormatVersion::new(2),
                ArtifactSemanticVersion::new(1),
                ArtifactSemanticVersion::new(2),
            ),
        );
        let reader =
            ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
        let writer =
            WriterCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(2)]);
        let edge = DeclaredCompatibilityEdge::new(
            family_id,
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(2),
            CompatibilityRelation::AdapterRequired,
        )
        .with_adapter(adapter(CompatibilityAdapterCostClass::BoundedRecordLocal));
        let edge_registry = CompatibilityEdgeRegistry::new(vec![edge]);
        let mut counters = CompatibilityAdmissionCounters::default();
        let rejection = rolling::plan_first_ship_rolling_upgrade(
            &mut counters,
            &edge_registry,
            &window,
            &[reader],
            &[writer],
        )
        .expect_err("first-ship rolling policy must not execute adapter edges");
        assert_eq!(
            rejection.kind(),
            CompatibilityRejectionKind::RollingWindowRejected
        );
        assert_eq!(counters.relation_recheck_count(), 1);
        assert_eq!(counters.rolling_window_rejection_count(), 1);
        assert_eq!(counters.adapter_hot_path_rejection_count(), 0);
    }

    #[test]
    fn compatibility_restore_admits_scoped_backup_with_declared_edge() {
        let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let backup_manifest = backup_manifest_for_family(family_id.clone(), 1);
        let target =
            RestoreCompatibilityTarget::new(family_id.clone(), ArtifactSemanticVersion::new(1));
        let scope = RestoreBackupScope::new(vec![family_id.clone()]);
        let conflicts = RestorePublicationConflictSet::new(Vec::new());
        let edges = CompatibilityEdgeRegistry::new(vec![native_edge(family_id)]);
        let mut counters = CompatibilityAdmissionCounters::default();
        let plan = restore::plan_restore_compatibility(
            &mut counters,
            &edges,
            &scope,
            &backup_manifest,
            &target,
            &conflicts,
        )
        .expect("scoped restore with declared native edge should admit");
        assert_eq!(plan.relation(), CompatibilityRelation::Native);
        assert_eq!(plan.publication_conflict_count(), 0);
        assert_eq!(counters.restore_accept_count(), 1);
        assert_eq!(counters.relation_recheck_count(), 1);
        assert_eq!(counters.artifact_row_scan_count(), 0);
    }

    #[test]
    fn compatibility_restore_rejects_out_of_scope_target_before_edge_scan() {
        let backup_family = CompatibilityFamilyKind::SnapshotRecord.family_id();
        let target_family = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let backup_manifest = backup_manifest_for_family(backup_family.clone(), 1);
        let target =
            RestoreCompatibilityTarget::new(target_family.clone(), ArtifactSemanticVersion::new(1));
        let scope = RestoreBackupScope::new(vec![backup_family]);
        let conflicts = RestorePublicationConflictSet::new(Vec::new());
        let mut counters = CompatibilityAdmissionCounters::default();
        let rejection = restore::plan_restore_compatibility(
            &mut counters,
            &CompatibilityEdgeRegistry::default(),
            &scope,
            &backup_manifest,
            &target,
            &conflicts,
        )
        .expect_err("restore must not scan target families outside backup scope");
        assert_eq!(
            rejection.kind(),
            CompatibilityRejectionKind::RestoreOutOfScopeScanRejected
        );
        assert_eq!(counters.restore_out_of_scope_scan_count(), 1);
        assert_eq!(counters.restore_rejection_count(), 1);
        assert_eq!(counters.relation_recheck_count(), 0);
    }

    #[test]
    fn compatibility_restore_rejects_publication_conflicts_before_witness() {
        let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let backup_manifest = backup_manifest_for_family(family_id.clone(), 1);
        let target =
            RestoreCompatibilityTarget::new(family_id.clone(), ArtifactSemanticVersion::new(1));
        let scope = RestoreBackupScope::new(vec![family_id.clone()]);
        let conflicts =
            RestorePublicationConflictSet::new(vec![RestorePublicationConflictUnit::new(
                family_id.clone(),
                RestorePublicationConflictKind::BranchHead,
            )]);
        let mut counters = CompatibilityAdmissionCounters::default();
        let rejection = restore::plan_restore_compatibility(
            &mut counters,
            &CompatibilityEdgeRegistry::default(),
            &scope,
            &backup_manifest,
            &target,
            &conflicts,
        )
        .expect_err("publication conflicts must reject before restore witness construction");
        assert_eq!(
            rejection.kind(),
            CompatibilityRejectionKind::RestorePublicationConflictRejected
        );
        assert_eq!(counters.restore_publication_conflict_rejection_count(), 1);
        assert_eq!(counters.restore_rejection_count(), 1);
        assert_eq!(counters.relation_recheck_count(), 0);
    }

    #[test]
    fn compatibility_restore_missing_edge_rejects_numeric_proximity() {
        let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let backup_manifest = backup_manifest_for_family(family_id.clone(), 1);
        let target =
            RestoreCompatibilityTarget::new(family_id.clone(), ArtifactSemanticVersion::new(2));
        let scope = RestoreBackupScope::new(vec![family_id]);
        let conflicts = RestorePublicationConflictSet::new(Vec::new());
        let mut counters = CompatibilityAdmissionCounters::default();
        let rejection = restore::plan_restore_compatibility(
            &mut counters,
            &CompatibilityEdgeRegistry::default(),
            &scope,
            &backup_manifest,
            &target,
            &conflicts,
        )
        .expect_err("restore must not infer compatibility from adjacent semantic versions");
        assert_eq!(
            rejection.kind(),
            CompatibilityRejectionKind::MissingCompatibilityEdge
        );
        assert_eq!(counters.relation_recheck_count(), 1);
        assert_eq!(counters.edge_missing_rejection_count(), 1);
        assert_eq!(counters.restore_rejection_count(), 1);
    }

    #[test]
    fn compatibility_restore_incompatible_edge_rejects_before_publication_witness() {
        let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let backup_manifest = backup_manifest_for_family(family_id.clone(), 1);
        let target =
            RestoreCompatibilityTarget::new(family_id.clone(), ArtifactSemanticVersion::new(2));
        let scope = RestoreBackupScope::new(vec![family_id.clone()]);
        let conflicts = RestorePublicationConflictSet::new(Vec::new());
        let edge_registry = CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
            family_id,
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(2),
            CompatibilityRelation::Incompatible,
        )]);
        let mut counters = CompatibilityAdmissionCounters::default();
        let rejection = restore::plan_restore_compatibility(
            &mut counters,
            &edge_registry,
            &scope,
            &backup_manifest,
            &target,
            &conflicts,
        )
        .expect_err("incompatible restore edge must reject before witness construction");
        assert_eq!(
            rejection.kind(),
            CompatibilityRejectionKind::RestoreCompatibilityRejected
        );
        assert_eq!(counters.relation_recheck_count(), 1);
        assert_eq!(counters.restore_rejection_count(), 1);
        assert_eq!(counters.restore_accept_count(), 0);
    }

    #[test]
    fn compatibility_disaster_recovery_windows_distinguish_truth_from_derived_acceleration() {
        let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let truth_window = DisasterRecoveryCompatibilityWindow::new(
            family_id.clone(),
            ArtifactCompatibilityWindow::native(1),
            DisasterRecoveryCompatibilityClass::AuthoritativeTruth,
        );
        let derived_window = DisasterRecoveryCompatibilityWindow::new(
            CompatibilityFamilyKind::SnapshotRecord.family_id(),
            ArtifactCompatibilityWindow::native(1),
            DisasterRecoveryCompatibilityClass::DerivedAcceleration,
        );
        let mut counters = CompatibilityAdmissionCounters::default();
        let truth_plan =
            restore::plan_disaster_recovery_compatibility(&mut counters, &truth_window);
        let derived_plan =
            restore::plan_disaster_recovery_compatibility(&mut counters, &derived_window);
        assert_eq!(
            truth_plan.class(),
            DisasterRecoveryCompatibilityClass::AuthoritativeTruth
        );
        assert_eq!(
            derived_plan.class(),
            DisasterRecoveryCompatibilityClass::DerivedAcceleration
        );
        assert_eq!(counters.disaster_recovery_truth_window_count(), 1);
        assert_eq!(counters.disaster_recovery_derived_window_count(), 1);
    }

    #[test]
    fn compatibility_certification_lane_ids_are_stable_unique_and_mandatory() {
        let mandatory = Milestone12CertificationLaneKind::mandatory_phase_5a();
        assert_eq!(mandatory.len(), 20);
        let mut seen = std::collections::BTreeSet::new();
        for kind in mandatory {
            assert_eq!(kind.lane_id().as_str(), kind.label());
            assert!(
                seen.insert(kind.lane_id()),
                "duplicate lane {}",
                kind.label()
            );
        }
        assert!(seen.contains(&Milestone12CertificationLaneKind::CatalogCompleteness.lane_id()));
        assert!(
            seen.contains(&Milestone12CertificationLaneKind::RollingAdapterEdgeRejected.lane_id())
        );
        assert!(
            seen.contains(&Milestone12CertificationLaneKind::RestoreMissingEdgeRejected.lane_id())
        );
        assert!(
            seen.contains(&Milestone12CertificationLaneKind::DisasterRecoveryTruthWindow.lane_id())
        );
    }

    #[test]
    fn compatibility_certification_matrix_requires_every_mandatory_lane() {
        let mut outcomes = milestone_12_certification_outcomes();
        let dropped = outcomes.pop().expect("fixture has mandatory lanes");
        assert_eq!(
            Milestone12CompatibilityMatrix::from_lane_outcomes(&outcomes),
            Err(Milestone12CertificationLaneRejection::MissingMandatoryLane)
        );

        outcomes.push(dropped.clone());
        outcomes.push(dropped);
        assert_eq!(
            Milestone12CompatibilityMatrix::from_lane_outcomes(&outcomes),
            Err(Milestone12CertificationLaneRejection::DuplicateLane)
        );
    }

    #[test]
    fn compatibility_certification_matrix_is_complete_and_deterministic() {
        let mut outcomes = milestone_12_certification_outcomes();
        outcomes.reverse();
        let matrix = Milestone12CompatibilityMatrix::from_lane_outcomes(&outcomes)
            .expect("all mandatory lanes should produce a complete matrix");
        assert_eq!(
            matrix.status(),
            Milestone12CompatibilityMatrixStatus::Complete
        );
        assert_eq!(
            matrix.entries().len(),
            Milestone12CertificationLaneKind::mandatory_phase_5a().len()
        );
        let observed = matrix
            .entries()
            .iter()
            .map(|entry| entry.lane_id().as_str())
            .collect::<Vec<_>>();
        let mut sorted = observed.clone();
        sorted.sort();
        assert_eq!(observed, sorted);
    }

    #[test]
    fn compatibility_certification_counter_contract_validates_report_shape() {
        let mut counters = CompatibilityAdmissionCounters::default();
        counters.record_relation_recheck();
        let report = Milestone12AdmissionReport::from_admission_counters(&counters);
        Milestone12CounterContract::phase_1()
            .validate_report(&report)
            .expect("phase-1 counter contract should cover report fields");

        let missing_counter_contract = Milestone12CounterContract {
            counter_names: MILESTONE_12_ADMISSION_REPORT_COUNTER_FIELD_NAMES
                .iter()
                .copied()
                .filter(|name| *name != "compatibility.restore.accept_count")
                .collect(),
        };
        assert_eq!(
            missing_counter_contract.validate_report(&report),
            Err(Milestone12CounterContractViolation::MissingReportCounter)
        );
    }

    #[test]
    fn compatibility_certification_bundle_preserves_lane_evidence() {
        let outcomes = milestone_12_certification_outcomes();
        let matrix = Milestone12CompatibilityMatrix::from_lane_outcomes(&outcomes)
            .expect("fixture should contain every mandatory lane");
        let mut counters = CompatibilityAdmissionCounters::default();
        counters.record_relation_recheck();
        let bundle = Milestone12CertificationEvidenceBundle::from_parts(
            Milestone12AdmissionReport::from_admission_counters(&counters),
            matrix,
            milestone_12_version_skew_report(),
            milestone_12_complexity_surface(),
            outcomes,
        )
        .expect("complete matrix with counter evidence should build certification bundle");

        assert_eq!(
            bundle.lane_outcomes().len(),
            Milestone12CertificationLaneKind::mandatory_phase_5a().len()
        );
        assert_eq!(bundle.run_summary().accepted_lane_count(), 7);
        assert_eq!(bundle.run_summary().rejected_lane_count(), 10);
        assert_eq!(bundle.rolling_evidence().admitted_lane_count(), 1);
        assert_eq!(bundle.rolling_evidence().rejected_lane_count(), 3);
        assert_eq!(bundle.restore_evidence().admitted_lane_count(), 1);
        assert_eq!(bundle.restore_evidence().rejected_lane_count(), 3);
    }

    #[test]
    fn compatibility_certification_bundle_rejects_matrix_outcome_mismatch() {
        let mut outcomes = milestone_12_certification_outcomes();
        let matrix = Milestone12CompatibilityMatrix::from_lane_outcomes(&outcomes)
            .expect("fixture should contain every mandatory lane");
        outcomes.pop();

        let mut counters = CompatibilityAdmissionCounters::default();
        counters.record_relation_recheck();
        assert_eq!(
            Milestone12CertificationEvidenceBundle::from_parts(
                Milestone12AdmissionReport::from_admission_counters(&counters),
                matrix,
                milestone_12_version_skew_report(),
                milestone_12_complexity_surface(),
                outcomes,
            ),
            Err(Milestone12CertificationLaneRejection::MatrixLaneMismatch)
        );
    }

    #[test]
    fn compatibility_certification_bundle_rejects_counterless_lane_evidence() {
        let outcomes = vec![Milestone12CertificationLaneOutcome::accepted(
            Milestone12CertificationLaneKind::CatalogCompleteness,
            milestone_12_certification_input(),
            CompatibilityRelation::Native,
            &CompatibilityAdmissionCounters::default(),
        )];
        let rejection = Milestone12CompatibilityMatrix::from_lane_outcomes(&outcomes)
            .expect_err("single-lane fixture is intentionally incomplete");
        assert_eq!(
            rejection,
            Milestone12CertificationLaneRejection::MissingMandatoryLane
        );

        let outcomes = milestone_12_certification_outcomes_with_zero_counter_lane();
        let matrix = Milestone12CompatibilityMatrix::from_lane_outcomes(&outcomes)
            .expect("fixture should contain every mandatory lane");
        assert_eq!(
            Milestone12CertificationEvidenceBundle::from_parts(
                Milestone12AdmissionReport::from_admission_counters(
                    &CompatibilityAdmissionCounters::default()
                ),
                matrix,
                milestone_12_version_skew_report(),
                milestone_12_complexity_surface(),
                outcomes,
            ),
            Err(Milestone12CertificationLaneRejection::CounterEvidenceMissing)
        );
    }

    #[test]
    fn compatibility_certification_rolling_outcome_preserves_plan_relation() {
        let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let window = RollingUpgradeWindow::new(
            family_id.clone(),
            ArtifactCompatibilityWindow::new(
                ArtifactFormatVersion::new(1),
                ArtifactFormatVersion::new(2),
                ArtifactSemanticVersion::new(1),
                ArtifactSemanticVersion::new(2),
            ),
        );
        let reader =
            ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
        let writer =
            WriterCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(2)]);
        let edge_registry = CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
            family_id,
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(2),
            CompatibilityRelation::ForwardRead,
        )]);
        let mut counters = CompatibilityAdmissionCounters::default();
        let plan = rolling::plan_first_ship_rolling_upgrade(
            &mut counters,
            &edge_registry,
            &window,
            &[reader],
            &[writer],
        )
        .expect("declared two-capability window should admit");
        let outcome = Milestone12CertificationLaneOutcome::from_rolling_plan(
            milestone_12_certification_input(),
            &plan,
            &counters,
        );
        assert_eq!(
            outcome.lane_kind(),
            Milestone12CertificationLaneKind::RollingTwoCapabilityAdmitted
        );
        assert_eq!(
            outcome.status(),
            Milestone12CertificationLaneStatus::Accepted
        );
        assert_eq!(outcome.relation(), Some(CompatibilityRelation::ForwardRead));
    }

    #[test]
    fn compatibility_certification_restore_outcome_preserves_plan_relation() {
        let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let backup_manifest = backup_manifest_for_family(family_id.clone(), 1);
        let target =
            RestoreCompatibilityTarget::new(family_id.clone(), ArtifactSemanticVersion::new(2));
        let scope = RestoreBackupScope::new(vec![family_id.clone()]);
        let conflicts = RestorePublicationConflictSet::new(Vec::new());
        let edge_registry = CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
            family_id,
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(2),
            CompatibilityRelation::BackwardRead,
        )]);
        let mut counters = CompatibilityAdmissionCounters::default();
        let plan = restore::plan_restore_compatibility(
            &mut counters,
            &edge_registry,
            &scope,
            &backup_manifest,
            &target,
            &conflicts,
        )
        .expect("declared restore edge should admit");
        let outcome = Milestone12CertificationLaneOutcome::from_restore_plan(
            milestone_12_certification_input(),
            &plan,
            &counters,
        );
        assert_eq!(
            outcome.lane_kind(),
            Milestone12CertificationLaneKind::RestoreScopedBackupAdmitted
        );
        assert_eq!(
            outcome.status(),
            Milestone12CertificationLaneStatus::Accepted
        );
        assert_eq!(
            outcome.relation(),
            Some(CompatibilityRelation::BackwardRead)
        );
    }

    #[test]
    fn compatibility_certification_rejection_outcome_preserves_missing_edge_kind() {
        let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
        let mut counters = CompatibilityAdmissionCounters::default();
        counters.record_relation_recheck();
        counters.record_edge_missing_rejection();
        let rejection = CompatibilityRejection::new(
            CompatibilityRejectionKind::MissingCompatibilityEdge,
            family_id,
            "missing edge",
        );
        let outcome = Milestone12CertificationLaneOutcome::from_compatibility_rejection(
            Milestone12CertificationLaneKind::AuthoritativeMissingEdgeRejected,
            milestone_12_certification_input(),
            &rejection,
            &counters,
        );
        assert_eq!(
            outcome.status(),
            Milestone12CertificationLaneStatus::Rejected
        );
        assert_eq!(
            outcome.rejection_kind(),
            Some(CompatibilityRejectionKind::MissingCompatibilityEdge)
        );
        assert_eq!(outcome.counters().edge_missing_rejection_count, 1);
    }

    fn native_edge(family_id: ArtifactFamilyId) -> DeclaredCompatibilityEdge {
        DeclaredCompatibilityEdge::new(
            family_id,
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(1),
            CompatibilityRelation::Native,
        )
    }

    fn backup_manifest_for_family(
        family_id: ArtifactFamilyId,
        version: u32,
    ) -> BackupCompatibilityManifest {
        let window = ArtifactCompatibilityWindow::native(version);
        let digest = manifests::CompatibilityManifestDigest::compute(&family_id, &window, "backup");
        BackupCompatibilityManifest::new(family_id, window, digest)
    }

    fn published_manifest_ledger(
        snapshot: &CompatibilityRegistrySnapshot,
    ) -> CompatibilityManifestPublicationLedger {
        let mut ledger = CompatibilityManifestPublicationLedger::new();
        for declaration in snapshot.declarations() {
            ledger.publish_declaration(declaration);
        }
        ledger
    }

    fn published_manifest_index(
        snapshot: &CompatibilityRegistrySnapshot,
    ) -> CompatibilityManifestIndex {
        let ledger = published_manifest_ledger(snapshot);
        CompatibilityManifestIndex::rebuild_from_recovered_manifests(snapshot, &ledger.recover())
    }

    fn quarantined_artifact_for_family(
        family_id: ArtifactFamilyId,
        version: u32,
        authority_label: &str,
    ) -> QuarantinedDecodedArtifact {
        quarantined_artifact_for_versions(family_id, version, version, version, authority_label)
    }

    fn quarantined_artifact_for_versions(
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

    fn frame_header(
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

    fn adapter(cost_class: CompatibilityAdapterCostClass) -> DeclaredCompatibilityAdapter {
        DeclaredCompatibilityAdapter::new(
            CompatibilityAdapterId::new("adapter"),
            CompatibilityAdapterDigest::new("digest"),
            cost_class,
        )
    }

    fn derived_family_declaration(
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

    fn synthetic_read_receipt(
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

    fn derived_rebuild_plan_for_test() -> DerivedCompatibilityReusePlan {
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

    fn milestone_12_certification_input() -> Milestone12CertificationLaneInput {
        Milestone12CertificationLaneInput::new(
            CompatibilityFamilyKind::CommitEnvelope.family_id(),
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(1),
            Some(CompatibilityRelation::Native),
            None,
        )
    }

    fn milestone_12_certification_outcomes() -> Vec<Milestone12CertificationLaneOutcome> {
        milestone_12_certification_outcomes_with_counterless_lane(None)
    }

    fn milestone_12_certification_outcomes_with_zero_counter_lane(
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
                    | Milestone12CertificationLaneKind::TierManifestNonAuthorityPreserved
                    | Milestone12CertificationLaneKind::RollingTwoCapabilityAdmitted
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
                }
            })
            .collect()
    }

    fn milestone_12_version_skew_report() -> Milestone12VersionSkewReport {
        Milestone12VersionSkewReport {
            mixed_version_store_lane_count: 1,
            mixed_version_replica_lane_count: 1,
            rolling_upgrade_skew_rejection_count: 1,
        }
    }

    fn milestone_12_complexity_surface() -> Milestone12ComplexitySurface {
        Milestone12ComplexitySurface {
            relation_recheck: Milestone12ComplexityPathStatus::verified("bounded relation recheck"),
            index_lookup: Milestone12ComplexityPathStatus::verified("manifest index lookup"),
            adapter_cost: Milestone12ComplexityPathStatus::verified("declared adapter cost class"),
            restore_scan: Milestone12ComplexityPathStatus::verified("backup-scope scan bound"),
        }
    }

    fn derived_lane_fixture(
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
}
