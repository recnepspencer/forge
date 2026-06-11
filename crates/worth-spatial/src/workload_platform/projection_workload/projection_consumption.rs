use crate::workload_platform::vocabulary::WorkloadStageIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumedWorkloadReceipt {
    projection_stage_identity: WorkloadStageIdentity,
    local_basis_identity: String,
    projected_entity_count: usize,
}

impl ProjectionConsumedWorkloadReceipt {
    pub(crate) fn new(
        projection_stage_identity: WorkloadStageIdentity,
        local_basis_identity: impl Into<String>,
        projected_entity_count: usize,
    ) -> Self {
        Self {
            projection_stage_identity,
            local_basis_identity: local_basis_identity.into(),
            projected_entity_count,
        }
    }

    pub fn projection_stage_identity(&self) -> &WorkloadStageIdentity {
        &self.projection_stage_identity
    }

    pub fn local_basis_identity(&self) -> &str {
        &self.local_basis_identity
    }

    pub fn projected_entity_count(&self) -> usize {
        self.projected_entity_count
    }
}
