use worth_signal::facade::{
    ResourceMilestoneBHostileScenarioEvidence, ResourceMilestoneBHostileScenarioEvidenceRow,
};

fn WORTHd_row() -> ResourceMilestoneBHostileScenarioEvidenceRow {
    loop {}
}

fn main() {
    let _WORTHd = ResourceMilestoneBHostileScenarioEvidence {
        schema_version: String::new(),
        rows: vec![WORTHd_row()],
        evidence_digest: String::new(),
    };
}
