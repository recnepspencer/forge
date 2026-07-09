use crate::application::{
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationBridgeContinuationRequest,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput,
    WorthQueryDeclarationSignalCompatibilityInput, WorthQueryDomainEntryMarker,
};

pub struct WorthQuerySignalCompatibilityOrchestrationInput<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    subject: WorthQueryDeclarationSignalCompatibilityInput<D, I>,
    required_aspect_contract: WorthQueryDeclarationAspectContract,
    bridge_request: Option<WorthQueryDeclarationBridgeContinuationRequest>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQuerySignalCompatibilityOrchestrationInput<D, I>
{
    pub fn new(subject: WorthQueryDeclarationSignalCompatibilityInput<D, I>) -> Self {
        Self {
            subject,
            required_aspect_contract: I::Family::aspect_contract(),
            bridge_request: None,
        }
    }

    pub fn with_required_aspect_contract(
        mut self,
        required_aspect_contract: WorthQueryDeclarationAspectContract,
    ) -> Self {
        self.required_aspect_contract = required_aspect_contract;
        self
    }

    pub fn with_bridge_request(
        mut self,
        bridge_request: WorthQueryDeclarationBridgeContinuationRequest,
    ) -> Self {
        self.bridge_request = Some(bridge_request);
        self
    }

    pub fn required_aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.required_aspect_contract
    }

    pub fn bridge_request(&self) -> Option<WorthQueryDeclarationBridgeContinuationRequest> {
        self.bridge_request
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthQueryDeclarationSignalCompatibilityInput<D, I>,
        WorthQueryDeclarationAspectContract,
        Option<WorthQueryDeclarationBridgeContinuationRequest>,
    ) {
        (
            self.subject,
            self.required_aspect_contract,
            self.bridge_request,
        )
    }
}
