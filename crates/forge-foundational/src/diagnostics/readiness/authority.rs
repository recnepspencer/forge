use forge_proof::{AuthorityMarker, AuthorityProves};

use crate::diagnostics::FoundationalDiagnosticProductionReadinessCertified;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticProductionReadinessAuthority(());

impl FoundationalDiagnosticProductionReadinessAuthority {
    pub(crate) const fn certification_boundary() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalDiagnosticProductionReadinessAuthority {}
impl AuthorityProves<FoundationalDiagnosticProductionReadinessCertified>
    for FoundationalDiagnosticProductionReadinessAuthority
{
}
