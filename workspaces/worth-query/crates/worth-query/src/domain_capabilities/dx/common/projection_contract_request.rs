use crate::projection_consumption::{
    ProjectMaterializedFacts, ProjectionConsumptionBindingContext, ProjectionConsumptionSource,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryProjectionContractRequest {
    source: ProjectionConsumptionSource,
    binding: ProjectionConsumptionBindingContext,
    requested_facts: ProjectMaterializedFacts,
}

impl WorthQueryProjectionContractRequest {
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
