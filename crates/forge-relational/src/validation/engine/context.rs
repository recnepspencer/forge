use crate::identity::data::KindId;
use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::MergedCommitPlan;

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

    pub fn relation_integrity_scope(
        &self,
        relation_kind_id: KindId,
    ) -> Option<&PreparedRelationIntegrityScope> {
        self.relation_integrity_scopes
            .as_ref()
            .and_then(|scopes| scopes.scope_for(relation_kind_id))
    }

    pub(crate) fn visible_unmasked_entity_record(
        &self,
        entity_id: crate::identity::data::EntityId,
    ) -> Option<crate::storage::data::EntityReadRecord> {
        self.runtime
            .read_truth()
            .project_version(self.version_id)
            .unmasked_entity_record(entity_id)
    }

    pub fn metrics(&self) -> InvariantMetrics<'runtime> {
        InvariantMetrics::new(self.runtime.performance_access())
    }

    pub fn relation_integrity_scope_budget(
        &self,
    ) -> &crate::config::data::RelationIntegrityScopeBudget {
        &self
            .runtime
            .config
            .execution
            .relation_integrity_scope_budget
    }
}
