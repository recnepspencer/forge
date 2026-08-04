use worth_query_admission::facade::graph_obligation::WorthQueryGraphWorkPlanIdentity;
use worth_query_installation::facade::ApplicationSchemaBindingIdentity;
use worth_relational::facade::runtime::{RelationalExecutionBasisIdentity, RelationalRuntime};

use crate::domain_computation::primary_graph::{
    WorthQueryPrimaryGraphIntegrationHandle, WorthQueryPrimaryGraphLayout,
};

use super::WorthQueryGraphWorkSessionIdentity;

/// The only application-query path to the primary graph's Relational runtime.
pub(in crate::domain_computation) struct WorthQueryGraphReadOwnerPort {
    binding: ApplicationSchemaBindingIdentity,
    graph: WorthQueryPrimaryGraphIntegrationHandle,
}

impl WorthQueryGraphReadOwnerPort {
    pub(in crate::domain_computation) fn new(
        binding: ApplicationSchemaBindingIdentity,
        graph: WorthQueryPrimaryGraphIntegrationHandle,
    ) -> Self {
        Self { binding, graph }
    }

    pub(super) fn execute<T>(
        &self,
        binding: &ApplicationSchemaBindingIdentity,
        read: impl FnOnce(&mut RelationalRuntime, &WorthQueryPrimaryGraphLayout) -> T,
    ) -> Result<T, WorthQueryGraphReadOwnerPortDenial> {
        if &self.binding != binding {
            return Err(WorthQueryGraphReadOwnerPortDenial::ForeignGraph);
        }
        Ok(self.graph.with_query_runtime_mut(read))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum WorthQueryGraphReadOwnerPortDenial {
    ForeignGraph,
}

/// Move-only proof that this exact session performed a read at its retained basis.
pub(in crate::domain_computation) struct WorthQuerySessionGraphReadProof {
    pub(super) session: WorthQueryGraphWorkSessionIdentity,
    pub(super) plan: WorthQueryGraphWorkPlanIdentity,
    pub(super) basis: RelationalExecutionBasisIdentity,
}

impl WorthQuerySessionGraphReadProof {
    pub(super) fn new(
        session: WorthQueryGraphWorkSessionIdentity,
        plan: WorthQueryGraphWorkPlanIdentity,
        basis: RelationalExecutionBasisIdentity,
    ) -> Self {
        Self {
            session,
            plan,
            basis,
        }
    }
}
