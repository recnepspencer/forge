use crate::capability::CommandId;
use crate::runtime::{
    WorthUiInteractionKind, WorthUiInteractionReadiness, WorthUiInteractionTarget,
    WorthUiMountedInteractionGesture, WorthUiPrimitiveFocusPosture,
};

use super::{WorthUiInteractionOperabilityBasis, WorthUiInteractionOperabilityPosture};

pub(super) fn classify_interaction_operability(
    primitive_disabled: bool,
    readiness: WorthUiInteractionReadiness,
    kind: WorthUiInteractionKind,
    target: &WorthUiInteractionTarget,
    gesture: WorthUiMountedInteractionGesture,
    primitive_focus: WorthUiPrimitiveFocusPosture,
) -> (
    WorthUiInteractionOperabilityPosture,
    WorthUiInteractionOperabilityBasis,
) {
    if primitive_disabled {
        return (
            WorthUiInteractionOperabilityPosture::Disabled,
            WorthUiInteractionOperabilityBasis::PrimitiveDisabled,
        );
    }
    if readiness == WorthUiInteractionReadiness::Disabled {
        return (
            WorthUiInteractionOperabilityPosture::ReadinessDisabled,
            WorthUiInteractionOperabilityBasis::InteractionReadinessDisabled,
        );
    }
    if !gesture_admits_interaction_kind(gesture, kind) {
        return (
            WorthUiInteractionOperabilityPosture::Unsupported,
            WorthUiInteractionOperabilityBasis::GestureMismatch,
        );
    }
    if kind == WorthUiInteractionKind::Command && command_target_is_unsupported(target) {
        return (
            WorthUiInteractionOperabilityPosture::Unsupported,
            WorthUiInteractionOperabilityBasis::UnsupportedCommandTarget,
        );
    }
    if kind == WorthUiInteractionKind::Focus
        && primitive_focus == WorthUiPrimitiveFocusPosture::None
    {
        return (
            WorthUiInteractionOperabilityPosture::Unsupported,
            WorthUiInteractionOperabilityBasis::NonFocusableTarget,
        );
    }
    (
        WorthUiInteractionOperabilityPosture::Eligible,
        WorthUiInteractionOperabilityBasis::Enabled,
    )
}

fn command_target_is_unsupported(target: &WorthUiInteractionTarget) -> bool {
    match target {
        WorthUiInteractionTarget::Command(command_id) => CommandId::new(command_id).is_err(),
        _ => false,
    }
}

fn gesture_admits_interaction_kind(
    gesture: WorthUiMountedInteractionGesture,
    kind: WorthUiInteractionKind,
) -> bool {
    matches!(gesture, WorthUiMountedInteractionGesture::PrimaryClick)
        && matches!(
            kind,
            WorthUiInteractionKind::Click
                | WorthUiInteractionKind::Submit
                | WorthUiInteractionKind::Command
                | WorthUiInteractionKind::Toggle
                | WorthUiInteractionKind::Open
                | WorthUiInteractionKind::Focus
        )
}
