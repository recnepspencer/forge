use crate::identity::data::VersionId;
use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::MergedCommitPlan;
use crate::validation::engine::state_view::InvariantStateView;
use crate::validation::engine::{InvariantObservation, InvariantObservationKind};

use super::structural_views::{StructuralAspectStateView, StructuralRelationView};
use super::touched_scope::{
    collect_touched_structural_set, StructuralCountView, TouchedStructuralSet,
};
use super::traversal::BoundedStructuralTraversal;

pub struct CustomInvariantScopePlanner<'runtime> {
    observation_kind: InvariantObservationKind,
    version_id: VersionId,
    current_version_id: VersionId,
    touched: TouchedStructuralSet,
    aspect_states: StructuralAspectStateView<'runtime>,
    relations: StructuralRelationView<'runtime>,
    counts: StructuralCountView,
    traversal: BoundedStructuralTraversal<'runtime>,
}

impl<'runtime> CustomInvariantScopePlanner<'runtime> {
    pub(crate) fn new(
        runtime: &'runtime RelationalRuntime,
        observation: &'runtime InvariantObservation<'runtime>,
        version_id: VersionId,
        merged_plan: Option<&'runtime MergedCommitPlan>,
    ) -> Self {
        let state_view = InvariantStateView::new(observation.partition_access(), version_id);
        let touched = collect_touched_structural_set(runtime, &state_view, merged_plan);
        let aspect_states = StructuralAspectStateView::new(state_view);
        let relations = StructuralRelationView::new(runtime, state_view);
        let counts = StructuralCountView::from_touched_scope(&touched);
        let traversal = BoundedStructuralTraversal::new(runtime, relations, &touched);
        Self {
            observation_kind: observation.kind(),
            version_id,
            current_version_id: runtime.current_version_id(),
            touched,
            aspect_states,
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
