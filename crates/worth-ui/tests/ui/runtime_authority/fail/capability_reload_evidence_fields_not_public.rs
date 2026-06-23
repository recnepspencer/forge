use worth_ui::facade::{
    WorthUiCapabilityReloadEvidence, WorthUiCapabilityReloadStatus,
};

fn main() {
    let _forged = WorthUiCapabilityReloadEvidence {
        runtime_instance_witness: 1,
        status: WorthUiCapabilityReloadStatus::Activated,
        denial_detail: None,
        active_snapshot_digest_before: 11,
        active_snapshot_digest_after: 12,
        candidate_snapshot_digest: Some(12),
        request_digest: 29,
        family_rows: Vec::new(),
        edited_delta_width: 1,
        family_rebuild_breadth: 6,
        source_parse_count: 1,
        registry_lookup_count: 1,
        artifact_tree_scan_count: 0,
        active_runtime_mutations_before_activation: 0,
        changed_facts: forged_changed_facts(),
    };
}

fn forged_changed_facts() -> worth_ui::facade::WorthUiCapabilityChangedFacts {
    panic!("fixture should fail before runtime construction")
}
