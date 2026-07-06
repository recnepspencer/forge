use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionIdentityLineageDenialKind::{
    ConflictingPersistentNamePropagationDenied, DuplicateRegionIdentityDenied,
};

use super::support::canonical_graph;
use super::support_mutations::{conflicting_persistent_name_bundle, duplicate_identity_bundle};

#[test]
fn conflicting_name_propagation_denies_before_decision_log_assembly() {
    let denial = conflicting_persistent_name_bundle(&canonical_graph())
        .mint_overlap_region_identity_lineage()
        .expect_err("conflicting propagated names must deny");

    assert_eq!(denial.kind(), ConflictingPersistentNamePropagationDenied);
}

#[test]
fn duplicate_region_identity_minting_denies() {
    let denial = duplicate_identity_bundle(&canonical_graph())
        .mint_overlap_region_identity_lineage()
        .expect_err("duplicate minted identities must deny");

    assert_eq!(denial.kind(), DuplicateRegionIdentityDenied);
}
