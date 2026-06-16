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
        theme_source_digest: 29,
        touched_theme_token_count: 1,
        theme_token_family_entry_count: 6,
        source_parse_count: 1,
        registry_lookup_count: 1,
        artifact_tree_scan_count: 0,
        active_runtime_mutations_before_activation: 0,
    };
}
