use worth_proof::{AuthorityMarker, AuthorityProves};

use super::super::CanonicalProductionReadinessCertified;

#[derive(Debug, PartialEq, Eq)]
pub struct CanonicalProductionReadinessAuthority(());

impl CanonicalProductionReadinessAuthority {
    #[cfg(test)]
    pub(crate) const fn new() -> Self {
        Self(())
    }

    pub(super) const fn certification_boundary() -> Self {
        Self(())
    }
}

impl AuthorityMarker for CanonicalProductionReadinessAuthority {}
impl AuthorityProves<CanonicalProductionReadinessCertified>
    for CanonicalProductionReadinessAuthority
{
}
