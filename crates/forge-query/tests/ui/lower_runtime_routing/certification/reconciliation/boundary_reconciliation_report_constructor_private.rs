use forge_query::facade::ForgeQueryLowerRuntimeBoundaryReconciliationReport;
use forge_query::facade::ForgeQueryLowerRuntimeBoundaryReconciliationRow;

fn main() {
    let rows: Vec<ForgeQueryLowerRuntimeBoundaryReconciliationRow> = Vec::new();
    let _ = ForgeQueryLowerRuntimeBoundaryReconciliationReport::new(rows);
}
