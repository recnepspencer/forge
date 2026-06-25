use super::{TraversalViewsReadSource, TraversalViewsReadStageReceipt};
use crate::derived_topology::invalidation_plan::migrated_products::traversal_views::TraversalViewsMigrationError;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

pub struct TraversalViewsReadStageExecutor;

impl TraversalViewsReadStageExecutor {
    pub fn execute(
        selected_plan: &DerivedInvalidationSelectedPlan,
        read_source: TraversalViewsReadSource,
    ) -> Result<TraversalViewsReadStageReceipt, TraversalViewsMigrationError> {
        TraversalViewsReadStageReceipt::from_selected_plan_and_read_source(
            selected_plan,
            read_source,
        )
    }
}
