use crate::projection::read_views::domain::read_proof::TopologyReadExecutionEngine;

pub(crate) struct TopologyQueryRuntimeDocContract {
    historical_read_execution_engine: &'static str,
    runtime_support_type_name: &'static str,
    runtime_support_read_family_surface_name: &'static str,
    snapshot_support_phrase: &'static str,
    historical_basis_phrase: &'static str,
    forbidden_legacy_doc_phrases: &'static [&'static str],
    forbidden_comment_artifacts: &'static [&'static str],
    forbidden_batch_first_tokens: &'static [&'static str],
}

impl TopologyQueryRuntimeDocContract {
    pub(crate) fn historical_read_execution_engine(&self) -> &str {
        self.historical_read_execution_engine
    }

    pub(crate) fn runtime_support_type_name(&self) -> &str {
        self.runtime_support_type_name
    }

    pub(crate) fn runtime_support_read_family_surface_name(&self) -> &str {
        self.runtime_support_read_family_surface_name
    }

    pub(crate) fn snapshot_support_phrase(&self) -> &str {
        self.snapshot_support_phrase
    }

    pub(crate) fn historical_basis_phrase(&self) -> &str {
        self.historical_basis_phrase
    }

    pub(crate) fn forbidden_legacy_doc_phrases(&self) -> &[&'static str] {
        self.forbidden_legacy_doc_phrases
    }

    pub(crate) fn forbidden_comment_artifacts(&self) -> &[&'static str] {
        self.forbidden_comment_artifacts
    }

    pub(crate) fn forbidden_batch_first_tokens(&self) -> &[&'static str] {
        self.forbidden_batch_first_tokens
    }
}

pub(crate) fn topology_query_runtime_doc_contract() -> TopologyQueryRuntimeDocContract {
    TopologyQueryRuntimeDocContract {
        historical_read_execution_engine: TopologyReadExecutionEngine::QueryRuntimeHistorical
            .as_str(),
        runtime_support_type_name: "TopologyRuntimeSupport",
        runtime_support_read_family_surface_name: "TopologyReadRequestFamily",
        snapshot_support_phrase: "The snapshot read-only runtime admits",
        historical_basis_phrase: "read-only execution through the historical basis-aware path",
        forbidden_legacy_doc_phrases: &[
            "historical topology read families are deferred",
            "historical topology reads are deferred",
            "snapshot read-only runtime blocks",
            "historical runtime families for now",
            "not migrated yet and remains explicit",
            "snapshot-index fallback",
            "snapshot_index fallback",
        ],
        forbidden_comment_artifacts: &["-topo", "for  without", "the -owned"],
        forbidden_batch_first_tokens: &[
            "query-runtime-batch",
            "same-batch",
            "batch_label",
            "batch_mutation_evidence()",
        ],
    }
}
