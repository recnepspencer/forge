use crate::ordinary_outcome::{
    WorthQueryOrdinaryContinuationCheckedTopologyKind,
    WorthQueryOrdinaryContributionComposedCheckedTopologyKind, WorthQueryOrdinaryPosture,
    WorthQueryOrdinaryPostureKind,
};
use crate::WorthQueryContributionComposedClassification;

use super::contribution::enrich_contribution_explanation;
use crate::recovery_boundary::foundational::{
    basis_posture_for_foundational_disclosure, diagnostic_context_for_stop_kind,
    support_context_for_basis_mismatch, support_context_for_stale_basis,
};
use crate::recovery_boundary::{
    WorthQueryRecoveryAction, WorthQueryRecoveryAspectPosture, WorthQueryRecoveryAuthoritySurface,
    WorthQueryRecoveryBasisPosture, WorthQueryRecoveryBrief, WorthQueryRecoveryExplanation,
    WorthQueryRecoverySourceFamily, WorthQueryRecoveryStopFamily, WorthQueryRecoveryStopKind,
};

pub(crate) fn recovery_brief_from_posture(
    posture: &WorthQueryOrdinaryPosture,
) -> WorthQueryRecoveryBrief {
    let stop_family = stop_family(posture);
    let stop_kind = stop_kind(posture);
    let source_family = source_family(stop_family);
    let authority_surface = authority_surface(posture, stop_kind);
    let recommended_action = recommended_action(posture, stop_kind);
    let explanation = enrich_explanation(
        WorthQueryRecoveryExplanation::new_with_source_family(
            posture.checked_topology().clone(),
            source_family,
        ),
        posture,
        stop_kind,
    );
    WorthQueryRecoveryBrief::new(
        stop_family,
        stop_kind,
        authority_surface,
        recommended_action,
        posture.reason(),
        explanation,
    )
}

fn enrich_explanation(
    explanation: WorthQueryRecoveryExplanation,
    posture: &WorthQueryOrdinaryPosture,
    stop_kind: WorthQueryRecoveryStopKind,
) -> WorthQueryRecoveryExplanation {
    let mut explanation = explanation
        .with_basis_posture(basis_posture(posture))
        .with_aspect_posture(aspect_posture(posture))
        .with_diagnostic_context(diagnostic_context_for_stop_kind(stop_kind));

    if posture.kind() == WorthQueryOrdinaryPostureKind::Stale {
        let support_context = support_context_for_stale_basis();
        explanation = explanation
            .with_basis_posture(basis_posture_for_foundational_disclosure(
                support_context.basis_disclosure(),
            ))
            .with_support_context(support_context);
    } else if posture.kind() == WorthQueryOrdinaryPostureKind::BasisMismatch {
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

fn stop_family(posture: &WorthQueryOrdinaryPosture) -> WorthQueryRecoveryStopFamily {
    let topology = posture.checked_topology();
    if topology.orchestration_stop_stage().is_some() {
        WorthQueryRecoveryStopFamily::DeclarationEntry
    } else if topology.binding_kind().is_some() {
        WorthQueryRecoveryStopFamily::Binding
    } else if topology.continuation_kind().is_some() {
        WorthQueryRecoveryStopFamily::Continuation
    } else if topology.signal_compatibility_orchestration_kind().is_some() {
        WorthQueryRecoveryStopFamily::SignalCompatibilityOrchestration
    } else {
        WorthQueryRecoveryStopFamily::ContributionComposedOrchestration
    }
}

fn source_family(stop_family: WorthQueryRecoveryStopFamily) -> WorthQueryRecoverySourceFamily {
    match stop_family {
        WorthQueryRecoveryStopFamily::Binding => WorthQueryRecoverySourceFamily::Binding,
        WorthQueryRecoveryStopFamily::Continuation => WorthQueryRecoverySourceFamily::Continuation,
        WorthQueryRecoveryStopFamily::ContributionComposedOrchestration => {
            WorthQueryRecoverySourceFamily::ContributionComposed
        }
        WorthQueryRecoveryStopFamily::DeclarationEntry => {
            WorthQueryRecoverySourceFamily::DeclarationEntry
        }
        WorthQueryRecoveryStopFamily::DeclarationReceipt => {
            WorthQueryRecoverySourceFamily::DeclarationReceipt
        }
        WorthQueryRecoveryStopFamily::DeclarationRoutePlan => {
            WorthQueryRecoverySourceFamily::DeclarationRoutePlan
        }
        WorthQueryRecoveryStopFamily::GroupedNeighborhoodOrchestration => {
            WorthQueryRecoverySourceFamily::GroupedNeighborhood
        }
        WorthQueryRecoveryStopFamily::SignalCompatibilityOrchestration => {
            WorthQueryRecoverySourceFamily::SignalCompatibility
        }
    }
}

fn stop_kind(posture: &WorthQueryOrdinaryPosture) -> WorthQueryRecoveryStopKind {
    if let Some(kind) = posture.checked_topology().continuation_kind() {
        return match kind {
            WorthQueryOrdinaryContinuationCheckedTopologyKind::AsyncRequestDrift => {
                WorthQueryRecoveryStopKind::AsyncRequestDrift
            }
            WorthQueryOrdinaryContinuationCheckedTopologyKind::ReplayDrift => {
                WorthQueryRecoveryStopKind::ReplayDrift
            }
            WorthQueryOrdinaryContinuationCheckedTopologyKind::RemaskDrift => {
                WorthQueryRecoveryStopKind::RemaskDrift
            }
            WorthQueryOrdinaryContinuationCheckedTopologyKind::PreviewCrossedResidue => {
                WorthQueryRecoveryStopKind::PreviewCrossedResidue
            }
            WorthQueryOrdinaryContinuationCheckedTopologyKind::StaleCompletion => {
                WorthQueryRecoveryStopKind::StaleCompletion
            }
            _ => stop_kind_from_posture(posture),
        };
    }
    stop_kind_from_posture(posture)
}

fn stop_kind_from_posture(posture: &WorthQueryOrdinaryPosture) -> WorthQueryRecoveryStopKind {
    match posture.checked_topology().contribution_composed_kind() {
        Some(WorthQueryOrdinaryContributionComposedCheckedTopologyKind::DeclarationDenied) => {
            WorthQueryRecoveryStopKind::DeclarationDenied
        }
        Some(WorthQueryOrdinaryContributionComposedCheckedTopologyKind::ContributionDenied) => {
            WorthQueryRecoveryStopKind::ContributionDenied
        }
        _ => match posture.kind() {
            WorthQueryOrdinaryPostureKind::Ambiguous => WorthQueryRecoveryStopKind::Ambiguous,
            WorthQueryOrdinaryPostureKind::AspectConflict => {
                WorthQueryRecoveryStopKind::AspectConflict
            }
            WorthQueryOrdinaryPostureKind::AuthorityMismatch => {
                WorthQueryRecoveryStopKind::AuthorityMismatch
            }
            WorthQueryOrdinaryPostureKind::BasisMismatch => {
                WorthQueryRecoveryStopKind::BasisMismatch
            }
            WorthQueryOrdinaryPostureKind::Deferred => WorthQueryRecoveryStopKind::Deferred,
            WorthQueryOrdinaryPostureKind::Denied => WorthQueryRecoveryStopKind::DeclarationDenied,
            WorthQueryOrdinaryPostureKind::ExplicitNarrowingRequired => {
                WorthQueryRecoveryStopKind::Ambiguous
            }
            WorthQueryOrdinaryPostureKind::Failed => WorthQueryRecoveryStopKind::Failed,
            WorthQueryOrdinaryPostureKind::MissingRequiredAspect => {
                WorthQueryRecoveryStopKind::MissingRequiredAspect
            }
            WorthQueryOrdinaryPostureKind::RebindRequired => {
                WorthQueryRecoveryStopKind::RebindRequired
            }
            WorthQueryOrdinaryPostureKind::Refused => WorthQueryRecoveryStopKind::Refused,
            WorthQueryOrdinaryPostureKind::Stale => WorthQueryRecoveryStopKind::Stale,
            WorthQueryOrdinaryPostureKind::Unavailable => WorthQueryRecoveryStopKind::Unavailable,
            WorthQueryOrdinaryPostureKind::Unsupported => WorthQueryRecoveryStopKind::Unsupported,
            WorthQueryOrdinaryPostureKind::WrongHandle => WorthQueryRecoveryStopKind::WrongHandle,
            WorthQueryOrdinaryPostureKind::WrongWorld => WorthQueryRecoveryStopKind::WrongWorld,
        },
    }
}

fn authority_surface(
    posture: &WorthQueryOrdinaryPosture,
    stop_kind: WorthQueryRecoveryStopKind,
) -> WorthQueryRecoveryAuthoritySurface {
    match stop_kind {
        WorthQueryRecoveryStopKind::WrongWorld => {
            WorthQueryRecoveryAuthoritySurface::AdmittedOperatingWorld
        }
        WorthQueryRecoveryStopKind::WrongHandle => {
            WorthQueryRecoveryAuthoritySurface::HandleIdentity
        }
        WorthQueryRecoveryStopKind::Ambiguous => WorthQueryRecoveryAuthoritySurface::InputNarrowing,
        WorthQueryRecoveryStopKind::Unavailable => {
            WorthQueryRecoveryAuthoritySurface::AvailabilityDiscovery
        }
        WorthQueryRecoveryStopKind::AspectConflict
        | WorthQueryRecoveryStopKind::MissingRequiredAspect
        | WorthQueryRecoveryStopKind::DeclarationDenied => {
            WorthQueryRecoveryAuthoritySurface::DeclarationMeaning
        }
        WorthQueryRecoveryStopKind::AuthorityMismatch | WorthQueryRecoveryStopKind::Refused => {
            WorthQueryRecoveryAuthoritySurface::AutomationBoundary
        }
        WorthQueryRecoveryStopKind::AsyncRequestDrift => {
            WorthQueryRecoveryAuthoritySurface::BoundInputContext
        }
        WorthQueryRecoveryStopKind::BasisMismatch => {
            if posture
                .checked_topology()
                .signal_compatibility_orchestration_kind()
                .is_some()
            {
                WorthQueryRecoveryAuthoritySurface::SignalCompatibility
            } else {
                WorthQueryRecoveryAuthoritySurface::TruthContinuationContext
            }
        }
        WorthQueryRecoveryStopKind::PreviewCrossedResidue
        | WorthQueryRecoveryStopKind::ReplayDrift
        | WorthQueryRecoveryStopKind::StaleCompletion => {
            WorthQueryRecoveryAuthoritySurface::TruthContinuationContext
        }
        WorthQueryRecoveryStopKind::Deferred | WorthQueryRecoveryStopKind::Unsupported => {
            WorthQueryRecoveryAuthoritySurface::SupportReadiness
        }
        WorthQueryRecoveryStopKind::RemaskDrift => {
            WorthQueryRecoveryAuthoritySurface::SupportReadiness
        }
        WorthQueryRecoveryStopKind::Stale => {
            WorthQueryRecoveryAuthoritySurface::TruthContinuationContext
        }
        WorthQueryRecoveryStopKind::RebindRequired => {
            WorthQueryRecoveryAuthoritySurface::BoundInputContext
        }
        WorthQueryRecoveryStopKind::ContributionDenied => {
            WorthQueryRecoveryAuthoritySurface::ContributionComposition
        }
        WorthQueryRecoveryStopKind::Failed => WorthQueryRecoveryAuthoritySurface::FailureEscalation,
    }
}

fn recommended_action(
    posture: &WorthQueryOrdinaryPosture,
    stop_kind: WorthQueryRecoveryStopKind,
) -> WorthQueryRecoveryAction {
    match stop_kind {
        WorthQueryRecoveryStopKind::AsyncRequestDrift => WorthQueryRecoveryAction::RebindContext,
        WorthQueryRecoveryStopKind::PreviewCrossedResidue => {
            WorthQueryRecoveryAction::UseExplicitHandoff
        }
        WorthQueryRecoveryStopKind::RemaskDrift => WorthQueryRecoveryAction::CheckSupport,
        WorthQueryRecoveryStopKind::ReplayDrift | WorthQueryRecoveryStopKind::StaleCompletion => {
            WorthQueryRecoveryAction::RefreshBasis
        }
        WorthQueryRecoveryStopKind::DeclarationDenied => {
            WorthQueryRecoveryAction::RepairDeclarationMeaning
        }
        WorthQueryRecoveryStopKind::ContributionDenied => {
            WorthQueryRecoveryAction::ReviewContributionIntent
        }
        _ => match posture.kind() {
            WorthQueryOrdinaryPostureKind::Ambiguous
            | WorthQueryOrdinaryPostureKind::ExplicitNarrowingRequired => {
                WorthQueryRecoveryAction::NarrowInput
            }
            WorthQueryOrdinaryPostureKind::AspectConflict
            | WorthQueryOrdinaryPostureKind::MissingRequiredAspect => {
                WorthQueryRecoveryAction::RepairDeclarationMeaning
            }
            WorthQueryOrdinaryPostureKind::AuthorityMismatch
            | WorthQueryOrdinaryPostureKind::Refused => {
                WorthQueryRecoveryAction::UseExplicitHandoff
            }
            WorthQueryOrdinaryPostureKind::BasisMismatch | WorthQueryOrdinaryPostureKind::Stale => {
                WorthQueryRecoveryAction::RefreshBasis
            }
            WorthQueryOrdinaryPostureKind::Deferred => WorthQueryRecoveryAction::RetryLater,
            WorthQueryOrdinaryPostureKind::Denied => WorthQueryRecoveryAction::InspectCheckedLane,
            WorthQueryOrdinaryPostureKind::Failed => WorthQueryRecoveryAction::EscalateFailure,
            WorthQueryOrdinaryPostureKind::RebindRequired => {
                WorthQueryRecoveryAction::RebindContext
            }
            WorthQueryOrdinaryPostureKind::Unavailable => {
                WorthQueryRecoveryAction::GatherAvailability
            }
            WorthQueryOrdinaryPostureKind::Unsupported => WorthQueryRecoveryAction::CheckSupport,
            WorthQueryOrdinaryPostureKind::WrongHandle => WorthQueryRecoveryAction::CorrectHandle,
            WorthQueryOrdinaryPostureKind::WrongWorld => WorthQueryRecoveryAction::CorrectWorld,
        },
    }
}

fn basis_posture(posture: &WorthQueryOrdinaryPosture) -> WorthQueryRecoveryBasisPosture {
    match posture.kind() {
        WorthQueryOrdinaryPostureKind::BasisMismatch => {
            WorthQueryRecoveryBasisPosture::BasisMismatch
        }
        WorthQueryOrdinaryPostureKind::Stale => WorthQueryRecoveryBasisPosture::StaleBasis,
        _ => WorthQueryRecoveryBasisPosture::Unknown,
    }
}

fn aspect_posture(posture: &WorthQueryOrdinaryPosture) -> WorthQueryRecoveryAspectPosture {
    if posture
        .checked_topology()
        .contribution_composed_kind()
        .is_some()
    {
        return WorthQueryRecoveryAspectPosture::CategoryScopedAspectComposition;
    }
    if posture
        .checked_topology()
        .signal_compatibility_orchestration_kind()
        .is_some()
    {
        return WorthQueryRecoveryAspectPosture::RequiredContract;
    }
    if posture.checked_topology().continuation_kind().is_some() {
        return WorthQueryRecoveryAspectPosture::AspectSensitiveReadmission;
    }
    match posture.kind() {
        WorthQueryOrdinaryPostureKind::AspectConflict
        | WorthQueryOrdinaryPostureKind::MissingRequiredAspect => {
            WorthQueryRecoveryAspectPosture::RequiredContract
        }
        _ => WorthQueryRecoveryAspectPosture::None,
    }
}

pub(crate) fn kind_for_contribution_posture(
    kind: WorthQueryOrdinaryContributionComposedCheckedTopologyKind,
) -> WorthQueryContributionComposedClassification {
    match kind {
        WorthQueryOrdinaryContributionComposedCheckedTopologyKind::Deferred
        | WorthQueryOrdinaryContributionComposedCheckedTopologyKind::DeclarationDenied
        | WorthQueryOrdinaryContributionComposedCheckedTopologyKind::Unsupported
        | WorthQueryOrdinaryContributionComposedCheckedTopologyKind::Failed => {
            WorthQueryContributionComposedClassification::NoContributionAdmitted
        }
        WorthQueryOrdinaryContributionComposedCheckedTopologyKind::ContributionDenied
        | WorthQueryOrdinaryContributionComposedCheckedTopologyKind::Stale
        | WorthQueryOrdinaryContributionComposedCheckedTopologyKind::RebindRequired => {
            WorthQueryContributionComposedClassification::PartiallyAdmitted
        }
    }
}
