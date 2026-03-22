use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::MergedCommitPlan;
use crate::identity::data::KindId;

use super::index_view::InvariantIndexView;
use super::metrics::InvariantMetrics;
use super::observation::InvariantObservation;
use super::request::{PreparedRelationIntegrityScope, PreparedRelationIntegrityScopes};
use super::state_view::InvariantStateView;

pub struct InvariantExecutionContext<'runtime> {
    observation: InvariantObservation<'runtime>,
    version_id: crate::identity::data::VersionId,
    current_version_id: crate::identity::data::VersionId,
    merged_plan: Option<&'runtime MergedCommitPlan>,
    runtime: &'runtime RelationalRuntime,
    relation_integrity_scopes: Option<PreparedRelationIntegrityScopes>,
}

impl<'runtime> InvariantExecutionContext<'runtime> {
    pub fn new(
        runtime: &'runtime RelationalRuntime,
        observation: InvariantObservation<'runtime>,
        version_id: crate::identity::data::VersionId,
        _execution_point: crate::validation::data::InvariantExecutionPoint,
        merged_plan: Option<&'runtime MergedCommitPlan>,
        relation_integrity_scopes: Option<PreparedRelationIntegrityScopes>,
    ) -> Self {
        Self {
            observation,
            version_id,
            current_version_id: runtime.current_version_id(),
            merged_plan,
            runtime,
            relation_integrity_scopes,
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

    pub fn relation_integrity_scope(
        &self,
        relation_kind_id: KindId,
    ) -> Option<&PreparedRelationIntegrityScope> {
        self.relation_integrity_scopes
            .as_ref()
            .and_then(|scopes| scopes.scope_for(relation_kind_id))
    }

    pub fn indexes(&self) -> InvariantIndexView<'runtime> {
        InvariantIndexView::new(self.runtime.index_access())
    }

    pub fn metrics(&self) -> InvariantMetrics<'runtime> {
        InvariantMetrics::new(self.runtime.performance_access())
    }
}
