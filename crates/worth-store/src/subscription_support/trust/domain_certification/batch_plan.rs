use crate::subscription_support::trust::certification::{
    SupportCertificationBatchScope, SupportCertificationBatchScopeKind,
};
use crate::subscription_support::trust::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use crate::subscription_support::trust::performance::{
    SupportTrustAllocationScope, SupportTrustDensityClass, SupportTrustPathClass,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportDomainCertificationBatchPlan {
    scenario_width: u64,
    family_role_row_width: u64,
    batch_scope: SupportCertificationBatchScope,
    max_scenario_rows: u64,
}

impl SupportDomainCertificationBatchPlan {
    pub fn new(
        scenario_width: u64,
        family_role_row_width: u64,
        batch_scope: SupportCertificationBatchScope,
        max_scenario_rows: u64,
    ) -> Result<Self, SupportTrustFailure> {
        if scenario_width == 0 || family_role_row_width == 0 || max_scenario_rows == 0 {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded,
                SupportTrustRecoveryPosture::RerunCertification,
                "domain certification plans require non-zero scenario, family-role, and budget widths",
            ));
        }
        if batch_scope.scope_kind() != SupportCertificationBatchScopeKind::DomainScenarioLocal {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustAccessStructureDebt,
                SupportTrustRecoveryPosture::RebuildTrustCache,
                "domain certification plans require domain-scenario-local batch scope",
            ));
        }
        if batch_scope.density_class() != SupportTrustDensityClass::DomainScenarioLocal
            || batch_scope.path_class() != SupportTrustPathClass::DomainCertificationPath
            || batch_scope.allocation_scope() != SupportTrustAllocationScope::DomainCertification
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustAccessStructureDebt,
                SupportTrustRecoveryPosture::RebuildTrustCache,
                "domain certification plans require domain density, path, and allocation scope",
            ));
        }
        if scenario_width > max_scenario_rows {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded,
                SupportTrustRecoveryPosture::RerunCertification,
                "domain certification scenario width exceeds the declared scenario budget",
            ));
        }
        Ok(Self {
            scenario_width,
            family_role_row_width,
            batch_scope,
            max_scenario_rows,
        })
    }

    pub fn scenario_width(&self) -> u64 {
        self.scenario_width
    }

    pub fn family_role_row_width(&self) -> u64 {
        self.family_role_row_width
    }

    pub fn batch_scope(&self) -> SupportCertificationBatchScope {
        self.batch_scope
    }
}
