use crate::projection_consumption::{
    ProjectMaterializedFacts, ProjectionConsumptionBindingContext, ProjectionConsumptionSource,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryProjectionContractRequest {
    source: ProjectionConsumptionSource,
    binding: ProjectionConsumptionBindingContext,
    requested_facts: ProjectMaterializedFacts,
}

impl ForgeQueryProjectionContractRequest {
    pub fn new(
        source: ProjectionConsumptionSource,
        binding: ProjectionConsumptionBindingContext,
        requested_facts: ProjectMaterializedFacts,
    ) -> Self {
        Self {
            source,
            binding,
            requested_facts,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ProjectionConsumptionSource,
        ProjectionConsumptionBindingContext,
        ProjectMaterializedFacts,
    ) {
        (self.source, self.binding, self.requested_facts)
    }
}
