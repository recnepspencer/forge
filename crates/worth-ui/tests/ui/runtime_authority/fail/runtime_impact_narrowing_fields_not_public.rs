use worth_ui::facade::{
    WorthUiAccessibilityInvalidation, WorthUiImpactLookupCounters,
    WorthUiRuntimeImpactNarrowing,
};

fn main() {
    let _ = WorthUiRuntimeImpactNarrowing {
        active_artifact_digest: 1,
        candidate_artifact_digest: 2,
        affected_source_modules: Vec::new(),
        affected_handles: Vec::new(),
        affected_subtree_digests: Vec::new(),
        command_binding_invalidations: Vec::new(),
        token_invalidations: Vec::new(),
        accessibility_invalidation: accessibility_invalidation(),
        renderer_resource_invalidations: Vec::new(),
        query_dependency_invalidations: Vec::new(),
        lane_impact: None,
        full_artifact_handle_count: 0,
        counters: counters(),
    };
}

fn accessibility_invalidation() -> WorthUiAccessibilityInvalidation {
    unimplemented!()
}

fn counters() -> WorthUiImpactLookupCounters {
    unimplemented!()
}
