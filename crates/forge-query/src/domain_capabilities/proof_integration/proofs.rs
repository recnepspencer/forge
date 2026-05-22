use forge_proof::{AuthorityMarker, AuthorityProves, AuthorityWitness, Proof, ProofMarker};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RequestedContributionProofMarker;
impl ProofMarker for RequestedContributionProofMarker {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct EligibleContributionProofMarker;
impl ProofMarker for EligibleContributionProofMarker {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AdmittedContributionProofMarker;
impl ProofMarker for AdmittedContributionProofMarker {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MaterializationReadyContributionProofMarker;
impl ProofMarker for MaterializationReadyContributionProofMarker {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DomainCapabilityContributionAuthority;
impl AuthorityMarker for DomainCapabilityContributionAuthority {}
impl AuthorityProves<RequestedContributionProofMarker> for DomainCapabilityContributionAuthority {}
impl AuthorityProves<EligibleContributionProofMarker> for DomainCapabilityContributionAuthority {}
impl AuthorityProves<AdmittedContributionProofMarker> for DomainCapabilityContributionAuthority {}
impl AuthorityProves<MaterializationReadyContributionProofMarker>
    for DomainCapabilityContributionAuthority
{
}

pub(crate) type RequestedContributionProof =
    Proof<RequestedContributionProofMarker, DomainCapabilityContributionAuthority>;
pub(crate) type EligibleContributionProof =
    Proof<EligibleContributionProofMarker, DomainCapabilityContributionAuthority>;
pub(crate) type AdmittedContributionProof =
    Proof<AdmittedContributionProofMarker, DomainCapabilityContributionAuthority>;
pub(crate) type MaterializationReadyContributionProof =
    Proof<MaterializationReadyContributionProofMarker, DomainCapabilityContributionAuthority>;

fn contribution_authority() -> AuthorityWitness<DomainCapabilityContributionAuthority> {
    AuthorityWitness::from_authority_marker(DomainCapabilityContributionAuthority)
}

pub(crate) fn requested_contribution_proof() -> RequestedContributionProof {
    Proof::from_authority_witness(&contribution_authority())
}

pub(crate) fn eligible_contribution_proof() -> EligibleContributionProof {
    Proof::from_authority_witness(&contribution_authority())
}

pub(crate) fn admitted_contribution_proof() -> AdmittedContributionProof {
    Proof::from_authority_witness(&contribution_authority())
}

pub(crate) fn materialization_ready_contribution_proof() -> MaterializationReadyContributionProof {
    Proof::from_authority_witness(&contribution_authority())
}
