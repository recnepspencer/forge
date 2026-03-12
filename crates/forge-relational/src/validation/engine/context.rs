use crate::logic::runtime::{PartitionAccess, RelationalRuntime};
use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::{InvariantExecutionPoint, InvariantPlanContract};

use super::index_view::InvariantIndexView;
use super::metrics::InvariantMetrics;
use super::state_view::InvariantStateView;

pub struct InvariantExecutionContext<'runtime> {
    pub state: &'runtime dyn PartitionAccess,
    pub version_id: crate::identity::data::VersionId,
    pub current_version_id: crate::identity::data::VersionId,
    pub execution_point: InvariantExecutionPoint,
    pub plan_contract: Option<InvariantPlanContract>,
    pub merged_plan: Option<&'runtime MergedCommitPlan>,
    runtime: &'runtime RelationalRuntime,
}

impl<'runtime> InvariantExecutionContext<'runtime> {
    pub fn new(
        runtime: &'runtime RelationalRuntime,
        state: &'runtime dyn PartitionAccess,
        version_id: crate::identity::data::VersionId,
        execution_point: InvariantExecutionPoint,
        merged_plan: Option<&'runtime MergedCommitPlan>,
    ) -> Self {
        Self {
            state,
            version_id,
            current_version_id: runtime.current_version_id(),
            execution_point,
            plan_contract: merged_plan.map(InvariantPlanContract::from_merged_plan),
            merged_plan,
            runtime,
        }
    }

    pub fn state_view(&self) -> InvariantStateView<'runtime> {
        InvariantStateView::new(self.state, self.version_id)
    }

    pub fn runtime(&self) -> &'runtime RelationalRuntime {
        self.runtime
    }

    pub fn indexes(&self) -> InvariantIndexView<'runtime> {
        InvariantIndexView::new(self.runtime.index_access())
    }

    pub fn metrics(&self) -> InvariantMetrics<'runtime> {
        InvariantMetrics::new(self.runtime.performance_access())
    }
}
