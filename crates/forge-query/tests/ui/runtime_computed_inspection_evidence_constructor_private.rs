use forge_query::facade::{ForgeQueryAuthorityLane, ForgeQueryComputedInspectionEvidence};

fn main() {
    let _forged = ForgeQueryComputedInspectionEvidence {
        name: "computed.forged".to_string(),
        authority_lane: ForgeQueryAuthorityLane::DerivedRuntimeState,
        upstream_live_views: Vec::new(),
        upstream_derived_views: Vec::new(),
        dependency_aspects: Vec::new(),
        produced_aspects: Vec::new(),
        materialized_row_count: 0,
        pending_patch_count: 0,
    };
}
