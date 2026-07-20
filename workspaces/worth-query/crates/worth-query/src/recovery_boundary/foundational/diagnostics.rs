use worth_foundational::facade::{
    FoundationalDiagnosticDenialClass, FoundationalDiagnosticOutcomeKind,
};

use crate::recovery_boundary::{
    WorthQueryRecoveryFoundationalDiagnosticContext, WorthQueryRecoveryStopKind,
};

pub(crate) fn diagnostic_context_for_stop_kind(
    stop_kind: WorthQueryRecoveryStopKind,
) -> WorthQueryRecoveryFoundationalDiagnosticContext {
    match stop_kind {
        WorthQueryRecoveryStopKind::Deferred => {
            WorthQueryRecoveryFoundationalDiagnosticContext::new(
                FoundationalDiagnosticOutcomeKind::Deferred,
                None,
            )
        }
        WorthQueryRecoveryStopKind::Unsupported => {
            WorthQueryRecoveryFoundationalDiagnosticContext::new(
                FoundationalDiagnosticOutcomeKind::Unsupported,
                Some(FoundationalDiagnosticDenialClass::UnsupportedDenied),
            )
        }
        WorthQueryRecoveryStopKind::AspectConflict
        | WorthQueryRecoveryStopKind::AsyncRequestDrift
        | WorthQueryRecoveryStopKind::AuthorityMismatch
        | WorthQueryRecoveryStopKind::BasisMismatch
        | WorthQueryRecoveryStopKind::PreviewCrossedResidue
        | WorthQueryRecoveryStopKind::RemaskDrift
        | WorthQueryRecoveryStopKind::ReplayDrift
        | WorthQueryRecoveryStopKind::WrongHandle
        | WorthQueryRecoveryStopKind::WrongWorld => {
            WorthQueryRecoveryFoundationalDiagnosticContext::new(
                FoundationalDiagnosticOutcomeKind::Mismatch,
                None,
            )
        }
        WorthQueryRecoveryStopKind::Ambiguous
        | WorthQueryRecoveryStopKind::Unavailable
        | WorthQueryRecoveryStopKind::Failed
        | WorthQueryRecoveryStopKind::RebindRequired
        | WorthQueryRecoveryStopKind::Refused
        | WorthQueryRecoveryStopKind::Stale
        | WorthQueryRecoveryStopKind::StaleCompletion => {
            WorthQueryRecoveryFoundationalDiagnosticContext::new(
                FoundationalDiagnosticOutcomeKind::Partial,
                None,
            )
        }
        WorthQueryRecoveryStopKind::MissingRequiredAspect
        | WorthQueryRecoveryStopKind::DeclarationDenied
        | WorthQueryRecoveryStopKind::ContributionDenied => {
            WorthQueryRecoveryFoundationalDiagnosticContext::new(
                FoundationalDiagnosticOutcomeKind::Denied,
                Some(FoundationalDiagnosticDenialClass::DomainDenied),
            )
        }
    }
}
