use serde::Serialize;
use sha2::{Digest, Sha256};

use super::admission::{
    plan_read_compatibility, CompatibilityAdapterCostClass, CompatibilityAdapterDigest,
    CompatibilityAdapterId, CompatibilityAdmissionBatch, CompatibilityAdmissionCounters,
    CompatibilityEdgeRegistry, CompatibilityReadAdmissionOutcome, CompatibilityReadIntent,
    CompatibilityRejectionKind, CompatibilityRelation, DeclaredCompatibilityAdapter,
    DeclaredCompatibilityEdge, ReaderCapabilitySet, WriterCapabilitySet,
};
use super::catalog::{
    CompatibilityFamilyKind, CompatibilityRegistry, CompatibilityRegistrySnapshot,
    DerivedFamilyDeclaration,
};
use super::certification::{
    Milestone12CertificationLaneInput, Milestone12CertificationLaneKind,
    Milestone12CertificationLaneOutcome, Milestone12CertificationLaneRejection,
    Milestone12CertificationLaneStatus, Milestone12CompatibilityMatrix,
};
use super::decoding::QuarantinedDecodedArtifact;
use super::derived::{
    plan_derived_lane_compatibility, DerivedBasisCompatibilityInput,
    DerivedCompatibilityLaneRegistry,
};
use super::manifests::{
    ArtifactCompatibilityWindow, ArtifactFamilyId, ArtifactFormatVersion, ArtifactSemanticVersion,
    CompatibilityManifestDigest, CompatibilityManifestPublicationLedger,
};
use super::restore::{
    plan_disaster_recovery_compatibility, plan_restore_compatibility, BackupCompatibilityManifest,
    DisasterRecoveryCompatibilityClass, DisasterRecoveryCompatibilityWindow, RestoreBackupScope,
    RestoreCompatibilityTarget, RestorePublicationConflictKind, RestorePublicationConflictSet,
    RestorePublicationConflictUnit,
};
use super::rolling::{plan_first_ship_rolling_upgrade, RollingUpgradeWindow};
use crate::evidence::{
    Milestone12AdmissionReport, Milestone12CertificationEvidenceBundle,
    Milestone12ComplexityPathStatus, Milestone12ComplexitySurface, Milestone12VersionSkewReport,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12ArtifactFormatEvolutionCertification {
    evidence_bundle: Milestone12CertificationEvidenceBundle,
    digest_set: Milestone12CertificationDigestSet,
    diagnostics: Milestone12CertificationDiagnostics,
}

impl Milestone12ArtifactFormatEvolutionCertification {
    pub fn evidence_bundle(&self) -> &Milestone12CertificationEvidenceBundle {
        &self.evidence_bundle
    }

    pub fn digest_set(&self) -> &Milestone12CertificationDigestSet {
        &self.digest_set
    }

    pub fn diagnostics(&self) -> &Milestone12CertificationDiagnostics {
        &self.diagnostics
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationDigestSet {
    artifact_digest: String,
    failure_digest: String,
    compatibility_matrix_digest: String,
    version_skew_digest: String,
    diagnostics_digest: String,
    counter_snapshot_digest: String,
}

impl Milestone12CertificationDigestSet {
    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }

    pub fn compatibility_matrix_digest(&self) -> &str {
        &self.compatibility_matrix_digest
    }

    pub fn version_skew_digest(&self) -> &str {
        &self.version_skew_digest
    }

    pub fn diagnostics_digest(&self) -> &str {
        &self.diagnostics_digest
    }

    pub fn counter_snapshot_digest(&self) -> &str {
        &self.counter_snapshot_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationDiagnostics {
    lane_count: usize,
    runtime_gap_labels: Vec<&'static str>,
}

impl Milestone12CertificationDiagnostics {
    pub fn lane_count(&self) -> usize {
        self.lane_count
    }

    pub fn runtime_gap_labels(&self) -> &[&'static str] {
        &self.runtime_gap_labels
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationScenario {
    label: &'static str,
}

impl Milestone12CertificationScenario {
    pub fn first_ship() -> Self {
        Self {
            label: "first_ship_artifact_format_evolution",
        }
    }

    pub fn label(&self) -> &'static str {
        self.label
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationFixture {
    scenario: Milestone12CertificationScenario,
    registry_snapshot: CompatibilityRegistrySnapshot,
}

impl Milestone12CertificationFixture {
    pub(crate) fn first_ship() -> Self {
        Self {
            scenario: Milestone12CertificationScenario::first_ship(),
            registry_snapshot: CompatibilityRegistry::first_ship(),
        }
    }

    pub fn scenario(&self) -> Milestone12CertificationScenario {
        self.scenario
    }

    pub fn registry_snapshot(&self) -> &CompatibilityRegistrySnapshot {
        &self.registry_snapshot
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationRunner {
    fixture: Milestone12CertificationFixture,
}

impl Milestone12CertificationRunner {
    pub fn first_ship() -> Self {
        Self {
            fixture: Milestone12CertificationFixture::first_ship(),
        }
    }

    pub fn run(
        &self,
    ) -> Result<
        Milestone12ArtifactFormatEvolutionCertification,
        Milestone12CertificationLaneRejection,
    > {
        let lane_outcomes = self.lane_outcomes()?;
        let compatibility_matrix =
            Milestone12CompatibilityMatrix::from_lane_outcomes(&lane_outcomes)?;
        let admission_report =
            Milestone12AdmissionReport::aggregate(lane_outcomes.iter().map(|lane| lane.counters()));
        let version_skew_report = version_skew_report(&admission_report);
        let complexity_surface = complexity_surface();
        let evidence_bundle = Milestone12CertificationEvidenceBundle::from_parts(
            admission_report.clone(),
            compatibility_matrix,
            version_skew_report.clone(),
            complexity_surface,
            lane_outcomes.clone(),
        )?;
        let diagnostics = Milestone12CertificationDiagnostics {
            lane_count: lane_outcomes.len(),
            runtime_gap_labels: runtime_gap_labels(),
        };
        let digest_set = digest_set(
            &lane_outcomes,
            &version_skew_report,
            &diagnostics,
            &admission_report,
        );
        Ok(Milestone12ArtifactFormatEvolutionCertification {
            evidence_bundle,
            digest_set,
            diagnostics,
        })
    }

    fn lane_outcomes(
        &self,
    ) -> Result<Vec<Milestone12CertificationLaneOutcome>, Milestone12CertificationLaneRejection>
    {
        let snapshot = self.fixture.registry_snapshot();
        let manifest_index = recovered_manifest_index(snapshot);
        let catalog_index =
            super::admission::CompatibilityManifestIndex::rebuild_from_registry(snapshot);
        let mut outcomes =
            Vec::with_capacity(Milestone12CertificationLaneKind::mandatory_phase_5a().len());
        outcomes.push(Milestone12CertificationLaneOutcome::non_admitted(
            Milestone12CertificationLaneKind::CatalogCompleteness,
            lane_input(
                CompatibilityFamilyKind::CommitEnvelope.family_id(),
                1,
                1,
                None,
                None,
            ),
            Milestone12CertificationLaneStatus::EvidenceOnly,
            catalog_index.rebuild_counters(),
        ));
        outcomes.extend(authoritative_lanes(&manifest_index)?);
        outcomes.extend(derived_lanes(snapshot, &manifest_index)?);
        outcomes.extend(rolling_lanes());
        outcomes.extend(restore_lanes());
        outcomes.extend(disaster_recovery_lanes());
        Ok(outcomes)
    }
}

fn authoritative_lanes(
    manifest_index: &super::admission::CompatibilityManifestIndex,
) -> Result<Vec<Milestone12CertificationLaneOutcome>, Milestone12CertificationLaneRejection> {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact = artifact_for_family(CompatibilityFamilyKind::CommitEnvelope, 1);
    let lanes = [
        (
            Milestone12CertificationLaneKind::AuthoritativeNativeRead,
            CompatibilityRelation::Native,
            Some(CompatibilityRelation::Native),
            None,
            1,
        ),
        (
            Milestone12CertificationLaneKind::AuthoritativeForwardRead,
            CompatibilityRelation::ForwardRead,
            Some(CompatibilityRelation::ForwardRead),
            None,
            2,
        ),
        (
            Milestone12CertificationLaneKind::AuthoritativeBackwardRead,
            CompatibilityRelation::BackwardRead,
            Some(CompatibilityRelation::BackwardRead),
            None,
            2,
        ),
    ];
    let mut outcomes = Vec::new();
    for (kind, relation, expected_relation, expected_rejection, target) in lanes {
        let edge_registry = CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
            family_id.clone(),
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(target),
            relation,
        )]);
        outcomes.push(read_lane(
            kind,
            manifest_index,
            &edge_registry,
            &artifact,
            target,
            expected_relation,
            expected_rejection,
        )?);
    }

    outcomes.push(read_lane(
        Milestone12CertificationLaneKind::AuthoritativeMissingEdgeRejected,
        manifest_index,
        &CompatibilityEdgeRegistry::new(Vec::new()),
        &artifact,
        2,
        None,
        Some(CompatibilityRejectionKind::MissingCompatibilityEdge),
    )?);
    let incompatible_edges = CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
        family_id,
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(1),
        CompatibilityRelation::Incompatible,
    )]);
    outcomes.push(read_lane(
        Milestone12CertificationLaneKind::AuthoritativeIncompatibleEdgeRejected,
        manifest_index,
        &incompatible_edges,
        &artifact,
        1,
        None,
        Some(CompatibilityRejectionKind::DeclaredIncompatibleRelation),
    )?);
    Ok(outcomes)
}

fn read_lane(
    kind: Milestone12CertificationLaneKind,
    manifest_index: &super::admission::CompatibilityManifestIndex,
    edge_registry: &CompatibilityEdgeRegistry,
    artifact: &QuarantinedDecodedArtifact,
    target_semantic_version: u32,
    expected_relation: Option<CompatibilityRelation>,
    expected_rejection: Option<CompatibilityRejectionKind>,
) -> Result<Milestone12CertificationLaneOutcome, Milestone12CertificationLaneRejection> {
    let mut batch = CompatibilityAdmissionBatch::new();
    let family_id = artifact.family_id().clone();
    let reader = ReaderCapabilitySet::new(
        family_id.clone(),
        vec![ArtifactSemanticVersion::new(target_semantic_version)],
    );
    let intent = CompatibilityReadIntent::new(
        family_id.clone(),
        ArtifactSemanticVersion::new(target_semantic_version),
    );
    let outcome = match plan_read_compatibility(
        &mut batch,
        manifest_index,
        edge_registry,
        &reader,
        &intent,
        artifact,
    ) {
        Ok(receipt) => CompatibilityReadAdmissionOutcome::accepted(&receipt, batch.counters()),
        Err(rejection) => {
            CompatibilityReadAdmissionOutcome::rejected(artifact, &rejection, batch.counters())
        }
    };
    Milestone12CertificationLaneOutcome::from_read_outcome(
        kind,
        lane_input(
            family_id,
            artifact.semantic_version().value(),
            target_semantic_version,
            expected_relation,
            expected_rejection,
        ),
        &outcome,
    )
}

fn derived_lanes(
    snapshot: &CompatibilityRegistrySnapshot,
    manifest_index: &super::admission::CompatibilityManifestIndex,
) -> Result<Vec<Milestone12CertificationLaneOutcome>, Milestone12CertificationLaneRejection> {
    let lanes = DerivedCompatibilityLaneRegistry::from_compatibility_snapshot(snapshot).snapshot();
    let mut outcomes = Vec::new();
    outcomes.push(derived_lane(
        snapshot,
        &lanes,
        manifest_index,
        CompatibilityFamilyKind::SnapshotRecord,
        Milestone12CertificationLaneKind::DerivedSnapshotReuseAccepted,
        CompatibilityRelation::Native,
        1,
        ArtifactCompatibilityWindow::native(1),
        Some(CompatibilityRelation::Native),
        None,
    )?);
    outcomes.push(derived_lane(
        snapshot,
        &lanes,
        manifest_index,
        CompatibilityFamilyKind::Milestone6LayoutBlockChunkRecord,
        Milestone12CertificationLaneKind::DerivedLayoutBasisRejected,
        CompatibilityRelation::Native,
        1,
        ArtifactCompatibilityWindow::native(2),
        None,
        Some(CompatibilityRejectionKind::DerivedBasisIncompatible),
    )?);
    outcomes.push(derived_lane(
        snapshot,
        &lanes,
        manifest_index,
        CompatibilityFamilyKind::Milestone9BulkRecord,
        Milestone12CertificationLaneKind::DerivedBulkResumeRejected,
        CompatibilityRelation::ForwardRead,
        2,
        ArtifactCompatibilityWindow::native(1),
        None,
        Some(CompatibilityRejectionKind::BulkResumeCompatibilityRejected),
    )?);
    outcomes.push(derived_lane(
        snapshot,
        &lanes,
        manifest_index,
        CompatibilityFamilyKind::Milestone13TieringRecord,
        Milestone12CertificationLaneKind::TierManifestNonAuthorityPreserved,
        CompatibilityRelation::Native,
        1,
        ArtifactCompatibilityWindow::native(1),
        Some(CompatibilityRelation::Native),
        None,
    )?);
    Ok(outcomes)
}

fn derived_lane(
    snapshot: &CompatibilityRegistrySnapshot,
    lanes: &super::derived::DerivedCompatibilityLaneSnapshot,
    manifest_index: &super::admission::CompatibilityManifestIndex,
    family_kind: CompatibilityFamilyKind,
    lane_kind: Milestone12CertificationLaneKind,
    relation: CompatibilityRelation,
    target_semantic_version: u32,
    required_window: ArtifactCompatibilityWindow,
    expected_relation: Option<CompatibilityRelation>,
    expected_rejection: Option<CompatibilityRejectionKind>,
) -> Result<Milestone12CertificationLaneOutcome, Milestone12CertificationLaneRejection> {
    let family_id = family_kind.family_id();
    let artifact = artifact_for_family(family_kind, 1);
    let edge_registry = CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
        family_id.clone(),
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(target_semantic_version),
        relation,
    )]);
    let mut batch = CompatibilityAdmissionBatch::new();
    let reader = ReaderCapabilitySet::new(
        family_id.clone(),
        vec![ArtifactSemanticVersion::new(target_semantic_version)],
    );
    let intent = CompatibilityReadIntent::new(
        family_id.clone(),
        ArtifactSemanticVersion::new(target_semantic_version),
    );
    let receipt = match plan_read_compatibility(
        &mut batch,
        manifest_index,
        &edge_registry,
        &reader,
        &intent,
        &artifact,
    ) {
        Ok(receipt) => receipt,
        Err(rejection) => {
            return Ok(
                Milestone12CertificationLaneOutcome::from_compatibility_rejection(
                    lane_kind,
                    lane_input(
                        family_id,
                        1,
                        target_semantic_version,
                        expected_relation,
                        expected_rejection,
                    ),
                    &rejection,
                    batch.counters(),
                ),
            );
        }
    };
    let lane_declaration = lanes
        .get_by_family_kind(family_kind)
        .expect("first-ship derived lane exists")
        .clone();
    let derived_family = DerivedFamilyDeclaration::new(
        snapshot
            .get(family_kind)
            .expect("first-ship derived family exists")
            .clone(),
    );
    let input =
        DerivedBasisCompatibilityInput::new(lane_declaration, derived_family, required_window);
    let plan = plan_derived_lane_compatibility(batch.counters_mut(), &input, &artifact, &receipt);
    let certification_input = lane_input(
        family_id,
        artifact.semantic_version().value(),
        target_semantic_version,
        expected_relation,
        expected_rejection,
    );
    Ok(match plan {
        Ok(plan) => Milestone12CertificationLaneOutcome::from_derived_plan(
            lane_kind,
            certification_input,
            &plan,
            batch.counters(),
        ),
        Err(rejection) => Milestone12CertificationLaneOutcome::from_compatibility_rejection(
            lane_kind,
            certification_input,
            &rejection,
            batch.counters(),
        ),
    })
}

fn rolling_lanes() -> Vec<Milestone12CertificationLaneOutcome> {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let window = rolling_window(family_id.clone());
    let reader = ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
    let writer = WriterCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(2)]);
    vec![
        rolling_lane(
            Milestone12CertificationLaneKind::RollingTwoCapabilityAdmitted,
            &window,
            &[reader.clone()],
            &[writer.clone()],
            CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
                family_id.clone(),
                ArtifactSemanticVersion::new(1),
                ArtifactSemanticVersion::new(2),
                CompatibilityRelation::ForwardRead,
            )]),
            Some(CompatibilityRelation::ForwardRead),
            None,
        ),
        rolling_lane(
            Milestone12CertificationLaneKind::RollingMultiWriterRejected,
            &window,
            &[reader.clone()],
            &[writer.clone(), writer.clone()],
            CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
                family_id.clone(),
                ArtifactSemanticVersion::new(1),
                ArtifactSemanticVersion::new(2),
                CompatibilityRelation::ForwardRead,
            )]),
            None,
            Some(CompatibilityRejectionKind::RollingMultiWriterRejected),
        ),
        rolling_lane(
            Milestone12CertificationLaneKind::RollingMissingEdgeRejected,
            &window,
            &[reader.clone()],
            &[writer.clone()],
            CompatibilityEdgeRegistry::new(Vec::new()),
            None,
            Some(CompatibilityRejectionKind::MissingCompatibilityEdge),
        ),
        rolling_lane(
            Milestone12CertificationLaneKind::RollingAdapterEdgeRejected,
            &window,
            &[reader],
            &[writer],
            CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
                family_id.clone(),
                ArtifactSemanticVersion::new(1),
                ArtifactSemanticVersion::new(2),
                CompatibilityRelation::AdapterRequired,
            )
            .with_adapter(adapter(CompatibilityAdapterCostClass::BoundedBatchLocal))]),
            None,
            Some(CompatibilityRejectionKind::RollingWindowRejected),
        ),
    ]
}

fn rolling_lane(
    lane_kind: Milestone12CertificationLaneKind,
    window: &RollingUpgradeWindow,
    readers: &[ReaderCapabilitySet],
    writers: &[WriterCapabilitySet],
    edge_registry: CompatibilityEdgeRegistry,
    expected_relation: Option<CompatibilityRelation>,
    expected_rejection: Option<CompatibilityRejectionKind>,
) -> Milestone12CertificationLaneOutcome {
    let mut counters = CompatibilityAdmissionCounters::default();
    let input = lane_input(
        window.family_id().clone(),
        1,
        2,
        expected_relation,
        expected_rejection,
    );
    match plan_first_ship_rolling_upgrade(&mut counters, &edge_registry, window, readers, writers) {
        Ok(plan) => Milestone12CertificationLaneOutcome::from_rolling_plan(input, &plan, &counters),
        Err(rejection) => Milestone12CertificationLaneOutcome::from_compatibility_rejection(
            lane_kind, input, &rejection, &counters,
        ),
    }
}

fn restore_lanes() -> Vec<Milestone12CertificationLaneOutcome> {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let edge_registry = CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
        family_id.clone(),
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(2),
        CompatibilityRelation::BackwardRead,
    )]);
    vec![
        restore_lane(
            Milestone12CertificationLaneKind::RestoreScopedBackupAdmitted,
            family_id.clone(),
            family_id.clone(),
            RestoreBackupScope::new(vec![family_id.clone()]),
            RestorePublicationConflictSet::new(Vec::new()),
            edge_registry.clone(),
            Some(CompatibilityRelation::BackwardRead),
            None,
        ),
        restore_lane(
            Milestone12CertificationLaneKind::RestoreOutOfScopeRejected,
            family_id.clone(),
            CompatibilityFamilyKind::SnapshotRecord.family_id(),
            RestoreBackupScope::new(vec![family_id.clone()]),
            RestorePublicationConflictSet::new(Vec::new()),
            edge_registry.clone(),
            None,
            Some(CompatibilityRejectionKind::RestoreOutOfScopeScanRejected),
        ),
        restore_lane(
            Milestone12CertificationLaneKind::RestorePublicationConflictRejected,
            family_id.clone(),
            family_id.clone(),
            RestoreBackupScope::new(vec![family_id.clone()]),
            RestorePublicationConflictSet::new(vec![RestorePublicationConflictUnit::new(
                family_id.clone(),
                RestorePublicationConflictKind::BranchHead,
            )]),
            edge_registry,
            None,
            Some(CompatibilityRejectionKind::RestorePublicationConflictRejected),
        ),
        restore_lane(
            Milestone12CertificationLaneKind::RestoreMissingEdgeRejected,
            family_id.clone(),
            family_id.clone(),
            RestoreBackupScope::new(vec![family_id.clone()]),
            RestorePublicationConflictSet::new(Vec::new()),
            CompatibilityEdgeRegistry::new(Vec::new()),
            None,
            Some(CompatibilityRejectionKind::MissingCompatibilityEdge),
        ),
    ]
}

fn restore_lane(
    lane_kind: Milestone12CertificationLaneKind,
    backup_family_id: ArtifactFamilyId,
    target_family_id: ArtifactFamilyId,
    backup_scope: RestoreBackupScope,
    conflicts: RestorePublicationConflictSet,
    edge_registry: CompatibilityEdgeRegistry,
    expected_relation: Option<CompatibilityRelation>,
    expected_rejection: Option<CompatibilityRejectionKind>,
) -> Milestone12CertificationLaneOutcome {
    let backup_manifest = backup_manifest(backup_family_id.clone(), 1);
    let target =
        RestoreCompatibilityTarget::new(target_family_id.clone(), ArtifactSemanticVersion::new(2));
    let mut counters = CompatibilityAdmissionCounters::default();
    let input = lane_input(
        target_family_id,
        1,
        2,
        expected_relation,
        expected_rejection,
    );
    match plan_restore_compatibility(
        &mut counters,
        &edge_registry,
        &backup_scope,
        &backup_manifest,
        &target,
        &conflicts,
    ) {
        Ok(plan) => Milestone12CertificationLaneOutcome::from_restore_plan(input, &plan, &counters),
        Err(rejection) => Milestone12CertificationLaneOutcome::from_compatibility_rejection(
            lane_kind, input, &rejection, &counters,
        ),
    }
}

fn disaster_recovery_lanes() -> Vec<Milestone12CertificationLaneOutcome> {
    vec![
        disaster_recovery_lane(
            Milestone12CertificationLaneKind::DisasterRecoveryTruthWindow,
            CompatibilityFamilyKind::CommitEnvelope.family_id(),
            DisasterRecoveryCompatibilityClass::AuthoritativeTruth,
        ),
        disaster_recovery_lane(
            Milestone12CertificationLaneKind::DisasterRecoveryDerivedWindow,
            CompatibilityFamilyKind::SnapshotRecord.family_id(),
            DisasterRecoveryCompatibilityClass::DerivedAcceleration,
        ),
    ]
}

fn disaster_recovery_lane(
    lane_kind: Milestone12CertificationLaneKind,
    family_id: ArtifactFamilyId,
    class: DisasterRecoveryCompatibilityClass,
) -> Milestone12CertificationLaneOutcome {
    let mut counters = CompatibilityAdmissionCounters::default();
    let window = DisasterRecoveryCompatibilityWindow::new(
        family_id.clone(),
        ArtifactCompatibilityWindow::native(1),
        class,
    );
    let plan = plan_disaster_recovery_compatibility(&mut counters, &window);
    Milestone12CertificationLaneOutcome::from_disaster_recovery_plan(
        lane_kind,
        lane_input(family_id, 1, 1, None, None),
        &plan,
        &counters,
    )
}

fn recovered_manifest_index(
    snapshot: &CompatibilityRegistrySnapshot,
) -> super::admission::CompatibilityManifestIndex {
    let mut ledger = CompatibilityManifestPublicationLedger::new();
    for declaration in snapshot.declarations() {
        ledger.publish_declaration(declaration);
    }
    super::admission::CompatibilityManifestIndex::rebuild_from_recovered_manifests(
        snapshot,
        &ledger.recover(),
    )
}

fn artifact_for_family(
    family_kind: CompatibilityFamilyKind,
    version: u32,
) -> QuarantinedDecodedArtifact {
    let family_id = family_kind.family_id();
    let window = ArtifactCompatibilityWindow::native(version);
    let digest = CompatibilityManifestDigest::compute(
        &family_id,
        &window,
        family_kind.authority_classification().label(),
    );
    QuarantinedDecodedArtifact::new(
        family_id,
        ArtifactFormatVersion::new(version),
        ArtifactSemanticVersion::new(version),
        digest,
        format!("structural-digest-{}", family_kind.label()),
        format!("diagnostic-context-{}", family_kind.label()),
    )
}

fn backup_manifest(family_id: ArtifactFamilyId, version: u32) -> BackupCompatibilityManifest {
    let window = ArtifactCompatibilityWindow::native(version);
    let digest = CompatibilityManifestDigest::compute(&family_id, &window, "backup");
    BackupCompatibilityManifest::new(family_id, window, digest)
}

fn rolling_window(family_id: ArtifactFamilyId) -> RollingUpgradeWindow {
    RollingUpgradeWindow::new(
        family_id,
        ArtifactCompatibilityWindow::new(
            ArtifactFormatVersion::new(1),
            ArtifactFormatVersion::new(2),
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(2),
        ),
    )
}

fn adapter(cost_class: CompatibilityAdapterCostClass) -> DeclaredCompatibilityAdapter {
    DeclaredCompatibilityAdapter::new(
        CompatibilityAdapterId::new("first-ship-certification-adapter"),
        CompatibilityAdapterDigest::new("first-ship-certification-adapter-digest"),
        cost_class,
    )
}

fn lane_input(
    family_id: ArtifactFamilyId,
    source_semantic_version: u32,
    target_semantic_version: u32,
    expected_relation: Option<CompatibilityRelation>,
    expected_rejection_kind: Option<CompatibilityRejectionKind>,
) -> Milestone12CertificationLaneInput {
    Milestone12CertificationLaneInput::new(
        family_id,
        ArtifactSemanticVersion::new(source_semantic_version),
        ArtifactSemanticVersion::new(target_semantic_version),
        expected_relation,
        expected_rejection_kind,
    )
}

fn version_skew_report(report: &Milestone12AdmissionReport) -> Milestone12VersionSkewReport {
    Milestone12VersionSkewReport {
        mixed_version_store_lane_count: report.rolling_window_admission_count,
        mixed_version_replica_lane_count: report.rolling_window_admission_count,
        rolling_upgrade_skew_rejection_count: report.mixed_version_skew_count,
    }
}

fn complexity_surface() -> Milestone12ComplexitySurface {
    Milestone12ComplexitySurface {
        relation_recheck: Milestone12ComplexityPathStatus::verified(
            "certification lanes recheck declared edges through bounded edge registry lookups",
        ),
        index_lookup: Milestone12ComplexityPathStatus::verified(
            "certification lanes use manifest index lookup counters, not artifact row scans",
        ),
        adapter_cost: Milestone12ComplexityPathStatus::verified(
            "adapter lanes preserve declared cost class and reject runtime execution paths",
        ),
        restore_scan: Milestone12ComplexityPathStatus::verified(
            "restore lanes prove backup-scope bounds and out-of-scope rejection counters",
        ),
    }
}

fn runtime_gap_labels() -> Vec<&'static str> {
    vec![
        "durable_manifest_persistence_deferred",
        "facade_read_write_restore_integration_deferred",
        "restore_publication_execution_deferred",
        "rolling_writer_publication_deferred",
        "adapter_execution_deferred",
        "derived_rebuild_execution_deferred",
    ]
}

fn digest_set(
    lane_outcomes: &[Milestone12CertificationLaneOutcome],
    version_skew_report: &Milestone12VersionSkewReport,
    diagnostics: &Milestone12CertificationDiagnostics,
    admission_report: &Milestone12AdmissionReport,
) -> Milestone12CertificationDigestSet {
    let accepted = lane_outcomes
        .iter()
        .filter(|lane| lane.status() == Milestone12CertificationLaneStatus::Accepted)
        .cloned()
        .collect::<Vec<_>>();
    let rejected = lane_outcomes
        .iter()
        .filter(|lane| lane.status() == Milestone12CertificationLaneStatus::Rejected)
        .cloned()
        .collect::<Vec<_>>();
    Milestone12CertificationDigestSet {
        artifact_digest: digest_of(&accepted),
        failure_digest: digest_of(&rejected),
        compatibility_matrix_digest: digest_of(&lane_outcomes),
        version_skew_digest: digest_of(version_skew_report),
        diagnostics_digest: digest_of(diagnostics),
        counter_snapshot_digest: digest_of(admission_report),
    }
}

fn digest_of<T: Serialize>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("certification evidence must serialize");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn artifact_format_evolution_runner_emits_every_mandatory_lane_once() {
        let certification = Milestone12CertificationRunner::first_ship()
            .run()
            .expect("first-ship certification should run");
        let observed = certification
            .evidence_bundle()
            .lane_outcomes()
            .iter()
            .map(|lane| lane.lane_kind())
            .collect::<BTreeSet<_>>();
        let expected = Milestone12CertificationLaneKind::mandatory_phase_5a()
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        assert_eq!(observed, expected);
        assert_eq!(
            certification
                .evidence_bundle()
                .run_summary()
                .accepted_lane_count(),
            7
        );
        assert_eq!(
            certification
                .evidence_bundle()
                .run_summary()
                .rejected_lane_count(),
            10
        );
        assert_eq!(
            certification.diagnostics().lane_count(),
            Milestone12CertificationLaneKind::mandatory_phase_5a().len()
        );
    }

    #[test]
    fn artifact_format_evolution_runner_is_deterministic() {
        let left = Milestone12CertificationRunner::first_ship()
            .run()
            .expect("first run should succeed");
        let right = Milestone12CertificationRunner::first_ship()
            .run()
            .expect("second run should succeed");
        assert_eq!(left.digest_set(), right.digest_set());
        assert_eq!(left.evidence_bundle(), right.evidence_bundle());
    }

    #[test]
    fn artifact_format_evolution_runner_preserves_authoritative_rejections() {
        let certification = Milestone12CertificationRunner::first_ship()
            .run()
            .expect("first-ship certification should run");
        assert_eq!(
            lane(
                &certification,
                Milestone12CertificationLaneKind::AuthoritativeMissingEdgeRejected
            )
            .rejection_kind(),
            Some(CompatibilityRejectionKind::MissingCompatibilityEdge)
        );
        assert_eq!(
            lane(
                &certification,
                Milestone12CertificationLaneKind::AuthoritativeIncompatibleEdgeRejected
            )
            .rejection_kind(),
            Some(CompatibilityRejectionKind::DeclaredIncompatibleRelation)
        );
    }

    #[test]
    fn artifact_format_evolution_runner_preserves_derived_lane_evidence() {
        let certification = Milestone12CertificationRunner::first_ship()
            .run()
            .expect("first-ship certification should run");
        assert_eq!(
            lane(
                &certification,
                Milestone12CertificationLaneKind::DerivedSnapshotReuseAccepted
            )
            .status(),
            Milestone12CertificationLaneStatus::Accepted
        );
        assert_eq!(
            lane(
                &certification,
                Milestone12CertificationLaneKind::DerivedLayoutBasisRejected
            )
            .rejection_kind(),
            Some(CompatibilityRejectionKind::DerivedBasisIncompatible)
        );
        assert_eq!(
            lane(
                &certification,
                Milestone12CertificationLaneKind::DerivedBulkResumeRejected
            )
            .rejection_kind(),
            Some(CompatibilityRejectionKind::BulkResumeCompatibilityRejected)
        );
    }

    #[test]
    fn artifact_format_evolution_runner_preserves_rolling_restore_and_dr_evidence() {
        let certification = Milestone12CertificationRunner::first_ship()
            .run()
            .expect("first-ship certification should run");
        assert_eq!(
            lane(
                &certification,
                Milestone12CertificationLaneKind::RollingTwoCapabilityAdmitted
            )
            .relation(),
            Some(CompatibilityRelation::ForwardRead)
        );
        assert_eq!(
            lane(
                &certification,
                Milestone12CertificationLaneKind::RollingAdapterEdgeRejected
            )
            .rejection_kind(),
            Some(CompatibilityRejectionKind::RollingWindowRejected)
        );
        assert_eq!(
            lane(
                &certification,
                Milestone12CertificationLaneKind::RestoreScopedBackupAdmitted
            )
            .relation(),
            Some(CompatibilityRelation::BackwardRead)
        );
        assert_eq!(
            lane(
                &certification,
                Milestone12CertificationLaneKind::RestoreOutOfScopeRejected
            )
            .rejection_kind(),
            Some(CompatibilityRejectionKind::RestoreOutOfScopeScanRejected)
        );
        assert_eq!(
            lane(
                &certification,
                Milestone12CertificationLaneKind::DisasterRecoveryTruthWindow
            )
            .status(),
            Milestone12CertificationLaneStatus::EvidenceOnly
        );
    }

    #[test]
    fn artifact_format_evolution_runner_emits_digest_and_gap_evidence() {
        let certification = Milestone12CertificationRunner::first_ship()
            .run()
            .expect("first-ship certification should run");
        assert_eq!(certification.digest_set().artifact_digest().len(), 64);
        assert_eq!(certification.digest_set().failure_digest().len(), 64);
        assert_eq!(certification.digest_set().diagnostics_digest().len(), 64);
        assert_eq!(
            certification.digest_set().counter_snapshot_digest().len(),
            64
        );
        assert!(certification
            .diagnostics()
            .runtime_gap_labels()
            .contains(&"adapter_execution_deferred"));
        assert_eq!(
            lane(
                &certification,
                Milestone12CertificationLaneKind::CatalogCompleteness
            )
            .counters()
            .manifest_entries_visited,
            super::super::catalog::FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT as u64
        );
    }

    fn lane(
        certification: &Milestone12ArtifactFormatEvolutionCertification,
        kind: Milestone12CertificationLaneKind,
    ) -> &Milestone12CertificationLaneOutcome {
        certification
            .evidence_bundle()
            .lane_outcomes()
            .iter()
            .find(|lane| lane.lane_kind() == kind)
            .expect("lane exists")
    }
}
