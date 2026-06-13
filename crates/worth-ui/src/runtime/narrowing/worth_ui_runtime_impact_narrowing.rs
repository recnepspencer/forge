use crate::runtime::{
    WorthUiAccessibilityInvalidation, WorthUiCommandBindingInvalidation,
    WorthUiImpactLookupCounters, WorthUiLaneImpactClassification,
    WorthUiQueryDependencyInvalidation, WorthUiRendererResourceInvalidation,
    WorthUiTokenInvalidation,
};
use crate::source::{WorthUiArtifactHandle, WorthUiArtifactSubtreeDigest, WorthUiSourceModuleId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeImpactNarrowing {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    affected_source_modules: Vec<String>,
    affected_handles: Vec<WorthUiArtifactHandle>,
    affected_subtree_digests: Vec<WorthUiArtifactSubtreeDigest>,
    command_binding_invalidations: Vec<WorthUiCommandBindingInvalidation>,
    token_invalidations: Vec<WorthUiTokenInvalidation>,
    accessibility_invalidation: WorthUiAccessibilityInvalidation,
    renderer_resource_invalidations: Vec<WorthUiRendererResourceInvalidation>,
    query_dependency_invalidations: Vec<WorthUiQueryDependencyInvalidation>,
    lane_impact: Option<WorthUiLaneImpactClassification>,
    full_artifact_handle_count: usize,
    counters: WorthUiImpactLookupCounters,
}

impl WorthUiRuntimeImpactNarrowing {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        affected_source_modules: Vec<WorthUiSourceModuleId>,
        affected_handles: Vec<WorthUiArtifactHandle>,
        affected_subtree_digests: Vec<WorthUiArtifactSubtreeDigest>,
        command_binding_invalidations: Vec<WorthUiCommandBindingInvalidation>,
        token_invalidations: Vec<WorthUiTokenInvalidation>,
        accessibility_invalidation: WorthUiAccessibilityInvalidation,
        renderer_resource_invalidations: Vec<WorthUiRendererResourceInvalidation>,
        query_dependency_invalidations: Vec<WorthUiQueryDependencyInvalidation>,
        lane_impact: Option<WorthUiLaneImpactClassification>,
        full_artifact_handle_count: usize,
        counters: WorthUiImpactLookupCounters,
    ) -> Self {
        let mut affected_source_modules = affected_source_modules
            .into_iter()
            .map(|module_id| module_id.as_str().to_owned())
            .collect::<Vec<_>>();
        affected_source_modules.sort();
        affected_source_modules.dedup();
        let mut affected_handles = affected_handles;
        affected_handles.sort();
        affected_handles.dedup();
        let mut affected_subtree_digests = affected_subtree_digests;
        affected_subtree_digests.sort();
        affected_subtree_digests.dedup();
        let mut query_dependency_invalidations = query_dependency_invalidations;
        query_dependency_invalidations.sort();
        query_dependency_invalidations.dedup();

        Self {
            active_artifact_digest,
            candidate_artifact_digest,
            affected_source_modules,
            affected_handles,
            affected_subtree_digests,
            command_binding_invalidations,
            token_invalidations,
            accessibility_invalidation,
            renderer_resource_invalidations,
            query_dependency_invalidations,
            lane_impact,
            full_artifact_handle_count,
            counters,
        }
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn affected_source_modules(&self) -> &[String] {
        &self.affected_source_modules
    }

    pub fn affected_subtree_digests(&self) -> &[WorthUiArtifactSubtreeDigest] {
        &self.affected_subtree_digests
    }

    pub fn affected_handle_count(&self) -> usize {
        self.affected_handles.len()
    }

    pub(crate) fn affected_handles_for_runtime(&self) -> &[WorthUiArtifactHandle] {
        &self.affected_handles
    }

    pub fn full_artifact_handle_count(&self) -> usize {
        self.full_artifact_handle_count
    }

    pub fn command_binding_invalidations(&self) -> &[WorthUiCommandBindingInvalidation] {
        &self.command_binding_invalidations
    }

    pub fn token_invalidations(&self) -> &[WorthUiTokenInvalidation] {
        &self.token_invalidations
    }

    pub fn accessibility_invalidation(&self) -> &WorthUiAccessibilityInvalidation {
        &self.accessibility_invalidation
    }

    pub fn renderer_resource_invalidations(&self) -> &[WorthUiRendererResourceInvalidation] {
        &self.renderer_resource_invalidations
    }

    pub fn query_dependency_invalidations(&self) -> &[WorthUiQueryDependencyInvalidation] {
        &self.query_dependency_invalidations
    }

    pub fn lane_impact(&self) -> Option<&WorthUiLaneImpactClassification> {
        self.lane_impact.as_ref()
    }

    pub fn counters(&self) -> WorthUiImpactLookupCounters {
        self.counters
    }
}
