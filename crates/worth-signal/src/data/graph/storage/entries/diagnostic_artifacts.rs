use crate::data::error::SignalError;
use crate::data::graph::signal_graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::trace::{
    CausalityMetadata, ColdArtifactRecord, ExecutionTraceStamp, RetainedDiagnosticArtifact,
    TraceSummary,
};

use super::NodeReplayProjection;

impl SignalGraph {
    pub fn set_trace_summary(
        &mut self,
        id: NodeId,
        summary: Option<TraceSummary>,
    ) -> Result<(), SignalError> {
        let mut entry = self.get_entry_mut(id)?;
        entry.set_retained_diagnostic_artifact(summary.map(|summary| RetainedDiagnosticArtifact {
            labels: summary.labels,
            ..RetainedDiagnosticArtifact::default()
        }));
        Ok(())
    }

    pub fn causality_of(&self, node: NodeId) -> Result<Option<&CausalityMetadata>, SignalError> {
        Ok(self
            .cold_ref(node)?
            .and_then(|cold| cold.causality.as_ref()))
    }

    pub(crate) fn node_execution_trace_stamp(
        &self,
        node: NodeId,
    ) -> Result<Option<ExecutionTraceStamp>, SignalError> {
        Ok(self.cold_ref(node)?.and_then(|cold| cold.execution_trace))
    }

    pub(crate) fn node_retained_diagnostic_artifact(
        &self,
        node: NodeId,
    ) -> Result<Option<&RetainedDiagnosticArtifact>, SignalError> {
        crate::data::access_counters::note_retained_artifact_read();
        Ok(self
            .cold_ref(node)?
            .and_then(|cold| cold.retained_artifact.as_ref()))
    }

    pub(crate) fn node_cold_artifact_record(
        &self,
        node: NodeId,
    ) -> Result<Option<&ColdArtifactRecord>, SignalError> {
        Ok(self
            .cold_ref(node)?
            .and_then(|cold| cold.retained_artifact.as_ref()))
    }

    pub(crate) fn node_lineage_artifact_id(
        &self,
        node: NodeId,
    ) -> Result<Option<crate::diagnostics::lineage::LineageArtifactId>, SignalError> {
        Ok(self
            .warm_ref(node)?
            .runtime_artifact_state
            .as_ref()
            .and_then(|state| state.lineage_artifact_id().get()))
    }

    pub(crate) fn node_replay_projection(
        &self,
        node: NodeId,
    ) -> Result<NodeReplayProjection, SignalError> {
        let runtime_artifact_state = self.warm_ref(node)?.runtime_artifact_state.as_ref();
        let lineage_artifact_id =
            runtime_artifact_state.and_then(|state| state.lineage_artifact_id().get());
        let (persistent_correspondence_kind, composition_region_count) = runtime_artifact_state
            .and_then(|state| state.reuse_boundary_authority())
            .map(|authority| {
                (
                    authority.persistent_correspondence_kind(),
                    authority.composition_region_count(),
                )
            })
            .unwrap_or((None, 0));
        Ok(NodeReplayProjection {
            lineage_artifact_id,
            persistent_correspondence_kind,
            composition_region_count: (composition_region_count > 0)
                .then_some(composition_region_count),
        })
    }

    pub fn set_causality(
        &mut self,
        node: NodeId,
        causality: Option<CausalityMetadata>,
    ) -> Result<(), SignalError> {
        if causality.is_some() {
            self.cold_mut(node)?.causality = causality;
        } else if let Some(cold) = self.arena.cold[node.index() as usize].as_mut() {
            cold.causality = None;
        }
        self.trim_cold_if_empty(node);
        self.record_branch_mutation_causality(node);
        Ok(())
    }

    pub(crate) fn stamp_runtime_artifact_lineage_and_execution(
        &mut self,
        node: NodeId,
        artifact_id: crate::diagnostics::lineage::LineageArtifactId,
        execution_record_id: crate::logic::planner::ExecutionRecordId,
        semantic_segment_id: crate::logic::planner::SemanticSegmentId,
    ) -> Result<(), SignalError> {
        let Some(runtime) = self.warm_mut(node)?.runtime_artifact_state.as_mut() else {
            return Ok(());
        };
        runtime.set_lineage_artifact_id(Some(artifact_id));
        self.cold_mut(node)?.execution_trace = Some(ExecutionTraceStamp {
            execution_record_id: Some(execution_record_id.0),
            semantic_segment_id: Some(semantic_segment_id.0),
        });
        Ok(())
    }
}
