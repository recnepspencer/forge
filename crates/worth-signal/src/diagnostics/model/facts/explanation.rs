use crate::data::aspect::AspectMask;
use crate::data::handle::NodeId;
use crate::data::node::{ContextRequirement, EvaluationCondition, NodeState};
use crate::data::trace::{
    CausalityMetadata, ColdArtifactRecord, ExecutionTraceStamp, RuntimeArtifactState,
};
use crate::logic::explain::{NodeExplanation, RewiringSummary};

use super::projection::compact_retained_explanation;
use super::vocabulary::ExplanationFact;

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

    pub fn from_runtime_projection(
        node: NodeId,
        state: NodeState,
        contract_reads: AspectMask,
        contract_produces: AspectMask,
        contract_partition_scope: Option<Vec<crate::data::output::PartitionSubscription>>,
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

    pub(crate) fn compact_explanation_from_runtime_projection(
        node: NodeId,
        state: NodeState,
        contract_reads: AspectMask,
        contract_produces: AspectMask,
        contract_partition_scope: Option<Vec<crate::data::output::PartitionSubscription>>,
        required_context: ContextRequirement,
        condition: EvaluationCondition,
        runtime: &RuntimeArtifactState,
        retained: Option<&ColdArtifactRecord>,
        execution: Option<ExecutionTraceStamp>,
        causality: Option<&CausalityMetadata>,
        rewiring: Option<RewiringSummary>,
    ) -> NodeExplanation {
        compact_retained_explanation(
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
        )
    }
}
