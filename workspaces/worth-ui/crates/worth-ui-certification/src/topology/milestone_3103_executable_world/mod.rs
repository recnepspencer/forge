mod courtroom_contract;
mod evidence_classification;
mod evidence_document;
mod opening_budget;

use std::fs;
use std::path::Path;

use crate::topology::WorkspaceSourceInventory;

const CONTRACT: &str = "_docs/worth-ui/milestone-3.10.3-phase-1-evidence-inventory.toml";
const BASELINE: &str = "_docs/worth-ui/milestone-3.10.3-phase-1-opening-baseline.json";
const LEDGER: &str = "_docs/worth-ui/milestone-3.10.3-phase-1-proof-ledger.csv";
const CONTRACT_FINGERPRINT: &str = "2bf2afc386903f79";
const BASELINE_FINGERPRINT: &str = "cd9b6136ee6dcb6e";

pub fn audit_milestone_3103_phase1(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    let repository_root = inventory
        .root()
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "Worth UI workspace should have a repository parent".to_owned())?;
    audit_document_fingerprint(
        repository_root,
        CONTRACT,
        CONTRACT_FINGERPRINT,
        "evidence inventory",
    )?;
    audit_document_fingerprint(
        repository_root,
        BASELINE,
        BASELINE_FINGERPRINT,
        "opening baseline",
    )?;
    let contract = evidence_document::load_toml(&repository_root.join(CONTRACT))?;
    let baseline = evidence_document::load_json(&repository_root.join(BASELINE))?;

    audit_header(&contract)?;
    evidence_classification::audit(repository_root, &contract)?;
    courtroom_contract::audit(&contract)?;
    opening_budget::audit(inventory, &contract, &baseline)?;
    audit_ledger(repository_root)
}

fn audit_document_fingerprint(
    repository_root: &Path,
    relative: &str,
    expected: &str,
    name: &str,
) -> Result<(), String> {
    let text = evidence_document::load_text(&repository_root.join(relative))?;
    let observed = evidence_document::canonical_fingerprint(&text);
    if observed == expected {
        Ok(())
    } else {
        Err(format!("Phase 1 {name} changed: {observed} != {expected}"))
    }
}

fn audit_header(contract: &toml::Value) -> Result<(), String> {
    if evidence_document::toml_text(contract, "schema")?
        != "worth-ui.milestone-3.10.3.phase-1-evidence-inventory.v1"
        || evidence_document::toml_text(contract, "milestone")? != "3.10.3"
        || contract.get("phase").and_then(toml::Value::as_integer) != Some(1)
        || evidence_document::toml_text(contract, "status")? != "closed"
    {
        return Err("Milestone 3.10.3 Phase 1 evidence inventory header drifted".to_owned());
    }
    let product = contract
        .get("canonical_product")
        .ok_or_else(|| "Phase 1 inventory should freeze [canonical_product]".to_owned())?;
    for (field, expected) in [
        ("package", "worth-ui-platform-pulse"),
        ("binary", "worth-ui-platform-pulse"),
        (
            "source",
            "workspaces/worth-ui/apps/platform-pulse/app/main.wui",
        ),
        (
            "manifest",
            "workspaces/worth-ui/apps/platform-pulse/Cargo.toml",
        ),
        (
            "default_source_root",
            "workspaces/worth-ui/apps/platform-pulse/app",
        ),
        ("native_frame", "PlatformPulseNativeFrame"),
        ("native_entry", "eframe::run_native"),
        (
            "scenario",
            "worth-ui.platform-pulse.executable-world.lifecycle",
        ),
    ] {
        let actual = evidence_document::toml_text(product, field)?;
        if actual != expected {
            return Err(format!(
                "canonical product `{field}` should be `{expected}`; found `{actual}`"
            ));
        }
    }
    Ok(())
}

fn audit_ledger(repository_root: &Path) -> Result<(), String> {
    let text = fs::read_to_string(repository_root.join(LEDGER))
        .map_err(|error| format!("`{LEDGER}` should be readable: {error}"))?;
    let rows = text.lines().skip(1).collect::<Vec<_>>();
    if rows.len() != 15 {
        return Err(format!(
            "Phase 1 proof ledger should contain fifteen guarantees; found {}",
            rows.len()
        ));
    }
    for (index, row) in rows.iter().enumerate() {
        let expected_id = format!("P1-{:02}", index + 1);
        if !row.starts_with(&format!("{expected_id},")) {
            return Err(format!(
                "Phase 1 proof ledger row {} should be `{expected_id}`",
                index + 1
            ));
        }
        if !row.contains(",\"PROVED\",\"") {
            return Err(format!(
                "Phase 1 proof ledger `{expected_id}` should be PROVED"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod negative_fixtures;
