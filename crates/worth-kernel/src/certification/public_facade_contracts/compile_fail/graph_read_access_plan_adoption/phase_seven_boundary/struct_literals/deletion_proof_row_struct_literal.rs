use worth_kernel::graph_read_access_plan_adoption::{
    WorthGraphReadAccessHardDeletionProofRow, WorthGraphReadAccessHardDeletionStatus,
};

fn main() {
    let _ = WorthGraphReadAccessHardDeletionProofRow {
        label: String::new(),
        source_path: String::new(),
        owner: String::new(),
        blocker: None,
        removal_trigger: String::new(),
        status: WorthGraphReadAccessHardDeletionStatus::Deleted,
        row_digest: String::new(),
    };
}
