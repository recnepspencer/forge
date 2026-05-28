use crate::ordinary_outcome::{
    ForgeQueryOrdinaryContributionComposedCheckedTopologyKind, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};
use crate::ForgeQueryContributionComposedClassification;

use super::contribution::enrich_contribution_explanation;
use crate::recovery_boundary::foundational::{
    basis_posture_for_foundational_disclosure, diagnostic_context_for_stop_kind,
    support_context_for_basis_mismatch, support_context_for_stale_basis,
};
use crate::recovery_boundary::{
    ForgeQueryRecoveryAction, ForgeQueryRecoveryAspectPosture, ForgeQueryRecoveryAuthoritySurface,
    ForgeQueryRecoveryBasisPosture, ForgeQueryRecoveryBrief, ForgeQueryRecoveryExplanation,
    ForgeQueryRecoverySourceFamily, ForgeQueryRecoveryStopFamily, ForgeQueryRecoveryStopKind,
};

pub(crate) fn recovery_brief_from_posture(
    posture: &ForgeQueryOrdinaryPosture,
) -> ForgeQueryRecoveryBrief {
    let stop_family = stop_family(posture);
    let stop_kind = stop_kind(posture);
    let source_family = source_family(stop_family);
    let authority_surface = authority_surface(posture, stop_kind);
    let recommended_action = recommended_action(posture, stop_kind);
    let explanation = enrich_explanation(
        ForgeQueryRecoveryExplanation::new_with_source_family(
            posture.checked_topology().clone(),
            source_family,
        ),
        posture,
        stop_kind,
    );
    ForgeQueryRecoveryBrief::new(
        stop_family,
        stop_kind,
        authority_surface,
        recommended_action,
        posture.reason(),
        explanation,
    )
}

fn enrich_explanation(
    explanation: ForgeQueryRecoveryExplanation,
    posture: &ForgeQueryOrdinaryPosture,
    stop_kind: ForgeQueryRecoveryStopKind,
) -> ForgeQueryRecoveryExplanation {
    let mut explanation = explanation
        .with_basis_posture(basis_posture(posture))
        .with_aspect_posture(aspect_posture(posture))
        .with_diagnostic_context(diagnostic_context_for_stop_kind(stop_kind));

    if posture.kind() == ForgeQueryOrdinaryPostureKind::Stale {
        let support_context = support_context_for_stale_basis();
        explanation = explanation
            .with_basis_posture(basis_posture_for_foundational_disclosure(
                support_context.basis_disclosure(),
            ))
            .with_support_context(support_context);
    } else if posture.kind() == ForgeQueryOrdinaryPostureKind::BasisMismatch {
        explanation = explanation.with_support_context(support_context_for_basis_mismatch());
    }

    if posture
        .checked_topology()
        .contribution_composed_kind()
        .is_some()
    {
        explanation = enrich_contribution_explanation(explanation, posture);
    }

    explanation
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

fn source_family(stop_family: ForgeQueryRecoveryStopFamily) -> ForgeQueryRecoverySourceFamily {
    match stop_family {
        ForgeQueryRecoveryStopFamily::Binding => ForgeQueryRecoverySourceFamily::Binding,
        ForgeQueryRecoveryStopFamily::Continuation => ForgeQueryRecoverySourceFamily::Continuation,
        ForgeQueryRecoveryStopFamily::ContributionComposedOrchestration => {
            ForgeQueryRecoverySourceFamily::ContributionComposed
        }
        ForgeQueryRecoveryStopFamily::DeclarationEntry => {
            ForgeQueryRecoverySourceFamily::DeclarationEntry
        }
        ForgeQueryRecoveryStopFamily::DeclarationReceipt => {
            ForgeQueryRecoverySourceFamily::DeclarationReceipt
        }
        ForgeQueryRecoveryStopFamily::DeclarationRoutePlan => {
            ForgeQueryRecoverySourceFamily::DeclarationRoutePlan
        }
        ForgeQueryRecoveryStopFamily::GroupedNeighborhoodOrchestration => {
            ForgeQueryRecoverySourceFamily::GroupedNeighborhood
        }
        ForgeQueryRecoveryStopFamily::SignalCompatibilityOrchestration => {
            ForgeQueryRecoverySourceFamily::SignalCompatibility
        }
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

fn basis_posture(posture: &ForgeQueryOrdinaryPosture) -> ForgeQueryRecoveryBasisPosture {
    match posture.kind() {
        ForgeQueryOrdinaryPostureKind::BasisMismatch => {
            ForgeQueryRecoveryBasisPosture::BasisMismatch
        }
        ForgeQueryOrdinaryPostureKind::Stale => ForgeQueryRecoveryBasisPosture::StaleBasis,
        _ => ForgeQueryRecoveryBasisPosture::Unknown,
    }
}

fn aspect_posture(posture: &ForgeQueryOrdinaryPosture) -> ForgeQueryRecoveryAspectPosture {
    if posture
        .checked_topology()
        .contribution_composed_kind()
        .is_some()
    {
        return ForgeQueryRecoveryAspectPosture::CategoryScopedAspectComposition;
    }
    if posture
        .checked_topology()
        .signal_compatibility_orchestration_kind()
        .is_some()
    {
        return ForgeQueryRecoveryAspectPosture::RequiredContract;
    }
    if posture.checked_topology().continuation_kind().is_some() {
        return ForgeQueryRecoveryAspectPosture::AspectSensitiveReadmission;
    }
    match posture.kind() {
        ForgeQueryOrdinaryPostureKind::AspectConflict
        | ForgeQueryOrdinaryPostureKind::MissingRequiredAspect => {
            ForgeQueryRecoveryAspectPosture::RequiredContract
        }
        _ => ForgeQueryRecoveryAspectPosture::None,
    }
}

pub(crate) fn kind_for_contribution_posture(
    kind: ForgeQueryOrdinaryContributionComposedCheckedTopologyKind,
) -> ForgeQueryContributionComposedClassification {
    match kind {
        ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Deferred
        | ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::DeclarationDenied
        | ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Unsupported
        | ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Failed => {
            ForgeQueryContributionComposedClassification::NoContributionAdmitted
        }
        ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::ContributionDenied
        | ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Stale
        | ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::RebindRequired => {
            ForgeQueryContributionComposedClassification::PartiallyAdmitted
        }
    }
}
