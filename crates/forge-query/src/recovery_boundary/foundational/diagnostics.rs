use forge_foundational::facade::{
    FoundationalDiagnosticDenialClass, FoundationalDiagnosticOutcomeKind,
};

use crate::recovery_boundary::{
    ForgeQueryRecoveryFoundationalDiagnosticContext, ForgeQueryRecoveryStopKind,
};

pub(crate) fn diagnostic_context_for_stop_kind(
    stop_kind: ForgeQueryRecoveryStopKind,
) -> ForgeQueryRecoveryFoundationalDiagnosticContext {
    match stop_kind {
        ForgeQueryRecoveryStopKind::Deferred => {
            ForgeQueryRecoveryFoundationalDiagnosticContext::new(
                FoundationalDiagnosticOutcomeKind::Deferred,
                None,
            )
        }
        ForgeQueryRecoveryStopKind::Unsupported => {
            ForgeQueryRecoveryFoundationalDiagnosticContext::new(
                FoundationalDiagnosticOutcomeKind::Unsupported,
                Some(FoundationalDiagnosticDenialClass::UnsupportedDenied),
            )
        }
        ForgeQueryRecoveryStopKind::AspectConflict
        | ForgeQueryRecoveryStopKind::AsyncRequestDrift
        | ForgeQueryRecoveryStopKind::AuthorityMismatch
        | ForgeQueryRecoveryStopKind::BasisMismatch
        | ForgeQueryRecoveryStopKind::PreviewCrossedResidue
        | ForgeQueryRecoveryStopKind::RemaskDrift
        | ForgeQueryRecoveryStopKind::ReplayDrift
        | ForgeQueryRecoveryStopKind::WrongHandle
        | ForgeQueryRecoveryStopKind::WrongWorld => {
            ForgeQueryRecoveryFoundationalDiagnosticContext::new(
                FoundationalDiagnosticOutcomeKind::Mismatch,
                None,
            )
        }
        ForgeQueryRecoveryStopKind::Ambiguous
        | ForgeQueryRecoveryStopKind::Unavailable
        | ForgeQueryRecoveryStopKind::Failed
        | ForgeQueryRecoveryStopKind::RebindRequired
        | ForgeQueryRecoveryStopKind::Refused
        | ForgeQueryRecoveryStopKind::Stale
        | ForgeQueryRecoveryStopKind::StaleCompletion => {
            ForgeQueryRecoveryFoundationalDiagnosticContext::new(
                FoundationalDiagnosticOutcomeKind::Partial,
                None,
            )
        }
        ForgeQueryRecoveryStopKind::MissingRequiredAspect
        | ForgeQueryRecoveryStopKind::DeclarationDenied
        | ForgeQueryRecoveryStopKind::ContributionDenied => {
            ForgeQueryRecoveryFoundationalDiagnosticContext::new(
                FoundationalDiagnosticOutcomeKind::Denied,
                Some(FoundationalDiagnosticDenialClass::DomainDenied),
            )
        }
    }
}
