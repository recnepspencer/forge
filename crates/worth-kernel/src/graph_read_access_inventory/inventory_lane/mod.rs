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
pub(crate) use closeout::WorthGraphReadAccessInventoryCollector;
#[cfg(test)]
pub(crate) use coverage::{
    covered_graph_read_sources, current_worth_graph_read_access_surface_inventory_for_tests,
    validate_discovered_graph_read_surfaces, WorthGraphReadAccessDiscoveredSurface,
};
#[cfg(test)]
pub(crate) use residue::WorthGraphReadAccessCappedResidueBuilder;
#[cfg(test)]
pub(in crate::graph_read_access_inventory::inventory_lane) use residue::{
    graph_read_bypass_residue_cap_inventory, graph_read_bypass_residue_manifest_for_report,
};
#[cfg(test)]
pub(crate) use row::WorthGraphReadAccessInventoryRowBuilder;
#[cfg(test)]
pub(crate) use scope::WorthGraphReadAccessScopeSubstitutionRole;
#[cfg(test)]
pub(crate) use test_fixtures::{
    branch_declaration_candidate_row_for_tests, declaration_candidate_row_with_scope_for_tests,
    deletion_target_row, future_receipt_declaration_candidate_row_for_tests,
    preview_declaration_candidate_row_for_tests, spatial_declaration_candidate_row_for_tests,
};
