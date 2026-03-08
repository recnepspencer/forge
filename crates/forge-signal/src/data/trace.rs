//! Domain-free trace payloads for per-node evaluation metadata.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Lightweight evaluation trace summary for one node recomputation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TraceSummary {
    /// Opaque deterministic hash for the evaluated output.
    pub output_hash: u128,
    /// Whether this node executed `compute` during the last evaluation.
    #[serde(default)]
    pub recomputed: bool,
    /// Number of dependencies observed during the last clean evaluation.
    #[serde(default)]
    pub dependency_count: u32,
    /// Number of upstream inputs that differed from the cached snapshot.
    #[serde(default)]
    pub meaningful_input_changes: u32,
    /// Optional structured labels for diagnostics.
    #[serde(default)]
    pub labels: Vec<String>,
}

/// Opaque structured causality payload for host-provided provenance.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct CausalityMetadata {
    /// Stable kind label for the payload producer.
    pub kind: String,
    /// Opaque string fields surfaced in explanations and debug output.
    #[serde(default)]
    pub fields: BTreeMap<String, String>,
}
