mod closing_evidence;

use std::fs;
use std::path::Path;

use crate::topology::WorkspaceSourceInventory;

const EVIDENCE: &str = "_docs/worth-ui/milestone-3.10.3-phase-5-closing-evidence.json";
const LEDGER: &str = "_docs/worth-ui/milestone-3.10.3-phase-5-proof-ledger.csv";

pub fn audit_milestone_3103_phase5_cost_closure(
    inventory: &WorkspaceSourceInventory,
) -> Result<(), String> {
    let repository_root = inventory
        .root()
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "Worth UI workspace should have a repository parent".to_owned())?;
    let evidence = closing_evidence::load(&repository_root.join(EVIDENCE))?;
    closing_evidence::audit(repository_root, &evidence)?;
    audit_ledger(
        &fs::read_to_string(repository_root.join(LEDGER))
            .map_err(|error| format!("`{LEDGER}` should be readable: {error}"))?,
    )
}

fn audit_ledger(text: &str) -> Result<(), String> {
    let rows = text.lines().skip(1).collect::<Vec<_>>();
    if rows.len() != 12 {
        return Err(format!(
            "Phase 5 proof ledger should contain twelve guarantees; found {}",
            rows.len()
        ));
    }
    for (index, row) in rows.iter().enumerate() {
        let expected_id = format!("P5-{:02}", index + 1);
        if !row.starts_with(&format!("{expected_id},")) {
            return Err(format!(
                "Phase 5 proof ledger row {} should be `{expected_id}`",
                index + 1
            ));
        }
        if !row.contains(",\"PROVED\",\"") {
            return Err(format!(
                "Phase 5 proof ledger `{expected_id}` should be PROVED"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod negative_fixtures;
