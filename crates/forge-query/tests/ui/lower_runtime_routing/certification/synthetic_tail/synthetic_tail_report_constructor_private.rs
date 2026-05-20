use forge_query::facade::ForgeQueryLowerRuntimeSyntheticTailReport;
use forge_query::facade::ForgeQueryLowerRuntimeSyntheticTailRow;

fn main() {
    let rows: Vec<ForgeQueryLowerRuntimeSyntheticTailRow> = Vec::new();
    let _ = ForgeQueryLowerRuntimeSyntheticTailReport::new(rows);
}
