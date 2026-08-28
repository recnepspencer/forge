use crate::identity::data::KindId;
use crate::runtime::{
    RelationalPreparationRuntime, RelationalRuntime, RuntimeInstrumentation,
    SchemaContractRuntimeSubsystem,
};

/// Read-only validation projection shared by the runtime and preparation port.
#[derive(Clone, Copy)]
pub(crate) struct InvariantRuntimeView<'a> {
    pub(crate) config: &'a crate::runtime::RelationalRuntimeConfig,
    pub(crate) schema_contract_runtime: &'a SchemaContractRuntimeSubsystem,
    instrumentation: &'a RuntimeInstrumentation,
    #[cfg(test)]
    current_version_id: crate::identity::data::VersionId,
    entity_count: usize,
    relation_count: usize,
    version_depth: usize,
    snapshot_pressure: bool,
}

impl<'a> InvariantRuntimeView<'a> {
    pub(crate) fn from_runtime(runtime: &'a RelationalRuntime) -> Self {
        Self {
            config: &runtime.config,
            schema_contract_runtime: &runtime.schema_contract_runtime,
            instrumentation: &runtime.services.instrumentation,
            #[cfg(test)]
            current_version_id: runtime.current_version_id(),
            entity_count: runtime.storage_access().entity_slot_count(),
            relation_count: runtime.storage_access().relation_slot_count(),
            version_depth: runtime.history().commit_count(),
            snapshot_pressure: runtime.visibility.active_snapshot_count() > 10,
        }
    }

    pub(crate) fn from_preparation_for_state(
        runtime: &'a RelationalPreparationRuntime,
        state: &crate::branch::RelationalBranchRootState,
    ) -> Self {
        Self {
            config: &runtime.config,
            schema_contract_runtime: &runtime.schema_contract_runtime,
            instrumentation: &runtime.services.instrumentation,
            #[cfg(test)]
            current_version_id: runtime.current_version_id(),
            entity_count: state.entity_slot_count(),
            relation_count: state.relation_slot_count(),
            version_depth: runtime.current_version_id().0 as usize,
            snapshot_pressure: runtime.published_snapshot_count() > 10,
        }
    }

    pub(crate) fn performance_access(&self) -> crate::performance::PerformanceAccess<'a> {
        crate::performance::PerformanceAccess::from_instrumentation(self.instrumentation)
    }

    #[cfg(test)]
    pub(crate) const fn current_version_id(&self) -> crate::identity::data::VersionId {
        self.current_version_id
    }

    pub(crate) fn entity_aspect_plan(
        &self,
        kind_id: KindId,
    ) -> Option<&crate::schema::data::LoweredAspectContractPlan> {
        self.schema_contract_runtime
            .aspect_contract_plans
            .entity_plans
            .get(&kind_id)
    }

    pub(crate) const fn invariant_scale_inputs(&self) -> (usize, usize, usize, bool) {
        (
            self.entity_count,
            self.relation_count,
            self.version_depth,
            self.snapshot_pressure,
        )
    }
}
