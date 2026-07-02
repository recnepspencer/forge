use forge_store_physical_isolation::PhysicalIsolationEntryAdmission;

fn main() {
    let _ = PhysicalIsolationEntryAdmission {
        identity: todo!(),
        root_epoch_basis: todo!(),
        recovered_root: String::new(),
        admitted_page_lsn_frontier: None,
        replayed_frames: 0,
        source_candidate_count: 0,
        evidence: todo!(),
    };
}
