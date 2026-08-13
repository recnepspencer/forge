use crate::identity::data::VersionId;
use crate::runtime::RelationalRuntime;
use crate::validation::engine::state_view::InvariantStateView;
use crate::validation::engine::{InvariantObservation, InvariantObservationKind};

use super::structural_views::{StructuralAspectStateView, StructuralRelationView};
use super::traversal::{BoundedStructuralTraversal, CustomInvariantTraversalSummary};
use crate::validation::data::{
    CustomInvariantTouchedSummary, StructuralCountView, TouchedStructuralSet,
};
use std::sync::Arc;

pub struct CustomInvariantExecutionContext<'runtime> {
    runtime: &'runtime RelationalRuntime,
    observation_kind: InvariantObservationKind,
    version_id: VersionId,
    current_version_id: VersionId,
    touched: Arc<TouchedStructuralSet>,
    aspect_states: StructuralAspectStateView<'runtime>,
    relations: StructuralRelationView<'runtime>,
    counts: StructuralCountView,
    traversal: BoundedStructuralTraversal<'runtime>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CustomInvariantProvenance {
    pub observation_kind: InvariantObservationKind,
    pub version_id: VersionId,
    pub current_version_id: VersionId,
    pub touched: CustomInvariantTouchedSummary,
    pub counts: StructuralCountView,
    pub traversal: CustomInvariantTraversalSummary,
}

impl<'runtime> CustomInvariantExecutionContext<'runtime> {
    pub(crate) fn new(
        runtime: &'runtime RelationalRuntime,
        observation: &'runtime InvariantObservation<'runtime>,
        version_id: VersionId,
        prepared_scope: &super::scope_planner::PreparedCustomInvariantScope,
    ) -> Self {
        let state_view = InvariantStateView::new(observation.partition_access(), version_id);
        let touched = prepared_scope.retain_touched();
        let aspect_states = StructuralAspectStateView::new(state_view);
        let relations = StructuralRelationView::new(runtime, state_view);
        let counts = StructuralCountView::from_touched_scope(&touched);
        let traversal = BoundedStructuralTraversal::new(runtime, relations, &touched);
        Self {
            runtime,
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

    pub(crate) fn runtime(&self) -> &'runtime RelationalRuntime {
        self.runtime
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

    pub fn provenance(&self) -> CustomInvariantProvenance {
        CustomInvariantProvenance {
            observation_kind: self.observation_kind,
            version_id: self.version_id,
            current_version_id: self.current_version_id,
            touched: self.touched.provenance_summary(),
            counts: self.counts,
            traversal: self.traversal.summary(),
        }
    }
}
