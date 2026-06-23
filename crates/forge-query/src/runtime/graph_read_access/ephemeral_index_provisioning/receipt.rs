use super::{
    ForgeQueryEphemeralGraphIndex, ForgeQueryEphemeralGraphIndexCounters,
    ForgeQueryEphemeralGraphIndexScope, ForgeQueryEphemeralGraphIndexScopeKind,
};
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEphemeralGraphIndexReceipt {
    digest: String,
    plan_digest: String,
    scope_digest: String,
    index_digest: String,
    scope_kind: ForgeQueryEphemeralGraphIndexScopeKind,
    actual_allocated_bytes: usize,
    admitted_byte_budget: usize,
    active_resource_count_after_scope: usize,
    counters: ForgeQueryEphemeralGraphIndexCounters,
}

impl ForgeQueryEphemeralGraphIndexReceipt {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn plan_digest(&self) -> &str {
        &self.plan_digest
    }

    pub fn scope_digest(&self) -> &str {
        &self.scope_digest
    }

    pub fn index_digest(&self) -> &str {
        &self.index_digest
    }

    pub fn scope_kind(&self) -> &ForgeQueryEphemeralGraphIndexScopeKind {
        &self.scope_kind
    }

    pub fn actual_allocated_bytes(&self) -> usize {
        self.actual_allocated_bytes
    }

    pub fn admitted_byte_budget(&self) -> usize {
        self.admitted_byte_budget
    }

    pub fn counters(&self) -> &ForgeQueryEphemeralGraphIndexCounters {
        &self.counters
    }

    pub fn active_resource_count_after_scope(&self) -> usize {
        self.active_resource_count_after_scope
    }

    pub fn orphan_resource_count(&self) -> usize {
        self.counters.orphan_resource_count()
    }

    pub(in crate::runtime::graph_read_access::ephemeral_index_provisioning) fn finalized(
        index: &ForgeQueryEphemeralGraphIndex,
        scope: &ForgeQueryEphemeralGraphIndexScope,
        admitted_byte_budget: usize,
        active_resource_count_after_scope: usize,
        counters: ForgeQueryEphemeralGraphIndexCounters,
    ) -> Self {
        let plan_digest = index.plan_digest().to_string();
        let scope_digest = scope.digest().to_string();
        let index_digest = index.index_digest().to_string();
        let scope_kind = scope.kind().clone();
        let actual_allocated_bytes = index.allocated_bytes();
        let digest = hash_parts(&[
            "forge_query_ephemeral_graph_index_receipt_v1".to_string(),
            format!("plan:{plan_digest}"),
            format!("scope:{scope_digest}"),
            format!("index:{index_digest}"),
            format!("scope_kind:{}", scope_kind.as_str()),
            format!("allocated:{actual_allocated_bytes}"),
            format!("budget:{admitted_byte_budget}"),
            format!("active_after_scope:{active_resource_count_after_scope}"),
            counters.digest_part(),
        ]);
        Self {
            digest,
            plan_digest,
            scope_digest,
            index_digest,
            scope_kind,
            actual_allocated_bytes,
            admitted_byte_budget,
            active_resource_count_after_scope,
            counters,
        }
    }
}
