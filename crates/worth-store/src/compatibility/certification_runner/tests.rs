use std::collections::BTreeSet;

use super::super::admission::CompatibilityRejectionKind;
use super::super::admission::{CompatibilityReadAdmissionOutcome, CompatibilityRelation};
use super::{
    Milestone12ArtifactFormatEvolutionCertification, Milestone12CertificationLaneKind,
    Milestone12CertificationLaneOutcome, Milestone12CertificationLaneStatus,
    Milestone12CertificationRunner,
};

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
        9
    );
    assert_eq!(
        certification
            .evidence_bundle()
            .run_summary()
            .rejected_lane_count(),
        11
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
            Milestone12CertificationLaneKind::MaintenanceSummaryRebuildAdmitted
        )
        .status(),
        Milestone12CertificationLaneStatus::Accepted
    );
    assert_eq!(
        lane(
            &certification,
            Milestone12CertificationLaneKind::MaintenanceSummaryRebuildAdmitted
        )
        .counters()
        .maintenance_compatibility_rebuild_admission_count,
        1
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
            Milestone12CertificationLaneKind::AdapterParityAdmitted
        )
        .relation(),
        Some(CompatibilityRelation::AdapterRequired)
    );
    assert_eq!(
        lane(
            &certification,
            Milestone12CertificationLaneKind::AdapterParityDigestRejected
        )
        .rejection_kind(),
        Some(CompatibilityRejectionKind::AdapterParityFailure)
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
    assert!(!certification
        .diagnostics()
        .runtime_gap_labels()
        .contains(&"facade_read_write_restore_integration_deferred"));
    assert!(!certification
        .diagnostics()
        .runtime_gap_labels()
        .contains(&"adapter_execution_deferred"));
    assert!(!certification
        .diagnostics()
        .runtime_gap_labels()
        .contains(&"derived_rebuild_execution_deferred"));
    assert!(!certification
        .diagnostics()
        .runtime_gap_labels()
        .contains(&"rolling_writer_publication_deferred"));
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
