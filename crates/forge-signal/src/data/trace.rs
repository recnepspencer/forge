//! Domain-free trace summary payload for per-node evaluation metadata.

use serde::{Deserialize, Serialize};

/// Lightweight evaluation trace summary for one node recomputation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TraceSummary {
    /// Opaque deterministic hash for the evaluated output.
    pub output_hash: u128,
    /// Optional structured labels for diagnostics.
    pub labels: Vec<String>,
}

