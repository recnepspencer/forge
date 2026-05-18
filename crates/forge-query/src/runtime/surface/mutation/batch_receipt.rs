use crate::identity::hash_parts;
use crate::runtime::{
    ForgeQueryAuthorityLane, ForgeQueryBatchMutationEvidence,
    ForgeQueryGraphCompositionAssumptionSummary, ForgeQueryGraphCompositionBreadth,
    ForgeQueryGraphCompositionEvidence, ForgeQueryGraphCompositionLifecycleOutcomes,
    ForgeQueryGraphCompositionLineageSummary, ForgeQueryGraphCompositionProgram,
    ForgeQueryGraphCompositionResolutionMap, ForgeQueryIntentDecisionTraceEnvelope,
    ForgeQueryIntentExecutionProvenance, ForgeQueryRuntimeError,
};

use super::batch_receipt_aggregates::{
    batch_bridge_evidence_from_receipts, derive_batch_receipt_aggregates,
};
use super::ForgeQueryWriteReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBatchWriteReceipt {
    write_receipts: Vec<ForgeQueryWriteReceipt>,
    authority_lane: ForgeQueryAuthorityLane,
    basis_lane: ForgeQueryAuthorityLane,
    batch_mutation_evidence: ForgeQueryBatchMutationEvidence,
    graph_composition_breadth: ForgeQueryGraphCompositionBreadth,
    graph_composition_program: ForgeQueryGraphCompositionProgram,
    graph_composition_resolution_map: ForgeQueryGraphCompositionResolutionMap,
    batch_digest: String,
    touched_aspect_paths: Vec<String>,
    affected_live_view_ids: Vec<String>,
    affected_derived_view_ids: Vec<String>,
    considered_computed_view_count: usize,
    considered_effect_count: usize,
    delivered_effect_count: usize,
    pending_write_intent_count: usize,
    suppressed_effect_count: usize,
    meaningful_effect_suppression_count: usize,
    effect_expression_failure_count: usize,
    refresh_fallback: bool,
    decision_trace_envelope: Option<ForgeQueryIntentDecisionTraceEnvelope>,
    execution_provenance: Option<ForgeQueryIntentExecutionProvenance>,
}

impl ForgeQueryBatchWriteReceipt {
    pub(in crate::runtime) fn new(
        write_receipts: Vec<ForgeQueryWriteReceipt>,
        authority_lane: ForgeQueryAuthorityLane,
        basis_lane: ForgeQueryAuthorityLane,
        graph_composition_breadth: ForgeQueryGraphCompositionBreadth,
        graph_composition_program: ForgeQueryGraphCompositionProgram,
        touched_aspect_paths: Vec<String>,
        affected_live_view_ids: Vec<String>,
        affected_derived_view_ids: Vec<String>,
        considered_computed_view_count: usize,
        considered_effect_count: usize,
        delivered_effect_count: usize,
        pending_write_intent_count: usize,
        suppressed_effect_count: usize,
        meaningful_effect_suppression_count: usize,
        effect_expression_failure_count: usize,
        refresh_fallback: bool,
        decision_trace_envelope: Option<ForgeQueryIntentDecisionTraceEnvelope>,
        execution_provenance: Option<ForgeQueryIntentExecutionProvenance>,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        if write_receipts.is_empty() {
            return Err(ForgeQueryRuntimeError::Workspace(
                crate::memory_workspace::ForgeQueryWorkspaceError::new(
                    "mutation batch must produce at least one write receipt",
                ),
            ));
        }
        if write_receipts
            .iter()
            .any(|receipt| receipt.authority_lane() != authority_lane)
        {
            return Err(ForgeQueryRuntimeError::Workspace(
                crate::memory_workspace::ForgeQueryWorkspaceError::new(
                    "mutation batch may not mix authority lanes",
                ),
            ));
        }

        let graph_lifecycle_digest =
            ForgeQueryGraphCompositionLifecycleOutcomes::derive(&graph_composition_program)
                .map(|outcomes| outcomes.lifecycle_digest().to_string())
                .unwrap_or_else(|| "none".to_string());
        let batch_digest = hash_parts(
            &std::iter::once("forge_query_batch_write_receipt_v1".to_string())
                .chain(std::iter::once(format!(
                    "graph-breadth:{}",
                    graph_composition_breadth.breadth_digest()
                )))
                .chain(std::iter::once(format!(
                    "graph-program:{}",
                    graph_composition_program.program_digest()
                )))
                .chain(std::iter::once(format!(
                    "graph-lifecycle:{}",
                    graph_lifecycle_digest
                )))
                .chain(
                    write_receipts
                        .iter()
                        .map(|receipt| format!("commit:{}", receipt.commit_identity())),
                )
                .chain(
                    touched_aspect_paths
                        .iter()
                        .map(|path| format!("aspect:{path}")),
                )
                .chain(
                    affected_live_view_ids
                        .iter()
                        .map(|view| format!("live:{view}")),
                )
                .chain(
                    affected_derived_view_ids
                        .iter()
                        .map(|view| format!("derived:{view}")),
                )
                .collect::<Vec<_>>(),
        );

        let target_evidence = write_receipts
            .iter()
            .map(|receipt| receipt.target_evidence().clone())
            .collect::<Vec<_>>();
        let mutation_families = write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::mutation_family)
            .collect::<Vec<_>>();
        let existing_truth_assertions = write_receipts
            .iter()
            .map(|receipt| receipt.existing_truth_assertion_evidence().cloned())
            .collect::<Vec<_>>();
        let existing_truth_bindings = write_receipts
            .iter()
            .map(|receipt| receipt.existing_truth_binding_evidence().cloned())
            .collect::<Vec<_>>();
        let symbolic_target_references = write_receipts
            .iter()
            .map(|receipt| receipt.symbolic_target_reference_evidence().cloned())
            .collect::<Vec<_>>();
        let symbolic_aspect_resolutions = write_receipts
            .iter()
            .map(|receipt| receipt.symbolic_aspect_resolution_evidence().to_vec())
            .collect::<Vec<_>>();
        let naming_mutations = write_receipts
            .iter()
            .map(|receipt| receipt.naming_mutation_evidence().cloned())
            .collect::<Vec<_>>();
        let continuity_mutations = write_receipts
            .iter()
            .map(|receipt| receipt.continuity_mutation_evidence().cloned())
            .collect::<Vec<_>>();
        let aggregate_bridge = batch_bridge_evidence_from_receipts(&write_receipts);
        let batch_mutation_evidence = ForgeQueryBatchMutationEvidence::from_components(
            &mutation_families,
            &target_evidence,
            &existing_truth_assertions,
            &existing_truth_bindings,
            &symbolic_target_references,
            &symbolic_aspect_resolutions,
            &naming_mutations,
            &continuity_mutations,
            aggregate_bridge.as_ref(),
        )
        .expect("non-empty mutation batch must produce batch evidence");
        let graph_composition_resolution_map =
            ForgeQueryGraphCompositionResolutionMap::from_write_receipts(&write_receipts);

        Ok(Self {
            write_receipts,
            authority_lane,
            basis_lane,
            batch_mutation_evidence,
            graph_composition_breadth,
            graph_composition_program,
            graph_composition_resolution_map,
            batch_digest,
            touched_aspect_paths,
            affected_live_view_ids,
            affected_derived_view_ids,
            considered_computed_view_count,
            considered_effect_count,
            delivered_effect_count,
            pending_write_intent_count,
            suppressed_effect_count,
            meaningful_effect_suppression_count,
            effect_expression_failure_count,
            refresh_fallback,
            decision_trace_envelope,
            execution_provenance,
        })
    }

    pub fn from_write_receipts(
        write_receipts: Vec<ForgeQueryWriteReceipt>,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        if write_receipts.is_empty() {
            return Err(ForgeQueryRuntimeError::Workspace(
                crate::memory_workspace::ForgeQueryWorkspaceError::new(
                    "mutation batch must produce at least one write receipt",
                ),
            ));
        }
        let authority_lane = write_receipts[0].authority_lane();
        let basis_lane = write_receipts[0].basis_lane();
        if write_receipts
            .iter()
            .any(|receipt| receipt.authority_lane() != authority_lane)
        {
            return Err(ForgeQueryRuntimeError::Workspace(
                crate::memory_workspace::ForgeQueryWorkspaceError::new(
                    "mutation batch may not mix authority lanes",
                ),
            ));
        }
        if write_receipts
            .iter()
            .any(|receipt| receipt.basis_lane() != basis_lane)
        {
            return Err(ForgeQueryRuntimeError::Workspace(
                crate::memory_workspace::ForgeQueryWorkspaceError::new(
                    "mutation batch may not mix basis lanes",
                ),
            ));
        }

        let aggregates = derive_batch_receipt_aggregates(&write_receipts);

        Self::new(
            write_receipts,
            authority_lane,
            basis_lane,
            ForgeQueryGraphCompositionBreadth::empty(),
            ForgeQueryGraphCompositionProgram::empty(),
            aggregates.touched_aspect_paths,
            aggregates.affected_live_view_ids,
            aggregates.affected_derived_view_ids,
            aggregates.considered_computed_view_count,
            aggregates.considered_effect_count,
            aggregates.delivered_effect_count,
            aggregates.pending_write_intent_count,
            aggregates.suppressed_effect_count,
            aggregates.meaningful_effect_suppression_count,
            aggregates.effect_expression_failure_count,
            aggregates.refresh_fallback,
            None,
            None,
        )
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn basis_lane(&self) -> ForgeQueryAuthorityLane {
        self.basis_lane
    }

    pub fn batch_digest(&self) -> &str {
        &self.batch_digest
    }

    pub fn batch_mutation_evidence(&self) -> &ForgeQueryBatchMutationEvidence {
        &self.batch_mutation_evidence
    }

    pub fn graph_composition_breadth(&self) -> &ForgeQueryGraphCompositionBreadth {
        &self.graph_composition_breadth
    }

    pub fn graph_composition_program(&self) -> Option<&ForgeQueryGraphCompositionProgram> {
        (!self.graph_composition_program.is_empty()).then_some(&self.graph_composition_program)
    }

    pub fn graph_composition_evidence(&self) -> Option<ForgeQueryGraphCompositionEvidence> {
        let lifecycle_outcomes =
            ForgeQueryGraphCompositionLifecycleOutcomes::derive(&self.graph_composition_program)?;
        ForgeQueryGraphCompositionEvidence::derive(
            &self.write_receipts,
            &self.graph_composition_breadth,
            &lifecycle_outcomes,
            &self.graph_composition_resolution_map,
            self.affected_live_view_ids.len(),
            self.affected_derived_view_ids.len(),
            self.considered_computed_view_count,
        )
    }

    pub fn graph_composition_assumption_summary(
        &self,
    ) -> Option<ForgeQueryGraphCompositionAssumptionSummary> {
        if self.graph_composition_program.is_empty() {
            return None;
        }
        ForgeQueryGraphCompositionAssumptionSummary::derive(&self.write_receipts)
    }

    pub fn graph_composition_lineage_summary(
        &self,
    ) -> Option<ForgeQueryGraphCompositionLineageSummary> {
        if self.graph_composition_program.is_empty() {
            return None;
        }
        ForgeQueryGraphCompositionLineageSummary::derive(&self.write_receipts)
    }

    pub fn graph_composition_lifecycle_outcomes(
        &self,
    ) -> Option<ForgeQueryGraphCompositionLifecycleOutcomes> {
        ForgeQueryGraphCompositionLifecycleOutcomes::derive(&self.graph_composition_program)
    }

    pub fn graph_composition_resolution_map(&self) -> &ForgeQueryGraphCompositionResolutionMap {
        &self.graph_composition_resolution_map
    }

    pub fn write_count(&self) -> usize {
        self.write_receipts.len()
    }

    pub fn write_receipts(&self) -> &[ForgeQueryWriteReceipt] {
        &self.write_receipts
    }

    pub fn touched_aspect_paths(&self) -> &[String] {
        &self.touched_aspect_paths
    }

    pub fn affected_live_view_ids(&self) -> &[String] {
        &self.affected_live_view_ids
    }

    pub fn affected_derived_view_ids(&self) -> &[String] {
        &self.affected_derived_view_ids
    }

    pub fn considered_computed_view_count(&self) -> usize {
        self.considered_computed_view_count
    }

    pub fn considered_effect_count(&self) -> usize {
        self.considered_effect_count
    }

    pub fn delivered_effect_count(&self) -> usize {
        self.delivered_effect_count
    }

    pub fn pending_write_intent_count(&self) -> usize {
        self.pending_write_intent_count
    }

    pub fn suppressed_effect_count(&self) -> usize {
        self.suppressed_effect_count
    }

    pub fn meaningful_effect_suppression_count(&self) -> usize {
        self.meaningful_effect_suppression_count
    }

    pub fn effect_expression_failure_count(&self) -> usize {
        self.effect_expression_failure_count
    }

    pub fn refresh_fallback(&self) -> bool {
        self.refresh_fallback
    }

    pub fn decision_trace_envelope(&self) -> Option<&ForgeQueryIntentDecisionTraceEnvelope> {
        self.decision_trace_envelope.as_ref()
    }

    pub fn execution_provenance(&self) -> Option<&ForgeQueryIntentExecutionProvenance> {
        self.execution_provenance.as_ref()
    }
}
