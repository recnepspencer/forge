use crate::application::{
    WorthQueryDeclarationBridgeRoutingDenialCause, WorthQueryDeclarationInput,
    WorthQueryDomainEntryMarker,
};
use crate::continuation_pipeline::WorthQueryPreparedContinuationOutcome;

use super::support::ResolvedSignalContinuationTruth;

pub(super) fn prepared_outcome_from_signal_truth<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    signal_truth: &ResolvedSignalContinuationTruth,
) -> Option<WorthQueryPreparedContinuationOutcome<D, I>> {
    match signal_truth.posture {
        crate::continuation_pipeline::artifacts::WorthQueryPreparedContinuationSignalPosture::Compatible => {
            None
        }
        crate::continuation_pipeline::artifacts::WorthQueryPreparedContinuationSignalPosture::Deferred => {
            Some(WorthQueryPreparedContinuationOutcome::Deferred(
                signal_truth.reason.to_string(),
            ))
        }
        crate::continuation_pipeline::artifacts::WorthQueryPreparedContinuationSignalPosture::Denied => {
            Some(WorthQueryPreparedContinuationOutcome::Denied(
                signal_truth.reason.to_string(),
            ))
        }
        crate::continuation_pipeline::artifacts::WorthQueryPreparedContinuationSignalPosture::Failed => {
            Some(WorthQueryPreparedContinuationOutcome::Failed(
                signal_truth.reason.to_string(),
            ))
        }
    }
}

pub(super) fn prepared_outcome_from_bridge_denial_cause<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    cause: WorthQueryDeclarationBridgeRoutingDenialCause,
    reason: &str,
) -> WorthQueryPreparedContinuationOutcome<D, I> {
    match cause {
        WorthQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch => {
            WorthQueryPreparedContinuationOutcome::WrongHandle(reason.to_string())
        }
        WorthQueryDeclarationBridgeRoutingDenialCause::BridgeAuthorityUnavailable
        | WorthQueryDeclarationBridgeRoutingDenialCause::AuthorityAspectGap
        | WorthQueryDeclarationBridgeRoutingDenialCause::AuthorityAspectAmbiguity => {
            WorthQueryPreparedContinuationOutcome::AuthorityMismatch(reason.to_string())
        }
        WorthQueryDeclarationBridgeRoutingDenialCause::BasisLifecycleMismatch => {
            WorthQueryPreparedContinuationOutcome::BasisMismatch(reason.to_string())
        }
        WorthQueryDeclarationBridgeRoutingDenialCause::UnsupportedContinuationMode
        | WorthQueryDeclarationBridgeRoutingDenialCause::UnsupportedTruthContext => {
            WorthQueryPreparedContinuationOutcome::Unsupported(reason.to_string())
        }
        _ => WorthQueryPreparedContinuationOutcome::Denied(reason.to_string()),
    }
}

pub(super) fn prepared_outcome_from_binding_outcome<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    outcome: crate::binding_pipeline::WorthQueryBindingOutcome<
        crate::binding_pipeline::WorthQueryContinuationBindingInput<D, I>,
    >,
) -> (
    WorthQueryPreparedContinuationOutcome<D, I>,
    &'static str,
    crate::binding_pipeline::WorthQueryBindingNarrowingDecision,
) {
    match outcome {
        crate::binding_pipeline::WorthQueryBindingOutcome::Ambiguous(reason) => (
            WorthQueryPreparedContinuationOutcome::Ambiguous(reason.reason().to_string()),
            "continuation_binding",
            crate::binding_pipeline::WorthQueryBindingNarrowingDecision::new(
                "prepared continuation stopped because continuation binding remained ambiguous",
            ),
        ),
        crate::binding_pipeline::WorthQueryBindingOutcome::Unavailable(reason) => (
            WorthQueryPreparedContinuationOutcome::Unavailable(reason.reason().to_string()),
            "continuation_binding",
            crate::binding_pipeline::WorthQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::WorthQueryBindingOutcome::WrongWorld(reason) => (
            WorthQueryPreparedContinuationOutcome::WrongWorld(reason.reason().to_string()),
            "world_alignment",
            crate::binding_pipeline::WorthQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::WorthQueryBindingOutcome::WrongHandle(reason) => (
            WorthQueryPreparedContinuationOutcome::WrongHandle(reason.reason().to_string()),
            "handle_alignment",
            crate::binding_pipeline::WorthQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::WorthQueryBindingOutcome::Stale(reason) => (
            WorthQueryPreparedContinuationOutcome::Stale(reason.reason().to_string()),
            "basis_freshness",
            crate::binding_pipeline::WorthQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::WorthQueryBindingOutcome::RebindRequired(reason) => (
            WorthQueryPreparedContinuationOutcome::RebindRequired(reason.reason().to_string()),
            "continuation_binding",
            crate::binding_pipeline::WorthQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::WorthQueryBindingOutcome::AuthorityMismatch(reason) => (
            WorthQueryPreparedContinuationOutcome::AuthorityMismatch(reason.reason().to_string()),
            "authority_alignment",
            crate::binding_pipeline::WorthQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::WorthQueryBindingOutcome::BasisMismatch(reason) => (
            WorthQueryPreparedContinuationOutcome::BasisMismatch(reason.reason().to_string()),
            "basis_alignment",
            crate::binding_pipeline::WorthQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::WorthQueryBindingOutcome::MissingRequiredAspect(reason) => (
            WorthQueryPreparedContinuationOutcome::Denied(reason.reason().to_string()),
            "aspect_fit",
            crate::binding_pipeline::WorthQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::WorthQueryBindingOutcome::AspectConflict(reason) => (
            WorthQueryPreparedContinuationOutcome::Denied(reason.reason().to_string()),
            "aspect_fit",
            crate::binding_pipeline::WorthQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::WorthQueryBindingOutcome::ExplicitNarrowingRequired(reason) => (
            WorthQueryPreparedContinuationOutcome::RebindRequired(reason.reason().to_string()),
            "continuation_binding",
            crate::binding_pipeline::WorthQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::WorthQueryBindingOutcome::Unsupported(reason) => (
            WorthQueryPreparedContinuationOutcome::Unsupported(reason.reason().to_string()),
            "continuation_binding",
            crate::binding_pipeline::WorthQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::WorthQueryBindingOutcome::Bound(_) => unreachable!(),
    }
}

pub(super) fn prepared_outcome_token<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    outcome: &WorthQueryPreparedContinuationOutcome<D, I>,
) -> &str {
    match outcome {
        WorthQueryPreparedContinuationOutcome::Prepared(prepared) => prepared.prepared_digest(),
        WorthQueryPreparedContinuationOutcome::Ambiguous(_) => "ambiguous",
        WorthQueryPreparedContinuationOutcome::Unavailable(_) => "unavailable",
        WorthQueryPreparedContinuationOutcome::WrongWorld(_) => "wrong_world",
        WorthQueryPreparedContinuationOutcome::WrongHandle(_) => "wrong_handle",
        WorthQueryPreparedContinuationOutcome::InstalledAuthorityDrift(_) => {
            "installed_authority_drift"
        }
        WorthQueryPreparedContinuationOutcome::Stale(_) => "stale",
        WorthQueryPreparedContinuationOutcome::RebindRequired(_) => "rebind_required",
        WorthQueryPreparedContinuationOutcome::AuthorityMismatch(_) => "authority_mismatch",
        WorthQueryPreparedContinuationOutcome::BasisMismatch(_) => "basis_mismatch",
        WorthQueryPreparedContinuationOutcome::Unsupported(_) => "unsupported",
        WorthQueryPreparedContinuationOutcome::Deferred(_) => "deferred",
        WorthQueryPreparedContinuationOutcome::Denied(_) => "denied",
        WorthQueryPreparedContinuationOutcome::Failed(_) => "failed",
    }
}
