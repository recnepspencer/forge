//! Domain-free trace payloads for per-node evaluation metadata.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::data::core_profile::StableHashValue;
use crate::data::output::{ChangedRegion, MemoizedResultOrigin, OutputChange, OutputIdentity};
use crate::diagnostics::lineage::LineageArtifactId;

/// Lightweight evaluation trace summary for one node recomputation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TraceSummary {
    /// Opaque deterministic hash for the evaluated output.
    pub output_hash: StableHashValue,
    /// Optional stable identity for the evaluated output artifact.
    #[serde(default)]
    pub output_identity: Option<OutputIdentity>,
    /// Runtime-normalized output change classification.
    #[serde(default)]
    pub output_change: OutputChange,
    /// Whether this node executed `compute` during the last evaluation.
    #[serde(default)]
    pub recomputed: bool,
    /// Number of dependencies observed during the last clean evaluation.
    #[serde(default)]
    pub dependency_count: u32,
    /// Number of upstream inputs that differed from the cached snapshot.
    #[serde(default)]
    pub meaningful_input_changes: u32,
    /// Number of distinct partitions reported as changed.
    #[serde(default)]
    pub changed_partition_count: u32,
    /// Whether downstream invalidation was suppressed after evaluation.
    #[serde(default)]
    pub propagation_suppressed: bool,
    /// Generic changed-region metadata for partition-aware nodes.
    #[serde(default)]
    pub changed_regions: Vec<ChangedRegion>,
    /// Family namespace for keyed computations, when relevant.
    #[serde(default)]
    pub keyed_family: Option<String>,
    /// Key inside the computation family, when relevant.
    #[serde(default)]
    pub keyed_key: Option<String>,
    /// How the last result was produced.
    #[serde(default)]
    pub memoized_origin: MemoizedResultOrigin,
    /// Optional structured labels for diagnostics.
    #[serde(default)]
    pub labels: Vec<String>,
    /// Last planner/execution record id that touched this node, when available.
    #[serde(default)]
    pub execution_record_id: Option<u64>,
    /// Semantic segment id that produced the last trace, when available.
    #[serde(default)]
    pub semantic_segment_id: Option<u64>,
    /// Current signal-lineage artifact id for this node's evaluated artifact.
    #[serde(default)]
    pub lineage_artifact_id: Option<LineageArtifactId>,
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
