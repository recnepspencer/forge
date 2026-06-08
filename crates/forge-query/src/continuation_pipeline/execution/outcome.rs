use crate::application::{
    ForgeQueryDeclarationBridgeRoutingDenialCause, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};
use crate::continuation_pipeline::ForgeQueryPreparedContinuationOutcome;

use super::support::ResolvedSignalContinuationTruth;

pub(super) fn prepared_outcome_from_signal_truth<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    signal_truth: &ResolvedSignalContinuationTruth,
) -> Option<ForgeQueryPreparedContinuationOutcome<D, I>> {
    match signal_truth.posture {
        crate::continuation_pipeline::artifacts::ForgeQueryPreparedContinuationSignalPosture::Compatible => {
            None
        }
        crate::continuation_pipeline::artifacts::ForgeQueryPreparedContinuationSignalPosture::Deferred => {
            Some(ForgeQueryPreparedContinuationOutcome::Deferred(
                signal_truth.reason.to_string(),
            ))
        }
        crate::continuation_pipeline::artifacts::ForgeQueryPreparedContinuationSignalPosture::Denied => {
            Some(ForgeQueryPreparedContinuationOutcome::Denied(
                signal_truth.reason.to_string(),
            ))
        }
        crate::continuation_pipeline::artifacts::ForgeQueryPreparedContinuationSignalPosture::Failed => {
            Some(ForgeQueryPreparedContinuationOutcome::Failed(
                signal_truth.reason.to_string(),
            ))
        }
    }
}

pub(super) fn prepared_outcome_from_bridge_denial_cause<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    cause: ForgeQueryDeclarationBridgeRoutingDenialCause,
    reason: &str,
) -> ForgeQueryPreparedContinuationOutcome<D, I> {
    match cause {
        ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch => {
            ForgeQueryPreparedContinuationOutcome::WrongHandle(reason.to_string())
        }
        ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeAuthorityUnavailable
        | ForgeQueryDeclarationBridgeRoutingDenialCause::AuthorityAspectGap
        | ForgeQueryDeclarationBridgeRoutingDenialCause::AuthorityAspectAmbiguity => {
            ForgeQueryPreparedContinuationOutcome::AuthorityMismatch(reason.to_string())
        }
        ForgeQueryDeclarationBridgeRoutingDenialCause::BasisLifecycleMismatch => {
            ForgeQueryPreparedContinuationOutcome::BasisMismatch(reason.to_string())
        }
        ForgeQueryDeclarationBridgeRoutingDenialCause::UnsupportedContinuationMode
        | ForgeQueryDeclarationBridgeRoutingDenialCause::UnsupportedTruthContext => {
            ForgeQueryPreparedContinuationOutcome::Unsupported(reason.to_string())
        }
        _ => ForgeQueryPreparedContinuationOutcome::Denied(reason.to_string()),
    }
}

pub(super) fn prepared_outcome_from_binding_outcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    outcome: crate::binding_pipeline::ForgeQueryBindingOutcome<
        crate::binding_pipeline::ForgeQueryContinuationBindingInput<D, I>,
    >,
) -> (
    ForgeQueryPreparedContinuationOutcome<D, I>,
    &'static str,
    crate::binding_pipeline::ForgeQueryBindingNarrowingDecision,
) {
    match outcome {
        crate::binding_pipeline::ForgeQueryBindingOutcome::Ambiguous(reason) => (
            ForgeQueryPreparedContinuationOutcome::Ambiguous(reason.reason().to_string()),
            "continuation_binding",
            crate::binding_pipeline::ForgeQueryBindingNarrowingDecision::new(
                "prepared continuation stopped because continuation binding remained ambiguous",
            ),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::Unavailable(reason) => (
            ForgeQueryPreparedContinuationOutcome::Unavailable(reason.reason().to_string()),
            "continuation_binding",
            crate::binding_pipeline::ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::WrongWorld(reason) => (
            ForgeQueryPreparedContinuationOutcome::WrongWorld(reason.reason().to_string()),
            "world_alignment",
            crate::binding_pipeline::ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::WrongHandle(reason) => (
            ForgeQueryPreparedContinuationOutcome::WrongHandle(reason.reason().to_string()),
            "handle_alignment",
            crate::binding_pipeline::ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::Stale(reason) => (
            ForgeQueryPreparedContinuationOutcome::Stale(reason.reason().to_string()),
            "basis_freshness",
            crate::binding_pipeline::ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::RebindRequired(reason) => (
            ForgeQueryPreparedContinuationOutcome::RebindRequired(reason.reason().to_string()),
            "continuation_binding",
            crate::binding_pipeline::ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::AuthorityMismatch(reason) => (
            ForgeQueryPreparedContinuationOutcome::AuthorityMismatch(reason.reason().to_string()),
            "authority_alignment",
            crate::binding_pipeline::ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::BasisMismatch(reason) => (
            ForgeQueryPreparedContinuationOutcome::BasisMismatch(reason.reason().to_string()),
            "basis_alignment",
            crate::binding_pipeline::ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::MissingRequiredAspect(reason) => (
            ForgeQueryPreparedContinuationOutcome::Denied(reason.reason().to_string()),
            "aspect_fit",
            crate::binding_pipeline::ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::AspectConflict(reason) => (
            ForgeQueryPreparedContinuationOutcome::Denied(reason.reason().to_string()),
            "aspect_fit",
            crate::binding_pipeline::ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::ExplicitNarrowingRequired(reason) => (
            ForgeQueryPreparedContinuationOutcome::RebindRequired(reason.reason().to_string()),
            "continuation_binding",
            crate::binding_pipeline::ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::Unsupported(reason) => (
            ForgeQueryPreparedContinuationOutcome::Unsupported(reason.reason().to_string()),
            "continuation_binding",
            crate::binding_pipeline::ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::Bound(_) => unreachable!(),
    }
}

pub(super) fn prepared_outcome_token<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    outcome: &ForgeQueryPreparedContinuationOutcome<D, I>,
) -> &str {
    match outcome {
        ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared.prepared_digest(),
        ForgeQueryPreparedContinuationOutcome::Ambiguous(_) => "ambiguous",
        ForgeQueryPreparedContinuationOutcome::Unavailable(_) => "unavailable",
        ForgeQueryPreparedContinuationOutcome::WrongWorld(_) => "wrong_world",
        ForgeQueryPreparedContinuationOutcome::WrongHandle(_) => "wrong_handle",
        ForgeQueryPreparedContinuationOutcome::Stale(_) => "stale",
        ForgeQueryPreparedContinuationOutcome::RebindRequired(_) => "rebind_required",
        ForgeQueryPreparedContinuationOutcome::AuthorityMismatch(_) => "authority_mismatch",
        ForgeQueryPreparedContinuationOutcome::BasisMismatch(_) => "basis_mismatch",
        ForgeQueryPreparedContinuationOutcome::Unsupported(_) => "unsupported",
        ForgeQueryPreparedContinuationOutcome::Deferred(_) => "deferred",
        ForgeQueryPreparedContinuationOutcome::Denied(_) => "denied",
        ForgeQueryPreparedContinuationOutcome::Failed(_) => "failed",
    }
}
