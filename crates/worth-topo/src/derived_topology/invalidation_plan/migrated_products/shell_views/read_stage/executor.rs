use super::{ShellViewReadSource, ShellViewReadStageReceipt};
use crate::derived_topology::invalidation_plan::migrated_products::shell_views::ShellViewMigrationError;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

pub struct ShellViewReadStageExecutor;

impl ShellViewReadStageExecutor {
    pub fn execute(
        selected_plan: &DerivedInvalidationSelectedPlan,
        read_source: ShellViewReadSource,
    ) -> Result<ShellViewReadStageReceipt, ShellViewMigrationError> {
        ShellViewReadStageReceipt::from_selected_plan_and_read_source(selected_plan, read_source)
    }
}
