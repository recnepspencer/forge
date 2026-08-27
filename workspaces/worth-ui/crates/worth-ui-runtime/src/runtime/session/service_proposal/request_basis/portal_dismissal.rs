use core::num::NonZeroU64;

#[derive(Debug)]
pub(in crate::runtime) struct UiPortalDismissalServiceRequestAuthority {
    portal: crate::runtime::portal::UiPortalIdentity,
}

impl super::sealed::Sealed for UiPortalDismissalServiceRequestAuthority {}

impl super::UiServiceRequestOriginAuthority for UiPortalDismissalServiceRequestAuthority {
    fn service_request_origin(&self) -> super::UiServiceRequestOrigin {
        super::UiServiceRequestOrigin::HostObservation
    }
}

impl super::UiServiceRequestBasis<UiPortalDismissalServiceRequestAuthority> {
    pub(in crate::runtime) fn from_portal_dismissal(
        transition: &crate::runtime::portal::UiPreparedPortalServiceTransition,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
        application: crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity,
    ) -> Result<Self, super::UiServiceRequestBasisDenial> {
        let request = transition.request();
        let issued = NonZeroU64::new(request.idempotency().lineage())
            .ok_or(super::UiServiceRequestBasisDenial::IdentityExhausted)?;
        let identity = super::UiServiceRequestIdentity(issued);
        Self::seal(super::UiServiceRequestBasisInput {
            identity,
            causal_parent: None,
            causal_root: identity,
            application,
            surface: super::UiServiceSurfaceBasis {
                semantic_surface: request.semantic_surface(),
                host_surface: presentation.host_surface(),
                binding: presentation.binding(),
            },
            presentation: Some(presentation),
            source_order: super::UiServiceSourceOrder(issued),
            cancellation: super::UiServiceCancellationIdentity(issued),
            resource_budget: super::UiServiceResourceBudgetIdentity(issued),
            authority: UiPortalDismissalServiceRequestAuthority {
                portal: transition.portal(),
            },
        })
    }
}

impl UiPortalDismissalServiceRequestAuthority {
    pub(in crate::runtime) const fn portal(&self) -> crate::runtime::portal::UiPortalIdentity {
        self.portal
    }
}
