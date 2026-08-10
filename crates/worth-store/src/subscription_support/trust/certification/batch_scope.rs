use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::super::performance::{
    SupportTrustAllocationScope, SupportTrustDensityClass, SupportTrustPathClass,
};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[allow(dead_code)]
pub enum SupportCertificationBatchScopeKind {
    FamilyLocal,
    BasisLocal,
    CertificationScopeLocal,
    DomainScenarioLocal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SupportCertificationBatchScope {
    scope_kind: SupportCertificationBatchScopeKind,
    density_class: SupportTrustDensityClass,
    path_class: SupportTrustPathClass,
    allocation_scope: SupportTrustAllocationScope,
    row_count: u64,
    expected_index_probes: u64,
    expected_receipt_reuse_count: u64,
    expected_allocation_count: u64,
}

impl SupportCertificationBatchScope {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        scope_kind: SupportCertificationBatchScopeKind,
        density_class: SupportTrustDensityClass,
        path_class: SupportTrustPathClass,
        allocation_scope: SupportTrustAllocationScope,
        row_count: u64,
        expected_index_probes: u64,
        expected_receipt_reuse_count: u64,
        expected_allocation_count: u64,
    ) -> Result<Self, SupportTrustFailure> {
        if row_count == 0 {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "support trust certification batch scopes require at least one row",
            ));
        }
        let density_matches_scope = matches!(
            (scope_kind, density_class),
            (
                SupportCertificationBatchScopeKind::FamilyLocal,
                SupportTrustDensityClass::FamilyLocal
            ) | (
                SupportCertificationBatchScopeKind::BasisLocal,
                SupportTrustDensityClass::BasisLocal
            ) | (
                SupportCertificationBatchScopeKind::CertificationScopeLocal,
                SupportTrustDensityClass::CertificationScopeLocal
            ) | (
                SupportCertificationBatchScopeKind::DomainScenarioLocal,
                SupportTrustDensityClass::DomainScenarioLocal
            )
        );
        if !density_matches_scope
            || path_class == SupportTrustPathClass::ForegroundResumeTrustPath
            || allocation_scope == SupportTrustAllocationScope::ForegroundScratch
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustAccessStructureDebt,
                SupportTrustRecoveryPosture::RebuildTrustCache,
                "support trust certification batch scopes must use matching bounded batch density, path, and allocation",
            ));
        }
        Ok(Self {
            scope_kind,
            density_class,
            path_class,
            allocation_scope,
            row_count,
            expected_index_probes,
            expected_receipt_reuse_count,
            expected_allocation_count,
        })
    }

    pub fn row_count(&self) -> u64 {
        self.row_count
    }

    pub fn scope_kind(&self) -> SupportCertificationBatchScopeKind {
        self.scope_kind
    }

    pub fn density_class(&self) -> SupportTrustDensityClass {
        self.density_class
    }

    pub fn path_class(&self) -> SupportTrustPathClass {
        self.path_class
    }

    pub fn allocation_scope(&self) -> SupportTrustAllocationScope {
        self.allocation_scope
    }

    pub fn expected_receipt_reuse_count(&self) -> u64 {
        self.expected_receipt_reuse_count
    }

    pub fn expected_index_probes(&self) -> u64 {
        self.expected_index_probes
    }

    pub fn expected_allocation_count(&self) -> u64 {
        self.expected_allocation_count
    }
}
