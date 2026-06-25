use worth_kernel::graph_read_access_plan_adoption::WorthGraphReadAccessHardDeletionCappedResidueRow;

fn main() {
    let _ = WorthGraphReadAccessHardDeletionCappedResidueRow {
        source_path: String::new(),
        owner: String::new(),
        blocker: String::new(),
        removal_trigger: String::new(),
        observed_residue_count: 0,
        allowed_residue_count: 0,
        row_digest: String::new(),
    };
}
