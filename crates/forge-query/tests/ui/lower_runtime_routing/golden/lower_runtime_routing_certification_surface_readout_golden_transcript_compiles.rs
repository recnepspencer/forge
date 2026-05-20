use forge_query::facade::{
    forge_query_lower_runtime_acceptance_suite,
    forge_query_lower_runtime_boundary_reconciliation_report,
    forge_query_lower_runtime_synthetic_tail_report, ForgeQueryLowerRuntimeAcceptanceLane,
};

fn main() {
    let acceptance = forge_query_lower_runtime_acceptance_suite();
    let reconciliation = forge_query_lower_runtime_boundary_reconciliation_report();
    let synthetic_tail = forge_query_lower_runtime_synthetic_tail_report();

    let _ = acceptance
        .lane(ForgeQueryLowerRuntimeAcceptanceLane::Control)
        .digest();
    let _ = acceptance
        .lane(ForgeQueryLowerRuntimeAcceptanceLane::Hostile)
        .digest();
    let _ = reconciliation.report_digest();
    let _ = reconciliation.rows().len();
    let _ = synthetic_tail.report_digest();
    let _ = synthetic_tail.justification_digest();
}
