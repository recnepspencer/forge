use super::{
    WorthUiNativeIntentPosture, WorthUiNativeIntentStop, WorthUiNativeIntentStopped,
    WorthUiNativeIntentTransition,
};

#[derive(Clone, Copy)]
pub(super) struct NativePostureTarget {
    pub(super) graph_node: crate::graph::UiGraphNodeIdentity,
    pub(super) target: crate::facade::interaction::UiPresentedInteractionTargetView,
    pub(super) definition: crate::facade::intent::UiIntentId,
}

impl NativePostureTarget {
    pub(super) fn product(route: &crate::facade::intent::UiResolvedProductIntentRoute) -> Self {
        Self {
            graph_node: route.graph_node(),
            target: route.target(),
            definition: route.definition_id(),
        }
    }

    pub(super) fn confirmation(
        route: &crate::facade::intent::UiResolvedConfirmationIntentRoute,
    ) -> Self {
        Self {
            graph_node: route.graph_node(),
            target: route.source().target(),
            definition: route.definition_id(),
        }
    }
}

pub(super) fn stopped(
    stop: WorthUiNativeIntentStop,
    posture: Option<WorthUiNativeIntentPosture>,
) -> WorthUiNativeIntentTransition {
    WorthUiNativeIntentTransition::Stopped(WorthUiNativeIntentStopped { stop, posture })
}

pub(super) fn confirmation_stop_posture(
    reason: &crate::facade::intent::UiIntentConfirmationStopReason,
) -> crate::fact_contract::UiIntentPostureKind {
    use crate::facade::intent::UiIntentConfirmationStopReason as Reason;
    match reason {
        Reason::AlreadyContinued
        | Reason::AlreadyStopped
        | Reason::MonotonicTimeRegressed { .. }
        | Reason::Expired { .. }
        | Reason::ApplicationWorldChanged
        | Reason::ApplicationGenerationChanged
        | Reason::ConfirmationRouteChanged
        | Reason::ProductRouteChanged
        | Reason::ConfirmationNotPresented
        | Reason::ConfirmationPresentationStale
        | Reason::ConfirmationTargetChanged(_)
        | Reason::TargetChanged(_)
        | Reason::PayloadInputChanged
        | Reason::OperabilityDependencyChanged
        | Reason::PolicyChanged
        | Reason::ConfirmationPolicyChanged
        | Reason::OccupancyChanged => crate::fact_contract::UiIntentPostureKind::StaleConfirmation,
        Reason::CandidateNotExclusivelyConfirmable
        | Reason::MonotonicTimeRequired { .. }
        | Reason::ChallengeExpiryOverflow
        | Reason::ChallengeCapacityExceeded { .. }
        | Reason::ChallengeIdentityExhausted
        | Reason::NoPendingChallenge { .. }
        | Reason::AmbiguousPendingChallenges { .. }
        | Reason::LifecycleCancelled(_) => crate::fact_contract::UiIntentPostureKind::Denied,
    }
}
