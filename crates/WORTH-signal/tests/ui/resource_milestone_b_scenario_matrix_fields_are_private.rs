use worth_signal::facade::{
    ResourceMilestoneBScenarioMatrix, ResourceMilestoneBScenarioMatrixSummary,
    ResourceMilestoneBScenarioRow,
};

fn WORTHd_row() -> ResourceMilestoneBScenarioRow {
    loop {}
}

fn WORTHd_summary() -> ResourceMilestoneBScenarioMatrixSummary {
    loop {}
}

fn main() {
    let _WORTHd = ResourceMilestoneBScenarioMatrix {
        schema_version: String::new(),
        bundle_digest: String::new(),
        rows: vec![WORTHd_row()],
        summary: WORTHd_summary(),
        matrix_digest: String::new(),
        passed: true,
    };
}
