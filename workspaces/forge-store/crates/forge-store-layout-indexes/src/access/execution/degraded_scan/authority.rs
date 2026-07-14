use forge_proof::{AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness};

pub(in crate::access::execution::degraded_scan) struct DegradedScanReadinessAuthority;
impl AuthorityMarker for DegradedScanReadinessAuthority {}

pub(in crate::access::execution::degraded_scan) struct DegradedScanLoweringCapability;
impl CapabilityMarker for DegradedScanLoweringCapability {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::access::execution::degraded_scan) enum DegradedScanReadinessDeferred {}

pub(in crate::access::execution::degraded_scan) fn readiness_authority(
) -> AuthorityWitness<DegradedScanReadinessAuthority> {
    AuthorityWitness::from_authority_marker(DegradedScanReadinessAuthority)
}

pub(in crate::access::execution::degraded_scan) fn lowering_capability(
) -> CapabilityWitness<DegradedScanLoweringCapability> {
    CapabilityWitness::from_capability_marker(DegradedScanLoweringCapability)
}
