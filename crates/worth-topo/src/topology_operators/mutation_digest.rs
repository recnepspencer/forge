use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyMutationSequenceDigest {
    pub algorithm: String,
    pub digest_hex: String,
    pub row_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyMutationDigest {
    pub digest: TopologyMutationSequenceDigest,
    pub mutation_record_count: usize,
    pub family_count: usize,
    pub changed_scope_count: usize,
    pub naming_scope_count: usize,
    pub derived_region_count: usize,
    pub fallback_policy_count: usize,
    pub fallback_rejection_policy_count: usize,
}
