use crate::application::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationBridgeContinuationRequest, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationRouteIntent,
    ForgeQueryDeclarationSignalCompatibilityInput, ForgeQueryDomainEntryMarker,
};

pub enum ForgeQuerySignalCompatibilityOrchestrationSubject<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    SignalCompatibility(ForgeQueryDeclarationSignalCompatibilityInput<D, I>),
    Progressed {
        progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
        route_intent: Option<ForgeQueryDeclarationRouteIntent>,
    },
}

pub struct ForgeQuerySignalCompatibilityOrchestrationInput<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    subject: ForgeQuerySignalCompatibilityOrchestrationSubject<D, I>,
    required_aspect_contract: ForgeQueryDeclarationAspectContract,
    bridge_request: Option<ForgeQueryDeclarationBridgeContinuationRequest>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQuerySignalCompatibilityOrchestrationInput<D, I>
{
    pub fn new(subject: ForgeQueryDeclarationSignalCompatibilityInput<D, I>) -> Self {
        Self {
            subject: ForgeQuerySignalCompatibilityOrchestrationSubject::SignalCompatibility(
                subject,
            ),
            required_aspect_contract: I::Family::aspect_contract(),
            bridge_request: None,
        }
    }

    pub fn from_progressed(progression: ForgeQueryAdmittedDeclarationProgression<D, I>) -> Self {
        Self {
            subject: ForgeQuerySignalCompatibilityOrchestrationSubject::Progressed {
                progression,
                route_intent: None,
            },
            required_aspect_contract: I::Family::aspect_contract(),
            bridge_request: None,
        }
    }

    pub fn with_required_aspect_contract(
        mut self,
        required_aspect_contract: ForgeQueryDeclarationAspectContract,
    ) -> Self {
        self.required_aspect_contract = required_aspect_contract;
        self
    }

    pub fn with_bridge_request(
        mut self,
        bridge_request: ForgeQueryDeclarationBridgeContinuationRequest,
    ) -> Self {
        self.bridge_request = Some(bridge_request);
        self
    }

    pub fn with_route_intent(mut self, route_intent: ForgeQueryDeclarationRouteIntent) -> Self {
        if let ForgeQuerySignalCompatibilityOrchestrationSubject::Progressed {
            route_intent: existing,
            ..
        } = &mut self.subject
        {
            *existing = Some(route_intent);
        }
        self
    }

    pub fn required_aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.required_aspect_contract
    }

    pub fn bridge_request(&self) -> Option<ForgeQueryDeclarationBridgeContinuationRequest> {
        self.bridge_request
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ForgeQuerySignalCompatibilityOrchestrationSubject<D, I>,
        ForgeQueryDeclarationAspectContract,
        Option<ForgeQueryDeclarationBridgeContinuationRequest>,
    ) {
        (
            self.subject,
            self.required_aspect_contract,
            self.bridge_request,
        )
    }
}
