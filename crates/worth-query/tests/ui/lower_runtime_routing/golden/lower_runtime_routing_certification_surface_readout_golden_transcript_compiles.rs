use worth_query::facade::runtime::{worth_query_lower_runtime_acceptance_suite, worth_query_lower_runtime_boundary_reconciliation_report, WorthQueryLowerRuntimeAcceptanceLane};
use worth_query::facade::certification::worth_query_lower_runtime_synthetic_tail_report;

fn main() {
    std::thread::Builder::new()
        .stack_size(16 * 1024 * 1024)
        .spawn(run)
        .expect("golden transcript thread should spawn")
        .join()
        .expect("golden transcript should run");
}

fn run() {
    let acceptance = worth_query_lower_runtime_acceptance_suite();
    let reconciliation = worth_query_lower_runtime_boundary_reconciliation_report();
    let synthetic_tail = worth_query_lower_runtime_synthetic_tail_report();

    let _ = acceptance
        .lane(WorthQueryLowerRuntimeAcceptanceLane::Control)
        .digest();
    let _ = acceptance
        .lane(WorthQueryLowerRuntimeAcceptanceLane::Hostile)
        .digest();
    let _ = reconciliation.report_digest();
    let _ = reconciliation.rows().len();
    let _ = synthetic_tail.report_digest();
    let _ = synthetic_tail.justification_digest();
}
