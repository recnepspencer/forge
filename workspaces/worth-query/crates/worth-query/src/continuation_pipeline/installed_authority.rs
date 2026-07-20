use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};
use crate::domain_installation::WorthQueryInstalledDomainAuthorityWitness;

use super::WorthQueryPreparedContinuation;

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryPreparedContinuation<D, I>
{
    pub fn installed_authority(&self) -> &WorthQueryInstalledDomainAuthorityWitness {
        self.bridge_routing()
            .envelope()
            .foundational_evidence()
            .installed_authority()
    }
}
