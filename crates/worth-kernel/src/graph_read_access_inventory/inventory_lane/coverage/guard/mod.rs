mod coverage_guard;
mod guard_report;

pub(crate) use coverage_guard::validate_current_graph_read_surfaces;
#[cfg(test)]
#[cfg(test)]
pub(crate) use coverage_guard::validate_discovered_graph_read_surfaces;
pub use guard_report::WorthGraphReadAccessCoverageGuardReport;
