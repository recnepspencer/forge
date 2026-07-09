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
    WorthQueryFamilyBindingContract, WorthQueryFamilyContextExtractorContract,
    WorthQueryFamilyTargetResolverContract,
};
pub use outcome::{
    WorthQueryBindingAmbiguity, WorthQueryBindingAspectConflict,
    WorthQueryBindingAuthorityMismatch, WorthQueryBindingBasisMismatch, WorthQueryBindingChecked,
    WorthQueryBindingExplicitNarrowingRequired, WorthQueryBindingMissingRequiredAspect,
    WorthQueryBindingOutcome, WorthQueryBindingRebindRequired, WorthQueryBindingStale,
    WorthQueryBindingUnavailable, WorthQueryBindingUnsupported, WorthQueryBindingWrongHandle,
    WorthQueryBindingWrongWorld,
};
pub use request::{
    WorthQueryContinuationBindingInput, WorthQueryContinuationBindingRequest,
    WorthQueryDeclarationBindingRequest, WorthQueryEnvelopeBindingRequest,
    WorthQueryReceiptBindingRequest, WorthQueryResolveContinuationFromTargetRequest,
    WorthQueryResolveEnvelopeFromTargetRequest, WorthQueryResolveReceiptFromTargetRequest,
    WorthQueryResolveRouteFromTargetRequest, WorthQueryRouteBindingRequest,
};
pub use source::{
    WorthQueryBindingSourceKind, WorthQueryDeclarationContextCandidate,
    WorthQueryEnvelopeContextCandidate, WorthQueryEnvelopeResolverSubject,
    WorthQueryProgressionContextCandidate, WorthQueryReceiptResolverSubject,
    WorthQueryRouteResolverSubject,
};
pub use specificity::WorthQueryBindingSpecificity;
pub use transcript::{
    WorthQueryBindingAspectFitReport, WorthQueryBindingCandidateRecord,
    WorthQueryBindingLinkedArtifacts, WorthQueryBindingNarrowingDecision,
    WorthQueryBindingRequestDescriptor, WorthQueryBindingTranscript, WorthQueryBindingWitnessCheck,
};
#[allow(unused_imports)]
pub use witness::{
    WorthQueryBindingAuthorityWitness, WorthQueryBindingBasisWitness,
    WorthQueryBindingContextWitness, WorthQueryBindingFamilyWitness,
    WorthQueryBindingTargetWitnessSet,
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
