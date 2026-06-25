use super::super::super::inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};
use super::super::catalog::WorthGraphReadAccessCoveredSource;
use super::super::discovery::{
    current_worth_graph_read_access_discovered_surface_report,
    WorthGraphReadAccessDiscoveredSurface, WorthGraphReadAccessDiscoveredSurfaceReport,
};
use super::guard_report::WorthGraphReadAccessCoverageGuardReport;

pub(crate) fn validate_current_graph_read_surfaces(
    covered_sources: &[WorthGraphReadAccessCoveredSource],
) -> Result<WorthGraphReadAccessCoverageGuardReport, WorthGraphReadAccessInventoryError> {
    let discovered_report = current_worth_graph_read_access_discovered_surface_report();
    validate_discovered_graph_read_surface_report(discovered_report, covered_sources)
}

#[cfg(test)]
pub(crate) fn validate_discovered_graph_read_surfaces(
    discovered_surfaces: &[WorthGraphReadAccessDiscoveredSurface],
    covered_sources: &[WorthGraphReadAccessCoveredSource],
) -> Result<WorthGraphReadAccessCoverageGuardReport, WorthGraphReadAccessInventoryError> {
    validate_discovered_graph_read_surface_slice(discovered_surfaces, covered_sources)
}

fn validate_discovered_graph_read_surface_report(
    discovered_report: WorthGraphReadAccessDiscoveredSurfaceReport,
    covered_sources: &[WorthGraphReadAccessCoveredSource],
) -> Result<WorthGraphReadAccessCoverageGuardReport, WorthGraphReadAccessInventoryError> {
    validate_discovered_graph_read_surface_slice(discovered_report.surfaces(), covered_sources)
}

fn validate_discovered_graph_read_surface_slice(
    discovered_surfaces: &[WorthGraphReadAccessDiscoveredSurface],
    covered_sources: &[WorthGraphReadAccessCoveredSource],
) -> Result<WorthGraphReadAccessCoverageGuardReport, WorthGraphReadAccessInventoryError> {
    for discovered in discovered_surfaces {
        if is_covered(discovered, covered_sources) {
            continue;
        }
        if !looks_graph_read_shaped(discovered.evidence()) {
            continue;
        }
        let kind = if discovered.is_test_support() {
            WorthGraphReadAccessInventoryErrorKind::ProductionShapedTestSupportUnclassified
        } else {
            WorthGraphReadAccessInventoryErrorKind::UnclassifiedGraphReadSurface
        };
        return Err(error(kind));
    }

    Ok(WorthGraphReadAccessCoverageGuardReport::clean(
        discovered_surfaces.len(),
        covered_sources.len(),
        discovered_surfaces.len(),
    ))
}

fn is_covered(
    discovered: &WorthGraphReadAccessDiscoveredSurface,
    covered_sources: &[WorthGraphReadAccessCoveredSource],
) -> bool {
    covered_sources
        .iter()
        .any(|covered| covered.source_path() == discovered.source_path())
}

fn looks_graph_read_shaped(evidence: &str) -> bool {
    let normalized_evidence = evidence.to_ascii_lowercase();
    GRAPH_READ_SHAPES
        .iter()
        .any(|shape| normalized_evidence.contains(shape))
}

const GRAPH_READ_SHAPES: &[&str] = &[
    "adjacency",
    "broad scan",
    "fabricated",
    "frontier",
    "graph_read",
    "local cache",
    "local topology",
    "neighborhood",
    "no-n-plus-one",
    "read-proof",
    "read receipt",
    "relation loop",
    "receipt",
    "relationship",
];

const fn error(kind: WorthGraphReadAccessInventoryErrorKind) -> WorthGraphReadAccessInventoryError {
    WorthGraphReadAccessInventoryError::new(kind)
}
