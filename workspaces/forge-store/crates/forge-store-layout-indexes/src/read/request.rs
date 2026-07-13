use forge_store_budgets::PreExecutionBudgetEnvelope;
use forge_store_contracts::WalRecordFamily;
use forge_store_physical_format::{PhysicalPageId, PhysicalRecordSlot, PhysicalSegmentId};
use forge_store_security::StoreCurrentSecurityScopeWitnessSet;
use forge_store_wal::StoreWalRecordIdentity;

#[derive(Debug)]
pub struct WalLookupRequest<'a> {
    pub(super) catalog: &'a crate::BootstrapCatalogReadAdmission,
    pub(super) security: &'a StoreCurrentSecurityScopeWitnessSet,
    pub(super) record_family: WalRecordFamily,
    pub(super) record_identity: StoreWalRecordIdentity,
    pub(super) probe_sequence: u64,
    pub(super) budget: PreExecutionBudgetEnvelope,
    pub(super) source: crate::BaselineLsmLookupSource,
}

impl<'a> WalLookupRequest<'a> {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        catalog: &'a crate::BootstrapCatalogReadAdmission,
        security: &'a StoreCurrentSecurityScopeWitnessSet,
        record_family: WalRecordFamily,
        record_identity: StoreWalRecordIdentity,
        probe_sequence: u64,
        budget: PreExecutionBudgetEnvelope,
        source: crate::BaselineLsmLookupSource,
    ) -> Self {
        Self {
            catalog,
            security,
            record_family,
            record_identity,
            probe_sequence,
            budget,
            source,
        }
    }
}

#[derive(Debug)]
pub struct PageLookupRequest<'a> {
    pub(super) catalog: &'a crate::BootstrapCatalogReadAdmission,
    pub(super) security: &'a StoreCurrentSecurityScopeWitnessSet,
    pub(super) segment: PhysicalSegmentId,
    pub(super) page: PhysicalPageId,
    pub(super) probe_slot: PhysicalRecordSlot,
    pub(super) kind: PageLookupKind,
    pub(super) budget: PreExecutionBudgetEnvelope,
    pub(super) source: crate::BaselineBTreeReadSource,
}

impl<'a> PageLookupRequest<'a> {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        catalog: &'a crate::BootstrapCatalogReadAdmission,
        security: &'a StoreCurrentSecurityScopeWitnessSet,
        segment: PhysicalSegmentId,
        page: PhysicalPageId,
        probe_slot: PhysicalRecordSlot,
        budget: PreExecutionBudgetEnvelope,
        source: crate::BaselineBTreeReadSource,
    ) -> Self {
        Self::with_kind(
            catalog,
            security,
            segment,
            page,
            probe_slot,
            PageLookupKind::Point,
            budget,
            source,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub const fn range(
        catalog: &'a crate::BootstrapCatalogReadAdmission,
        security: &'a StoreCurrentSecurityScopeWitnessSet,
        segment: PhysicalSegmentId,
        page: PhysicalPageId,
        probe_slot: PhysicalRecordSlot,
        budget: PreExecutionBudgetEnvelope,
        source: crate::BaselineBTreeReadSource,
    ) -> Self {
        Self::with_kind(
            catalog,
            security,
            segment,
            page,
            probe_slot,
            PageLookupKind::Range,
            budget,
            source,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub const fn prefix(
        catalog: &'a crate::BootstrapCatalogReadAdmission,
        security: &'a StoreCurrentSecurityScopeWitnessSet,
        segment: PhysicalSegmentId,
        page: PhysicalPageId,
        probe_slot: PhysicalRecordSlot,
        budget: PreExecutionBudgetEnvelope,
        source: crate::BaselineBTreeReadSource,
    ) -> Self {
        Self::with_kind(
            catalog,
            security,
            segment,
            page,
            probe_slot,
            PageLookupKind::Prefix,
            budget,
            source,
        )
    }

    #[allow(clippy::too_many_arguments)]
    const fn with_kind(
        catalog: &'a crate::BootstrapCatalogReadAdmission,
        security: &'a StoreCurrentSecurityScopeWitnessSet,
        segment: PhysicalSegmentId,
        page: PhysicalPageId,
        probe_slot: PhysicalRecordSlot,
        kind: PageLookupKind,
        budget: PreExecutionBudgetEnvelope,
        source: crate::BaselineBTreeReadSource,
    ) -> Self {
        Self {
            catalog,
            security,
            segment,
            page,
            probe_slot,
            kind,
            budget,
            source,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PageLookupKind {
    Point,
    Range,
    Prefix,
}
