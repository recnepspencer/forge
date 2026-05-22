use forge_proof::{AuthorityMarker, AuthorityProves, AuthorityWitness};

use super::surfaces::FoundationalPerformanceCertified;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalPerformanceCertifiedAttachmentAuthority(());

impl FoundationalPerformanceCertifiedAttachmentAuthority {
    pub(crate) const fn milestone_8_certified_lane() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalPerformanceCertifiedAttachmentAuthority {}

impl AuthorityProves<FoundationalPerformanceCertified>
    for FoundationalPerformanceCertifiedAttachmentAuthority
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalPerformanceCertifiedReadmissionAuthority(());

impl FoundationalPerformanceCertifiedReadmissionAuthority {
    pub(crate) const fn milestone_8_certified_lane() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalPerformanceCertifiedReadmissionAuthority {}

pub fn foundational_performance_certified_attachment_authority(
) -> AuthorityWitness<FoundationalPerformanceCertifiedAttachmentAuthority> {
    AuthorityWitness::from_authority_marker(
        FoundationalPerformanceCertifiedAttachmentAuthority::milestone_8_certified_lane(),
    )
}

pub fn foundational_performance_certified_readmission_authority(
) -> AuthorityWitness<FoundationalPerformanceCertifiedReadmissionAuthority> {
    AuthorityWitness::from_authority_marker(
        FoundationalPerformanceCertifiedReadmissionAuthority::milestone_8_certified_lane(),
    )
}
