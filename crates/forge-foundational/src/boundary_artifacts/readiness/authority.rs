use forge_proof::{AuthorityMarker, AuthorityProves};

use crate::boundary_artifacts::FoundationalBoundaryArtifactProductionReadinessCertified;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryArtifactProductionReadinessAuthority(());

impl FoundationalBoundaryArtifactProductionReadinessAuthority {
    pub(crate) const fn certification_boundary() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalBoundaryArtifactProductionReadinessAuthority {}
impl AuthorityProves<FoundationalBoundaryArtifactProductionReadinessCertified>
    for FoundationalBoundaryArtifactProductionReadinessAuthority
{
}
