use forge_query::facade::{ForgeQueryAuthorityLane, ForgeQueryComputedInspectionEvidence};

fn main() {
    let _forged = ForgeQueryComputedInspectionEvidence {
        name: "computed.forged".to_string(),
        authority_lane: ForgeQueryAuthorityLane::DerivedRuntimeState,
        upstream_live_views: Vec::new(),
        upstream_derived_views: Vec::new(),
        dependency_aspects: Vec::new(),
        produced_aspects: Vec::new(),
        incremental_delivery: true,
        materialized_row_count: 0,
        pending_patch_count: 0,
        pending_incremental_patch_count: 0,
        pending_refresh_fallback_count: 0,
        declaration_digest: String::new(),
        dependency_digest: String::new(),
        produced_aspect_digest: String::new(),
        materialization_digest: String::new(),
        pending_patch_digest: String::new(),
        inspection_digest: String::new(),
    };
}
