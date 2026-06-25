use forge_query::facade::ForgeQueryReadReceipt;
use worth_kernel::graph_read_access_plan_adoption::current_worth_graph_read_access_hard_deletion_closeout;

fn main() {
    let receipt: ForgeQueryReadReceipt = panic!("raw Query receipt cannot seed Phase 7");
    let _ = current_worth_graph_read_access_hard_deletion_closeout(&receipt);
}
