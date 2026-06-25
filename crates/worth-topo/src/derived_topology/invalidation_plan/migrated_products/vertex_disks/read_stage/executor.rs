use super::{VertexDiskReadSource, VertexDiskReadStageReceipt};
use crate::derived_topology::invalidation_plan::migrated_products::vertex_disks::VertexDiskMigrationError;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

pub struct VertexDiskReadStageExecutor;

impl VertexDiskReadStageExecutor {
    pub fn execute(
        selected_plan: &DerivedInvalidationSelectedPlan,
        read_source: VertexDiskReadSource,
    ) -> Result<VertexDiskReadStageReceipt, VertexDiskMigrationError> {
        VertexDiskReadStageReceipt::from_selected_plan_and_read_source(selected_plan, read_source)
    }
}
