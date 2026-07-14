use worth_store_budgets::PreExecutionBudgetEnvelope;
use worth_store_contracts::WalRecordFamily;
use worth_store_security::StoreCurrentSecurityScopeWitnessSet;
use worth_store_wal::StoreWalRecordIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmCompactionAdmissionRequest<'a> {
    pub(super) security: &'a StoreCurrentSecurityScopeWitnessSet,
    pub(super) record_family: WalRecordFamily,
    pub(super) record_identity: StoreWalRecordIdentity,
    pub(super) budget: PreExecutionBudgetEnvelope,
}

impl<'a> LsmCompactionAdmissionRequest<'a> {
    pub const fn new(
        security: &'a StoreCurrentSecurityScopeWitnessSet,
        record_family: WalRecordFamily,
        record_identity: StoreWalRecordIdentity,
        budget: PreExecutionBudgetEnvelope,
    ) -> Self {
        Self {
            security,
            record_family,
            record_identity,
            budget,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmRunPublicationAdmissionRequest<'a> {
    pub(super) security: &'a StoreCurrentSecurityScopeWitnessSet,
    pub(super) record_family: WalRecordFamily,
    pub(super) record_identity: StoreWalRecordIdentity,
    pub(super) budget: PreExecutionBudgetEnvelope,
}

impl<'a> LsmRunPublicationAdmissionRequest<'a> {
    pub const fn new(
        security: &'a StoreCurrentSecurityScopeWitnessSet,
        record_family: WalRecordFamily,
        record_identity: StoreWalRecordIdentity,
        budget: PreExecutionBudgetEnvelope,
    ) -> Self {
        Self {
            security,
            record_family,
            record_identity,
            budget,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmReplayAdmissionRequest<'a> {
    pub(super) catalog: &'a crate::BootstrapCatalogReadAdmission,
    pub(super) security: &'a StoreCurrentSecurityScopeWitnessSet,
    pub(super) record_family: WalRecordFamily,
    pub(super) record_identity: StoreWalRecordIdentity,
    pub(super) source: &'a worth_store_lsm_authority::AdmittedLsmReplaySource,
    pub(super) budget: PreExecutionBudgetEnvelope,
}

impl<'a> LsmReplayAdmissionRequest<'a> {
    pub const fn new(
        catalog: &'a crate::BootstrapCatalogReadAdmission,
        security: &'a StoreCurrentSecurityScopeWitnessSet,
        record_family: WalRecordFamily,
        record_identity: StoreWalRecordIdentity,
        source: &'a worth_store_lsm_authority::AdmittedLsmReplaySource,
        budget: PreExecutionBudgetEnvelope,
    ) -> Self {
        Self {
            catalog,
            security,
            record_family,
            record_identity,
            source,
            budget,
        }
    }
}
