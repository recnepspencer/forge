use super::failure::{SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportTrustAccessPath {
    PointLookup,
    BoundedRange,
    BatchLookup,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportTrustAccessIndexKind {
    PrimarySupportIdentity,
    FamilyRole,
    Basis,
    CursorCheckpoint,
    OperationalAction,
    Epoch,
    CertificationRow,
    DomainScenario,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustAccessStructurePlan {
    index_kind: SupportTrustAccessIndexKind,
    access_path: SupportTrustAccessPath,
    rebuild_authority: String,
    epoch_invalidation_basis: String,
    read_amplification_counter: String,
}

impl SupportTrustAccessStructurePlan {
    pub fn new(
        index_kind: SupportTrustAccessIndexKind,
        access_path: SupportTrustAccessPath,
        rebuild_authority: impl Into<String>,
        epoch_invalidation_basis: impl Into<String>,
        read_amplification_counter: impl Into<String>,
    ) -> Result<Self, SupportTrustFailure> {
        if access_path == SupportTrustAccessPath::Rejected {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustAccessStructureDebt,
                SupportTrustRecoveryPosture::RebuildTrustCache,
                "support trust access structure plans cannot lower to rejected access",
            ));
        }
        let rebuild_authority = require_non_empty("rebuild authority", rebuild_authority)?;
        let epoch_invalidation_basis =
            require_non_empty("epoch invalidation basis", epoch_invalidation_basis)?;
        let read_amplification_counter =
            require_non_empty("read amplification counter", read_amplification_counter)?;
        Ok(Self {
            index_kind,
            access_path,
            rebuild_authority,
            epoch_invalidation_basis,
            read_amplification_counter,
        })
    }

    pub fn index_kind(&self) -> SupportTrustAccessIndexKind {
        self.index_kind
    }

    pub fn access_path(&self) -> SupportTrustAccessPath {
        self.access_path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportTrustDensityClass {
    SingleSupportArtifact,
    FamilyLocal,
    RoleLocal,
    BasisLocal,
    CursorCheckpointLocal,
    OperationalActionLocal,
    CertificationScopeLocal,
    DomainScenarioLocal,
    StoreGlobalRejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportTrustPathClass {
    ForegroundResumeTrustPath,
    BatchCertificationPath,
    DomainCertificationPath,
    RoadmapHandoffPath,
    TrustCacheRebuildPath,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportTrustAllocationScope {
    ForegroundScratch,
    BatchCertification,
    DomainCertification,
    RoadmapHandoff,
    TrustCacheRebuild,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportTrustCloneBoundary {
    NoClone,
    CertificationSnapshot,
    RoadmapHandoffSnapshot,
    RetryIsolation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SupportTrustEvidenceBudget {
    max_receipt_bytes: u64,
    max_receipt_count: u64,
    max_batch_artifacts: u64,
}

impl SupportTrustEvidenceBudget {
    pub fn new(
        max_receipt_bytes: u64,
        max_receipt_count: u64,
        max_batch_artifacts: u64,
    ) -> Result<Self, SupportTrustFailure> {
        if max_receipt_bytes == 0 || max_receipt_count == 0 || max_batch_artifacts == 0 {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "support trust evidence and batch budgets must be non-zero",
            ));
        }
        Ok(Self {
            max_receipt_bytes,
            max_receipt_count,
            max_batch_artifacts,
        })
    }

    pub fn admits(&self, receipt_bytes: u64, receipt_count: u64, batch_artifacts: u64) -> bool {
        receipt_bytes <= self.max_receipt_bytes
            && receipt_count <= self.max_receipt_count
            && batch_artifacts <= self.max_batch_artifacts
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustPerformancePlan {
    path_class: SupportTrustPathClass,
    density_class: SupportTrustDensityClass,
    access_path: SupportTrustAccessPath,
    allocation_scope: SupportTrustAllocationScope,
    expected_index_probes: u64,
    expected_receipt_reuse: u64,
    expected_allocation_count: u64,
    expected_clone_count: u64,
    clone_boundary: SupportTrustCloneBoundary,
}

impl SupportTrustPerformancePlan {
    pub fn new(
        path_class: SupportTrustPathClass,
        density_class: SupportTrustDensityClass,
        access_path: SupportTrustAccessPath,
        allocation_scope: SupportTrustAllocationScope,
        expected_index_probes: u64,
        expected_receipt_reuse: u64,
        expected_allocation_count: u64,
        expected_clone_count: u64,
        clone_boundary: SupportTrustCloneBoundary,
    ) -> Result<Self, SupportTrustFailure> {
        if density_class == SupportTrustDensityClass::StoreGlobalRejected
            || access_path == SupportTrustAccessPath::Rejected
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustAccessStructureDebt,
                SupportTrustRecoveryPosture::RebuildTrustCache,
                "store-global or rejected trust access paths cannot be admitted",
            ));
        }
        if path_class == SupportTrustPathClass::ForegroundResumeTrustPath
            && allocation_scope != SupportTrustAllocationScope::ForegroundScratch
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "foreground trust classification requires foreground scratch allocation",
            ));
        }
        if expected_clone_count == 0 && clone_boundary != SupportTrustCloneBoundary::NoClone {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "support trust clone boundary must be NoClone when clone count is zero",
            ));
        }
        if expected_clone_count > 0 && clone_boundary == SupportTrustCloneBoundary::NoClone {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "support trust clone count requires an explicit clone boundary reason",
            ));
        }
        Ok(Self {
            path_class,
            density_class,
            access_path,
            allocation_scope,
            expected_index_probes,
            expected_receipt_reuse,
            expected_allocation_count,
            expected_clone_count,
            clone_boundary,
        })
    }

    pub fn path_class(&self) -> SupportTrustPathClass {
        self.path_class
    }

    pub fn density_class(&self) -> SupportTrustDensityClass {
        self.density_class
    }

    pub fn access_path(&self) -> SupportTrustAccessPath {
        self.access_path
    }

    pub fn allocation_scope(&self) -> SupportTrustAllocationScope {
        self.allocation_scope
    }

    pub fn expected_index_probes(&self) -> u64 {
        self.expected_index_probes
    }

    pub fn expected_receipt_reuse(&self) -> u64 {
        self.expected_receipt_reuse
    }

    pub fn expected_allocation_count(&self) -> u64 {
        self.expected_allocation_count
    }

    pub fn expected_clone_count(&self) -> u64 {
        self.expected_clone_count
    }

    pub fn clone_boundary(&self) -> SupportTrustCloneBoundary {
        self.clone_boundary
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportTrustComplexityStatus {
    Verified,
    Debt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustComplexityContract {
    path_name: String,
    bound: String,
    status: SupportTrustComplexityStatus,
    max_index_probes: u64,
    max_receipt_loads: u64,
    max_global_scans: u64,
}

impl SupportTrustComplexityContract {
    pub fn verified(
        path_name: impl Into<String>,
        bound: impl Into<String>,
        max_index_probes: u64,
        max_receipt_loads: u64,
        max_global_scans: u64,
    ) -> Result<Self, SupportTrustFailure> {
        if max_global_scans != 0 {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
                SupportTrustRecoveryPosture::RerunCertification,
                "verified support trust contracts cannot admit global scans",
            ));
        }
        Ok(Self {
            path_name: path_name.into(),
            bound: bound.into(),
            status: SupportTrustComplexityStatus::Verified,
            max_index_probes,
            max_receipt_loads,
            max_global_scans,
        })
    }

    pub fn status(&self) -> SupportTrustComplexityStatus {
        self.status
    }

    pub fn max_global_scans(&self) -> u64 {
        self.max_global_scans
    }
}

fn require_non_empty(
    label: &'static str,
    value: impl Into<String>,
) -> Result<String, SupportTrustFailure> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustAccessStructureDebt,
            SupportTrustRecoveryPosture::RebuildTrustCache,
            format!("support trust access structure {label} must be non-empty"),
        ));
    }
    Ok(value)
}
