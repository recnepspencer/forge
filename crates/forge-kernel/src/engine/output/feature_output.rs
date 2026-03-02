//! Feature output data type.
//!
//! DOMAIN: The domain output of a feature evaluation.
//! Audit metadata lives in the `OperationResult` envelope, not here.

use serde::{Deserialize, Serialize};

use forge_topo::transactions::TopologyState;

use crate::geometry::facade::GeometryStore;

/// The output of a feature evaluation.
///
/// Domain-only data — audit metadata (decisions, replay, lineage) lives
/// in the `OperationResult` envelope that wraps this, not in the output itself.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureOutput {
    /// The resulting topology (snapshot).
    pub topology: TopologyState,
    /// The resulting unified geometry.
    pub geometry: GeometryStore,
}
