use forge_proof::{AuthorityMarker, AuthorityProves};

use crate::transitions::FoundationalTransitionProductionReadinessCertified;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalTransitionProductionReadinessAuthority(());

impl FoundationalTransitionProductionReadinessAuthority {
    pub(crate) const fn certification_boundary() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalTransitionProductionReadinessAuthority {}
impl AuthorityProves<FoundationalTransitionProductionReadinessCertified>
    for FoundationalTransitionProductionReadinessAuthority
{
}
