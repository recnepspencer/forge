use crate::identity::data::VersionId;
use crate::runtime::RelationalRuntime;
use crate::transactions::data::MergedCommitPlan;
use crate::validation::engine::state_view::InvariantStateView;
use crate::validation::engine::{InvariantObservation, InvariantObservationKind};

use super::structural_views::{StructuralAspectStateView, StructuralRelationView};
use super::touched_scope_collection::collect_touched_structural_set;
use super::traversal::BoundedStructuralTraversal;
use crate::validation::data::{StructuralCountView, TouchedStructuralSet};
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct PreparedCustomInvariantScope {
    touched: Arc<TouchedStructuralSet>,
}

impl PreparedCustomInvariantScope {
    pub(crate) fn capture(
        observation: &InvariantObservation<'_>,
        version_id: VersionId,
        merged_plan: Option<&MergedCommitPlan>,
    ) -> Self {
        let state_view = InvariantStateView::new(
            observation.enforcement_partition_access(),
            observation.enforcement_version_id(version_id),
        );
        Self {
            touched: Arc::new(collect_touched_structural_set(&state_view, merged_plan)),
        }
    }

    pub(crate) fn retain_touched(&self) -> Arc<TouchedStructuralSet> {
        Arc::clone(&self.touched)
    }
}

pub struct CustomInvariantScopePlanner<'runtime> {
    observation_kind: InvariantObservationKind,
    version_id: VersionId,
    current_version_id: VersionId,
    touched: Arc<TouchedStructuralSet>,
    aspect_states: StructuralAspectStateView<'runtime>,
    committed_aspect_states: StructuralAspectStateView<'runtime>,
    relations: StructuralRelationView<'runtime>,
    counts: StructuralCountView,
    traversal: BoundedStructuralTraversal<'runtime>,
}

impl<'runtime> CustomInvariantScopePlanner<'runtime> {
    #[cfg(test)]
    pub(crate) fn new(
        runtime: &'runtime RelationalRuntime,
        observation: &'runtime InvariantObservation<'runtime>,
        version_id: VersionId,
        prepared_scope: &PreparedCustomInvariantScope,
    ) -> Self {
        Self::new_at_current_version(
            runtime,
            observation,
            version_id,
            runtime.current_version_id(),
            prepared_scope,
        )
    }

    pub(crate) fn new_at_current_version(
        runtime: &'runtime RelationalRuntime,
        observation: &'runtime InvariantObservation<'runtime>,
        version_id: VersionId,
        current_version_id: VersionId,
        prepared_scope: &PreparedCustomInvariantScope,
    ) -> Self {
        let state_view = InvariantStateView::new(
            observation.enforcement_partition_access(),
            observation.enforcement_version_id(version_id),
        );
        let committed_state_view =
            InvariantStateView::new(observation.committed_partition_access(), current_version_id);
        let touched = prepared_scope.retain_touched();
        let aspect_states = StructuralAspectStateView::new(state_view);
        let relations = StructuralRelationView::new(state_view);
        let counts = StructuralCountView::from_touched_scope(&touched);
        let traversal =
            BoundedStructuralTraversal::new(runtime.performance_access(), relations, &touched);
        Self {
            observation_kind: observation.kind(),
            version_id,
            current_version_id,
            touched,
            aspect_states,
            committed_aspect_states: StructuralAspectStateView::new(committed_state_view),
            relations,
            counts,
            traversal,
        }
    }

    pub fn observation_kind(&self) -> InvariantObservationKind {
        self.observation_kind
    }

    pub fn version_id(&self) -> VersionId {
        self.version_id
    }

    pub fn current_version_id(&self) -> VersionId {
        self.current_version_id
    }

    pub fn touched(&self) -> &TouchedStructuralSet {
        &self.touched
    }

    pub fn aspect_states(&self) -> StructuralAspectStateView<'runtime> {
        self.aspect_states
    }

    /// Read the immutable committed basis that the proposed view is checked against.
    pub fn committed_aspect_states(&self) -> StructuralAspectStateView<'runtime> {
        self.committed_aspect_states
    }

    pub fn relations(&self) -> StructuralRelationView<'runtime> {
        self.relations
    }

    pub fn counts(&self) -> StructuralCountView {
        self.counts
    }

    pub fn traversal(&self) -> &BoundedStructuralTraversal<'runtime> {
        &self.traversal
    }
}
