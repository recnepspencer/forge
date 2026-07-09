use worth_proof::{AuthorityMarker, AuthorityProves};

use crate::boundary_evidence::FoundationalBoundaryEvidenceProductionReadinessCertified;

#[derive(Debug, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceProductionReadinessAuthority(());

impl FoundationalBoundaryEvidenceProductionReadinessAuthority {
    #[cfg(test)]
    pub(crate) const fn new() -> Self {
        Self(())
    }

    pub(super) const fn certification_boundary() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalBoundaryEvidenceProductionReadinessAuthority {}
impl AuthorityProves<FoundationalBoundaryEvidenceProductionReadinessCertified>
    for FoundationalBoundaryEvidenceProductionReadinessAuthority
{
}
