use worth_store_contracts::{
    S10BackupExportReadinessNonClaim, S10CompactionReadinessNonClaim,
    S10RepairScanReadinessNonClaim,
};
use worth_store_io_scheduler::{
    S10BackupExportIoReadinessHandoff, S10CompactionIoReadinessHandoff,
    S10RepairScanIoReadinessHandoff,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S10CompactionIoReadinessSeed {
    handoff: S10CompactionIoReadinessHandoff,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S10BackupExportIoReadinessSeed {
    handoff: S10BackupExportIoReadinessHandoff,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S10RepairScanIoReadinessSeed {
    handoff: S10RepairScanIoReadinessHandoff,
}

pub fn admit_s10_compaction_io_readiness_seed(
    handoff: S10CompactionIoReadinessHandoff,
) -> S10CompactionIoReadinessSeed {
    S10CompactionIoReadinessSeed { handoff }
}

pub fn admit_s10_backup_export_io_readiness_seed(
    handoff: S10BackupExportIoReadinessHandoff,
) -> S10BackupExportIoReadinessSeed {
    S10BackupExportIoReadinessSeed { handoff }
}

pub fn admit_s10_repair_scan_io_readiness_seed(
    handoff: S10RepairScanIoReadinessHandoff,
) -> S10RepairScanIoReadinessSeed {
    S10RepairScanIoReadinessSeed { handoff }
}

impl S10CompactionIoReadinessSeed {
    pub const fn handoff(&self) -> &S10CompactionIoReadinessHandoff {
        &self.handoff
    }

    pub const fn non_claims(&self) -> &[S10CompactionReadinessNonClaim; 3] {
        self.handoff.non_claims()
    }

    pub const fn carries_compaction_product_claim(&self) -> bool {
        false
    }
}

impl S10BackupExportIoReadinessSeed {
    pub const fn handoff(&self) -> &S10BackupExportIoReadinessHandoff {
        &self.handoff
    }

    pub const fn non_claims(&self) -> &[S10BackupExportReadinessNonClaim; 3] {
        self.handoff.non_claims()
    }

    pub const fn carries_backup_restore_claim(&self) -> bool {
        false
    }
}

impl S10RepairScanIoReadinessSeed {
    pub const fn handoff(&self) -> &S10RepairScanIoReadinessHandoff {
        &self.handoff
    }

    pub const fn non_claims(&self) -> &[S10RepairScanReadinessNonClaim; 3] {
        self.handoff.non_claims()
    }

    pub const fn carries_repair_authorization_claim(&self) -> bool {
        false
    }
}
