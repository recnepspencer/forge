use super::{MaterializedGraphReadSource, MaterializedGraphReadStageReceipt};
use crate::derived_topology::invalidation_plan::migrated_products::materialized_graph::MaterializedGraphMigrationError;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

#[derive(Debug, Clone, Copy, Default)]
pub struct MaterializedGraphReadStageExecutor;

impl MaterializedGraphReadStageExecutor {
    pub fn execute(
        selected_plan: &DerivedInvalidationSelectedPlan,
        read_source: MaterializedGraphReadSource,
    ) -> Result<MaterializedGraphReadStageReceipt, MaterializedGraphMigrationError> {
        MaterializedGraphReadStageReceipt::from_selected_plan_and_read_source(
            selected_plan,
            read_source,
        )
    }
}
