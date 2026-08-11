use worth_store::physical_runtime::CompletedPhysicalRecoveryFreshReopen;
use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_recovery_physics::{PhysicalSourceSelection, RecoveryPlanningCounters};

use crate::entry::{
    PhysicalRecoveryPublicationCounters, PhysicalRecoveryPublicationSettlementLedger,
    PhysicalRecoveryReopenCounters, PhysicalRecoveryStagingCounters,
    PhysicalRecoveryStagingSettlementLedger,
};
use crate::handoff::RecoveryOperationFateSet;

use super::{NamespaceDurableState, RecoveryPublicationExpectation};

pub struct ReopenedPhysicalRecovery {
    pub(crate) state: NamespaceDurableState,
    pub(crate) expectation: RecoveryPublicationExpectation,
    pub(crate) publication_counters: PhysicalRecoveryPublicationCounters,
    pub(crate) publication_settlement: PhysicalRecoveryPublicationSettlementLedger,
    pub(crate) reopened: CompletedPhysicalRecoveryFreshReopen,
    pub(crate) reopen_counters: PhysicalRecoveryReopenCounters,
}

impl ReopenedPhysicalRecovery {
    pub(crate) const fn new(
        state: NamespaceDurableState,
        expectation: RecoveryPublicationExpectation,
        publication_counters: PhysicalRecoveryPublicationCounters,
        publication_settlement: PhysicalRecoveryPublicationSettlementLedger,
        reopened: CompletedPhysicalRecoveryFreshReopen,
        reopen_counters: PhysicalRecoveryReopenCounters,
    ) -> Self {
        Self {
            state,
            expectation,
            publication_counters,
            publication_settlement,
            reopened,
            reopen_counters,
        }
    }

    pub fn store_identity(&self) -> StableStoreIdentity {
        self.state.authority.media.store_identity()
    }
    pub const fn recovered_root(
        &self,
    ) -> &worth_store_physical_format::DurablePhysicalRootManifest {
        self.reopened.root()
    }
    pub const fn reopen_counters(&self) -> PhysicalRecoveryReopenCounters {
        self.reopen_counters
    }
    pub const fn fresh_reopen(&self) -> &CompletedPhysicalRecoveryFreshReopen {
        &self.reopened
    }
    pub const fn publication_expectation(&self) -> &RecoveryPublicationExpectation {
        &self.expectation
    }
    pub const fn publication_counters(&self) -> PhysicalRecoveryPublicationCounters {
        self.publication_counters
    }
    pub const fn publication_settlement(&self) -> &PhysicalRecoveryPublicationSettlementLedger {
        &self.publication_settlement
    }
    pub const fn operation_fates(&self) -> &RecoveryOperationFateSet {
        &self.state.fates
    }
    pub const fn selected_sources(&self) -> &PhysicalSourceSelection {
        &self.state.selection
    }
    pub const fn planning_counters(&self) -> RecoveryPlanningCounters {
        self.state.planning_counters
    }
    pub const fn staging_counters(&self) -> PhysicalRecoveryStagingCounters {
        self.state.staging_counters
    }
    pub const fn staging_settlements(&self) -> &PhysicalRecoveryStagingSettlementLedger {
        &self.state.staging_settlements
    }
    pub fn is_quiescent(&self) -> bool {
        self.state.coordination.is_ready()
    }

    /// Consumes the recovery-only runtime and returns the narrow Store-owned
    /// recovered-runtime handoff after publication and fresh reopen close.
    pub fn finish(self) -> crate::entry::PhysicalRecoveryOutcome {
        crate::orchestration::finish_recovery(self)
    }
}
