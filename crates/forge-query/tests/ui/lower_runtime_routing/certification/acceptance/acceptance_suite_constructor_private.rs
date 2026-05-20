use forge_query::facade::ForgeQueryLowerRuntimeAcceptanceRow;
use forge_query::facade::ForgeQueryLowerRuntimeAcceptanceSuite;

fn main() {
    let rows: Vec<ForgeQueryLowerRuntimeAcceptanceRow> = Vec::new();
    let _ = ForgeQueryLowerRuntimeAcceptanceSuite::new(rows);
}
