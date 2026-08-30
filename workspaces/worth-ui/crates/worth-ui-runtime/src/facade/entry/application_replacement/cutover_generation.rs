use super::cutover::WorthUiCutoverGenerationBasis;
use super::{
    WorthUiActiveApplicationSession, WorthUiApplicationCutoverDenial,
    WorthUiPendingApplicationCutover,
};

impl WorthUiActiveApplicationSession {
    pub(super) fn validate_cutover_generation_basis(
        &self,
        pending: &WorthUiPendingApplicationCutover,
        admitted_delta: &crate::graph::UiAdmittedAllocationCatalogDelta,
    ) -> Result<WorthUiCutoverGenerationBasis, WorthUiApplicationCutoverDenial> {
        if !pending.basis.admits_catalog_delta(admitted_delta) {
            return Err(WorthUiApplicationCutoverDenial::PreparedApplicationGraphMismatch);
        }
        let candidate_authority = pending.pending_activation.candidate_application_authority();
        let active = candidate_authority.generation_identity().clone();
        debug_assert_eq!(pending.basis.next_generation(), &active);
        debug_assert_eq!(pending.next_app.generation_identity(), &active);
        if !pending
            .basis
            .admits_application_authority(candidate_authority)
        {
            return Err(WorthUiApplicationCutoverDenial::PreparedApplicationAuthorityMismatch);
        }
        Ok(WorthUiCutoverGenerationBasis {
            prior: self.generation_identity().clone(),
            active,
        })
    }
}
