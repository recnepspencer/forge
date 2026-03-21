use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::MergedCommitPlan;

use super::index_view::InvariantIndexView;
use super::metrics::InvariantMetrics;
use super::observation::InvariantObservation;
use super::state_view::InvariantStateView;

pub struct InvariantExecutionContext<'runtime> {
    observation: InvariantObservation<'runtime>,
    version_id: crate::identity::data::VersionId,
    current_version_id: crate::identity::data::VersionId,
    merged_plan: Option<&'runtime MergedCommitPlan>,
    runtime: &'runtime RelationalRuntime,
}

impl<'runtime> InvariantExecutionContext<'runtime> {
    pub fn new(
        runtime: &'runtime RelationalRuntime,
        observation: InvariantObservation<'runtime>,
        version_id: crate::identity::data::VersionId,
        _execution_point: crate::validation::data::InvariantExecutionPoint,
        merged_plan: Option<&'runtime MergedCommitPlan>,
    ) -> Self {
        Self {
            observation,
            version_id,
            current_version_id: runtime.current_version_id(),
            merged_plan,
            runtime,
        }
    }

    pub fn state_view(&self) -> InvariantStateView<'_> {
        InvariantStateView::new(self.observation.partition_access(), self.version_id)
    }

    pub fn partition_access(&self) -> &dyn crate::storage::overlay::PartitionAccess {
        self.observation.partition_access()
    }

    pub fn current_version_id(&self) -> crate::identity::data::VersionId {
        self.current_version_id
    }

    pub fn merged_plan(&self) -> Option<&'runtime MergedCommitPlan> {
        self.merged_plan
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
