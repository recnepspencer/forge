/// Complete-successor authority projected into execution-plan lowering without
/// pretending one changed receipt represents every carried catalog row.
#[derive(Clone, Debug)]
pub(crate) struct UiAllocationCatalogSuccessorLoweringInput {
    projection: crate::runtime::allocation_planning::WorthUiAllocationPlanningProjection,
    allocation_identity_digest: u64,
}

impl UiAllocationCatalogSuccessorLoweringInput {
    pub(crate) fn seal(
        pending: &crate::runtime::WorthUiPendingActivation,
        allocation_identity_digest: u64,
    ) -> Self {
        Self {
            projection: pending.allocation_planning_projection().clone(),
            allocation_identity_digest,
        }
    }

    pub(crate) fn projection(
        &self,
    ) -> &crate::runtime::allocation_planning::WorthUiAllocationPlanningProjection {
        &self.projection
    }

    pub(crate) fn allocation_identity_digest(&self) -> u64 {
        self.allocation_identity_digest
    }
}
