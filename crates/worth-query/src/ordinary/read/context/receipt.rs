use crate::identity::hash_parts;

use super::{WorthQueryReadContextAdmissionCounters, WorthQueryReadContextKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReadContextReceipt {
    context_kind: WorthQueryReadContextKind,
    canonical_query_digest: String,
    policy_tenant_admission_digest: Option<String>,
    relationship_proof_admission_digest: Option<String>,
    graph_authority_admission_digest: String,
    counters: WorthQueryReadContextAdmissionCounters,
    digest: String,
}

impl WorthQueryReadContextReceipt {
    pub fn context_kind(&self) -> WorthQueryReadContextKind {
        self.context_kind
    }

    pub fn canonical_query_digest(&self) -> &str {
        &self.canonical_query_digest
    }

    pub fn policy_tenant_admission_digest(&self) -> Option<&str> {
        self.policy_tenant_admission_digest.as_deref()
    }

    pub fn relationship_proof_admission_digest(&self) -> Option<&str> {
        self.relationship_proof_admission_digest.as_deref()
    }

    pub fn graph_authority_admission_digest(&self) -> &str {
        &self.graph_authority_admission_digest
    }

    pub fn counters(&self) -> &WorthQueryReadContextAdmissionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub(crate) fn new(
        context_kind: WorthQueryReadContextKind,
        canonical_query_digest: String,
        policy_tenant_admission_digest: Option<String>,
        relationship_proof_admission_digest: Option<String>,
        graph_authority_admission_digest: String,
        counters: WorthQueryReadContextAdmissionCounters,
    ) -> Self {
        let digest = hash_parts(&[
            "worth_query_read_context_receipt_v1".to_string(),
            format!("context:{}", context_kind.as_str()),
            format!("query:{canonical_query_digest}"),
            format!(
                "policy_tenant:{}",
                policy_tenant_admission_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "relationship_proof:{}",
                relationship_proof_admission_digest
                    .as_deref()
                    .unwrap_or("none")
            ),
            format!("graph_authority:{graph_authority_admission_digest}"),
            counters.digest_part(),
        ]);
        Self {
            context_kind,
            canonical_query_digest,
            policy_tenant_admission_digest,
            relationship_proof_admission_digest,
            graph_authority_admission_digest,
            counters,
            digest,
        }
    }
}
