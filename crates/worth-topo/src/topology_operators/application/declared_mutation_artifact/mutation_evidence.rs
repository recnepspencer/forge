use forge_query::facade::{
    ForgeQueryBatchWriteReceiptInspection, ForgeQueryExistingTruthAssertionMode,
};

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
