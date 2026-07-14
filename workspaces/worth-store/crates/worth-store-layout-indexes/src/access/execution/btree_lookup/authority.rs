use worth_proof::{AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness};

pub(in crate::access::execution::btree_lookup) struct BTreeLookupReadinessAuthority;
impl AuthorityMarker for BTreeLookupReadinessAuthority {}

pub(in crate::access::execution::btree_lookup) struct BTreeLookupLoweringCapability;
impl CapabilityMarker for BTreeLookupLoweringCapability {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::access::execution::btree_lookup) enum BTreeLookupReadinessDeferred {}

pub(in crate::access::execution::btree_lookup) fn readiness_authority(
) -> AuthorityWitness<BTreeLookupReadinessAuthority> {
    AuthorityWitness::from_authority_marker(BTreeLookupReadinessAuthority)
}

pub(in crate::access::execution::btree_lookup) fn lowering_capability(
) -> CapabilityWitness<BTreeLookupLoweringCapability> {
    CapabilityWitness::from_capability_marker(BTreeLookupLoweringCapability)
}
