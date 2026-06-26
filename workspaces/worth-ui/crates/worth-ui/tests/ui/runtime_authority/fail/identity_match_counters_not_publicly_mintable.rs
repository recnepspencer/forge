use worth_ui::facade::WorthUiIdentityMatchCounters;

fn main() {
    let _ = WorthUiIdentityMatchCounters {
        active_nodes_indexed: 0,
        candidate_nodes_indexed: 0,
        stable_seed_lookups: 0,
        duplicate_active_identity_count: 0,
        duplicate_candidate_identity_count: 0,
        identity_kind_mismatch_count: 0,
        matches_emitted: 0,
        unmatched_active_count: 0,
        unmatched_candidate_count: 0,
    };
}
