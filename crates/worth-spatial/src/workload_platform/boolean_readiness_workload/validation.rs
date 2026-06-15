use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::planar_contracts::contract_bundle::{
    PlanarM7ReadinessDenialKind, PlanarM7ReadinessFamily, PlanarM7ReadinessReceipt,
};
use crate::workload_platform::evidence_ledger::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceLedgerError, WorkloadEvidenceStage,
};
use crate::workload_platform::projection_fact_parity::{
    ProjectionFactParityCase, ProjectionFactParityLane, ProjectionFactParityReceipt,
};

use super::blocker_evidence::PlanarBooleanReadinessBlocker;
use super::denial::{
    PlanarBooleanReadinessWorkloadDenial, PlanarBooleanReadinessWorkloadDenialKind,
};
use super::evidence_basis::PlanarBooleanReadinessEvidenceBasis;
use super::required_stage::PlanarBooleanReadinessRequiredStage;

pub(crate) fn validate_readiness_workload_basis(
    basis: &PlanarBooleanReadinessEvidenceBasis,
    declaration: &str,
) -> Result<(), PlanarBooleanReadinessWorkloadDenial> {
    if declaration.trim().is_empty() {
        return Err(denial(
            PlanarBooleanReadinessWorkloadDenialKind::MissingDeclaration,
            None,
            "Boolean-readiness workload requires a human-readable declaration.",
            "missing-declaration",
            0,
        ));
    }
    reject_explicit_blocker(basis)?;
    assert_ledger_is_real(basis.evidence_ledger())?;
    assert_parity_matches_ledger(basis.evidence_ledger(), basis.parity_receipt())?;
    Ok(())
}

pub(crate) fn validate_readiness_receipt(
    receipt: &PlanarM7ReadinessReceipt,
    parity: &ProjectionFactParityReceipt,
) -> Result<(), PlanarBooleanReadinessWorkloadDenial> {
    if !receipt.is_acceptable_m7_input() {
        return Err(query_boundary_denial(
            "M7 readiness receipt was not acceptable as pre-boolean input.",
        ));
    }
    if receipt.boolean_result().is_some() || receipt.imprint_action().is_some() {
        return Err(denial(
            PlanarBooleanReadinessWorkloadDenialKind::BooleanExecutionAlreadyPresent,
            Some(PlanarBooleanReadinessRequiredStage::ContractBundle),
            "Boolean-readiness workload must stop before boolean result or imprint action.",
            receipt.readiness_digest(),
            PlanarBooleanReadinessRequiredStage::ALL.len(),
        ));
    }
    assert_readiness_consumes_parity_receipts(receipt, parity)?;
    Ok(())
}

pub(crate) fn readiness_workload_digest(
    evidence_ledger: &CompleteWorkloadEvidenceLedger,
    parity_receipt: &ProjectionFactParityReceipt,
    receipt: &PlanarM7ReadinessReceipt,
    declaration: &str,
) -> String {
    let mut parts = evidence_ledger
        .rows()
        .iter()
        .map(|row| format!("{:?}:{}", row.stage(), row.evidence_identity()))
        .collect::<Vec<_>>();
    parts.push(format!(
        "projection_fact_parity:{}",
        parity_receipt.parity_digest()
    ));
    parts.push(format!("m7_readiness:{}", receipt.readiness_digest()));
    parts.push(format!("declaration:{declaration}"));
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn m7_denial_to_workload_denial(
    kind: PlanarM7ReadinessDenialKind,
    reason: &str,
) -> PlanarBooleanReadinessWorkloadDenial {
    let workload_kind = match kind {
        PlanarM7ReadinessDenialKind::MismatchedRetainedFacts
        | PlanarM7ReadinessDenialKind::MismatchedRecoveryPosture => {
            PlanarBooleanReadinessWorkloadDenialKind::RecoveryOrReplayMismatch
        }
        PlanarM7ReadinessDenialKind::MismatchedProjectionConsumption
        | PlanarM7ReadinessDenialKind::MismatchedDiagnostics
        | PlanarM7ReadinessDenialKind::MismatchedStructuralIdentity
        | PlanarM7ReadinessDenialKind::MismatchedMotionPosture
        | PlanarM7ReadinessDenialKind::MismatchedBooleanReadinessRoot => {
            PlanarBooleanReadinessWorkloadDenialKind::ProjectionOrParityMismatch
        }
        PlanarM7ReadinessDenialKind::BooleanExecutionAlreadyPresent => {
            PlanarBooleanReadinessWorkloadDenialKind::BooleanExecutionAlreadyPresent
        }
        _ => PlanarBooleanReadinessWorkloadDenialKind::QueryBoundaryMismatch,
    };
    denial(
        workload_kind,
        Some(PlanarBooleanReadinessRequiredStage::ContractBundle),
        reason,
        format!("m7-readiness-denial:{kind:?}:{reason}"),
        PlanarBooleanReadinessRequiredStage::ALL.len() - 1,
    )
}

pub(crate) fn query_boundary_denial(reason: &str) -> PlanarBooleanReadinessWorkloadDenial {
    denial(
        PlanarBooleanReadinessWorkloadDenialKind::QueryBoundaryMismatch,
        Some(PlanarBooleanReadinessRequiredStage::ContractBundle),
        reason,
        format!("query-boundary:{reason}"),
        PlanarBooleanReadinessRequiredStage::ALL.len() - 1,
    )
}

fn reject_explicit_blocker(
    basis: &PlanarBooleanReadinessEvidenceBasis,
) -> Result<(), PlanarBooleanReadinessWorkloadDenial> {
    let Some(blocker_evidence) = basis.blocker_evidence() else {
        return Ok(());
    };
    let blocker = blocker_evidence.blocker();
    let reason = blocker_evidence.reason();
    let (kind, stage) = match blocker {
        PlanarBooleanReadinessBlocker::PolicyRequired => (
            PlanarBooleanReadinessWorkloadDenialKind::PolicyRequired,
            PlanarBooleanReadinessRequiredStage::UserResponse,
        ),
        PlanarBooleanReadinessBlocker::CleanFailure => (
            PlanarBooleanReadinessWorkloadDenialKind::CleanFailure,
            PlanarBooleanReadinessRequiredStage::Diagnostics,
        ),
        PlanarBooleanReadinessBlocker::UnsupportedWorkloadFamily => (
            PlanarBooleanReadinessWorkloadDenialKind::UnsupportedWorkloadFamily,
            PlanarBooleanReadinessRequiredStage::SurfaceSupport,
        ),
        PlanarBooleanReadinessBlocker::PredicateUncertainty => (
            PlanarBooleanReadinessWorkloadDenialKind::PredicateUncertainty,
            PlanarBooleanReadinessRequiredStage::Projection,
        ),
        PlanarBooleanReadinessBlocker::OrientationFlipLocalization => (
            PlanarBooleanReadinessWorkloadDenialKind::OrientationFlipLocalization,
            PlanarBooleanReadinessRequiredStage::Diagnostics,
        ),
        PlanarBooleanReadinessBlocker::KernelSummarySubstitution => (
            PlanarBooleanReadinessWorkloadDenialKind::KernelSummarySubstitution,
            PlanarBooleanReadinessRequiredStage::ContractBundle,
        ),
    };
    Err(denial(
        kind,
        Some(stage),
        reason,
        blocker_evidence.evidence_digest(),
        consumed_before(stage),
    ))
}

fn assert_ledger_is_real(
    ledger: &CompleteWorkloadEvidenceLedger,
) -> Result<(), PlanarBooleanReadinessWorkloadDenial> {
    ledger
        .guards()
        .assert_uses_real_topology()
        .and_then(|guard| guard.assert_binding_is_receipt_backed())
        .and_then(|guard| guard.assert_projection_is_receipt_backed())
        .and_then(|guard| guard.assert_transform_changed_geometry())
        .and_then(|guard| guard.assert_replay_consumed_retained_artifact())
        .and_then(|guard| guard.assert_counters_are_receipt_backed())
        .and_then(|guard| guard.assert_no_fixture_arithmetic_as_truth())
        .and_then(|guard| guard.assert_no_synthetic_end_to_end_claim())
        .map(|_| ())
        .map_err(|error| {
            let ledger_error = WorkloadEvidenceLedgerError::from(error);
            denial(
                PlanarBooleanReadinessWorkloadDenialKind::MissingRequiredStage,
                None,
                ledger_error.human_reason(),
                format!("ledger:{ledger_error:?}"),
                0,
            )
        })
}

fn assert_parity_matches_ledger(
    ledger: &CompleteWorkloadEvidenceLedger,
    parity: &ProjectionFactParityReceipt,
) -> Result<(), PlanarBooleanReadinessWorkloadDenial> {
    if parity.case() != ProjectionFactParityCase::AdmittedAcrossAllLanes {
        return Err(denial(
            PlanarBooleanReadinessWorkloadDenialKind::ProjectionOrParityMismatch,
            Some(PlanarBooleanReadinessRequiredStage::ProjectionFactParity),
            "Boolean-readiness requires admitted projection fact parity across every lane.",
            parity.parity_digest(),
            PlanarBooleanReadinessRequiredStage::ALL.len() - 3,
        ));
    }
    if parity.workload_basis_identity() != workload_basis_identity(ledger) {
        return Err(denial(
            PlanarBooleanReadinessWorkloadDenialKind::ProjectionOrParityMismatch,
            Some(PlanarBooleanReadinessRequiredStage::ProjectionFactParity),
            "Projection fact parity came from a different workload evidence ledger.",
            parity.parity_digest(),
            PlanarBooleanReadinessRequiredStage::ALL.len() - 3,
        ));
    }
    if parity.counters().receipt_backed_lanes() != ProjectionFactParityLane::REQUIRED.len() {
        return Err(denial(
            PlanarBooleanReadinessWorkloadDenialKind::ProjectionOrParityMismatch,
            Some(PlanarBooleanReadinessRequiredStage::ProjectionFactParity),
            "Projection fact parity must be receipt-backed across every required lane.",
            parity.parity_digest(),
            PlanarBooleanReadinessRequiredStage::ALL.len() - 3,
        ));
    }
    Ok(())
}

fn assert_readiness_consumes_parity_receipts(
    receipt: &PlanarM7ReadinessReceipt,
    parity: &ProjectionFactParityReceipt,
) -> Result<(), PlanarBooleanReadinessWorkloadDenial> {
    require_family_matches_lane(
        receipt,
        parity,
        PlanarM7ReadinessFamily::RetainedPlanarFacts,
        ProjectionFactParityLane::Retained,
        "Retained facts in final readiness must match the retained lane certified by parity.",
    )?;
    require_family_matches_lane(
        receipt,
        parity,
        PlanarM7ReadinessFamily::ProjectionConsumedFacts,
        ProjectionFactParityLane::ProjectionConsumed,
        "Projection-consumed facts in final readiness must match the projection lane certified by parity.",
    )?;
    require_family_matches_lane(
        receipt,
        parity,
        PlanarM7ReadinessFamily::RecoveryPosture,
        ProjectionFactParityLane::Recovered,
        "Recovery posture in final readiness must match the recovery lane certified by parity.",
    )?;
    require_family_matches_lane(
        receipt,
        parity,
        PlanarM7ReadinessFamily::Diagnostics,
        ProjectionFactParityLane::Diagnostics,
        "Diagnostics in final readiness must match the diagnostic lane certified by parity.",
    )
}

fn require_family_matches_lane(
    receipt: &PlanarM7ReadinessReceipt,
    parity: &ProjectionFactParityReceipt,
    family: PlanarM7ReadinessFamily,
    lane: ProjectionFactParityLane,
    reason: &'static str,
) -> Result<(), PlanarBooleanReadinessWorkloadDenial> {
    let Some(family_row) = receipt
        .family_rows()
        .iter()
        .find(|row| row.family() == family)
    else {
        return Err(denial(
            PlanarBooleanReadinessWorkloadDenialKind::MissingRequiredStage,
            Some(PlanarBooleanReadinessRequiredStage::ContractBundle),
            reason,
            format!("missing-family:{family:?}"),
            PlanarBooleanReadinessRequiredStage::ALL.len() - 1,
        ));
    };
    let Some(lane_evidence) = parity.evidence_for_lane(lane) else {
        return Err(denial(
            PlanarBooleanReadinessWorkloadDenialKind::ProjectionOrParityMismatch,
            Some(PlanarBooleanReadinessRequiredStage::ProjectionFactParity),
            reason,
            format!("missing-lane:{lane:?}"),
            PlanarBooleanReadinessRequiredStage::ALL.len() - 3,
        ));
    };
    if family_row.receipt_digest() != lane_evidence.source_receipt_identity() {
        return Err(denial(
            mismatch_kind_for_lane(lane),
            Some(PlanarBooleanReadinessRequiredStage::ProjectionFactParity),
            reason,
            format!(
                "family:{family:?}:{} lane:{lane:?}:{}",
                family_row.receipt_digest(),
                lane_evidence.source_receipt_identity()
            ),
            PlanarBooleanReadinessRequiredStage::ALL.len() - 2,
        ));
    }
    Ok(())
}

fn mismatch_kind_for_lane(
    lane: ProjectionFactParityLane,
) -> PlanarBooleanReadinessWorkloadDenialKind {
    match lane {
        ProjectionFactParityLane::Retained
        | ProjectionFactParityLane::Replayed
        | ProjectionFactParityLane::Recovered => {
            PlanarBooleanReadinessWorkloadDenialKind::RecoveryOrReplayMismatch
        }
        _ => PlanarBooleanReadinessWorkloadDenialKind::ProjectionOrParityMismatch,
    }
}

fn workload_basis_identity(ledger: &CompleteWorkloadEvidenceLedger) -> String {
    let parts = ProjectionFactParityLane::REQUIRED
        .iter()
        .filter_map(|lane| stage_for_lane(*lane))
        .filter_map(|stage| {
            ledger
                .evidence_for_stage(stage)
                .map(|identity| format!("{stage:?}:{identity}"))
        })
        .collect::<Vec<_>>();
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

fn stage_for_lane(lane: ProjectionFactParityLane) -> Option<WorkloadEvidenceStage> {
    match lane {
        ProjectionFactParityLane::Live => Some(WorkloadEvidenceStage::Topology),
        ProjectionFactParityLane::Projected | ProjectionFactParityLane::ProjectionConsumed => {
            Some(WorkloadEvidenceStage::Projection)
        }
        ProjectionFactParityLane::Retained | ProjectionFactParityLane::Replayed => {
            Some(WorkloadEvidenceStage::RetainedReplay)
        }
        ProjectionFactParityLane::Transformed => Some(WorkloadEvidenceStage::Transform),
        ProjectionFactParityLane::Recovered | ProjectionFactParityLane::Diagnostics => {
            Some(WorkloadEvidenceStage::Diagnostics)
        }
        ProjectionFactParityLane::LocalRebuild => Some(WorkloadEvidenceStage::Response),
    }
}

fn denial(
    kind: PlanarBooleanReadinessWorkloadDenialKind,
    failed_stage: Option<PlanarBooleanReadinessRequiredStage>,
    reason: impl Into<String>,
    evidence_digest: impl Into<String>,
    consumed: usize,
) -> PlanarBooleanReadinessWorkloadDenial {
    PlanarBooleanReadinessWorkloadDenial::new(kind, failed_stage, reason, evidence_digest, consumed)
}

fn consumed_before(stage: PlanarBooleanReadinessRequiredStage) -> usize {
    PlanarBooleanReadinessRequiredStage::ALL
        .iter()
        .position(|candidate| *candidate == stage)
        .unwrap_or(0)
}
