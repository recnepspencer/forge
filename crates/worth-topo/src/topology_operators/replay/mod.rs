use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyOperatorDigest {
    pub algorithm: String,
    pub digest_hex: String,
    pub row_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyEditDigest {
    pub digest: TopologyOperatorDigest,
    pub contract_count: usize,
    pub family_count: usize,
    pub changed_scope_count: usize,
    pub naming_scope_count: usize,
    pub derived_region_count: usize,
    pub fallback_policy_count: usize,
    pub fallback_rejection_policy_count: usize,
}
