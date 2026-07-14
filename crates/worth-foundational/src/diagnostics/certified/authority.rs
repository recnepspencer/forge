use worth_proof::{AuthorityMarker, AuthorityProves, AuthorityWitness};

use super::surfaces::FoundationalDiagnosticCertified;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticCertifiedAttachmentAuthority(());

impl FoundationalDiagnosticCertifiedAttachmentAuthority {
    pub(crate) const fn milestone_6_phase_5() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalDiagnosticCertifiedAttachmentAuthority {}
impl AuthorityProves<FoundationalDiagnosticCertified>
    for FoundationalDiagnosticCertifiedAttachmentAuthority
{
}

pub fn foundational_diagnostic_certified_attachment_authority(
) -> AuthorityWitness<FoundationalDiagnosticCertifiedAttachmentAuthority> {
    AuthorityWitness::from_authority_marker(
        FoundationalDiagnosticCertifiedAttachmentAuthority::milestone_6_phase_5(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticCertifiedReadmissionAuthority(());

impl FoundationalDiagnosticCertifiedReadmissionAuthority {
    pub(crate) const fn milestone_6_phase_5_boundary() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalDiagnosticCertifiedReadmissionAuthority {}

pub fn foundational_diagnostic_certified_readmission_authority(
) -> AuthorityWitness<FoundationalDiagnosticCertifiedReadmissionAuthority> {
    AuthorityWitness::from_authority_marker(
        FoundationalDiagnosticCertifiedReadmissionAuthority::milestone_6_phase_5_boundary(),
    )
}
