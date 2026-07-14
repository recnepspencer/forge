use worth_query::facade::runtime::WorthQueryLowerRuntimeBoundaryReconciliationReport;
use worth_query::facade::runtime::WorthQueryLowerRuntimeBoundaryReconciliationRow;

fn main() {
    let rows: Vec<WorthQueryLowerRuntimeBoundaryReconciliationRow> = Vec::new();
    let _ = WorthQueryLowerRuntimeBoundaryReconciliationReport::new(rows);
}
