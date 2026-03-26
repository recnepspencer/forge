//! Domain-free trace payloads for per-node evaluation metadata.

use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::data::core_profile::StableHashValue;
use crate::data::handle::NodeId;
use crate::data::output::{
    ArtifactContinuityToken, CanonicalChangedRegions, ChangedRegion, MemoizedResultOrigin,
    OutputChange, OutputIdentity, PartitionSubscription,
};
use crate::data::proof::PartitionScopeSet;
use crate::data::reuse::{
    ReuseBasis, ReuseBoundaryAuthority, ReuseBoundaryContext, ReuseCertificationRecord,
    ReuseOrigin,
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

/// Cold execution/segment metadata used by diagnostics and replay-facing
/// summaries, but not required for hot mutation semantics.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ExecutionTraceStamp {
    #[serde(default)]
    pub execution_record_id: Option<u64>,
    #[serde(default)]
    pub semantic_segment_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(transparent)]
pub struct ContinuityAuthorityToken(
    Option<ArtifactContinuityToken>,
);

impl ContinuityAuthorityToken {
    pub fn new(token: Option<ArtifactContinuityToken>) -> Self {
        Self(token)
    }

    pub fn as_ref(&self) -> Option<&ArtifactContinuityToken> {
        self.0.as_ref()
    }

    pub fn clone_inner(&self) -> Option<ArtifactContinuityToken> {
        self.0.clone()
    }

    pub fn into_inner(self) -> Option<ArtifactContinuityToken> {
        self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(transparent)]
pub struct CompactChangedScopeProof(
    PartitionScopeSet,
);

impl CompactChangedScopeProof {
    pub fn new(scopes: PartitionScopeSet) -> Self {
        Self(scopes)
    }

    pub fn as_slice(&self) -> &[crate::data::output::PartitionSubscription] {
        self.0.as_slice()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clone_inner(&self) -> PartitionScopeSet {
        self.0.clone()
    }
}

impl Deref for CompactChangedScopeProof {
    type Target = PartitionScopeSet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CompactChangedScopeProof {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(transparent)]
pub struct ReuseOperationalBasis(
    ReuseBasis,
);

impl ReuseOperationalBasis {
    pub fn new(basis: ReuseBasis) -> Self {
        Self(basis)
    }

    pub fn clone_inner(&self) -> ReuseBasis {
        self.0.clone()
    }
}

impl Deref for ReuseOperationalBasis {
    type Target = ReuseBasis;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ReuseOperationalBasis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(transparent)]
pub struct ArtifactTransitionKey(
    Option<LineageArtifactId>,
);

impl ArtifactTransitionKey {
    pub fn new(artifact_id: Option<LineageArtifactId>) -> Self {
        Self(artifact_id)
    }

    pub fn get(self) -> Option<LineageArtifactId> {
        self.0
    }

    pub fn set(&mut self, artifact_id: Option<LineageArtifactId>) {
        self.0 = artifact_id;
    }
}

pub(crate) const COLD_ARTIFACT_INTENT_LABEL_LIMIT: usize = 4;

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
    pub continuity_token: ContinuityAuthorityToken,
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
    pub changed_scopes: CompactChangedScopeProof,
    /// How the last result was produced.
    #[serde(default)]
    pub memoized_origin: MemoizedResultOrigin,
    /// Compact runtime truth for how the current artifact became current.
    #[serde(default)]
    pub reuse_basis: ReuseOperationalBasis,
    /// Realized runtime origin for how the current artifact became current.
    #[serde(default)]
    pub reuse_origin: ReuseOrigin,
    /// Compact hot authority for certifying later reuse of this artifact.
    #[serde(default)]
    pub reuse_boundary_authority: Option<ReuseBoundaryAuthority>,
    /// Current signal-lineage artifact id for this node's evaluated artifact.
    #[serde(default)]
    pub lineage_artifact_id: ArtifactTransitionKey,
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
    /// Rich cold reuse boundary detail retained for explanation/forensics.
    #[serde(default)]
    pub reuse_boundary_context: Option<ReuseBoundaryContext>,
}

/// Explicit write packet for artifact hot/cold lanes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ArtifactWriteDelta {
    #[serde(default)]
    pub runtime: Option<RuntimeArtifactState>,
    #[serde(default)]
    pub retained: Option<RetainedDiagnosticArtifact>,
}

/// Explicit hot-lane write packet for runtime artifact state updates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct HotArtifactWrite {
    #[serde(default)]
    pub runtime: Option<RuntimeArtifactState>,
    #[serde(default)]
    pub cold_intent: Option<ColdArtifactIntent>,
}

/// Bounded cold-path seed emitted by the hot execution lane.
///
/// This is intentionally smaller in semantic scope than a fully materialized
/// retained artifact record. It carries only the canonical cold facts we may
/// choose to retain eagerly under the active diagnostics policy.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ColdArtifactIntent {
    #[serde(default)]
    pub changed_regions: CanonicalChangedRegions,
    #[serde(default)]
    pub labels: SmallVec<[String; COLD_ARTIFACT_INTENT_LABEL_LIMIT]>,
    #[serde(default)]
    pub keyed_family: Option<String>,
    #[serde(default)]
    pub keyed_key: Option<String>,
    #[serde(default)]
    pub reuse_certification: Option<ReuseCertificationRecord>,
    #[serde(default)]
    pub reuse_boundary_context: Option<ReuseBoundaryContext>,
}

/// Cold retained record kept off the operational hot path.
pub type ColdArtifactRecord = RetainedDiagnosticArtifact;

impl ColdArtifactIntent {
    pub fn is_empty(&self) -> bool {
        self.changed_regions.is_empty()
            && self.labels.is_empty()
            && self.keyed_family.is_none()
            && self.keyed_key.is_none()
            && self.reuse_certification.is_none()
            && self.reuse_boundary_context.is_none()
    }

    pub fn materialize_record(self) -> Option<ColdArtifactRecord> {
        if self.is_empty() {
            return None;
        }
        Some(ColdArtifactRecord {
            changed_regions: self.changed_regions,
            labels: self.labels.into_vec(),
            keyed_family: self.keyed_family,
            keyed_key: self.keyed_key,
            reuse_certification: self.reuse_certification,
            reuse_boundary_context: self.reuse_boundary_context,
        })
    }
}

/// Cold historical artifact record assembled for explanation, lineage
/// expansion, and retained reporting surfaces.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HistoricalArtifactRecord {
    pub node: NodeId,
    pub runtime: RuntimeArtifactState,
    #[serde(default)]
    pub retained: Option<ColdArtifactRecord>,
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
        retained: Option<&ColdArtifactRecord>,
        execution: Option<ExecutionTraceStamp>,
    ) -> Self {
        Self {
            output_hash: runtime.output_hash,
            output_identity: runtime.output_identity.clone(),
            continuity_token: runtime.continuity_token.clone_inner(),
            output_change: runtime.output_change,
            recomputed: runtime.recomputed,
            dependency_count: runtime.dependency_count,
            meaningful_input_changes: runtime.meaningful_input_changes,
            changed_partition_count: runtime.changed_partition_count,
            propagation_suppressed: runtime.propagation_suppressed,
            changed_regions: retained
                .map(|artifact| artifact.changed_regions.as_slice().to_vec())
                .unwrap_or_else(|| scopes_to_regions_from_slice(runtime.changed_scopes.as_slice())),
            keyed_family: retained.and_then(|artifact| artifact.keyed_family.clone()),
            keyed_key: retained.and_then(|artifact| artifact.keyed_key.clone()),
            memoized_origin: runtime.memoized_origin,
            reuse_basis: runtime.reuse_basis.clone_inner(),
            reuse_origin: runtime.reuse_origin,
            reuse_boundary_context: retained
                .and_then(|artifact| artifact.reuse_boundary_context.clone()),
            labels: retained
                .map(|artifact| artifact.labels.clone())
                .unwrap_or_default(),
            execution_record_id: execution.and_then(|stamp| stamp.execution_record_id),
            semantic_segment_id: execution.and_then(|stamp| stamp.semantic_segment_id),
            lineage_artifact_id: runtime.lineage_artifact_id.get(),
        }
    }

    pub fn from_record(record: &HistoricalArtifactRecord) -> Self {
        Self::from_parts(&record.runtime, record.retained.as_ref(), None)
    }
}

pub fn assemble_historical_artifact_record(
    node: NodeId,
    runtime: Option<&RuntimeArtifactState>,
    retained: Option<&ColdArtifactRecord>,
    causality: Option<&CausalityMetadata>,
) -> Option<HistoricalArtifactRecord> {
    Some(HistoricalArtifactRecord {
        node,
        runtime: runtime?.clone(),
        retained: retained.cloned(),
        causality: causality.cloned(),
    })
}

#[allow(dead_code)]
pub fn assemble_trace_summary(
    runtime: Option<&RuntimeArtifactState>,
    retained: Option<&ColdArtifactRecord>,
) -> Option<TraceSummary> {
    assemble_trace_summary_with_execution(runtime, retained, None)
}

pub fn assemble_trace_summary_with_execution(
    runtime: Option<&RuntimeArtifactState>,
    retained: Option<&ColdArtifactRecord>,
    execution: Option<ExecutionTraceStamp>,
) -> Option<TraceSummary> {
    Some(TraceSummary::from_parts(runtime?, retained, execution))
}

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticArtifactParity {
    output_hash: StableHashValue,
    output_identity: Option<OutputIdentity>,
    continuity_token: Option<ArtifactContinuityToken>,
    output_change: OutputChange,
    recomputed: bool,
    dependency_count: u32,
    meaningful_input_changes: u32,
    changed_partition_count: u32,
    propagation_suppressed: bool,
    changed_regions: Vec<ChangedRegion>,
    keyed_family: Option<String>,
    keyed_key: Option<String>,
    memoized_origin: MemoizedResultOrigin,
    reuse_basis: ReuseBasis,
    reuse_origin: ReuseOrigin,
    reuse_boundary_context: Option<ReuseBoundaryContext>,
    labels: Vec<String>,
    artifact_transition_key: ArtifactTransitionKey,
}

#[cfg_attr(not(test), allow(dead_code))]
impl SemanticArtifactParity {
    pub fn from_historical_artifact_record(record: &HistoricalArtifactRecord) -> Self {
        Self::from_runtime_and_cold(&record.runtime, record.retained.as_ref())
    }

    pub fn from_trace_summary(summary: &TraceSummary) -> Self {
        let mut changed_regions = summary.changed_regions.clone();
        changed_regions.sort();
        Self {
            output_hash: summary.output_hash,
            output_identity: summary.output_identity.clone(),
            continuity_token: summary.continuity_token.clone(),
            output_change: summary.output_change,
            recomputed: summary.recomputed,
            dependency_count: summary.dependency_count,
            meaningful_input_changes: summary.meaningful_input_changes,
            changed_partition_count: summary.changed_partition_count,
            propagation_suppressed: summary.propagation_suppressed,
            changed_regions,
            keyed_family: summary.keyed_family.clone(),
            keyed_key: summary.keyed_key.clone(),
            memoized_origin: summary.memoized_origin,
            reuse_basis: summary.reuse_basis.clone(),
            reuse_origin: summary.reuse_origin,
            reuse_boundary_context: summary.reuse_boundary_context.clone(),
            labels: summary.labels.clone(),
            artifact_transition_key: ArtifactTransitionKey::new(summary.lineage_artifact_id),
        }
    }

    fn from_runtime_and_cold(
        runtime: &RuntimeArtifactState,
        retained: Option<&ColdArtifactRecord>,
    ) -> Self {
        let mut changed_regions = retained
            .map(|artifact| artifact.changed_regions.as_slice().to_vec())
            .unwrap_or_else(|| scopes_to_regions_from_slice(runtime.changed_scopes.as_slice()));
        changed_regions.sort();
        Self {
            output_hash: runtime.output_hash,
            output_identity: runtime.output_identity.clone(),
            continuity_token: runtime.continuity_token.clone_inner(),
            output_change: runtime.output_change,
            recomputed: runtime.recomputed,
            dependency_count: runtime.dependency_count,
            meaningful_input_changes: runtime.meaningful_input_changes,
            changed_partition_count: runtime.changed_partition_count,
            propagation_suppressed: runtime.propagation_suppressed,
            changed_regions,
            keyed_family: retained.and_then(|artifact| artifact.keyed_family.clone()),
            keyed_key: retained.and_then(|artifact| artifact.keyed_key.clone()),
            memoized_origin: runtime.memoized_origin,
            reuse_basis: runtime.reuse_basis.clone_inner(),
            reuse_origin: runtime.reuse_origin,
            reuse_boundary_context: retained
                .and_then(|artifact| artifact.reuse_boundary_context.clone()),
            labels: retained
                .map(|artifact| artifact.labels.clone())
                .unwrap_or_default(),
            artifact_transition_key: runtime.lineage_artifact_id,
        }
    }
}

fn scopes_to_regions_from_slice(scopes: &[PartitionSubscription]) -> Vec<ChangedRegion> {
    scopes
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
