use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryAspectTouch, ForgeQueryAuthorityLane, ForgeQueryBatchMutationEvidence,
    ForgeQueryDerivedMaterializationTarget, ForgeQueryGraphCompositionBreadth,
    ForgeQueryGraphCompositionLifecycleOutcomes, ForgeQueryGraphCompositionProgram,
    ForgeQueryGraphCompositionResolutionMap, ForgeQueryIntentDecisionTraceEnvelope,
    ForgeQueryIntentExecutionProvenance, ForgeQueryJournalPosition, ForgeQueryLiveArtifactTarget,
    ForgeQueryRuntimeError,
};

use super::batch_receipt_aggregates::{
    batch_bridge_evidence_from_receipts, derive_batch_receipt_aggregates,
};
use super::batch_receipt_identity::{evidence_touch_identities, evidence_value_identities};
use super::ForgeQueryWriteReceipt;

mod graph_composition_accessors;
mod graph_obligation_accessors;
mod terminal_affected_view_accessors;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBatchWriteReceipt {
    write_receipts: Vec<ForgeQueryWriteReceipt>,
    authority_lane: ForgeQueryAuthorityLane,
    basis_lane: ForgeQueryAuthorityLane,
    batch_mutation_evidence: ForgeQueryBatchMutationEvidence,
    graph_composition_breadth: ForgeQueryGraphCompositionBreadth,
    graph_composition_program: ForgeQueryGraphCompositionProgram,
    graph_composition_resolution_map: ForgeQueryGraphCompositionResolutionMap,
    batch_digest: ForgeQueryEvidenceIdentity,
    touched_aspects: Vec<ForgeQueryAspectTouch>,
    affected_live_view_targets: Vec<ForgeQueryLiveArtifactTarget>,
    affected_derived_view_targets: Vec<ForgeQueryDerivedMaterializationTarget>,
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
    obligation_dispatch: Option<crate::runtime::ForgeQueryAuthoritativeMutationObligationDispatch>,
}

impl ForgeQueryBatchWriteReceipt {
    pub(in crate::runtime) fn new(
        write_receipts: Vec<ForgeQueryWriteReceipt>,
        authority_lane: ForgeQueryAuthorityLane,
        basis_lane: ForgeQueryAuthorityLane,
        graph_composition_breadth: ForgeQueryGraphCompositionBreadth,
        graph_composition_program: ForgeQueryGraphCompositionProgram,
        touched_aspects: Vec<ForgeQueryAspectTouch>,
        affected_live_view_targets: Vec<ForgeQueryLiveArtifactTarget>,
        affected_derived_view_targets: Vec<ForgeQueryDerivedMaterializationTarget>,
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
        obligation_dispatch: Option<
            crate::runtime::ForgeQueryAuthoritativeMutationObligationDispatch,
        >,
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
                .map(|outcomes| outcomes.lifecycle_evidence_digest().clone());
        let journal_position_identities = write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::journal_position)
            .map(ForgeQueryJournalPosition::evidence_identity)
            .collect::<Vec<_>>();
        let batch_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::BatchWriteReceipt)
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("graph_breadth_digest"),
                    graph_composition_breadth.breadth_evidence_digest(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("graph_program_digest"),
                    graph_composition_program.program_evidence_digest(),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("graph_lifecycle_digest"),
                    graph_lifecycle_digest.as_ref(),
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("write_commit_identity"),
                    write_receipts
                        .iter()
                        .map(ForgeQueryWriteReceipt::commit_evidence_identity),
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("journal_position_identity"),
                    journal_position_identities.iter(),
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("touched_aspect"),
                    evidence_touch_identities("batch-touched-aspect", &touched_aspects).iter(),
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("affected_live_view_id"),
                    evidence_value_identities(
                        "batch-affected-live-view",
                        &terminal_live_view_ids(&affected_live_view_targets),
                    )
                    .iter(),
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("affected_derived_view_id"),
                    evidence_value_identities(
                        "batch-affected-derived-view",
                        &terminal_derived_view_ids(&affected_derived_view_targets),
                    )
                    .iter(),
                )
                .seal();

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
        let causality_evidence = write_receipts
            .iter()
            .map(|receipt| receipt.causality_evidence().cloned())
            .collect::<Vec<_>>();
        let provenance_evidence = write_receipts
            .iter()
            .map(|receipt| receipt.provenance_evidence().cloned())
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
            &causality_evidence,
            &provenance_evidence,
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
            touched_aspects,
            affected_live_view_targets,
            affected_derived_view_targets,
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
            obligation_dispatch,
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
            aggregates.touched_aspects,
            aggregates.affected_live_view_targets,
            aggregates.affected_derived_view_targets,
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
        self.batch_digest.as_str()
    }

    pub(in crate::runtime) fn batch_identity(&self) -> &ForgeQueryEvidenceIdentity {
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

    pub fn write_count(&self) -> usize {
        self.write_receipts.len()
    }

    pub fn write_receipts(&self) -> &[ForgeQueryWriteReceipt] {
        &self.write_receipts
    }

    pub fn journal_positions(&self) -> impl Iterator<Item = &ForgeQueryJournalPosition> {
        self.write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::journal_position)
    }

    pub fn admitted_touched_aspects(&self) -> &[ForgeQueryAspectTouch] {
        &self.touched_aspects
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

    pub fn obligation_dispatch(
        &self,
    ) -> Option<&crate::runtime::ForgeQueryAuthoritativeMutationObligationDispatch> {
        self.obligation_dispatch.as_ref()
    }
}

pub(in crate::runtime) fn terminal_live_view_ids(
    targets: &[ForgeQueryLiveArtifactTarget],
) -> Vec<String> {
    targets
        .iter()
        .map(|target| target.view_name().to_string())
        .collect()
}

pub(in crate::runtime) fn terminal_derived_view_ids(
    targets: &[ForgeQueryDerivedMaterializationTarget],
) -> Vec<String> {
    targets
        .iter()
        .map(|target| target.view_name().to_string())
        .collect()
}
