use worth_signal::facade::{
    ResourceMilestoneCPolicyScenarioMatrix, ResourceMilestoneCPolicyScenarioMatrixSummary,
    ResourceMilestoneCPolicyScenarioRow,
};

fn WORTHd_row() -> ResourceMilestoneCPolicyScenarioRow {
    loop {}
}

fn WORTHd_summary() -> ResourceMilestoneCPolicyScenarioMatrixSummary {
    loop {}
}

fn main() {
    let _WORTHd = ResourceMilestoneCPolicyScenarioMatrix {
        schema_version: String::new(),
        rows: vec![WORTHd_row()],
        summary: WORTHd_summary(),
        matrix_digest: String::new(),
        passed: true,
    };
}
