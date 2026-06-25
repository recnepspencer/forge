use super::{RadialRingReadSource, RadialRingReadStageReceipt};
use crate::derived_topology::invalidation_plan::migrated_products::radial_rings::RadialRingMigrationError;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

pub struct RadialRingReadStageExecutor;

impl RadialRingReadStageExecutor {
    pub fn execute(
        selected_plan: &DerivedInvalidationSelectedPlan,
        read_source: RadialRingReadSource,
    ) -> Result<RadialRingReadStageReceipt, RadialRingMigrationError> {
        RadialRingReadStageReceipt::from_selected_plan_and_read_source(selected_plan, read_source)
    }
}
