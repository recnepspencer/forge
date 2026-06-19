use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationStateLoadCounters {
    loaded_state_scope_count: usize,
    traversed_edge_count: usize,
    materialized_row_count: usize,
    counters_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationStateLoadCounters {
    pub fn none() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn new(
        loaded_state_scope_count: usize,
        traversed_edge_count: usize,
        materialized_row_count: usize,
    ) -> Self {
        let counters_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::GraphObligationStateLoadCounters,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("loaded_state_scope_count"),
            loaded_state_scope_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("traversed_edge_count"),
            traversed_edge_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("materialized_row_count"),
            materialized_row_count,
        )
        .seal();
        Self {
            loaded_state_scope_count,
            traversed_edge_count,
            materialized_row_count,
            counters_digest,
        }
    }

    pub fn loaded_state_scope_count(&self) -> usize {
        self.loaded_state_scope_count
    }

    pub fn traversed_edge_count(&self) -> usize {
        self.traversed_edge_count
    }

    pub fn materialized_row_count(&self) -> usize {
        self.materialized_row_count
    }

    pub fn counters_digest(&self) -> &str {
        self.counters_digest.as_str()
    }

    pub(crate) fn counters_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.counters_digest
    }
}
