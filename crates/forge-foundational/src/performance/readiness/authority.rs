use forge_proof::{AuthorityMarker, AuthorityProves};

use crate::performance::FoundationalPerformanceProductionReadinessCertified;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalPerformanceProductionReadinessAuthority(());

impl FoundationalPerformanceProductionReadinessAuthority {
    pub(crate) const fn certification_boundary() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalPerformanceProductionReadinessAuthority {}
impl AuthorityProves<FoundationalPerformanceProductionReadinessCertified>
    for FoundationalPerformanceProductionReadinessAuthority
{
}
