use forge_proof::{AuthorityMarker, AuthorityProves};

use crate::profiles::FoundationalProfileProductionReadinessCertified;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfileProductionReadinessAuthority(());

impl FoundationalProfileProductionReadinessAuthority {
    pub(crate) const fn certification_boundary() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalProfileProductionReadinessAuthority {}
impl AuthorityProves<FoundationalProfileProductionReadinessCertified>
    for FoundationalProfileProductionReadinessAuthority
{
}
