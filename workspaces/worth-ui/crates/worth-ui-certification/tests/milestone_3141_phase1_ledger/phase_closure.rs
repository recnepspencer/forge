use std::collections::BTreeMap;

use super::{ledger_document, parse, Row};

pub(super) fn validate_proved_run_uniqueness(rows: &BTreeMap<String, Row>) -> Result<(), String> {
    for field in ["run_nonce", "retained_result_artifact"] {
        let mut observed = std::collections::BTreeSet::new();
        for value in rows
            .values()
            .filter(|row| row["result"] == "PROVED")
            .map(|row| row[field].as_str())
        {
            if !observed.insert(value) {
                return Err(format!("proved rows reuse {field}"));
            }
        }
    }
    Ok(())
}

pub(super) fn validate_phase_closure(
    rows: &BTreeMap<String, Row>,
    through_phase: u8,
) -> Result<(), String> {
    super::phase_progression::validate_closure(rows, through_phase)
}

#[test]
#[ignore = "milestone closure gate: run only after every Phase 2 row has final evidence"]
fn phase_two_closure_requires_every_phase_one_and_two_row() {
    let rows = parse(&ledger_document()).expect("the milestone ledger should parse");
    validate_phase_closure(&rows, 2)
        .expect("every Phase 1 and Phase 2 requirement must be final-source proved");
}
