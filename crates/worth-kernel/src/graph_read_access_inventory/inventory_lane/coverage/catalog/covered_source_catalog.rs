use std::collections::BTreeSet;

use super::super::super::inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};
use super::source_roots::COVERED_GRAPH_READ_SOURCES;
use super::WorthGraphReadAccessCoveredSource;

pub(super) fn covered_graph_read_sources(
) -> Result<&'static [WorthGraphReadAccessCoveredSource], WorthGraphReadAccessInventoryError> {
    validate_catalog(COVERED_GRAPH_READ_SOURCES)?;
    Ok(COVERED_GRAPH_READ_SOURCES)
}

fn validate_catalog(
    sources: &[WorthGraphReadAccessCoveredSource],
) -> Result<(), WorthGraphReadAccessInventoryError> {
    let mut source_paths = BTreeSet::new();
    for source in sources {
        if !source_paths.insert(source.source_path()) {
            return Err(error(
                WorthGraphReadAccessInventoryErrorKind::DuplicateCoveredSourcePath,
            ));
        }
    }

    for required in REQUIRED_COVERED_SOURCE_PATHS {
        if !source_paths.contains(*required) {
            return Err(error(
                WorthGraphReadAccessInventoryErrorKind::MissingRequiredCoveredSource,
            ));
        }
    }
    Ok(())
}

const REQUIRED_COVERED_SOURCE_PATHS: &[&str] = &[
    "crates/worth-topo/src/projection/read_views/domain",
    "crates/worth-topo/src/projection/runtime_boundary/read_execution",
    "crates/worth-kernel/src/query_adoption/graph_read_access",
    "crates/worth-kernel/src/workload_composition",
    "crates/worth-kernel/src/binding",
    "crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction",
    "crates/worth-spatial/src/workload_platform/planar_boolean_events",
    "crates/worth-spatial/src/workload_platform/evidence_ledger",
];

const fn error(kind: WorthGraphReadAccessInventoryErrorKind) -> WorthGraphReadAccessInventoryError {
    WorthGraphReadAccessInventoryError::new(kind)
}
