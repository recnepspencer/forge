use crate::{
    S10BackupExportReadinessNonClaim, S10RepairScanReadinessNonClaim, S11OperatorReadinessNonClaim,
    S6MaterializedCertificationAdoptionReceipt, S7PlacementReadinessNonClaim,
};

use super::{
    denial::S6ProductionReadinessClosureDenial,
    later_handoff_seeds::{
        S6ClosedS10BackupExportAdmissionSeed, S6ClosedS10RepairAdmissionSeed,
        S6ClosedS11SecureIoFoundationAdmissionSeed, S6ClosedS7PlacementAdmissionSeed,
    },
    proof::S6ProductionReadinessProof,
    receipt::{S6ProductionReadinessClosure, S6ProductionReadinessPosture},
    residual_debt::S6ResidualDebtLedger,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6LaterMilestoneNonClaimBoundaries {
    s7_placement_non_claims: [S7PlacementReadinessNonClaim; 3],
    s10_backup_export_non_claims: [S10BackupExportReadinessNonClaim; 3],
    s10_repair_non_claims: [S10RepairScanReadinessNonClaim; 3],
    s11_secure_io_non_claims: [S11OperatorReadinessNonClaim; 4],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct S6ProductionReadinessClosureInput {
    phase13_adoption: S6MaterializedCertificationAdoptionReceipt,
    requested_posture: S6ProductionReadinessPosture,
}

pub fn close_s6_production_readiness(
    input: S6ProductionReadinessClosureInput,
) -> Result<S6ProductionReadinessClosure, S6ProductionReadinessClosureDenial> {
    reject_unclosed_phase13_evidence(&input.phase13_adoption)?;
    let non_claims = S6LaterMilestoneNonClaimBoundaries::from_required_non_claims();
    let residual_debt = S6ResidualDebtLedger::from_phase13_adoption(&input.phase13_adoption)?;
    if input.requested_posture == S6ProductionReadinessPosture::PlatformGrade
        && residual_debt.contains_non_platform_grade_posture()
    {
        return Err(S6ProductionReadinessClosureDenial::ResidualDebtCannotBePlatformGrade);
    }
    let proof =
        S6ProductionReadinessProof::from_phase13_adoption(&input.phase13_adoption, &non_claims)?;
    if !proof.is_checked_for_s6_closeout() {
        return Err(S6ProductionReadinessClosureDenial::Phase13EvidenceCannotSatisfyReadiness);
    }
    Ok(S6ProductionReadinessClosure::new(
        input.requested_posture,
        proof,
        residual_debt,
        S6ClosedS7PlacementAdmissionSeed::from_closed_s6_readiness(
            non_claims.s7_placement_non_claims(),
        ),
        S6ClosedS10BackupExportAdmissionSeed::from_closed_s6_readiness(
            non_claims.s10_backup_export_non_claims(),
        ),
        S6ClosedS10RepairAdmissionSeed::from_closed_s6_readiness(
            non_claims.s10_repair_non_claims(),
        ),
        S6ClosedS11SecureIoFoundationAdmissionSeed::from_closed_s6_readiness(
            non_claims.s11_secure_io_non_claims(),
        ),
    ))
}

impl S6ProductionReadinessClosureInput {
    pub fn from_phase13_adoption(
        phase13_adoption: S6MaterializedCertificationAdoptionReceipt,
    ) -> Self {
        Self {
            phase13_adoption,
            requested_posture: S6ProductionReadinessPosture::ResidualDebtPresent,
        }
    }

    pub const fn requesting_platform_grade(mut self) -> Self {
        self.requested_posture = S6ProductionReadinessPosture::PlatformGrade;
        self
    }
}

impl S6LaterMilestoneNonClaimBoundaries {
    pub(crate) const fn from_required_non_claims() -> Self {
        Self {
            s7_placement_non_claims: S7PlacementReadinessNonClaim::required(),
            s10_backup_export_non_claims: S10BackupExportReadinessNonClaim::required(),
            s10_repair_non_claims: S10RepairScanReadinessNonClaim::required(),
            s11_secure_io_non_claims: S11OperatorReadinessNonClaim::required(),
        }
    }

    pub const fn later_handoff_boundary_count(&self) -> usize {
        4
    }

    pub const fn s7_placement_non_claims(&self) -> [S7PlacementReadinessNonClaim; 3] {
        self.s7_placement_non_claims
    }

    pub const fn s10_backup_export_non_claims(&self) -> [S10BackupExportReadinessNonClaim; 3] {
        self.s10_backup_export_non_claims
    }

    pub const fn s10_repair_non_claims(&self) -> [S10RepairScanReadinessNonClaim; 3] {
        self.s10_repair_non_claims
    }

    pub const fn s11_secure_io_non_claims(&self) -> [S11OperatorReadinessNonClaim; 4] {
        self.s11_secure_io_non_claims
    }
}

fn reject_unclosed_phase13_evidence(
    adoption: &S6MaterializedCertificationAdoptionReceipt,
) -> Result<(), S6ProductionReadinessClosureDenial> {
    if adoption.profile_count() != 6
        || !adoption.profile_boundary_certification_only()
        || adoption.performance_receipt_count() != 5
        || adoption.counter_strengths().is_empty()
        || !adoption.proof().checked_execution()
        || adoption.proof().readmission_boundaries() != 5
        || !adoption
            .proof_topology()
            .is_checked_for_closeout(adoption.proof())
        || adoption.residual_debt_rows().is_empty()
    {
        return Err(S6ProductionReadinessClosureDenial::Phase13EvidenceCannotSatisfyReadiness);
    }
    Ok(())
}
