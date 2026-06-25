use worth_kernel::graph_read_access_inventory::inventory_lane::{
    WorthGraphReadAccessScopeBinding, WorthGraphReadAccessScopeExpectation,
    WorthGraphReadAccessScopeFamily, WorthGraphReadAccessScopeKind,
};

fn main() {
    let _binding = WorthGraphReadAccessScopeBinding {
        source_path: "crates/worth-topo/src/projection/read_views/domain".to_string(),
        scope_kind: WorthGraphReadAccessScopeKind::SelectedObligation,
        scope_family: WorthGraphReadAccessScopeFamily::TopologyReadLedger,
        scope_expectation:
            WorthGraphReadAccessScopeExpectation::MilestoneSevenDeclarationCandidateInput,
        selected_obligation_index: Some(0),
        authority_digest: Some("authority-a".to_string()),
        touch_descriptor_digest: Some("touch-a".to_string()),
        execution_proof_digest: Some("execution-a".to_string()),
        selected_registration_digest: Some("registration-a".to_string()),
        adoption_manifest_digest: None,
        certification_boundary: None,
    };
}
