use crate::application::WorthQueryDeclarationAspectContract;
use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};
use crate::binding_pipeline::WorthQueryContinuationBindingInput;

use super::WorthQueryPreparedContinuation;

pub struct WorthQueryPreparedContinuationRequest<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    input: WorthQueryContinuationBindingInput<D, I>,
    required_aspect_contract: Option<WorthQueryDeclarationAspectContract>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryPreparedContinuationRequest<D, I>
{
    pub fn new(input: WorthQueryContinuationBindingInput<D, I>) -> Self {
        Self {
            input,
            required_aspect_contract: None,
        }
    }

    pub fn with_required_aspect_contract(
        mut self,
        contract: WorthQueryDeclarationAspectContract,
    ) -> Self {
        self.required_aspect_contract = Some(contract);
        self
    }

    pub fn required_aspect_contract(&self) -> Option<&WorthQueryDeclarationAspectContract> {
        self.required_aspect_contract.as_ref()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthQueryContinuationBindingInput<D, I>,
        Option<WorthQueryDeclarationAspectContract>,
    ) {
        (self.input, self.required_aspect_contract)
    }
}

pub struct WorthQueryExecutePreparedContinuationRequest<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    prepared: WorthQueryPreparedContinuation<D, I>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryExecutePreparedContinuationRequest<D, I>
{
    pub fn new(prepared: WorthQueryPreparedContinuation<D, I>) -> Self {
        Self { prepared }
    }

    pub(crate) fn into_prepared(self) -> WorthQueryPreparedContinuation<D, I> {
        self.prepared
    }
}
