use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::data::aspect::Aspect;
use crate::data::handle::NodeId;
use crate::data::node::{ContextRequirement, EvaluationCondition, NodeState};
use crate::data::output::PartitionSubscription;
use crate::data::trace::{
    assemble_historical_artifact_record, CausalityMetadata, ColdArtifactRecord,
    ExecutionTraceStamp, RuntimeArtifactState,
};
use crate::diagnostics::policy::DiagnosticsAvailability;
use crate::logic::explain::{CausalLink, RewiringSummary};
use crate::logic::explain::{NodeExplanation, UpstreamCause};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExplanationFact {
    pub node: NodeId,
    pub explanation: NodeExplanation,
    #[serde(default)]
    pub compact_projection: bool,
    pub materialization_mode: DiagnosticsAvailability,
    pub execution_record_id: Option<u64>,
    pub semantic_segment_id: Option<u64>,
    pub state: String,
    pub upstream_count: u32,
    pub propagation_suppressed: bool,
    pub changed_region_count: u32,
    pub output_change: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProvenanceFact {
    pub node: NodeId,
    pub materialization_mode: DiagnosticsAvailability,
    pub execution_record_id: Option<u64>,
    pub semantic_segment_id: Option<u64>,
    pub vertices: Vec<ProvenanceVertex>,
    pub edges: Vec<ProvenanceEdge>,
    pub causal_links: Vec<CausalLink>,
    pub rewiring: Option<RewiringSummary>,
    pub propagation_suppressed: bool,
    pub causality_kind: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ProvenanceVertexRole {
    Target,
    Upstream,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ProvenanceEdgeKind {
    Changed,
    SkippedByComparator,
    ConditionDeferred,
    Clean,
    MissingSnapshot,
    DependencyRemoved,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceVertex {
    pub node: NodeId,
    pub role: ProvenanceVertexRole,
    pub state: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceEdge {
    pub kind: ProvenanceEdgeKind,
    pub source: NodeId,
    pub aspect: Aspect,
    pub subscription: Option<PartitionSubscription>,
    pub cached_version: Option<u64>,
    pub current_version: Option<u64>,
    pub comparator: Option<String>,
    pub reason: Option<String>,
}

impl ExplanationFact {
    pub fn from_explanation(explanation: &NodeExplanation) -> Self {
        Self {
            node: explanation.node,
            explanation: explanation.clone(),
            compact_projection: false,
            materialization_mode: explanation.materialization_mode,
            execution_record_id: explanation.execution_record_id,
            semantic_segment_id: explanation.semantic_segment_id,
            state: format!("{:?}", explanation.state),
            upstream_count: explanation.upstream.len() as u32,
            propagation_suppressed: explanation.propagation_suppressed,
            changed_region_count: explanation.changed_regions.len() as u32,
            output_change: explanation
                .output_change
                .map(|change| format!("{change:?}")),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_runtime_projection(
        node: NodeId,
        state: NodeState,
        contract_reads: crate::data::aspect::AspectMask,
        contract_produces: crate::data::aspect::AspectMask,
        contract_partition_scope: Option<Vec<PartitionSubscription>>,
        required_context: ContextRequirement,
        condition: EvaluationCondition,
        runtime: &RuntimeArtifactState,
        retained: Option<&ColdArtifactRecord>,
        execution: Option<ExecutionTraceStamp>,
        causality: Option<&CausalityMetadata>,
        rewiring: Option<RewiringSummary>,
    ) -> Self {
        let mut fact = Self::from_explanation(&compact_retained_explanation(
            node,
            state,
            contract_reads,
            contract_produces,
            contract_partition_scope,
            required_context,
            condition,
            runtime,
            retained,
            execution,
            causality,
            rewiring,
        ));
        fact.compact_projection = true;
        fact
    }
}

impl ProvenanceFact {
    pub fn from_explanation(explanation: &NodeExplanation) -> Self {
        let mut vertices = BTreeMap::new();
        vertices.insert(
            explanation.node,
            ProvenanceVertex {
                node: explanation.node,
                role: ProvenanceVertexRole::Target,
                state: Some(format!("{:?}", explanation.state)),
            },
        );
        let mut edges = explanation
            .upstream
            .iter()
            .map(|cause| {
                let edge = ProvenanceEdge::from_upstream_cause(cause);
                vertices
                    .entry(edge.source)
                    .or_insert_with(|| ProvenanceVertex {
                        node: edge.source,
                        role: ProvenanceVertexRole::Upstream,
                        state: None,
                    });
                edge
            })
            .collect::<Vec<_>>();
        edges.sort_by_key(|edge| {
            (
                edge.source.index(),
                edge.source.generation(),
                edge.aspect.index(),
                edge.kind.clone(),
                edge.subscription
                    .as_ref()
                    .map(|scope| {
                        (
                            scope.partition.0.clone(),
                            scope.detail.clone().unwrap_or_default(),
                            scope.match_mode as u8,
                        )
                    })
                    .unwrap_or_default(),
            )
        });
        Self {
            node: explanation.node,
            materialization_mode: explanation.materialization_mode,
            execution_record_id: explanation.execution_record_id,
            semantic_segment_id: explanation.semantic_segment_id,
            vertices: vertices.into_values().collect(),
            edges,
            causal_links: explanation.causal_links.clone(),
            rewiring: explanation.rewiring.clone(),
            propagation_suppressed: explanation.propagation_suppressed,
            causality_kind: explanation.causality.as_ref().map(|c| c.kind.clone()),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_runtime_projection(
        node: NodeId,
        state: NodeState,
        contract_reads: crate::data::aspect::AspectMask,
        contract_produces: crate::data::aspect::AspectMask,
        contract_partition_scope: Option<Vec<PartitionSubscription>>,
        required_context: ContextRequirement,
        condition: EvaluationCondition,
        runtime: &RuntimeArtifactState,
        retained: Option<&ColdArtifactRecord>,
        execution: Option<ExecutionTraceStamp>,
        causality: Option<&CausalityMetadata>,
        rewiring: Option<RewiringSummary>,
    ) -> Self {
        Self::from_explanation(&compact_retained_explanation(
            node,
            state,
            contract_reads,
            contract_produces,
            contract_partition_scope,
            required_context,
            condition,
            runtime,
            retained,
            execution,
            causality,
            rewiring,
        ))
    }
}

#[allow(clippy::too_many_arguments)]
fn compact_retained_explanation(
    node: NodeId,
    state: NodeState,
    contract_reads: crate::data::aspect::AspectMask,
    contract_produces: crate::data::aspect::AspectMask,
    contract_partition_scope: Option<Vec<PartitionSubscription>>,
    required_context: ContextRequirement,
    condition: EvaluationCondition,
    runtime: &RuntimeArtifactState,
    retained: Option<&ColdArtifactRecord>,
    execution: Option<ExecutionTraceStamp>,
    causality: Option<&CausalityMetadata>,
    rewiring: Option<RewiringSummary>,
) -> NodeExplanation {
    NodeExplanation {
        node,
        materialization_mode: DiagnosticsAvailability::RetainedAvailable,
        state,
        dirty_aspects: Default::default(),
        contract_reads,
        contract_produces,
        contract_partition_scope,
        required_context,
        condition,
        historical_artifact_record: assemble_historical_artifact_record(
            node,
            Some(runtime),
            retained,
            causality,
        ),
        execution_record_id: execution.and_then(|stamp| stamp.execution_record_id),
        semantic_segment_id: execution.and_then(|stamp| stamp.semantic_segment_id),
        output_identity: runtime.output_identity.clone(),
        output_change: Some(runtime.output_change),
        changed_regions: retained
            .map(|artifact| artifact.changed_regions.as_slice().to_vec())
            .unwrap_or_default(),
        propagation_suppressed: runtime.propagation_suppressed,
        memoized_origin: Some(runtime.memoized_origin),
        reuse_basis: Some(runtime.reuse_basis.clone_inner()),
        reuse_origin: Some(runtime.reuse_origin),
        reuse_certification: retained.and_then(|artifact| artifact.reuse_certification.clone()),
        upstream: Vec::new(),
        causal_links: Vec::new(),
        rewiring,
        causality: causality.cloned(),
    }
}

impl ProvenanceEdge {
    fn from_upstream_cause(cause: &UpstreamCause) -> Self {
        match cause {
            UpstreamCause::Changed {
                source,
                aspect,
                subscription,
                cached_version,
                current_version,
                comparator,
                reason,
            } => Self {
                kind: ProvenanceEdgeKind::Changed,
                source: *source,
                aspect: *aspect,
                subscription: subscription.clone(),
                cached_version: Some(*cached_version),
                current_version: Some(*current_version),
                comparator: Some(format!("{comparator:?}")),
                reason: Some(format!("{reason:?}")),
            },
            UpstreamCause::SkippedByComparator {
                source,
                aspect,
                subscription,
                cached_version,
                current_version,
                comparator,
                reason,
            } => Self {
                kind: ProvenanceEdgeKind::SkippedByComparator,
                source: *source,
                aspect: *aspect,
                subscription: subscription.clone(),
                cached_version: Some(*cached_version),
                current_version: Some(*current_version),
                comparator: Some(format!("{comparator:?}")),
                reason: Some(format!("{reason:?}")),
            },
            UpstreamCause::ConditionDeferred {
                source,
                aspect,
                subscription,
                cached_version,
                current_version,
                condition: _,
                decision: _,
            } => Self {
                kind: ProvenanceEdgeKind::ConditionDeferred,
                source: *source,
                aspect: *aspect,
                subscription: subscription.clone(),
                cached_version: Some(*cached_version),
                current_version: Some(*current_version),
                comparator: None,
                reason: None,
            },
            UpstreamCause::Clean {
                source,
                aspect,
                subscription,
                cached_version,
                current_version,
            } => Self {
                kind: ProvenanceEdgeKind::Clean,
                source: *source,
                aspect: *aspect,
                subscription: subscription.clone(),
                cached_version: Some(*cached_version),
                current_version: Some(*current_version),
                comparator: None,
                reason: None,
            },
            UpstreamCause::MissingSnapshot {
                source,
                aspect,
                subscription,
                current_version,
            } => Self {
                kind: ProvenanceEdgeKind::MissingSnapshot,
                source: *source,
                aspect: *aspect,
                subscription: subscription.clone(),
                cached_version: None,
                current_version: *current_version,
                comparator: None,
                reason: None,
            },
            UpstreamCause::DependencyRemoved {
                source,
                aspect,
                subscription,
                cached_version,
            } => Self {
                kind: ProvenanceEdgeKind::DependencyRemoved,
                source: *source,
                aspect: *aspect,
                subscription: subscription.clone(),
                cached_version: Some(*cached_version),
                current_version: None,
                comparator: None,
                reason: None,
            },
        }
    }
}

pub type ExplanationFactTable = BTreeMap<NodeId, ExplanationFact>;
pub type ProvenanceFactTable = BTreeMap<NodeId, ProvenanceFact>;
