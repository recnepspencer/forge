//! Domain-free trace payloads for per-node evaluation metadata.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::data::core_profile::StableHashValue;
use crate::data::handle::NodeId;
use crate::data::output::{
    ArtifactContinuityToken, CanonicalChangedRegions, ChangedRegion, MemoizedResultOrigin,
    OutputChange, OutputIdentity,
};
use crate::data::proof::PartitionScopeSet;
use crate::data::reuse::{
    ReuseBasis, ReuseBoundaryContext, ReuseCertificationRecord, ReuseOrigin,
};
use crate::diagnostics::lineage::LineageArtifactId;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ArtifactAuthorityClass {
    #[default]
    TargetAuthoritative,
    BranchLocalSpeculative,
    DerivedOnly,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MergeAdoptability {
    #[default]
    Adoptable,
    NonAdoptableBranchLocal,
    NonAdoptableDerivedOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ArtifactMergeAuthority {
    #[serde(default)]
    pub authority_class: ArtifactAuthorityClass,
    #[serde(default)]
    pub adoptability: MergeAdoptability,
}

/// Hot operational artifact state retained directly on the node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct RuntimeArtifactState {
    /// Opaque deterministic hash for the evaluated output.
    pub output_hash: StableHashValue,
    /// Optional stable identity for the evaluated output artifact.
    #[serde(default)]
    pub output_identity: Option<OutputIdentity>,
    /// Optional host-defined continuity token for lineage preservation.
    #[serde(default)]
    pub continuity_token: Option<ArtifactContinuityToken>,
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
    /// Narrowed locality proof for partition-aware runtime behavior.
    #[serde(default)]
    pub changed_scopes: PartitionScopeSet,
    /// How the last result was produced.
    #[serde(default)]
    pub memoized_origin: MemoizedResultOrigin,
    /// Compact runtime truth for how the current artifact became current.
    #[serde(default)]
    pub reuse_basis: ReuseBasis,
    /// Realized runtime origin for how the current artifact became current.
    #[serde(default)]
    pub reuse_origin: ReuseOrigin,
    /// Boundary evidence for certifying later reuse of this artifact.
    #[serde(default)]
    pub reuse_boundary_context: Option<ReuseBoundaryContext>,
    /// Last planner/execution record id that touched this node, when available.
    #[serde(default)]
    pub execution_record_id: Option<u64>,
    /// Semantic segment id that produced the last trace, when available.
    #[serde(default)]
    pub semantic_segment_id: Option<u64>,
    /// Current signal-lineage artifact id for this node's evaluated artifact.
    #[serde(default)]
    pub lineage_artifact_id: Option<LineageArtifactId>,
    /// Typed authority/adoptability truth used by branch merge semantics.
    #[serde(default)]
    pub merge_authority: ArtifactMergeAuthority,
}

/// Cold retained artifact richness kept off the operational hot path.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct RetainedDiagnosticArtifact {
    /// Generic changed-region metadata retained for diagnostics and explain
    /// reconstruction.
    #[serde(default)]
    pub changed_regions: CanonicalChangedRegions,
    /// Optional structured labels for diagnostics.
    #[serde(default)]
    pub labels: Vec<String>,
    /// Family namespace for keyed computations, when relevant.
    #[serde(default)]
    pub keyed_family: Option<String>,
    /// Key inside the computation family, when relevant.
    #[serde(default)]
    pub keyed_key: Option<String>,
    /// Full cold-path proof for why reuse was legal, when retained.
    #[serde(default)]
    pub reuse_certification: Option<ReuseCertificationRecord>,
}

/// Explicit write packet for artifact hot/cold lanes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ArtifactWriteDelta {
    #[serde(default)]
    pub runtime: Option<RuntimeArtifactState>,
    #[serde(default)]
    pub retained: Option<RetainedDiagnosticArtifact>,
}

/// Cold historical artifact record assembled for explanation, lineage
/// expansion, and retained reporting surfaces.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HistoricalArtifactRecord {
    pub node: NodeId,
    pub runtime: RuntimeArtifactState,
    #[serde(default)]
    pub retained: Option<RetainedDiagnosticArtifact>,
    #[serde(default)]
    pub causality: Option<CausalityMetadata>,
}

/// Materialized trace view assembled for explanation, comparison, and
/// historical reporting surfaces.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TraceSummary {
    pub output_hash: StableHashValue,
    #[serde(default)]
    pub output_identity: Option<OutputIdentity>,
    #[serde(default)]
    pub continuity_token: Option<ArtifactContinuityToken>,
    #[serde(default)]
    pub output_change: OutputChange,
    #[serde(default)]
    pub recomputed: bool,
    #[serde(default)]
    pub dependency_count: u32,
    #[serde(default)]
    pub meaningful_input_changes: u32,
    #[serde(default)]
    pub changed_partition_count: u32,
    #[serde(default)]
    pub propagation_suppressed: bool,
    #[serde(default)]
    pub changed_regions: Vec<ChangedRegion>,
    #[serde(default)]
    pub keyed_family: Option<String>,
    #[serde(default)]
    pub keyed_key: Option<String>,
    #[serde(default)]
    pub memoized_origin: MemoizedResultOrigin,
    #[serde(default)]
    pub reuse_basis: ReuseBasis,
    #[serde(default)]
    pub reuse_origin: ReuseOrigin,
    #[serde(default)]
    pub reuse_boundary_context: Option<ReuseBoundaryContext>,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub execution_record_id: Option<u64>,
    #[serde(default)]
    pub semantic_segment_id: Option<u64>,
    #[serde(default)]
    pub lineage_artifact_id: Option<LineageArtifactId>,
}

impl TraceSummary {
    pub fn from_parts(
        runtime: &RuntimeArtifactState,
        retained: Option<&RetainedDiagnosticArtifact>,
    ) -> Self {
        Self {
            output_hash: runtime.output_hash,
            output_identity: runtime.output_identity.clone(),
            continuity_token: runtime.continuity_token.clone(),
            output_change: runtime.output_change,
            recomputed: runtime.recomputed,
            dependency_count: runtime.dependency_count,
            meaningful_input_changes: runtime.meaningful_input_changes,
            changed_partition_count: runtime.changed_partition_count,
            propagation_suppressed: runtime.propagation_suppressed,
            changed_regions: retained
                .map(|artifact| artifact.changed_regions.as_slice().to_vec())
                .unwrap_or_else(|| scopes_to_regions(&runtime.changed_scopes)),
            keyed_family: retained.and_then(|artifact| artifact.keyed_family.clone()),
            keyed_key: retained.and_then(|artifact| artifact.keyed_key.clone()),
            memoized_origin: runtime.memoized_origin,
            reuse_basis: runtime.reuse_basis.clone(),
            reuse_origin: runtime.reuse_origin,
            reuse_boundary_context: runtime.reuse_boundary_context.clone(),
            labels: retained
                .map(|artifact| artifact.labels.clone())
                .unwrap_or_default(),
            execution_record_id: runtime.execution_record_id,
            semantic_segment_id: runtime.semantic_segment_id,
            lineage_artifact_id: runtime.lineage_artifact_id,
        }
    }

    pub fn from_record(record: &HistoricalArtifactRecord) -> Self {
        Self::from_parts(&record.runtime, record.retained.as_ref())
    }
}

pub fn assemble_historical_artifact_record(
    node: NodeId,
    runtime: Option<&RuntimeArtifactState>,
    retained: Option<&RetainedDiagnosticArtifact>,
    causality: Option<&CausalityMetadata>,
) -> Option<HistoricalArtifactRecord> {
    Some(HistoricalArtifactRecord {
        node,
        runtime: runtime?.clone(),
        retained: retained.cloned(),
        causality: causality.cloned(),
    })
}

pub fn assemble_trace_summary(
    runtime: Option<&RuntimeArtifactState>,
    retained: Option<&RetainedDiagnosticArtifact>,
) -> Option<TraceSummary> {
    Some(TraceSummary::from_parts(runtime?, retained))
}

fn scopes_to_regions(scopes: &PartitionScopeSet) -> Vec<ChangedRegion> {
    scopes
        .as_slice()
        .iter()
        .map(|scope| match &scope.detail {
            Some(detail) => ChangedRegion::new(scope.partition.clone()).with_detail(detail.clone()),
            None => ChangedRegion::new(scope.partition.clone()),
        })
        .collect()
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
