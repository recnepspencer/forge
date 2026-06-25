use forge_query::facade::{
    ForgeQueryAuthoritativeMutationObligationDispatchProjection,
    ForgeQueryBatchWriteReceiptInspection, ForgeQueryExistingTruthAssertionMode,
    ForgeQueryGraphObligationDispatchContextKind,
};

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Debug, Clone)]
pub(crate) struct TopologyMutationApplicationEvidence {
    backend_verified_update_count: usize,
    backend_verified_delete_count: usize,
    graph_obligation_envelope_digest: Option<String>,
    graph_obligation_dispatch_digest: Option<String>,
    graph_obligation_execution_point: Option<ForgeQueryGraphObligationDispatchContextKind>,
    graph_obligation_selected_count: usize,
}

#[cfg_attr(not(test), allow(dead_code))]
impl TopologyMutationApplicationEvidence {
    #[cfg(test)]
    pub(crate) fn from_cutover_test_parts(
        graph_obligation_envelope_digest: Option<String>,
        graph_obligation_dispatch_digest: Option<String>,
        graph_obligation_selected_count: usize,
    ) -> Self {
        Self {
            backend_verified_update_count: 0,
            backend_verified_delete_count: 0,
            graph_obligation_envelope_digest,
            graph_obligation_dispatch_digest,
            graph_obligation_execution_point: None,
            graph_obligation_selected_count,
        }
    }

    pub(crate) fn from_inspection_and_graph_obligation_projection(
        inspection: &ForgeQueryBatchWriteReceiptInspection,
        graph_obligation_projection: Option<
            &ForgeQueryAuthoritativeMutationObligationDispatchProjection,
        >,
    ) -> Self {
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
            graph_obligation_envelope_digest: graph_obligation_projection
                .and_then(|projection| projection.envelope_digest())
                .map(str::to_string),
            graph_obligation_dispatch_digest: graph_obligation_projection
                .map(|projection| projection.dispatch_digest().to_string()),
            graph_obligation_execution_point: graph_obligation_projection
                .and_then(|projection| projection.context_kind()),
            graph_obligation_selected_count: graph_obligation_projection
                .map(|projection| projection.rows().len())
                .unwrap_or(0),
        }
    }

    pub(crate) fn backend_verified_update_count(&self) -> usize {
        self.backend_verified_update_count
    }

    pub(crate) fn backend_verified_delete_count(&self) -> usize {
        self.backend_verified_delete_count
    }

    pub(crate) fn graph_obligation_envelope_digest(&self) -> Option<&str> {
        self.graph_obligation_envelope_digest.as_deref()
    }

    pub(crate) fn graph_obligation_dispatch_digest(&self) -> Option<&str> {
        self.graph_obligation_dispatch_digest.as_deref()
    }

    pub(crate) fn graph_obligation_execution_point(
        &self,
    ) -> Option<ForgeQueryGraphObligationDispatchContextKind> {
        self.graph_obligation_execution_point
    }

    pub(crate) fn graph_obligation_selected_count(&self) -> usize {
        self.graph_obligation_selected_count
    }
}
