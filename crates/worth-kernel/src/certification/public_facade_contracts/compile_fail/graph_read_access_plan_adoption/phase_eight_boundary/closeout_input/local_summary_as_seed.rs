use worth_kernel::graph_read_access_plan_adoption::current_worth_graph_read_access_plan_adoption_closeout;

struct LocalGraphReadCloseoutSummary {
    receipt_rows: usize,
    posture_rows: usize,
}

fn main() {
    let summary = LocalGraphReadCloseoutSummary {
        receipt_rows: 1,
        posture_rows: 1,
    };
    let _ = current_worth_graph_read_access_plan_adoption_closeout(&summary);
}
