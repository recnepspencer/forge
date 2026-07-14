use worth_store_layout_indexes::layout_counters::{
    branch_delta_support_counter_evidence, continuation_support_counter_evidence,
    snapshot_support_counter_evidence, stable_basis_support_counter_evidence,
};

fn main() {
    let _ = snapshot_support_counter_evidence(4, 2);
    let _ = branch_delta_support_counter_evidence(4);
    let _ = stable_basis_support_counter_evidence(4);
    let _ = continuation_support_counter_evidence(4);
}
