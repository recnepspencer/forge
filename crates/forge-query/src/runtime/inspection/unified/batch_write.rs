use super::batch_write_digest::{
    build_batch_write_receipt_inspection_digest, ForgeQueryBatchWriteDigestInputs,
};
use super::ForgeQueryBatchWriteComponentInspection;
use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::memory_workspace::ForgeQueryCommitIdentity;
use crate::runtime::{
    ForgeQueryAuthorityLane, ForgeQueryBatchMutationEvidence, ForgeQueryBatchWriteReceipt,
    ForgeQueryGraphCompositionAssumptionSummary, ForgeQueryGraphCompositionBreadth,
    ForgeQueryGraphCompositionEvidence, ForgeQueryGraphCompositionLifecycleOutcomes,
    ForgeQueryGraphCompositionLineageSummary, ForgeQueryGraphCompositionProgram,
    ForgeQueryGraphCompositionResolutionMap,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBatchWriteReceiptInspection {
    authority_lane: ForgeQueryAuthorityLane,
    basis_lane: ForgeQueryAuthorityLane,
    batch_digest: ForgeQueryEvidenceIdentity,
    batch_mutation_evidence: ForgeQueryBatchMutationEvidence,
    graph_composition_breadth: ForgeQueryGraphCompositionBreadth,
    graph_composition_lifecycle_outcomes: Option<ForgeQueryGraphCompositionLifecycleOutcomes>,
    graph_composition_program: Option<ForgeQueryGraphCompositionProgram>,
    graph_composition_evidence: Option<ForgeQueryGraphCompositionEvidence>,
    graph_composition_resolution_map: ForgeQueryGraphCompositionResolutionMap,
    write_receipt_count: usize,
    commit_identities: Vec<ForgeQueryCommitIdentity>,
    journal_position_identities: Vec<ForgeQueryEvidenceIdentity>,
    component_operations: Vec<ForgeQueryBatchWriteComponentInspection>,
    touched_aspect_paths: Vec<String>,
    affected_live_view_ids: Vec<String>,
    affected_derived_view_ids: Vec<String>,
    inspection_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryBatchWriteReceiptInspection {
    pub(in crate::runtime) fn new(receipt: &ForgeQueryBatchWriteReceipt) -> Self {
        let commit_identities = receipt
            .write_receipts()
            .iter()
            .map(|entry| entry.commit_identity().clone())
            .collect::<Vec<_>>();
        let component_operations = receipt
            .write_receipts()
            .iter()
            .map(ForgeQueryBatchWriteComponentInspection::from_write_receipt)
            .collect::<Vec<_>>();
        let journal_position_identities = receipt
            .journal_positions()
            .map(|position| position.evidence_identity())
            .collect::<Vec<_>>();
        let batch_mutation_evidence = receipt.batch_mutation_evidence().clone();
        let graph_composition_breadth = receipt.graph_composition_breadth().clone();
        let graph_composition_lifecycle_outcomes = receipt.graph_composition_lifecycle_outcomes();
        let graph_composition_program = receipt.graph_composition_program().cloned();
        let graph_composition_evidence = receipt.graph_composition_evidence();
        let graph_composition_resolution_map = receipt.graph_composition_resolution_map().clone();
        let touched_aspect_paths = receipt.touched_aspect_paths().to_vec();
        let affected_live_view_ids = receipt.affected_live_view_ids().to_vec();
        let affected_derived_view_ids = receipt.affected_derived_view_ids().to_vec();
        let inspection_identity =
            build_batch_write_receipt_inspection_digest(ForgeQueryBatchWriteDigestInputs {
                authority_lane: receipt.authority_lane().as_str(),
                basis_lane: receipt.basis_lane().as_str(),
                batch_digest: receipt.batch_identity(),
                graph_composition_breadth: &graph_composition_breadth,
                graph_composition_lifecycle_outcomes: graph_composition_lifecycle_outcomes.as_ref(),
                graph_composition_program: graph_composition_program.as_ref(),
                graph_composition_evidence: graph_composition_evidence.as_ref(),
                batch_mutation_evidence: &batch_mutation_evidence,
                commit_identities: &commit_identities,
                journal_position_identities: &journal_position_identities,
                component_operations: &component_operations,
                graph_composition_resolution_map: &graph_composition_resolution_map,
                touched_aspect_paths: &touched_aspect_paths,
                affected_live_view_ids: &affected_live_view_ids,
                affected_derived_view_ids: &affected_derived_view_ids,
            });
        Self {
            authority_lane: receipt.authority_lane(),
            basis_lane: receipt.basis_lane(),
            batch_digest: receipt.batch_identity().clone(),
            batch_mutation_evidence,
            graph_composition_breadth,
            graph_composition_lifecycle_outcomes,
            graph_composition_program,
            graph_composition_evidence,
            graph_composition_resolution_map,
            write_receipt_count: receipt.write_count(),
            commit_identities,
            journal_position_identities,
            component_operations,
            touched_aspect_paths,
            affected_live_view_ids,
            affected_derived_view_ids,
            inspection_identity,
        }
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

    pub fn batch_mutation_evidence(&self) -> &ForgeQueryBatchMutationEvidence {
        &self.batch_mutation_evidence
    }

    pub fn graph_composition_breadth(&self) -> &ForgeQueryGraphCompositionBreadth {
        &self.graph_composition_breadth
    }

    pub fn graph_composition_lifecycle_outcomes(
        &self,
    ) -> Option<&ForgeQueryGraphCompositionLifecycleOutcomes> {
        self.graph_composition_lifecycle_outcomes.as_ref()
    }

    pub fn graph_composition_program(&self) -> Option<&ForgeQueryGraphCompositionProgram> {
        self.graph_composition_program.as_ref()
    }

    pub fn graph_composition_evidence(&self) -> Option<&ForgeQueryGraphCompositionEvidence> {
        self.graph_composition_evidence.as_ref()
    }

    pub fn graph_composition_assumption_summary(
        &self,
    ) -> Option<&ForgeQueryGraphCompositionAssumptionSummary> {
        self.graph_composition_evidence
            .as_ref()
            .and_then(ForgeQueryGraphCompositionEvidence::assumption_summary)
    }

    pub fn graph_composition_lineage_summary(
        &self,
    ) -> Option<&ForgeQueryGraphCompositionLineageSummary> {
        self.graph_composition_evidence
            .as_ref()
            .and_then(ForgeQueryGraphCompositionEvidence::lineage_summary)
    }

    pub fn graph_composition_resolution_map(&self) -> &ForgeQueryGraphCompositionResolutionMap {
        &self.graph_composition_resolution_map
    }

    pub fn write_receipt_count(&self) -> usize {
        self.write_receipt_count
    }

    pub fn commit_identities(&self) -> &[ForgeQueryCommitIdentity] {
        &self.commit_identities
    }

    pub fn journal_position_identities(&self) -> &[ForgeQueryEvidenceIdentity] {
        &self.journal_position_identities
    }

    pub fn component_operations(&self) -> &[ForgeQueryBatchWriteComponentInspection] {
        &self.component_operations
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

    pub fn inspection_digest(&self) -> &str {
        self.inspection_identity.as_str()
    }

    pub fn inspection_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.inspection_identity
    }
}
