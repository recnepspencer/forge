mod closeout;
mod declared_surface;
mod forbidden_surface;
mod ordinary_cutover_firewall;
mod phase_fifteen_public_proof_semantic_source_registry;
mod phase_fifteen_semantic_source_registry;
mod phase_fourteen_raw_construction_registry;
mod phase_fourteen_reintroduction_registry;
mod phase_twelve_semantic_source_registry;
mod private_surface_registry;
mod report;
mod semantic_source_registry;

#[cfg(test)]
mod phase_fourteen_tests;
#[cfg(test)]
mod tests;

pub use closeout::{
    current_worth_touched_graph_conflict_source_firewall_closeout,
    WorthTouchedGraphConflictSourceFirewallCloseout,
    WorthTouchedGraphConflictSourceFirewallCloseoutError,
    WorthTouchedGraphConflictSourceFirewallCloseoutErrorKind,
};
pub use forbidden_surface::WorthTouchedGraphConflictForbiddenSurface;
pub use ordinary_cutover_firewall::current_worth_touched_graph_conflict_source_firewall_report;
pub use report::{
    WorthTouchedGraphConflictSourceFirewallRegionReport,
    WorthTouchedGraphConflictSourceFirewallReport,
    WorthTouchedGraphConflictSourceFirewallViolation,
};

#[cfg(test)]
pub(crate) use ordinary_cutover_firewall::scan_worth_touched_graph_conflict_source_firewall_region_for_tests;
