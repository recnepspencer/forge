use super::{LoopCycleReadSource, LoopCycleReadStageReceipt};
use crate::derived_topology::invalidation_plan::migrated_products::loop_cycles::LoopCycleMigrationError;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

pub struct LoopCycleReadStageExecutor;

impl LoopCycleReadStageExecutor {
    pub fn execute(
        selected_plan: &DerivedInvalidationSelectedPlan,
        read_source: LoopCycleReadSource,
    ) -> Result<LoopCycleReadStageReceipt, LoopCycleMigrationError> {
        LoopCycleReadStageReceipt::from_selected_plan_and_read_source(selected_plan, read_source)
    }
}
