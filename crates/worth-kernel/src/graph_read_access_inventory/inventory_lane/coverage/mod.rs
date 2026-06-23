mod catalog;
mod current_inventory;
mod discovery;
mod guard;

#[cfg(test)]
mod scope_plan_assertions;
#[cfg(test)]
mod tests;

pub use current_inventory::current_worth_graph_read_access_surface_inventory;
pub use guard::WorthGraphReadAccessCoverageGuardReport;

#[cfg(test)]
pub(super) use catalog::covered_graph_read_sources;
#[cfg(test)]
pub(super) use current_inventory::current_worth_graph_read_access_surface_inventory_for_tests;
#[cfg(test)]
pub(super) use discovery::WorthGraphReadAccessDiscoveredSurface;
#[cfg(test)]
pub(super) use guard::validate_discovered_graph_read_surfaces;
#[cfg(test)]
pub(super) use scope_plan_assertions::assert_exact_scope_plan;
