use std::collections::BTreeSet;

use crate::identity::data::{KindId, PartitionId, VersionId};
use crate::inspection::data::{
    ConnectivityInspectionRequest, ConnectivityInspectionSummary, GraphInspectionRequest,
    GraphInspectionSummary, InspectionAccessPath, InspectionAvailability, InspectionDegradation,
    InspectionOrigin, InspectionScope,
};
use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::{RelationalReadView, RetentionPlan};
use crate::visibility::cache_state::cached_state_for_version;

pub struct InspectionAccess<'runtime> {
    pub(super) runtime: &'runtime RelationalRuntime,
}

impl RelationalRuntime {
    pub fn inspection_access(&self) -> InspectionAccess<'_> {
        InspectionAccess::new(self)
    }
}

impl<'runtime> InspectionAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub(super) fn read_view_for_scope(&self, scope: &InspectionScope) -> Option<RelationalReadView> {
        match scope {
            InspectionScope::Current => Some(
                self.runtime
                    .visibility_reads()
                    .read_version(self.runtime.current_version_id()),
            ),
            InspectionScope::Version(version_id) => {
                Some(self.runtime.visibility_reads().read_version(*version_id))
            }
            InspectionScope::Snapshot(handle) => self.runtime.visibility_reads().read_snapshot(handle),
        }
    }

    pub(super) fn scope_version_id(&self, scope: &InspectionScope) -> VersionId {
        match scope {
            InspectionScope::Current => self.runtime.current_version_id(),
            InspectionScope::Version(version_id) => *version_id,
            InspectionScope::Snapshot(handle) => handle.version_id,
        }
    }

    pub(super) fn scope_access_path(
        &self,
        scope: &InspectionScope,
        version_id: VersionId,
    ) -> InspectionAccessPath {
        match scope {
            InspectionScope::Current => InspectionAccessPath::DirectLookup,
            InspectionScope::Version(_)
                if cached_state_for_version(self.runtime, version_id).is_some() =>
            {
                InspectionAccessPath::HistoricalRetainedRead
            }
            InspectionScope::Version(_) => InspectionAccessPath::HistoricalReconstructedRead,
            InspectionScope::Snapshot(_) => InspectionAccessPath::SnapshotRead,
        }
    }

    pub(super) fn scope_availability(
        &self,
        scope: &InspectionScope,
        version_id: VersionId,
    ) -> InspectionAvailability {
        match scope {
            InspectionScope::Current => InspectionAvailability::Direct,
            InspectionScope::Version(_)
                if cached_state_for_version(self.runtime, version_id).is_some() =>
            {
                InspectionAvailability::Direct
            }
            InspectionScope::Version(_) => InspectionAvailability::Reconstructed,
            InspectionScope::Snapshot(_) => InspectionAvailability::Direct,
        }
    }

    pub(super) fn unavailable_graph_summary(
        &self,
        request: &GraphInspectionRequest,
        version_id: VersionId,
    ) -> GraphInspectionSummary {
        GraphInspectionSummary {
            scope: request.scope.clone(),
            version_id,
            partition_count: 0,
            entity_count: 0,
            relation_count: 0,
            entity_kinds: Vec::new(),
            relation_kinds: Vec::new(),
            origin: InspectionOrigin::VisibilitySnapshot,
            access_path: self.scope_access_path(&request.scope, version_id),
            availability: unavailable_scope_availability(&request.scope),
            degradations: summary_degradations(!request.summary_only, None),
        }
    }

    pub(super) fn budget_exceeded_graph_summary(
        &self,
        request: &GraphInspectionRequest,
        version_id: VersionId,
        degradation: InspectionDegradation,
    ) -> GraphInspectionSummary {
        GraphInspectionSummary {
            scope: request.scope.clone(),
            version_id,
            partition_count: 0,
            entity_count: 0,
            relation_count: 0,
            entity_kinds: Vec::new(),
            relation_kinds: Vec::new(),
            origin: match request.scope {
                InspectionScope::Current => InspectionOrigin::CurrentTruth,
                _ => InspectionOrigin::VisibilitySnapshot,
            },
            access_path: self.scope_access_path(&request.scope, version_id),
            availability: InspectionAvailability::UnavailableByBudget,
            degradations: summary_degradations(!request.summary_only, Some(degradation)),
        }
    }

    pub(super) fn unavailable_connectivity_summary(
        &self,
        request: &ConnectivityInspectionRequest,
        version_id: VersionId,
    ) -> ConnectivityInspectionSummary {
        ConnectivityInspectionSummary {
            scope: request.scope.clone(),
            version_id,
            component_count: 0,
            largest_component_size: 0,
            enumerated_entity_count: 0,
            components: Vec::new(),
            origin: InspectionOrigin::VisibilitySnapshot,
            access_path: self.scope_access_path(&request.scope, version_id),
            resolution_context: crate::inspection::data::InspectionResolutionContext::ConnectivityTraversal,
            availability: unavailable_scope_availability(&request.scope),
            degradations: summary_degradations(request.include_members, None),
        }
    }

    pub(super) fn budget_exceeded_connectivity_summary(
        &self,
        request: &ConnectivityInspectionRequest,
        version_id: VersionId,
        degradation: InspectionDegradation,
    ) -> ConnectivityInspectionSummary {
        ConnectivityInspectionSummary {
            scope: request.scope.clone(),
            version_id,
            component_count: 0,
            largest_component_size: 0,
            enumerated_entity_count: 0,
            components: Vec::new(),
            origin: match request.scope {
                InspectionScope::Current => InspectionOrigin::CurrentTruth,
                _ => InspectionOrigin::VisibilitySnapshot,
            },
            access_path: self.scope_access_path(&request.scope, version_id),
            resolution_context: crate::inspection::data::InspectionResolutionContext::ConnectivityTraversal,
            availability: InspectionAvailability::UnavailableByBudget,
            degradations: summary_degradations(request.include_members, Some(degradation)),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(super) struct ScopeFilter<T: Ord + Copy>(Option<BTreeSet<T>>);

impl<T: Ord + Copy> ScopeFilter<T> {
    pub(super) fn from_scope(scope: Option<&Vec<T>>) -> Self {
        Self(scope.map(|values| values.iter().copied().collect()))
    }

    pub(super) fn allows(&self, value: T) -> bool {
        self.0.as_ref().is_none_or(|scope| scope.contains(&value))
    }
}

pub(super) type PartitionScopeFilter = ScopeFilter<PartitionId>;
pub(super) type KindScopeFilter = ScopeFilter<KindId>;

pub(super) fn summary_degradations(
    include_full_detail: bool,
    extra: Option<InspectionDegradation>,
) -> Vec<InspectionDegradation> {
    let mut degradations = Vec::new();
    if !include_full_detail {
        degradations.push(InspectionDegradation::SummaryOnly);
    }
    if let Some(extra) = extra {
        degradations.push(extra);
    }
    degradations
}

pub(super) fn unavailable_scope_availability(scope: &InspectionScope) -> InspectionAvailability {
    match scope {
        InspectionScope::Snapshot(_) => InspectionAvailability::UnavailableByRetention,
        InspectionScope::Current | InspectionScope::Version(_) => {
            InspectionAvailability::UnavailableByMissingCanonicalArtifacts
        }
    }
}

pub(super) fn empty_retention_plan(retention_fence_version: VersionId) -> RetentionPlan {
    RetentionPlan {
        retention_fence_version,
        active_snapshot_count: 0,
        branch_pinned_entities: 0,
        replay_pinned_entities: 0,
        snapshot_pinned_entities: 0,
        branch_pinned_relations: 0,
        replay_pinned_relations: 0,
        snapshot_pinned_relations: 0,
        reclaimable_entities: 0,
        reclaimable_relations: 0,
    }
}
