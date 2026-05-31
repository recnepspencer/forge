use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryBatchWriteReceiptInspection,
    ForgeQueryExistingTruthAssertionMode,
};

use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::topology_operators::{
    NamingMutationContinuityMatrix, TopologyDeclaredMutationSequence, TopologyMutationDigest,
    TopologyMutationFamily, TopologyMutationNamingReport,
};

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Debug, Clone)]
pub(crate) struct TopologyDeclaredMutationArtifact {
    pub(crate) semantic_family_key: &'static str,
    pub(crate) families: Vec<TopologyMutationFamily>,
    pub(crate) receipt: ForgeQueryBatchWriteReceipt,
    pub(crate) inspection: ForgeQueryBatchWriteReceiptInspection,
    pub(crate) mutation_evidence: TopologyMutationApplicationEvidence,
    pub(crate) materialized: MaterializedTopologyView,
    pub(crate) topology_mutation_digest: TopologyMutationDigest,
    pub(crate) naming_continuity_matrix: NamingMutationContinuityMatrix,
    pub(crate) naming_report: TopologyMutationNamingReport,
}

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Debug, Clone, Copy)]
pub(crate) struct TopologyMutationApplicationEvidence {
    backend_verified_update_count: usize,
    backend_verified_delete_count: usize,
}

#[cfg_attr(not(test), allow(dead_code))]
impl TopologyMutationApplicationEvidence {
    pub(crate) fn from_inspection(inspection: &ForgeQueryBatchWriteReceiptInspection) -> Self {
        Self {
            backend_verified_update_count: inspection
                .component_operations()
                .iter()
                .filter(|operation| {
                    operation.family() == "update"
                        && operation
                            .existing_truth_assertion_evidence()
                            .is_some_and(|evidence| {
                                evidence.mode()
                                    == ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
                            })
                })
                .count(),
            backend_verified_delete_count: inspection
                .component_operations()
                .iter()
                .filter(|operation| {
                    operation.family() == "delete"
                        && operation
                            .existing_truth_assertion_evidence()
                            .is_some_and(|evidence| {
                                evidence.mode()
                                    == ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
                            })
                })
                .count(),
        }
    }

    pub(crate) fn backend_verified_update_count(&self) -> usize {
        self.backend_verified_update_count
    }

    pub(crate) fn backend_verified_delete_count(&self) -> usize {
        self.backend_verified_delete_count
    }
}

#[cfg_attr(not(test), allow(dead_code))]
impl TopologyDeclaredMutationArtifact {
    pub(crate) fn from_receipt(
        semantic_family_key: &'static str,
        sequence: &TopologyDeclaredMutationSequence,
        receipt: ForgeQueryBatchWriteReceipt,
        inspection: ForgeQueryBatchWriteReceiptInspection,
        materialized: MaterializedTopologyView,
    ) -> Self {
        Self {
            semantic_family_key,
            families: sequence.families().to_vec(),
            mutation_evidence: TopologyMutationApplicationEvidence::from_inspection(&inspection),
            receipt,
            inspection,
            materialized,
            topology_mutation_digest: sequence.topology_mutation_digest().clone(),
            naming_continuity_matrix: sequence.naming_continuity_matrix().clone(),
            naming_report: sequence.naming_report().clone(),
        }
    }

    pub(crate) fn semantic_family_key(&self) -> &'static str {
        self.semantic_family_key
    }

    pub(crate) fn mutation_evidence(&self) -> TopologyMutationApplicationEvidence {
        self.mutation_evidence
    }
}
