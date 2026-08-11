use crate::data::aspect::AspectMask;
use crate::data::handle::NodeId;
use crate::data::node::{ContextRequirement, EvaluationCondition, NodeState};
use crate::data::trace::{
    assemble_historical_artifact_record, CausalityMetadata, ColdArtifactRecord,
    ExecutionTraceStamp, RuntimeArtifactState,
};
use crate::diagnostics::policy::DiagnosticsAvailability;
use crate::logic::explain::{NodeExplanation, RewiringSummary};

pub(super) fn compact_retained_explanation(
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
        output_identity: runtime.output_identity().cloned(),
        output_change: Some(runtime.output_change()),
        changed_regions: retained
            .map(|artifact| artifact.changed_regions.as_slice().to_vec())
            .unwrap_or_default(),
        propagation_suppressed: runtime.propagation_suppressed(),
        memoized_origin: Some(runtime.memoized_origin()),
        reuse_basis: Some(runtime.reuse_basis().clone_inner()),
        reuse_origin: Some(runtime.reuse_origin()),
        reuse_certification: retained.and_then(|artifact| artifact.reuse_certification.clone()),
        upstream: Vec::new(),
        causal_links: Vec::new(),
        rewiring,
        causality: causality.cloned(),
    }
}
