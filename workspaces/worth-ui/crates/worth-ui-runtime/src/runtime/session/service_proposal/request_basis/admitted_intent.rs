use core::num::NonZeroU64;

#[derive(Debug)]
pub(in crate::runtime) struct UiAdmittedIntentServiceRequestAuthority {
    attempt: crate::runtime::intent_execution::UiIntentExecutionAttemptIdentity,
    idempotency: crate::runtime::intent_execution::UiIntentExecutionIdempotencyIdentity,
}

impl super::sealed::Sealed for UiAdmittedIntentServiceRequestAuthority {}

impl super::UiServiceRequestOriginAuthority for UiAdmittedIntentServiceRequestAuthority {
    fn service_request_origin(&self) -> super::UiServiceRequestOrigin {
        super::UiServiceRequestOrigin::AdmittedIntent
    }
}

impl super::UiServiceRequestBasis<UiAdmittedIntentServiceRequestAuthority> {
    pub(in crate::runtime) fn from_intent_consequence(
        handoff: &crate::runtime::intent_execution::UiIntentConsequenceHandoff,
        application: crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity,
    ) -> Result<Self, super::UiServiceRequestBasisDenial> {
        let idempotency = handoff.idempotency();
        let issued = NonZeroU64::new(idempotency.lineage())
            .ok_or(super::UiServiceRequestBasisDenial::IdentityExhausted)?;
        let identity = super::UiServiceRequestIdentity(issued);
        let target = handoff.target();
        let presentation = target.presentation();
        Self::seal(super::UiServiceRequestBasisInput {
            identity,
            causal_parent: None,
            causal_root: identity,
            application,
            surface: super::UiServiceSurfaceBasis {
                semantic_surface: target.surface(),
                host_surface: presentation.host_surface(),
                binding: target.binding(),
            },
            presentation: Some(presentation),
            source_order: super::UiServiceSourceOrder(issued),
            cancellation: super::UiServiceCancellationIdentity(issued),
            resource_budget: super::UiServiceResourceBudgetIdentity(issued),
            authority: UiAdmittedIntentServiceRequestAuthority {
                attempt: handoff.attempt(),
                idempotency,
            },
        })
    }
}

impl UiAdmittedIntentServiceRequestAuthority {
    pub(in crate::runtime) const fn attempt(
        &self,
    ) -> crate::runtime::intent_execution::UiIntentExecutionAttemptIdentity {
        self.attempt
    }

    pub(in crate::runtime) const fn idempotency(
        &self,
    ) -> crate::runtime::intent_execution::UiIntentExecutionIdempotencyIdentity {
        self.idempotency
    }
}
