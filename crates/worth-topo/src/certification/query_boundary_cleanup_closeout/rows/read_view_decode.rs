use crate::certification::error::TopologyCertificationError;

use super::super::support::{closed_row, domain_view_sources, ensure_all};
use super::super::TopologyQueryBoundaryCleanupArea;

pub(crate) fn certify_read_view_decode_row(
) -> Result<super::super::TopologyQueryBoundaryCleanupRow, TopologyCertificationError> {
    let sources = domain_view_sources()?;
    ensure_all(&sources, |source| !source.contains("ForgeQueryEntity"))?;
    ensure_all(&sources, |source| !source.contains("RetainedTopologyRows"))?;
    ensure_all(&sources, |source| !source.contains("serde_json::Value"))?;
    ensure_all(&sources, |source| !source.contains(".payload"))?;
    ensure_all(&sources, |source| !source.contains("get(\"relations\")"))?;
    ensure_all(&sources, |source| {
        !source.contains("get(\"relation_identities\")")
    })?;

    closed_row(
        TopologyQueryBoundaryCleanupArea::ReadViewDecode,
        "public read views consume typed neighborhood facts and no longer traverse retained query payload maps directly",
        Some("src/projection/runtime_boundary/read_execution/neighborhood_decode/mod.rs"),
        [
            "src/projection/read_views/domain/views/adjacency.rs",
            "src/projection/read_views/domain/views/local_rewire.rs",
            "src/projection/read_views/domain/views/loop_cycle.rs",
        ],
    )
}
