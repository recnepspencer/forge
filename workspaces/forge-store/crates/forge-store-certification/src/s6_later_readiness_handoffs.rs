use forge_store_io_scheduler::{
    BackgroundResourceBudget, IoSchedulerBackendCapabilityRequirement,
    S10BackupExportIoReadinessHandoff, S10CompactionIoReadinessHandoff,
    S10RepairScanIoReadinessHandoff, S11OperatorIoReadinessHandoff, S7PlacementIoReadinessHandoff,
    SecureIoPosture,
};
use forge_store_physical_backend::{BackendTargetProfile, CapabilityEvidenceClass};
use forge_store_readiness::{
    S10BackupExportReadinessNonClaim, S10CompactionReadinessNonClaim,
    S10RepairScanReadinessNonClaim, S11OperatorReadinessNonClaim, S6LaterMilestoneDestination,
    S7PlacementReadinessNonClaim,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6LaterReadinessHandoffCertification {
    placement: S6PlacementHandoffEvidence,
    compaction: S6CompactionHandoffEvidence,
    backup_export: S6BackupExportHandoffEvidence,
    repair_scan: S6RepairScanHandoffEvidence,
    operator: S6OperatorHandoffEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6PlacementHandoffEvidence {
    destination: S6LaterMilestoneDestination,
    non_claims: [S7PlacementReadinessNonClaim; 3],
    wait_count: u64,
    protected_byte_footprint: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6CompactionHandoffEvidence {
    destination: S6LaterMilestoneDestination,
    non_claims: [S10CompactionReadinessNonClaim; 3],
    compaction_pressure_units: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6BackupExportHandoffEvidence {
    destination: S6LaterMilestoneDestination,
    non_claims: [S10BackupExportReadinessNonClaim; 3],
    backup_pressure_units: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6RepairScanHandoffEvidence {
    destination: S6LaterMilestoneDestination,
    non_claims: [S10RepairScanReadinessNonClaim; 3],
    repair_pressure_units: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6OperatorHandoffEvidence {
    destination: S6LaterMilestoneDestination,
    non_claims: [S11OperatorReadinessNonClaim; 4],
    backend_requirement: IoSchedulerBackendCapabilityRequirement,
    backend_profile: BackendTargetProfile,
    backend_evidence_class: CapabilityEvidenceClass,
    secure_io_posture: SecureIoPosture,
    scope_checks: u64,
    backend_posture_checks: u64,
}

pub fn certify_s6_later_readiness_handoffs(
    placement: &S7PlacementIoReadinessHandoff,
    compaction: &S10CompactionIoReadinessHandoff,
    backup_export: &S10BackupExportIoReadinessHandoff,
    repair_scan: &S10RepairScanIoReadinessHandoff,
    operator: &S11OperatorIoReadinessHandoff,
) -> S6LaterReadinessHandoffCertification {
    S6LaterReadinessHandoffCertification {
        placement: S6PlacementHandoffEvidence::from_handoff(placement),
        compaction: S6CompactionHandoffEvidence::from_handoff(compaction),
        backup_export: S6BackupExportHandoffEvidence::from_handoff(backup_export),
        repair_scan: S6RepairScanHandoffEvidence::from_handoff(repair_scan),
        operator: S6OperatorHandoffEvidence::from_handoff(operator),
    }
}

impl S6LaterReadinessHandoffCertification {
    pub const fn placement(&self) -> &S6PlacementHandoffEvidence {
        &self.placement
    }

    pub const fn compaction(&self) -> &S6CompactionHandoffEvidence {
        &self.compaction
    }

    pub const fn backup_export(&self) -> &S6BackupExportHandoffEvidence {
        &self.backup_export
    }

    pub const fn repair_scan(&self) -> &S6RepairScanHandoffEvidence {
        &self.repair_scan
    }

    pub const fn operator(&self) -> &S6OperatorHandoffEvidence {
        &self.operator
    }

    pub const fn destination_count(&self) -> usize {
        5
    }
}

impl S6PlacementHandoffEvidence {
    fn from_handoff(handoff: &S7PlacementIoReadinessHandoff) -> Self {
        Self {
            destination: handoff.destination(),
            non_claims: *handoff.non_claims(),
            wait_count: handoff.foreground_interference().wait_count(),
            protected_byte_footprint: handoff.foreground_interference().protected_byte_footprint(),
        }
    }

    pub const fn destination(&self) -> S6LaterMilestoneDestination {
        self.destination
    }

    pub const fn non_claims(&self) -> &[S7PlacementReadinessNonClaim; 3] {
        &self.non_claims
    }

    pub const fn wait_count(&self) -> u64 {
        self.wait_count
    }

    pub const fn protected_byte_footprint(&self) -> u64 {
        self.protected_byte_footprint
    }
}

impl S6CompactionHandoffEvidence {
    fn from_handoff(handoff: &S10CompactionIoReadinessHandoff) -> Self {
        let counters = handoff.background_pacing_counters();
        Self {
            destination: handoff.destination(),
            non_claims: *handoff.non_claims(),
            compaction_pressure_units: total_background_units(counters.compaction_debt()),
        }
    }

    pub const fn destination(&self) -> S6LaterMilestoneDestination {
        self.destination
    }

    pub const fn non_claims(&self) -> &[S10CompactionReadinessNonClaim; 3] {
        &self.non_claims
    }

    pub const fn compaction_pressure_units(&self) -> u64 {
        self.compaction_pressure_units
    }
}

impl S6BackupExportHandoffEvidence {
    fn from_handoff(handoff: &S10BackupExportIoReadinessHandoff) -> Self {
        let counters = handoff.background_pacing_counters();
        Self {
            destination: handoff.destination(),
            non_claims: *handoff.non_claims(),
            backup_pressure_units: total_background_units(counters.backup_pressure()),
        }
    }

    pub const fn destination(&self) -> S6LaterMilestoneDestination {
        self.destination
    }

    pub const fn non_claims(&self) -> &[S10BackupExportReadinessNonClaim; 3] {
        &self.non_claims
    }

    pub const fn backup_pressure_units(&self) -> u64 {
        self.backup_pressure_units
    }
}

impl S6RepairScanHandoffEvidence {
    fn from_handoff(handoff: &S10RepairScanIoReadinessHandoff) -> Self {
        let counters = handoff.background_pacing_counters();
        Self {
            destination: handoff.destination(),
            non_claims: *handoff.non_claims(),
            repair_pressure_units: total_background_units(counters.repair_pressure()),
        }
    }

    pub const fn destination(&self) -> S6LaterMilestoneDestination {
        self.destination
    }

    pub const fn non_claims(&self) -> &[S10RepairScanReadinessNonClaim; 3] {
        &self.non_claims
    }

    pub const fn repair_pressure_units(&self) -> u64 {
        self.repair_pressure_units
    }
}

const fn total_background_units(budget: BackgroundResourceBudget) -> u64 {
    budget.queue_slots()
        + budget.bandwidth_tokens()
        + budget.flush_permits()
        + budget.sync_debt()
        + budget.read_ahead_window()
        + budget.write_back_window()
        + budget.dirty_page_budget()
        + budget.worker_permits()
        + budget.cache_residency_hints()
        + budget.reclaim_permits()
}

impl S6OperatorHandoffEvidence {
    fn from_handoff(handoff: &S11OperatorIoReadinessHandoff) -> Self {
        let counters = handoff.secure_io_counters();
        Self {
            destination: handoff.destination(),
            non_claims: *handoff.non_claims(),
            backend_requirement: handoff.backend_requirement(),
            backend_profile: handoff.backend_profile(),
            backend_evidence_class: handoff.backend_evidence_class(),
            secure_io_posture: handoff.secure_io_posture(),
            scope_checks: counters.scope_checks(),
            backend_posture_checks: counters.backend_posture_checks(),
        }
    }

    pub const fn destination(&self) -> S6LaterMilestoneDestination {
        self.destination
    }

    pub const fn non_claims(&self) -> &[S11OperatorReadinessNonClaim; 4] {
        &self.non_claims
    }

    pub const fn backend_requirement(&self) -> IoSchedulerBackendCapabilityRequirement {
        self.backend_requirement
    }

    pub const fn backend_profile(&self) -> BackendTargetProfile {
        self.backend_profile
    }

    pub const fn backend_evidence_class(&self) -> CapabilityEvidenceClass {
        self.backend_evidence_class
    }

    pub const fn secure_io_posture(&self) -> SecureIoPosture {
        self.secure_io_posture
    }

    pub const fn scope_checks(&self) -> u64 {
        self.scope_checks
    }

    pub const fn backend_posture_checks(&self) -> u64 {
        self.backend_posture_checks
    }
}
