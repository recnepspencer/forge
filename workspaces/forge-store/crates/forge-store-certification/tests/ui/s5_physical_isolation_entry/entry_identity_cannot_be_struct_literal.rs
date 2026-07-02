use forge_store_physical_isolation::PhysicalIsolationEntryIdentity;

fn main() {
    let _ = PhysicalIsolationEntryIdentity {
        recovered_root: String::new(),
        admitted_page_lsn_frontier: None,
        source_decision_digest: String::new(),
        replayed_frames: 0,
        source_candidate_count: 0,
    };
}
