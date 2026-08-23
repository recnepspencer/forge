mod certification_surfaces;
mod facade_runtime;
mod ledger;
mod phase4_runtime_subsystems;
mod phase5_product_api;
mod phase6_callable_surface;
mod phase7_adapter_boundary;
mod phase7_closing_evidence;
mod phase7_evidence;
mod phase7_historical_source_scope;
mod phase8_closeout;
mod runtime_language_ownership;
mod source_semantics;

use std::path::Path;

use crate::topology::WorkspaceSourceInventory;

pub fn audit_milestone_3101_inventory(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    let repository_root = inventory
        .root()
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "Worth UI workspace should have a repository parent".to_owned())?;
    let source_ledger = ledger::load(
        &repository_root.join("_docs/worth-ui/milestone-3.10.1-source-semantics-inventory.toml"),
    )?;
    let facade_ledger = ledger::load(
        &repository_root.join("_docs/worth-ui/milestone-3.10.1-facade-runtime-inventory.toml"),
    )?;
    let phase4_runtime_ledger = ledger::load(
        &repository_root.join("_docs/worth-ui/milestone-3.10.1-phase-4-runtime-subsystems.toml"),
    )?;
    let phase5_product_api = ledger::load(
        &repository_root.join("_docs/worth-ui/milestone-3.10.1-phase-5-product-api.toml"),
    )?;
    let phase6_callable_surface = ledger::load(
        &repository_root.join("_docs/worth-ui/milestone-3.10.1-phase-6-callable-surface.toml"),
    )?;
    let phase7_evidence = ledger::load(
        &repository_root.join("_docs/worth-ui/milestone-3.10.1-phase-7-evidence.toml"),
    )?;
    let phase8_closeout = ledger::load(
        &repository_root.join("_docs/worth-ui/milestone-3.10.1-phase-8-closeout.toml"),
    )?;

    source_semantics::audit(inventory, &source_ledger)?;
    facade_runtime::audit(inventory, &facade_ledger)?;
    phase4_runtime_subsystems::audit(inventory, &phase4_runtime_ledger)?;
    phase5_product_api::audit(inventory, &phase5_product_api)?;
    phase6_callable_surface::audit(inventory, &phase6_callable_surface)?;
    phase7_evidence::audit(inventory, &phase7_evidence)?;
    phase7_closing_evidence::audit(inventory, repository_root)?;
    phase8_closeout::audit(&phase8_closeout, repository_root)?;
    runtime_language_ownership::audit(inventory)
}

#[cfg(test)]
mod negative_fixtures;
