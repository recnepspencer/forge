use core::num::NonZeroU64;

#[derive(Debug)]
pub(in crate::runtime) struct UiPortalExitTerminalServiceRequestAuthority;

impl super::sealed::Sealed for UiPortalExitTerminalServiceRequestAuthority {}

impl super::UiServiceRequestOriginAuthority for UiPortalExitTerminalServiceRequestAuthority {
    fn service_request_origin(&self) -> super::UiServiceRequestOrigin {
        super::UiServiceRequestOrigin::ServiceContinuation
    }
}

impl super::UiServiceRequestBasis<UiPortalExitTerminalServiceRequestAuthority> {
    pub(in crate::runtime) fn from_portal_exit_terminal(
        transition: &crate::runtime::portal::UiPreparedPortalServiceTransition,
        retention: crate::runtime::portal::UiPortalExitRetentionReceipt,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
        application: crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity,
    ) -> Result<Self, super::UiServiceRequestBasisDenial> {
        if transition.portal() != retention.portal() {
            return Err(super::UiServiceRequestBasisDenial::RootIdentityMismatch);
        }
        let request = transition.request();
        let issued = NonZeroU64::new(request.idempotency().lineage())
            .ok_or(super::UiServiceRequestBasisDenial::IdentityExhausted)?;
        let parent = NonZeroU64::new(retention.causal_lineage())
            .ok_or(super::UiServiceRequestBasisDenial::IdentityExhausted)?;
        let identity = super::UiServiceRequestIdentity(issued);
        let causal_parent = super::UiServiceRequestIdentity(parent);
        Self::seal(super::UiServiceRequestBasisInput {
            identity,
            causal_parent: Some(causal_parent),
            causal_root: causal_parent,
            application,
            surface: super::UiServiceSurfaceBasis {
                semantic_surface: request.semantic_surface(),
                host_surface: presentation.host_surface(),
                binding: presentation.binding(),
            },
            presentation: Some(presentation),
            source_order: super::UiServiceSourceOrder(issued),
            cancellation: super::UiServiceCancellationIdentity(parent),
            resource_budget: super::UiServiceResourceBudgetIdentity(parent),
            authority: UiPortalExitTerminalServiceRequestAuthority,
        })
    }
}
