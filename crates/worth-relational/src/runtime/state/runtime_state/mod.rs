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

use std::collections::BTreeMap;

use crate::runtime::RelationalRuntimeConfig;
use crate::storage::overlay::PartitionState;

use super::{
    CommitStrategiesSubsystem, DurabilitySubsystem, HistorySubsystem, IndexingSubsystem,
    LineageSubsystem, PublicationSubsystem, RecordIdentitySubsystem, RuntimeServices,
    SchemaContractRuntimeSubsystem, VisibilitySubsystem,
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

#[derive(Debug)]
pub struct RelationalRuntime {
    pub(crate) config: RelationalRuntimeConfig,
    pub(crate) schema_contract_runtime: SchemaContractRuntimeSubsystem,
    pub(crate) commit_strategies: CommitStrategiesSubsystem,
    pub(crate) partitions: BTreeMap<crate::identity::data::PartitionId, PartitionState>,
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

impl Drop for RelationalRuntime {
    fn drop(&mut self) {
        self.owner_lifecycle.close();
        self.publication_owner.close();
        crate::indexes::purge_index_query_scratch_hints(self.runtime_instance_id());
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
