use forge_proof::{AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness};

pub(crate) struct ExecutionReadinessAuthority;
impl AuthorityMarker for ExecutionReadinessAuthority {}

pub(crate) struct ReadmissionAuthority;
impl AuthorityMarker for ReadmissionAuthority {}

pub(crate) struct LoweringCapability;
impl CapabilityMarker for LoweringCapability {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExecutionReadinessDeferred {}

pub(crate) fn readiness_authority() -> AuthorityWitness<ExecutionReadinessAuthority> {
    AuthorityWitness::from_authority_marker(ExecutionReadinessAuthority)
}

pub(crate) fn readmission_authority() -> AuthorityWitness<ReadmissionAuthority> {
    AuthorityWitness::from_authority_marker(ReadmissionAuthority)
}

pub(crate) fn lowering_capability() -> CapabilityWitness<LoweringCapability> {
    CapabilityWitness::from_capability_marker(LoweringCapability)
}
