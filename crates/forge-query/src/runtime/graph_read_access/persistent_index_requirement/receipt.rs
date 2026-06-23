use super::ForgeQueryPersistentGraphIndexRequirementCounters;
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPersistentGraphIndexRequirementReceipt {
    digest: String,
    declaration_digest: String,
    counters: ForgeQueryPersistentGraphIndexRequirementCounters,
}

impl ForgeQueryPersistentGraphIndexRequirementReceipt {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn counters(&self) -> &ForgeQueryPersistentGraphIndexRequirementCounters {
        &self.counters
    }

    pub(crate) fn new(
        declaration_digest: impl Into<String>,
        counters: ForgeQueryPersistentGraphIndexRequirementCounters,
    ) -> Self {
        let declaration_digest = declaration_digest.into();
        let digest = hash_parts(&[
            "forge_query_persistent_graph_index_requirement_receipt_v1".to_string(),
            format!("declaration:{declaration_digest}"),
            counters.digest_part(),
        ]);
        Self {
            digest,
            declaration_digest,
            counters,
        }
    }
}
