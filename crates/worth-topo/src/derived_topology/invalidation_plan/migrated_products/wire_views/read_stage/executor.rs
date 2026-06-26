use super::{WireViewReadSource, WireViewReadStageReceipt};
use crate::derived_topology::invalidation_plan::migrated_products::wire_views::WireViewMigrationError;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

pub struct WireViewReadStageExecutor;

impl WireViewReadStageExecutor {
    pub fn execute(
        selected_plan: &DerivedInvalidationSelectedPlan,
        read_source: WireViewReadSource,
    ) -> Result<WireViewReadStageReceipt, WireViewMigrationError> {
        WireViewReadStageReceipt::from_selected_plan_and_read_source(selected_plan, read_source)
    }
}
