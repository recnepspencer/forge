use super::{
    later_handoff_seeds::{
        S6ClosedS10BackupExportAdmissionSeed, S6ClosedS10RepairAdmissionSeed,
        S6ClosedS11SecureIoFoundationAdmissionSeed, S6ClosedS7PlacementAdmissionSeed,
    },
    proof::S6ProductionReadinessProof,
    residual_debt::S6ResidualDebtLedger,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6ProductionReadinessPosture {
    PlatformGrade,
    ResidualDebtPresent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct S6ProductionReadinessClosure {
    posture: S6ProductionReadinessPosture,
    proof: S6ProductionReadinessProof,
    residual_debt: S6ResidualDebtLedger,
    s7_placement: S6ClosedS7PlacementAdmissionSeed,
    s10_backup_export: S6ClosedS10BackupExportAdmissionSeed,
    s10_repair: S6ClosedS10RepairAdmissionSeed,
    s11_secure_io_foundation: S6ClosedS11SecureIoFoundationAdmissionSeed,
}

impl S6ProductionReadinessClosure {
    pub(crate) const fn new(
        posture: S6ProductionReadinessPosture,
        proof: S6ProductionReadinessProof,
        residual_debt: S6ResidualDebtLedger,
        s7_placement: S6ClosedS7PlacementAdmissionSeed,
        s10_backup_export: S6ClosedS10BackupExportAdmissionSeed,
        s10_repair: S6ClosedS10RepairAdmissionSeed,
        s11_secure_io_foundation: S6ClosedS11SecureIoFoundationAdmissionSeed,
    ) -> Self {
        Self {
            posture,
            proof,
            residual_debt,
            s7_placement,
            s10_backup_export,
            s10_repair,
            s11_secure_io_foundation,
        }
    }

    pub const fn posture(&self) -> S6ProductionReadinessPosture {
        self.posture
    }

    pub const fn proof(&self) -> S6ProductionReadinessProof {
        self.proof
    }

    pub const fn s7_placement_handoff(&self) -> S6ClosedS7PlacementAdmissionSeed {
        self.s7_placement
    }

    pub const fn s10_backup_export_handoff(&self) -> S6ClosedS10BackupExportAdmissionSeed {
        self.s10_backup_export
    }

    pub const fn s10_repair_handoff(&self) -> S6ClosedS10RepairAdmissionSeed {
        self.s10_repair
    }

    pub const fn s11_secure_io_foundation_handoff(
        &self,
    ) -> S6ClosedS11SecureIoFoundationAdmissionSeed {
        self.s11_secure_io_foundation
    }

    pub const fn residual_debt(&self) -> &S6ResidualDebtLedger {
        &self.residual_debt
    }
}
