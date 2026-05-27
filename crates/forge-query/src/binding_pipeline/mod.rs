mod extractors;
mod family_contracts;
mod outcome;
mod request;
mod resolvers;
mod source;
mod specificity;
mod transcript;
mod witness;

pub use family_contracts::{
    ForgeQueryFamilyBindingContract, ForgeQueryFamilyContextExtractorContract,
    ForgeQueryFamilyTargetResolverContract,
};
pub use outcome::{
    ForgeQueryBindingAmbiguity, ForgeQueryBindingAspectConflict,
    ForgeQueryBindingAuthorityMismatch, ForgeQueryBindingBasisMismatch, ForgeQueryBindingChecked,
    ForgeQueryBindingExplicitNarrowingRequired, ForgeQueryBindingMissingRequiredAspect,
    ForgeQueryBindingOutcome, ForgeQueryBindingRebindRequired, ForgeQueryBindingStale,
    ForgeQueryBindingUnavailable, ForgeQueryBindingUnsupported, ForgeQueryBindingWrongHandle,
    ForgeQueryBindingWrongWorld,
};
pub use request::{
    ForgeQueryContinuationBindingInput, ForgeQueryContinuationBindingRequest,
    ForgeQueryDeclarationBindingRequest, ForgeQueryEnvelopeBindingRequest,
    ForgeQueryReceiptBindingRequest, ForgeQueryResolveContinuationFromTargetRequest,
    ForgeQueryResolveEnvelopeFromTargetRequest, ForgeQueryResolveReceiptFromTargetRequest,
    ForgeQueryResolveRouteFromTargetRequest, ForgeQueryRouteBindingRequest,
};
pub use source::{
    ForgeQueryBindingSourceKind, ForgeQueryDeclarationContextCandidate,
    ForgeQueryEnvelopeContextCandidate, ForgeQueryEnvelopeResolverSubject,
    ForgeQueryProgressionContextCandidate, ForgeQueryReceiptResolverSubject,
    ForgeQueryRouteResolverSubject,
};
pub use specificity::ForgeQueryBindingSpecificity;
pub use transcript::{
    ForgeQueryBindingAspectFitReport, ForgeQueryBindingCandidateRecord,
    ForgeQueryBindingLinkedArtifacts, ForgeQueryBindingNarrowingDecision,
    ForgeQueryBindingRequestDescriptor, ForgeQueryBindingTranscript, ForgeQueryBindingWitnessCheck,
};
#[allow(unused_imports)]
pub use witness::{
    ForgeQueryBindingAuthorityWitness, ForgeQueryBindingBasisWitness,
    ForgeQueryBindingContextWitness, ForgeQueryBindingFamilyWitness,
    ForgeQueryBindingTargetWitnessSet,
};

pub(crate) use extractors::{
    bind_continuation_request_from_context_on_handle, bind_declaration_from_context_on_handle,
    bind_envelope_request_from_context_on_handle, bind_receipt_request_from_context_on_handle,
    bind_route_request_from_context_on_handle,
};
pub(crate) use resolvers::{
    bind_continuation_from_target_on_handle, bind_envelope_from_target_on_handle,
    bind_receipt_from_target_on_handle, bind_route_from_target_on_handle,
};

#[cfg(test)]
mod tests;
