mod manifest_contract;
mod source_contract;

use std::fs;
use std::path::Path;

use crate::topology::WorkspaceSourceInventory;

const LEDGER: &str = "_docs/worth-ui/milestone-3.10.3-phase-2-proof-ledger.csv";

pub fn audit_milestone_3103_phase2_product_contract(
    inventory: &WorkspaceSourceInventory,
) -> Result<(), String> {
    let repository_root = inventory
        .root()
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "Worth UI workspace should have a repository parent".to_owned())?;
    audit_live_product_contract(inventory)?;
    audit_ledger(
        &fs::read_to_string(repository_root.join(LEDGER))
            .map_err(|error| format!("`{LEDGER}` should be readable: {error}"))?,
    )
}

fn audit_live_product_contract(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    manifest_contract::audit(
        inventory.text("Cargo.toml"),
        inventory.text("apps/platform-pulse/Cargo.toml"),
    )?;
    source_contract::audit(inventory)
}

fn audit_ledger(text: &str) -> Result<(), String> {
    let rows = text.lines().skip(1).collect::<Vec<_>>();
    if rows.len() != 12 {
        return Err(format!(
            "Phase 2 proof ledger should contain twelve guarantees; found {}",
            rows.len()
        ));
    }
    for (index, row) in rows.iter().enumerate() {
        let expected_id = format!("P2-{:02}", index + 1);
        if !row.starts_with(&format!("{expected_id},")) {
            return Err(format!(
                "Phase 2 proof ledger row {} should be `{expected_id}`",
                index + 1
            ));
        }
        if !row.contains(",\"PROVED\",\"") {
            return Err(format!(
                "Phase 2 proof ledger `{expected_id}` should be PROVED"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod negative_fixtures;
