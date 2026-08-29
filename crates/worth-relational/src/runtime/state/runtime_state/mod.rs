mod branch_authority;
mod core_access;
mod merge_authority;
mod owner_lifecycle;
mod preparation_configuration;
mod preparation_runtime;
mod publication_lifecycle;
mod publication_settlement_registry;
#[cfg(test)]
mod test_support;

use crate::runtime::RelationalRuntimeConfig;

use super::{
    CommitStrategiesSubsystem, DurabilitySubsystem, HistorySubsystem, IndexingSubsystem,
    LineageSubsystem, PublicationSubsystem, RecordIdentitySubsystem, RuntimeServices,
    SchemaContractRuntimeSubsystem, StorageSubsystem, VisibilitySubsystem,
};

pub(in crate::runtime) use owner_lifecycle::RelationalRuntimeOwner;
pub(crate) use owner_lifecycle::{
    AdmittedRelationalRuntimeOperation, RelationalRuntimeOwnerBinding,
};
pub(crate) use preparation_configuration::RelationalPreparationConfigurationBinding;
pub(in crate::runtime) use preparation_configuration::RelationalPreparationConfigurationOwner;
pub(crate) use preparation_runtime::RelationalPreparationOwnerBinding;
pub(crate) use preparation_runtime::RelationalPreparationRuntime;
pub(in crate::runtime) use publication_lifecycle::RelationalRuntimePublicationOwner;
pub(crate) use publication_lifecycle::{
    RelationalCandidateRegistrationDenial, RelationalRuntimePublicationBinding,
};
pub(crate) use publication_settlement_registry::{
    DeferredRelationalSettlement, PendingRelationalPublicationSettlement,
    PerformedRelationalSettlement, RelationalPendingSettlementReservation,
    RelationalPublicationSettlementRegistry, RelationalSettlementClaim,
    RelationalSettlementReservationDenial, ReservedRelationalSettlement,
};

/// Every subsystem one relational runtime owns.
///
/// The state is held by shared ownership so an independently borrowable owner
/// service can address this exact authority without copying it and without a
/// runtime-wide lock. Only the owning [`RelationalRuntime`] keeps a lasting
/// strong handle; a service binds weakly and answers owner-unavailable once the
/// owner is gone.
#[derive(Debug)]
pub struct RelationalRuntimeState {
    pub(crate) config: RelationalRuntimeConfig,
    pub(crate) schema_contract_runtime: SchemaContractRuntimeSubsystem,
    pub(crate) commit_strategies: CommitStrategiesSubsystem,
    pub(crate) partitions: StorageSubsystem,
    pub(crate) visibility: VisibilitySubsystem,
    pub(crate) publication: PublicationSubsystem,
    pub(crate) history: HistorySubsystem,
    pub(crate) indexes: IndexingSubsystem,
    pub(crate) lineage: LineageSubsystem,
    pub(crate) durability: DurabilitySubsystem,
    pub(crate) record_identity: RecordIdentitySubsystem,
    pub(crate) services: RuntimeServices,
    pub(in crate::runtime) preparation_configuration: RelationalPreparationConfigurationOwner,
    pub(in crate::runtime) owner_lifecycle: RelationalRuntimeOwner,
    pub(in crate::runtime) publication_owner: RelationalRuntimePublicationOwner,
}

/// The owner handle callers hold.
///
/// It is the sole lasting strong owner of its state, so dropping it closes the
/// runtime's lifecycles and invalidates every service bound to that state. A
/// service that is mid-operation holds a transient strong handle, which keeps
/// the state alive exactly until that operation returns.
#[derive(Debug)]
pub struct RelationalRuntime {
    state: std::sync::Arc<RelationalRuntimeState>,
}

impl RelationalRuntime {
    pub(in crate::runtime) fn from_state(state: RelationalRuntimeState) -> Self {
        Self {
            state: std::sync::Arc::new(state),
        }
    }

    /// Rebind an owner handle onto state a service already holds.
    ///
    /// This does not create a second owner: the state's own lifecycles close
    /// exactly once, when the last handle to it goes away.
    pub(crate) fn from_shared(state: std::sync::Arc<RelationalRuntimeState>) -> Self {
        Self { state }
    }

    /// A weak binding to this runtime's exact state, for a service that must
    /// outlive its owner and deny once the owner is gone.
    pub(crate) fn state_binding(&self) -> std::sync::Weak<RelationalRuntimeState> {
        std::sync::Arc::downgrade(&self.state)
    }
}

impl std::ops::Deref for RelationalRuntime {
    type Target = RelationalRuntimeState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl std::ops::DerefMut for RelationalRuntime {
    /// Exclusive access to the owned state.
    ///
    /// Only reconfiguration, construction, and recovery need this. It is
    /// reachable because services bind weakly, so the owner is the only lasting
    /// strong handle; an operation in flight through a service holds the state
    /// transiently and no exclusive runtime borrow can exist at that moment.
    fn deref_mut(&mut self) -> &mut Self::Target {
        std::sync::Arc::get_mut(&mut self.state)
            .expect("the runtime owner is the only lasting strong handle to its state")
    }
}

impl Drop for RelationalRuntimeState {
    fn drop(&mut self) {
        self.owner_lifecycle.close();
        self.publication_owner.close();
        crate::indexes::purge_index_query_scratch_hints(self.services.runtime_instance_id());
    }
}

impl RelationalRuntime {
    pub(crate) fn synchronize_preparation_configuration(&self) {
        self.preparation_configuration
            .synchronize(&self.config, &self.schema_contract_runtime);
    }

    pub(crate) fn preparation_configuration_binding(
        &self,
    ) -> RelationalPreparationConfigurationBinding {
        self.preparation_configuration.binding()
    }

    pub(crate) fn preparation_runtime_snapshot(&self) -> RelationalPreparationRuntime {
        RelationalPreparationOwnerBinding::from_runtime(self).runtime_snapshot()
    }

    pub(crate) fn owner_binding(&self) -> RelationalRuntimeOwnerBinding {
        self.owner_lifecycle.binding()
    }

    pub(crate) fn publication_binding(&self) -> RelationalRuntimePublicationBinding {
        self.publication_owner.binding()
    }
}
