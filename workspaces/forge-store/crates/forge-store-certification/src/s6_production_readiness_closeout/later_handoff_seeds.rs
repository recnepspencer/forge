use forge_store_readiness::{
    S10BackupExportReadinessNonClaim, S10RepairScanReadinessNonClaim,
    S11OperatorReadinessNonClaim, S6LaterMilestoneDestination, S7PlacementReadinessNonClaim,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6ClosedS7PlacementAdmissionSeed {
    destination: S6LaterMilestoneDestination,
    non_claims: [S7PlacementReadinessNonClaim; 3],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6ClosedS10BackupExportAdmissionSeed {
    destination: S6LaterMilestoneDestination,
    non_claims: [S10BackupExportReadinessNonClaim; 3],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6ClosedS10RepairAdmissionSeed {
    destination: S6LaterMilestoneDestination,
    non_claims: [S10RepairScanReadinessNonClaim; 3],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6ClosedS11SecureIoFoundationAdmissionSeed {
    destination: S6LaterMilestoneDestination,
    non_claims: [S11OperatorReadinessNonClaim; 4],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct S6LaterMilestoneNonClaimBoundaries {
    s7_placement_non_claims: [S7PlacementReadinessNonClaim; 3],
    s10_backup_export_non_claims: [S10BackupExportReadinessNonClaim; 3],
    s10_repair_non_claims: [S10RepairScanReadinessNonClaim; 3],
    s11_secure_io_non_claims: [S11OperatorReadinessNonClaim; 4],
}

impl S6ClosedS7PlacementAdmissionSeed {
    pub(super) const fn from_closed_s6_readiness(
        non_claims: [S7PlacementReadinessNonClaim; 3],
    ) -> Self {
        Self {
            destination: S6LaterMilestoneDestination::S7Placement,
            non_claims,
        }
    }

    pub const fn destination(&self) -> S6LaterMilestoneDestination {
        self.destination
    }

    pub const fn non_claims(&self) -> &[S7PlacementReadinessNonClaim; 3] {
        &self.non_claims
    }
}

impl S6ClosedS10BackupExportAdmissionSeed {
    pub(super) const fn from_closed_s6_readiness(
        non_claims: [S10BackupExportReadinessNonClaim; 3],
    ) -> Self {
        Self {
            destination: S6LaterMilestoneDestination::S10BackupExport,
            non_claims,
        }
    }

    pub const fn destination(&self) -> S6LaterMilestoneDestination {
        self.destination
    }

    pub const fn non_claims(&self) -> &[S10BackupExportReadinessNonClaim; 3] {
        &self.non_claims
    }
}

impl S6ClosedS10RepairAdmissionSeed {
    pub(super) const fn from_closed_s6_readiness(
        non_claims: [S10RepairScanReadinessNonClaim; 3],
    ) -> Self {
        Self {
            destination: S6LaterMilestoneDestination::S10RepairScan,
            non_claims,
        }
    }

    pub const fn destination(&self) -> S6LaterMilestoneDestination {
        self.destination
    }

    pub const fn non_claims(&self) -> &[S10RepairScanReadinessNonClaim; 3] {
        &self.non_claims
    }
}

impl S6ClosedS11SecureIoFoundationAdmissionSeed {
    pub(super) const fn from_closed_s6_readiness(
        non_claims: [S11OperatorReadinessNonClaim; 4],
    ) -> Self {
        Self {
            destination: S6LaterMilestoneDestination::S11OperatorReadiness,
            non_claims,
        }
    }

    pub const fn destination(&self) -> S6LaterMilestoneDestination {
        self.destination
    }

    pub const fn non_claims(&self) -> &[S11OperatorReadinessNonClaim; 4] {
        &self.non_claims
    }
}

impl S6LaterMilestoneNonClaimBoundaries {
    pub(super) const fn from_required_non_claims() -> Self {
        Self {
            s7_placement_non_claims: S7PlacementReadinessNonClaim::required(),
            s10_backup_export_non_claims: S10BackupExportReadinessNonClaim::required(),
            s10_repair_non_claims: S10RepairScanReadinessNonClaim::required(),
            s11_secure_io_non_claims: S11OperatorReadinessNonClaim::required(),
        }
    }

    pub(super) const fn later_handoff_boundary_count(&self) -> usize {
        4
    }

    pub(super) const fn s7_placement_non_claims(&self) -> [S7PlacementReadinessNonClaim; 3] {
        self.s7_placement_non_claims
    }

    pub(super) const fn s10_backup_export_non_claims(
        &self,
    ) -> [S10BackupExportReadinessNonClaim; 3] {
        self.s10_backup_export_non_claims
    }

    pub(super) const fn s10_repair_non_claims(&self) -> [S10RepairScanReadinessNonClaim; 3] {
        self.s10_repair_non_claims
    }

    pub(super) const fn s11_secure_io_non_claims(&self) -> [S11OperatorReadinessNonClaim; 4] {
        self.s11_secure_io_non_claims
    }
}
