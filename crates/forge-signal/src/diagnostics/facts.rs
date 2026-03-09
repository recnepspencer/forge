use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::data::aspect::Aspect;
use crate::data::handle::NodeId;
use crate::data::output::PartitionSubscription;
use crate::logic::explain::{NodeExplanation, UpstreamCause};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExplanationFact {
    pub node: NodeId,
    pub explanation: NodeExplanation,
    pub execution_record_id: Option<u64>,
    pub semantic_segment_id: Option<u64>,
    pub state: String,
    pub upstream_count: u32,
    pub propagation_suppressed: bool,
    pub changed_region_count: u32,
    pub output_change: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceFact {
    pub node: NodeId,
    pub execution_record_id: Option<u64>,
    pub semantic_segment_id: Option<u64>,
    pub vertices: Vec<ProvenanceVertex>,
    pub edges: Vec<ProvenanceEdge>,
    pub propagation_suppressed: bool,
    pub causality_kind: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceVertex {
    pub node: NodeId,
    pub role: String,
    pub state: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceEdge {
    pub kind: String,
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
}

impl ProvenanceFact {
    pub fn from_explanation(explanation: &NodeExplanation) -> Self {
        let mut vertices = BTreeMap::new();
        vertices.insert(
            explanation.node,
            ProvenanceVertex {
                node: explanation.node,
                role: "Target".to_string(),
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
                        role: "Upstream".to_string(),
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
            execution_record_id: explanation.execution_record_id,
            semantic_segment_id: explanation.semantic_segment_id,
            vertices: vertices.into_values().collect(),
            edges,
            propagation_suppressed: explanation.propagation_suppressed,
            causality_kind: explanation.causality.as_ref().map(|c| c.kind.clone()),
        }
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
                kind: "Changed".to_string(),
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
                kind: "SkippedByComparator".to_string(),
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
                condition,
                decision,
            } => Self {
                kind: format!("ConditionDeferred::{condition:?}/{decision:?}"),
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
                kind: "Clean".to_string(),
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
                kind: "MissingSnapshot".to_string(),
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
                kind: "DependencyRemoved".to_string(),
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
