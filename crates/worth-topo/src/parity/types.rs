use serde::{Deserialize, Serialize};
use worth_schema::facade::{WorthDerivedInvalidationTarget, WorthMutationOrigin};

use crate::certification::WorthDeterministicDigest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDerivedEquivalenceContractReport {
    pub authority_snapshot_id: u64,
    pub authority_branch_id: String,
    pub authoritative_mutation_origin: WorthMutationOrigin,
    pub derivation_origin: WorthMutationOrigin,
    pub truth_basis_digest_hex: String,
    pub touched_aspect_count: usize,
    pub triggered_invalidation_targets: Vec<WorthDerivedInvalidationTarget>,
    pub precision_fallback_count: usize,
    pub precision_budget_fallback_count: usize,
    pub materialized_topology_digest: WorthDeterministicDigest,
    pub interpreted_topology_digest: WorthDeterministicDigest,
    pub derived_validation_digest: WorthDeterministicDigest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDerivedParityComparisonReport {
    pub authority_identity_match: bool,
    pub branch_identity_match: bool,
    pub invalidation_target_match: bool,
    pub materialized_topology_digest_match: bool,
    pub interpreted_topology_digest_match: bool,
    pub derived_validation_digest_match: bool,
    pub equivalent_derived_meaning: bool,
}
