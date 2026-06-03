use crate::certification::error::TopologyCertificationError;

use super::super::support::{closed_row, ensure_all, source_text};
use super::super::TopologyQueryBoundaryCleanupArea;

pub(crate) fn certify_operator_path_row(
) -> Result<super::super::TopologyQueryBoundaryCleanupRow, TopologyCertificationError> {
    let sources = [
        source_text("src/topology_operators/application/mod.rs")?,
        source_text("src/topology_operators/application/admission.rs")?,
        source_text("src/topology_operators/application/bindings.rs")?,
        source_text("src/topology_operators/application/existing_truth.rs")?,
    ];
    ensure_all(&sources, |source| !source.contains(".payload"))?;
    ensure_all(&sources[..2], |source| {
        !source.contains("workspace.materialize(") && !source.contains("serde_json::from_value")
    })?;
    closed_row(
        TopologyQueryBoundaryCleanupArea::OperatorPath,
        "operator path depends on typed binding facts and shared post-write consumption instead of raw row archaeology",
        Some("src/projection/runtime_boundary/query_runtime/operator_bindings.rs"),
        [
            "src/topology_operators/application/mod.rs",
            "src/topology_operators/application/bindings.rs",
        ],
    )
}
