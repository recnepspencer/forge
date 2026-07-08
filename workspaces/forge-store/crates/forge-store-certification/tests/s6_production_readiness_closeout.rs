#[allow(dead_code, unused_imports)]
#[path = "s6_access_policy_support.rs"]
mod s6_access_policy_support;
#[allow(dead_code, unused_imports)]
#[path = "s6_evidence_materialization_support/mod.rs"]
mod s6_evidence_materialization_support;

use forge_store_certification::{
    adopt_materialized_s6_certification_evidence_for_closeout,
    materialize_s6_certification_evidence, S6CertificationEvidenceAdoptionReceipt,
    S6CertificationRuntimeAuthorityDenial, StoreOwnedS6CertificationMaterializationSources,
};
use forge_store_readiness::{
    close_s6_production_readiness, S10BackupExportReadinessNonClaim,
    S10RepairScanReadinessNonClaim, S11OperatorReadinessNonClaim, S6LaterMilestoneDestination,
    S6ProductionReadinessClosure, S6ProductionReadinessClosureDenial,
    S6ProductionReadinessClosureInput, S6ProductionReadinessPosture,
    S6ReadinessResidualDebtEvidenceKind, S6ResidualDebtKind, S7PlacementReadinessNonClaim,
};

#[test]
fn phase14_closes_s6_readiness_from_phase13_materialized_evidence() {
    let closeout = close_s6_production_readiness(closeout_input(phase13_adoption()))
        .expect("phase 13 materialized evidence should close S.6 readiness");

    assert_eq!(
        closeout.posture(),
        S6ProductionReadinessPosture::ResidualDebtPresent
    );
    assert!(closeout.proof().is_checked_for_s6_closeout());
    assert_eq!(
        closeout
            .proof()
            .checked_topology()
            .readiness_readmission_boundaries(),
        5
    );
    assert_eq!(
        closeout
            .proof()
            .checked_topology()
            .executed_readmission_boundaries(),
        5
    );
    assert_eq!(
        closeout
            .proof()
            .checked_topology()
            .freshness_readmitted_boundaries(),
        5
    );
    assert!(closeout
        .residual_debt()
        .contains_non_platform_grade_posture());
    assert_eq!(
        closeout.s7_placement_handoff().destination(),
        S6LaterMilestoneDestination::S7Placement
    );
    assert_eq!(
        closeout.s7_placement_handoff().non_claims(),
        &S7PlacementReadinessNonClaim::required()
    );
    assert_eq!(
        closeout.s10_backup_export_handoff().destination(),
        S6LaterMilestoneDestination::S10BackupExport
    );
    assert_eq!(
        closeout.s10_backup_export_handoff().non_claims(),
        &S10BackupExportReadinessNonClaim::required()
    );
    assert_eq!(
        closeout.s10_repair_handoff().destination(),
        S6LaterMilestoneDestination::S10RepairScan
    );
    assert_eq!(
        closeout.s10_repair_handoff().non_claims(),
        &S10RepairScanReadinessNonClaim::required()
    );
    assert_eq!(
        closeout.s11_secure_io_foundation_handoff().destination(),
        S6LaterMilestoneDestination::S11OperatorReadiness
    );
    assert_eq!(
        closeout.s11_secure_io_foundation_handoff().non_claims(),
        &S11OperatorReadinessNonClaim::required()
    );
}

#[test]
fn independently_closed_phase14_readiness_replays_equivalent_artifacts() {
    let first = close_s6_production_readiness(closeout_input(phase13_adoption()))
        .expect("first closeout should pass");
    let second = close_s6_production_readiness(closeout_input(phase13_adoption()))
        .expect("second closeout should pass");

    assert_eq!(first, second);
}

#[test]
fn residual_debt_cannot_be_promoted_to_platform_grade_readiness() {
    let denial = close_s6_production_readiness(
        closeout_input(phase13_adoption()).requesting_platform_grade(),
    )
    .expect_err("residual S.6 debt blocks platform-grade readiness");

    assert_eq!(
        denial,
        S6ProductionReadinessClosureDenial::ResidualDebtCannotBePlatformGrade
    );
}

#[test]
fn residual_debt_rows_remain_typed_in_closeout() {
    let closeout = close_s6_production_readiness(closeout_input(phase13_adoption()))
        .expect("phase 13 materialized evidence should close S.6 readiness");
    let debt_kinds: Vec<_> = closeout
        .residual_debt()
        .rows()
        .iter()
        .map(|row| row.kind())
        .collect();

    assert_eq!(
        debt_kinds,
        vec![
            S6ResidualDebtKind::UnsupportedBackendProfile,
            S6ResidualDebtKind::UnavailableEvidence,
            S6ResidualDebtKind::DegradedBackendPosture,
            S6ResidualDebtKind::DeniedClaim,
            S6ResidualDebtKind::StaleEvidence,
            S6ResidualDebtKind::RebindRequired,
            S6ResidualDebtKind::ResidualQualificationDebt,
        ]
    );
    let debt_counts: Vec<_> = closeout
        .residual_debt()
        .rows()
        .iter()
        .map(|row| (row.kind(), row.observed_claims()))
        .collect();
    assert_eq!(
        debt_counts,
        vec![
            (S6ResidualDebtKind::UnsupportedBackendProfile, 1),
            (S6ResidualDebtKind::UnavailableEvidence, 2),
            (S6ResidualDebtKind::DegradedBackendPosture, 2),
            (S6ResidualDebtKind::DeniedClaim, 6),
            (S6ResidualDebtKind::StaleEvidence, 2),
            (S6ResidualDebtKind::RebindRequired, 1),
            (S6ResidualDebtKind::ResidualQualificationDebt, 6),
        ]
    );
}

#[test]
fn phase14_residual_debt_mirrors_each_phase13_adoption_exactly() {
    let (baseline_adoption, baseline_closeout) =
        closeout_from_sources(s6_evidence_materialization_support::sources());
    let (amplified_adoption, amplified_closeout) = closeout_from_sources(
        s6_evidence_materialization_support::source_variants::sources_with_amplified_required_residual_debt()
            .expect("amplified source still uses Store-owned materialization inputs"),
    );

    let baseline_adoption_debt = adoption_debt_counts(&baseline_adoption);
    let amplified_adoption_debt = adoption_debt_counts(&amplified_adoption);

    assert_ne!(
        baseline_adoption_debt, amplified_adoption_debt,
        "fixture variation must force closeout to preserve evidence-specific debt counts"
    );
    assert_eq!(
        closeout_debt_counts(&baseline_closeout),
        baseline_adoption_debt
    );
    assert_eq!(
        closeout_debt_counts(&amplified_closeout),
        amplified_adoption_debt
    );
}

#[test]
fn phase14_rejects_phase13_evidence_without_required_residual_debt() {
    let materialized = materialize_s6_certification_evidence(
        s6_evidence_materialization_support::source_variants::sources_without_required_residual_debt()
            .expect("near-miss source remains structurally materializable"),
    )
    .expect("near-miss still has materialized Phase 13 structure");

    let denial = adopt_materialized_s6_certification_evidence_for_closeout(&materialized)
        .expect_err("Phase 13 adoption must not invent residual debt for Phase 14");

    assert_eq!(
        denial,
        S6CertificationRuntimeAuthorityDenial::CertificationEvidenceCannotSatisfyCloseout
    );
}

fn closeout_input(
    adoption: S6CertificationEvidenceAdoptionReceipt,
) -> S6ProductionReadinessClosureInput {
    S6ProductionReadinessClosureInput::from_phase13_adoption(adoption)
}

fn phase13_adoption() -> S6CertificationEvidenceAdoptionReceipt {
    let materialized =
        materialize_s6_certification_evidence(s6_evidence_materialization_support::sources())
            .expect("phase 13 materialized evidence should build");
    adopt_materialized_s6_certification_evidence_for_closeout(&materialized)
        .expect("phase 13 adoption should pass")
}

fn closeout_from_sources(
    sources: StoreOwnedS6CertificationMaterializationSources,
) -> (
    S6CertificationEvidenceAdoptionReceipt,
    S6ProductionReadinessClosure,
) {
    let materialized =
        materialize_s6_certification_evidence(sources).expect("phase 13 evidence should build");
    let adoption = adopt_materialized_s6_certification_evidence_for_closeout(&materialized)
        .expect("phase 13 adoption should pass");
    let closeout = close_s6_production_readiness(closeout_input(adoption.clone()))
        .expect("phase 14 closeout should pass");
    (adoption, closeout)
}

fn adoption_debt_counts(
    adoption: &S6CertificationEvidenceAdoptionReceipt,
) -> Vec<(S6ResidualDebtKind, usize)> {
    adoption
        .residual_debt_rows()
        .iter()
        .map(|row| (closeout_debt_kind(row.kind()), row.observed_claims()))
        .collect()
}

fn closeout_debt_counts(
    closeout: &S6ProductionReadinessClosure,
) -> Vec<(S6ResidualDebtKind, usize)> {
    closeout
        .residual_debt()
        .rows()
        .iter()
        .map(|row| (row.kind(), row.observed_claims()))
        .collect()
}

fn closeout_debt_kind(kind: S6ReadinessResidualDebtEvidenceKind) -> S6ResidualDebtKind {
    match kind {
        S6ReadinessResidualDebtEvidenceKind::UnsupportedBackendProfile => {
            S6ResidualDebtKind::UnsupportedBackendProfile
        }
        S6ReadinessResidualDebtEvidenceKind::UnavailableEvidence => {
            S6ResidualDebtKind::UnavailableEvidence
        }
        S6ReadinessResidualDebtEvidenceKind::DegradedBackendPosture => {
            S6ResidualDebtKind::DegradedBackendPosture
        }
        S6ReadinessResidualDebtEvidenceKind::DeniedClaim => S6ResidualDebtKind::DeniedClaim,
        S6ReadinessResidualDebtEvidenceKind::StaleEvidence => S6ResidualDebtKind::StaleEvidence,
        S6ReadinessResidualDebtEvidenceKind::RebindRequired => S6ResidualDebtKind::RebindRequired,
        S6ReadinessResidualDebtEvidenceKind::ResidualQualificationDebt => {
            S6ResidualDebtKind::ResidualQualificationDebt
        }
    }
}
