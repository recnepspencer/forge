use crate::application::ForgeQueryDeclarationAspectContract;
use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::binding_pipeline::ForgeQueryContinuationBindingInput;

use super::ForgeQueryPreparedContinuation;

pub struct ForgeQueryPreparedContinuationRequest<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    input: ForgeQueryContinuationBindingInput<D, I>,
    required_aspect_contract: Option<ForgeQueryDeclarationAspectContract>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryPreparedContinuationRequest<D, I>
{
    pub fn new(input: ForgeQueryContinuationBindingInput<D, I>) -> Self {
        Self {
            input,
            required_aspect_contract: None,
        }
    }

    pub fn with_required_aspect_contract(
        mut self,
        contract: ForgeQueryDeclarationAspectContract,
    ) -> Self {
        self.required_aspect_contract = Some(contract);
        self
    }

    pub fn required_aspect_contract(&self) -> Option<&ForgeQueryDeclarationAspectContract> {
        self.required_aspect_contract.as_ref()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ForgeQueryContinuationBindingInput<D, I>,
        Option<ForgeQueryDeclarationAspectContract>,
    ) {
        (self.input, self.required_aspect_contract)
    }
}

pub struct ForgeQueryExecutePreparedContinuationRequest<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    prepared: ForgeQueryPreparedContinuation<D, I>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryExecutePreparedContinuationRequest<D, I>
{
    pub fn new(prepared: ForgeQueryPreparedContinuation<D, I>) -> Self {
        Self { prepared }
    }

    pub(crate) fn into_prepared(self) -> ForgeQueryPreparedContinuation<D, I> {
        self.prepared
    }
}
