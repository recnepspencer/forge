use crate::ordinary_outcome::{
    ForgeQueryOrdinaryContributionComposedCheckedTopologyKind, ForgeQueryOrdinaryOutcome,
    ForgeQueryOrdinaryPosture, ForgeQueryOrdinaryPostureKind,
};

use super::brief::{
    ForgeQueryRecoveryAction, ForgeQueryRecoveryAuthoritySurface, ForgeQueryRecoveryBrief,
    ForgeQueryRecoveryStopFamily, ForgeQueryRecoveryStopKind,
};
use super::explanation::ForgeQueryRecoveryExplanation;

pub fn forge_query_recovery_brief_from_ordinary_outcome<T>(
    outcome: &ForgeQueryOrdinaryOutcome<T>,
) -> Option<ForgeQueryRecoveryBrief> {
    match outcome {
        ForgeQueryOrdinaryOutcome::Bound(_) => None,
        ForgeQueryOrdinaryOutcome::Ambiguous(posture)
        | ForgeQueryOrdinaryOutcome::AspectConflict(posture)
        | ForgeQueryOrdinaryOutcome::AuthorityMismatch(posture)
        | ForgeQueryOrdinaryOutcome::BasisMismatch(posture)
        | ForgeQueryOrdinaryOutcome::Deferred(posture)
        | ForgeQueryOrdinaryOutcome::Denied(posture)
        | ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(posture)
        | ForgeQueryOrdinaryOutcome::Failed(posture)
        | ForgeQueryOrdinaryOutcome::MissingRequiredAspect(posture)
        | ForgeQueryOrdinaryOutcome::RebindRequired(posture)
        | ForgeQueryOrdinaryOutcome::Refused(posture)
        | ForgeQueryOrdinaryOutcome::Stale(posture)
        | ForgeQueryOrdinaryOutcome::Unavailable(posture)
        | ForgeQueryOrdinaryOutcome::Unsupported(posture)
        | ForgeQueryOrdinaryOutcome::WrongHandle(posture)
        | ForgeQueryOrdinaryOutcome::WrongWorld(posture) => {
            Some(recovery_brief_from_posture(posture))
        }
    }
}

pub(crate) fn recovery_brief_from_posture(
    posture: &ForgeQueryOrdinaryPosture,
) -> ForgeQueryRecoveryBrief {
    let stop_family = stop_family(posture);
    let stop_kind = stop_kind(posture);
    let authority_surface = authority_surface(posture, stop_kind);
    let recommended_action = recommended_action(posture, stop_kind);
    ForgeQueryRecoveryBrief::new(
        stop_family,
        stop_kind,
        authority_surface,
        recommended_action,
        posture.reason(),
        ForgeQueryRecoveryExplanation::new(posture.checked_topology().clone()),
    )
}

fn stop_family(posture: &ForgeQueryOrdinaryPosture) -> ForgeQueryRecoveryStopFamily {
    let topology = posture.checked_topology();
    if topology.orchestration_stop_stage().is_some() {
        ForgeQueryRecoveryStopFamily::DeclarationEntry
    } else if topology.binding_kind().is_some() {
        ForgeQueryRecoveryStopFamily::Binding
    } else if topology.continuation_kind().is_some() {
        ForgeQueryRecoveryStopFamily::Continuation
    } else if topology.signal_compatibility_orchestration_kind().is_some() {
        ForgeQueryRecoveryStopFamily::SignalCompatibilityOrchestration
    } else {
        ForgeQueryRecoveryStopFamily::ContributionComposedOrchestration
    }
}

fn stop_kind(posture: &ForgeQueryOrdinaryPosture) -> ForgeQueryRecoveryStopKind {
    match posture.checked_topology().contribution_composed_kind() {
        Some(ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::DeclarationDenied) => {
            ForgeQueryRecoveryStopKind::DeclarationDenied
        }
        Some(ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::ContributionDenied) => {
            ForgeQueryRecoveryStopKind::ContributionDenied
        }
        _ => match posture.kind() {
            ForgeQueryOrdinaryPostureKind::Ambiguous => ForgeQueryRecoveryStopKind::Ambiguous,
            ForgeQueryOrdinaryPostureKind::AspectConflict => {
                ForgeQueryRecoveryStopKind::AspectConflict
            }
            ForgeQueryOrdinaryPostureKind::AuthorityMismatch => {
                ForgeQueryRecoveryStopKind::AuthorityMismatch
            }
            ForgeQueryOrdinaryPostureKind::BasisMismatch => {
                ForgeQueryRecoveryStopKind::BasisMismatch
            }
            ForgeQueryOrdinaryPostureKind::Deferred => ForgeQueryRecoveryStopKind::Deferred,
            ForgeQueryOrdinaryPostureKind::Denied => ForgeQueryRecoveryStopKind::DeclarationDenied,
            ForgeQueryOrdinaryPostureKind::ExplicitNarrowingRequired => {
                ForgeQueryRecoveryStopKind::Ambiguous
            }
            ForgeQueryOrdinaryPostureKind::Failed => ForgeQueryRecoveryStopKind::Failed,
            ForgeQueryOrdinaryPostureKind::MissingRequiredAspect => {
                ForgeQueryRecoveryStopKind::MissingRequiredAspect
            }
            ForgeQueryOrdinaryPostureKind::RebindRequired => {
                ForgeQueryRecoveryStopKind::RebindRequired
            }
            ForgeQueryOrdinaryPostureKind::Refused => ForgeQueryRecoveryStopKind::Refused,
            ForgeQueryOrdinaryPostureKind::Stale => ForgeQueryRecoveryStopKind::Stale,
            ForgeQueryOrdinaryPostureKind::Unavailable => ForgeQueryRecoveryStopKind::Unavailable,
            ForgeQueryOrdinaryPostureKind::Unsupported => ForgeQueryRecoveryStopKind::Unsupported,
            ForgeQueryOrdinaryPostureKind::WrongHandle => ForgeQueryRecoveryStopKind::WrongHandle,
            ForgeQueryOrdinaryPostureKind::WrongWorld => ForgeQueryRecoveryStopKind::WrongWorld,
        },
    }
}

fn authority_surface(
    posture: &ForgeQueryOrdinaryPosture,
    stop_kind: ForgeQueryRecoveryStopKind,
) -> ForgeQueryRecoveryAuthoritySurface {
    match stop_kind {
        ForgeQueryRecoveryStopKind::WrongWorld => {
            ForgeQueryRecoveryAuthoritySurface::AdmittedOperatingWorld
        }
        ForgeQueryRecoveryStopKind::WrongHandle => {
            ForgeQueryRecoveryAuthoritySurface::HandleIdentity
        }
        ForgeQueryRecoveryStopKind::Ambiguous => ForgeQueryRecoveryAuthoritySurface::InputNarrowing,
        ForgeQueryRecoveryStopKind::Unavailable => {
            ForgeQueryRecoveryAuthoritySurface::AvailabilityDiscovery
        }
        ForgeQueryRecoveryStopKind::AspectConflict
        | ForgeQueryRecoveryStopKind::MissingRequiredAspect
        | ForgeQueryRecoveryStopKind::DeclarationDenied => {
            ForgeQueryRecoveryAuthoritySurface::DeclarationMeaning
        }
        ForgeQueryRecoveryStopKind::AuthorityMismatch | ForgeQueryRecoveryStopKind::Refused => {
            ForgeQueryRecoveryAuthoritySurface::AutomationBoundary
        }
        ForgeQueryRecoveryStopKind::BasisMismatch => {
            if posture
                .checked_topology()
                .signal_compatibility_orchestration_kind()
                .is_some()
            {
                ForgeQueryRecoveryAuthoritySurface::SignalCompatibility
            } else {
                ForgeQueryRecoveryAuthoritySurface::TruthContinuationContext
            }
        }
        ForgeQueryRecoveryStopKind::Deferred | ForgeQueryRecoveryStopKind::Unsupported => {
            ForgeQueryRecoveryAuthoritySurface::SupportReadiness
        }
        ForgeQueryRecoveryStopKind::Stale => {
            ForgeQueryRecoveryAuthoritySurface::TruthContinuationContext
        }
        ForgeQueryRecoveryStopKind::RebindRequired => {
            ForgeQueryRecoveryAuthoritySurface::BoundInputContext
        }
        ForgeQueryRecoveryStopKind::ContributionDenied => {
            ForgeQueryRecoveryAuthoritySurface::ContributionComposition
        }
        ForgeQueryRecoveryStopKind::Failed => ForgeQueryRecoveryAuthoritySurface::FailureEscalation,
    }
}

fn recommended_action(
    posture: &ForgeQueryOrdinaryPosture,
    stop_kind: ForgeQueryRecoveryStopKind,
) -> ForgeQueryRecoveryAction {
    match stop_kind {
        ForgeQueryRecoveryStopKind::DeclarationDenied => {
            ForgeQueryRecoveryAction::RepairDeclarationMeaning
        }
        ForgeQueryRecoveryStopKind::ContributionDenied => {
            ForgeQueryRecoveryAction::ReviewContributionIntent
        }
        _ => match posture.kind() {
            ForgeQueryOrdinaryPostureKind::Ambiguous
            | ForgeQueryOrdinaryPostureKind::ExplicitNarrowingRequired => {
                ForgeQueryRecoveryAction::NarrowInput
            }
            ForgeQueryOrdinaryPostureKind::AspectConflict
            | ForgeQueryOrdinaryPostureKind::MissingRequiredAspect => {
                ForgeQueryRecoveryAction::RepairDeclarationMeaning
            }
            ForgeQueryOrdinaryPostureKind::AuthorityMismatch
            | ForgeQueryOrdinaryPostureKind::Refused => {
                ForgeQueryRecoveryAction::UseExplicitHandoff
            }
            ForgeQueryOrdinaryPostureKind::BasisMismatch | ForgeQueryOrdinaryPostureKind::Stale => {
                ForgeQueryRecoveryAction::RefreshBasis
            }
            ForgeQueryOrdinaryPostureKind::Deferred => ForgeQueryRecoveryAction::RetryLater,
            ForgeQueryOrdinaryPostureKind::Denied => ForgeQueryRecoveryAction::InspectCheckedLane,
            ForgeQueryOrdinaryPostureKind::Failed => ForgeQueryRecoveryAction::EscalateFailure,
            ForgeQueryOrdinaryPostureKind::RebindRequired => {
                ForgeQueryRecoveryAction::RebindContext
            }
            ForgeQueryOrdinaryPostureKind::Unavailable => {
                ForgeQueryRecoveryAction::GatherAvailability
            }
            ForgeQueryOrdinaryPostureKind::Unsupported => ForgeQueryRecoveryAction::CheckSupport,
            ForgeQueryOrdinaryPostureKind::WrongHandle => ForgeQueryRecoveryAction::CorrectHandle,
            ForgeQueryOrdinaryPostureKind::WrongWorld => ForgeQueryRecoveryAction::CorrectWorld,
        },
    }
}
