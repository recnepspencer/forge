use schema::facade::platform::authority::{DerivedInvalidationTarget, MutationOrigin};
use serde::{Deserialize, Serialize};

use crate::certification::DeterministicDigest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedEquivalenceContractReport {
    pub authority_snapshot_id: u64,
    pub authority_branch_id: String,
    pub authoritative_mutation_origin: MutationOrigin,
    pub derivation_origin: MutationOrigin,
    pub truth_basis_digest_hex: String,
    pub touched_aspect_count: usize,
    pub triggered_invalidation_targets: Vec<DerivedInvalidationTarget>,
    pub precision_fallback_count: usize,
    pub precision_budget_fallback_count: usize,
    pub materialized_topology_digest: DeterministicDigest,
    pub interpreted_topology_digest: DeterministicDigest,
    pub derived_validation_digest: DeterministicDigest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedParityComparisonReport {
    pub authority_identity_match: bool,
    pub branch_identity_match: bool,
    pub invalidation_target_match: bool,
    pub materialized_topology_digest_match: bool,
    pub interpreted_topology_digest_match: bool,
    pub derived_validation_digest_match: bool,
    pub equivalent_derived_meaning: bool,
}
