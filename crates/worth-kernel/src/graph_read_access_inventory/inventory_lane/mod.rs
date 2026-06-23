mod bypass_audit;
mod closeout;
mod coverage;
mod inventory_error;
mod proof_input;
mod residue;
mod row;
mod scope;
mod seed;

#[cfg(test)]
mod hardening_tests;
#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_fixtures;

pub use bypass_audit::WorthGraphReadBypassAdoptionReport;
pub use closeout::{
    WorthGraphReadAccessCloseoutOwner, WorthGraphReadAccessInventoryCloseout,
    WorthGraphReadAccessInventoryCloseoutCounters, WorthGraphReadDeletedSourceReport,
};
pub use coverage::{
    current_worth_graph_read_access_surface_inventory, WorthGraphReadAccessCoverageGuardReport,
};
pub use inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};
pub use proof_input::{
    reject_fabricated_graph_read_receipt_proof, reject_local_support_row_graph_read_proof,
};
pub use residue::{WorthGraphReadAccessCappedResidueRow, WorthGraphReadAccessResidueGrowthPolicy};
pub use row::{
    WorthGraphReadAccessClassification, WorthGraphReadAccessCostPosture,
    WorthGraphReadAccessDeletionAction, WorthGraphReadAccessFollowOnWork,
    WorthGraphReadAccessInventoryRow, WorthGraphReadAccessMilestoneSevenDisposition,
    WorthGraphReadAccessOutOfScopeReason, WorthGraphReadAccessOwner,
};
pub use scope::{
    WorthGraphReadAccessScopeBinding, WorthGraphReadAccessScopeExpectation,
    WorthGraphReadAccessScopeFamily, WorthGraphReadAccessScopeKind,
    WorthGraphReadAccessScopePlanEntry, WorthGraphReadAccessScopePlanReport,
    WorthGraphReadAccessScopeReport,
};
pub use seed::WorthGraphReadAccessInventorySeed;

#[cfg(test)]
pub(super) use bypass_audit::graph_read_bypass_source_inventory_from_rows;
#[cfg(test)]
pub(super) use closeout::WorthGraphReadAccessInventoryCollector;
#[cfg(test)]
pub(super) use coverage::{
    covered_graph_read_sources, current_worth_graph_read_access_surface_inventory_for_tests,
    validate_discovered_graph_read_surfaces, WorthGraphReadAccessDiscoveredSurface,
};
#[cfg(test)]
pub(super) use residue::WorthGraphReadAccessCappedResidueBuilder;
#[cfg(test)]
pub(in crate::graph_read_access_inventory::inventory_lane) use residue::{
    graph_read_bypass_residue_cap_inventory, graph_read_bypass_residue_manifest_for_report,
};
#[cfg(test)]
pub(super) use row::WorthGraphReadAccessInventoryRowBuilder;
#[cfg(test)]
pub(super) use scope::{
    reject_read_access_plan_scope_substitution, WorthGraphReadAccessScopeSubstitutionRole,
};
