mod closing_evidence;
mod courtroom_contract;
mod destination_topology;
mod evidence_document;
mod opening_cost_budget;
mod source_to_pixel_contract;

use std::path::Path;

use crate::topology::WorkspaceSourceInventory;

pub fn audit_milestone_3102_pulse_seed(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    let repository_root = inventory
        .root()
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "Worth UI workspace should have a repository parent".to_owned())?;
    let contract = evidence_document::load_toml(
        &repository_root.join("_docs/worth-ui/milestone-3.10.2-phase-1-source-to-pixel.toml"),
    )?;
    let baseline = evidence_document::load_json(
        &repository_root.join("_docs/worth-ui/milestone-3.10.2-phase-1-opening-baseline.json"),
    )?;

    source_to_pixel_contract::audit(&contract)?;
    courtroom_contract::audit(&contract)?;
    destination_topology::audit(inventory, repository_root, &contract)?;
    opening_cost_budget::audit(inventory, &baseline)?;
    closing_evidence::audit(repository_root)
}

#[cfg(test)]
mod negative_fixtures;
