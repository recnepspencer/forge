pub(in crate::runtime::query_graph) mod composition_access_ops;
mod live_view_projection_ops;
mod support_catalog;

use forge_query::facade::runtime::ForgeQueryGraphObligationSupportStatus;

use crate::runtime::{
    WorthUiInteractionKind, WorthUiInteractionOperabilityBasis, WorthUiInteractionReadiness,
    WorthUiInteractionTarget, WorthUiLiveViewConditionalProjectionGraphPosture,
    WorthUiLiveViewControlProjectionGraphPosture, WorthUiPrimitiveFocusPosture,
    WorthUiUserIntentOperationFamily, WorthUiUserIntentTargetPosture,
};

use super::{
    operation_declaration::WorthUiQueryGraphOperationDeclaration,
    WorthUiLiveViewStateBindingGraphPosture, WorthUiPrimitiveContentGraphPosture,
    WorthUiPrimitiveEventGraphDispatchPosture, WorthUiQueryGraphObligationSemantic,
};

pub(in crate::runtime::query_graph) use live_view_projection_ops::{
    live_view_interaction_intent_operation, live_view_interaction_intent_operation_catalog,
    live_view_expression_projection_operation, live_view_expression_projection_operation_catalog,
    live_view_payload_projection_operation, live_view_payload_projection_operation_catalog,
    live_view_readiness_projection_operation, live_view_readiness_projection_operation_catalog,
};
pub(in crate::runtime::query_graph) use support_catalog::{
    composition_context_operation_catalog, composition_participation_operation_catalog,
    composition_topology_operation_catalog, live_view_conditional_projection_operation_catalog,
    live_view_control_projection_operation_catalog, live_view_state_binding_operation_catalog,
    mounted_interaction_operation_catalog, primitive_content_operation_catalog,
    primitive_event_operation_catalog, user_intent_target_operation_catalog,
};

pub(in crate::runtime::query_graph) fn mounted_interaction_operation(
    semantic: WorthUiQueryGraphObligationSemantic,
    basis: WorthUiInteractionOperabilityBasis,
    readiness: WorthUiInteractionReadiness,
    kind: WorthUiInteractionKind,
    target: &WorthUiInteractionTarget,
    focus: WorthUiPrimitiveFocusPosture,
) -> WorthUiQueryGraphOperationDeclaration {
    use ForgeQueryGraphObligationSupportStatus::{NotApplicable, Supported, Unsupported};
    let status = match semantic {
        WorthUiQueryGraphObligationSemantic::ActivationEligibility
            if matches!(
                basis,
                WorthUiInteractionOperabilityBasis::PrimitiveDisabled
                    | WorthUiInteractionOperabilityBasis::InteractionReadinessDisabled
            ) || readiness == WorthUiInteractionReadiness::Disabled =>
        {
            Unsupported
        }
        WorthUiQueryGraphObligationSemantic::CommandSupport
            if kind != WorthUiInteractionKind::Command =>
        {
            NotApplicable
        }
        WorthUiQueryGraphObligationSemantic::CommandSupport
            if matches!(target, WorthUiInteractionTarget::Command(_))
                && basis == WorthUiInteractionOperabilityBasis::UnsupportedCommandTarget =>
        {
            Unsupported
        }
        WorthUiQueryGraphObligationSemantic::InteractionFocusability
            if kind != WorthUiInteractionKind::Focus =>
        {
            NotApplicable
        }
        WorthUiQueryGraphObligationSemantic::InteractionFocusability
            if focus == WorthUiPrimitiveFocusPosture::None =>
        {
            Unsupported
        }
        WorthUiQueryGraphObligationSemantic::CapabilitySupport
            if matches!(
                basis,
                WorthUiInteractionOperabilityBasis::UnsupportedCommandTarget
                    | WorthUiInteractionOperabilityBasis::NonFocusableTarget
                    | WorthUiInteractionOperabilityBasis::GestureMismatch
                    | WorthUiInteractionOperabilityBasis::UnsupportedInteraction
            ) =>
        {
            Unsupported
        }
        _ => Supported,
    };
    WorthUiQueryGraphOperationDeclaration::new(semantic, status)
}

pub(in crate::runtime::query_graph) fn live_view_control_projection_operation(
    semantic: WorthUiQueryGraphObligationSemantic,
    posture: WorthUiLiveViewControlProjectionGraphPosture,
) -> WorthUiQueryGraphOperationDeclaration {
    use ForgeQueryGraphObligationSupportStatus::{Supported, Unsupported};
    let status = match semantic {
        WorthUiQueryGraphObligationSemantic::LiveViewControlProjectionKind
            if !posture.has_supported_kind() =>
        {
            Unsupported
        }
        WorthUiQueryGraphObligationSemantic::LiveViewControlOptionSource
            if !posture.has_supported_options() =>
        {
            Unsupported
        }
        WorthUiQueryGraphObligationSemantic::LiveViewControlCompatibility
            if !posture.has_compatible_replacement() =>
        {
            Unsupported
        }
        _ => Supported,
    };
    WorthUiQueryGraphOperationDeclaration::new(semantic, status)
}

pub(in crate::runtime::query_graph) fn live_view_conditional_projection_operation(
    semantic: WorthUiQueryGraphObligationSemantic,
    posture: WorthUiLiveViewConditionalProjectionGraphPosture,
) -> WorthUiQueryGraphOperationDeclaration {
    use ForgeQueryGraphObligationSupportStatus::{Supported, Unsupported};
    let status = match semantic {
        WorthUiQueryGraphObligationSemantic::LiveViewConditionalExpression
            if !posture.has_supported_condition() =>
        {
            Unsupported
        }
        WorthUiQueryGraphObligationSemantic::LiveViewConditionalParticipation
            if !posture.has_supported_participation() =>
        {
            Unsupported
        }
        _ => Supported,
    };
    WorthUiQueryGraphOperationDeclaration::new(semantic, status)
}

pub(in crate::runtime::query_graph) fn primitive_event_operation(
    semantic: WorthUiQueryGraphObligationSemantic,
    posture: WorthUiPrimitiveEventGraphDispatchPosture,
) -> WorthUiQueryGraphOperationDeclaration {
    use ForgeQueryGraphObligationSupportStatus::{DiagnosticOnly, NotApplicable, Supported};
    let status = match semantic {
        WorthUiQueryGraphObligationSemantic::EventDisabledBlock
            if posture == WorthUiPrimitiveEventGraphDispatchPosture::DisabledHit =>
        {
            DiagnosticOnly
        }
        WorthUiQueryGraphObligationSemantic::EventDisabledBlock => NotApplicable,
        WorthUiQueryGraphObligationSemantic::EventCapturePolicy
            if posture == WorthUiPrimitiveEventGraphDispatchPosture::Captured =>
        {
            Supported
        }
        WorthUiQueryGraphObligationSemantic::EventCapturePolicy => NotApplicable,
        WorthUiQueryGraphObligationSemantic::EventPropagation
            if posture == WorthUiPrimitiveEventGraphDispatchPosture::Bubbled =>
        {
            Supported
        }
        WorthUiQueryGraphObligationSemantic::EventPropagation => NotApplicable,
        WorthUiQueryGraphObligationSemantic::EventCursorPosture
            if matches!(
                posture,
                WorthUiPrimitiveEventGraphDispatchPosture::NoHit
                    | WorthUiPrimitiveEventGraphDispatchPosture::Denied
            ) =>
        {
            DiagnosticOnly
        }
        _ => Supported,
    };
    WorthUiQueryGraphOperationDeclaration::new(semantic, status)
}

pub(in crate::runtime::query_graph) fn primitive_content_operation(
    semantic: WorthUiQueryGraphObligationSemantic,
    posture: WorthUiPrimitiveContentGraphPosture,
) -> WorthUiQueryGraphOperationDeclaration {
    use ForgeQueryGraphObligationSupportStatus::{
        DiagnosticOnly, NotApplicable, Supported, Unsupported,
    };
    let status = match semantic {
        WorthUiQueryGraphObligationSemantic::ContentIconCapability
            if posture == WorthUiPrimitiveContentGraphPosture::UnsupportedCapability =>
        {
            Unsupported
        }
        WorthUiQueryGraphObligationSemantic::ContentVectorPosture
            if posture == WorthUiPrimitiveContentGraphPosture::NativeVector =>
        {
            Supported
        }
        WorthUiQueryGraphObligationSemantic::ContentVectorPosture
            if posture == WorthUiPrimitiveContentGraphPosture::FallbackEligible =>
        {
            DiagnosticOnly
        }
        WorthUiQueryGraphObligationSemantic::ContentVectorPosture => NotApplicable,
        WorthUiQueryGraphObligationSemantic::ContentSlotParticipation
            if posture == WorthUiPrimitiveContentGraphPosture::Accepted =>
        {
            NotApplicable
        }
        WorthUiQueryGraphObligationSemantic::ContentSchemaAdmission
            if posture == WorthUiPrimitiveContentGraphPosture::Denied =>
        {
            Unsupported
        }
        _ => Supported,
    };
    WorthUiQueryGraphOperationDeclaration::new(semantic, status)
}

pub(in crate::runtime::query_graph) fn user_intent_target_operation(
    semantic: WorthUiQueryGraphObligationSemantic,
    operation_family: WorthUiUserIntentOperationFamily,
    posture: WorthUiUserIntentTargetPosture,
) -> WorthUiQueryGraphOperationDeclaration {
    use ForgeQueryGraphObligationSupportStatus::{Supported, Unsupported};
    let status = match semantic {
        WorthUiQueryGraphObligationSemantic::TargetBindingPosture
            if posture != WorthUiUserIntentTargetPosture::Bound =>
        {
            Unsupported
        }
        WorthUiQueryGraphObligationSemantic::TargetOperationFamily
            if operation_family == WorthUiUserIntentOperationFamily::EventDispatch =>
        {
            Supported
        }
        _ => Supported,
    };
    WorthUiQueryGraphOperationDeclaration::new(semantic, status)
}

pub(in crate::runtime::query_graph) fn composition_topology_operation(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> WorthUiQueryGraphOperationDeclaration {
    WorthUiQueryGraphOperationDeclaration::new(
        semantic,
        ForgeQueryGraphObligationSupportStatus::Supported,
    )
}

pub(in crate::runtime::query_graph) fn composition_context_operation(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> WorthUiQueryGraphOperationDeclaration {
    WorthUiQueryGraphOperationDeclaration::new(
        semantic,
        ForgeQueryGraphObligationSupportStatus::Supported,
    )
}

pub(in crate::runtime::query_graph) fn composition_participation_operation(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> WorthUiQueryGraphOperationDeclaration {
    WorthUiQueryGraphOperationDeclaration::new(
        semantic,
        ForgeQueryGraphObligationSupportStatus::Supported,
    )
}

pub(in crate::runtime::query_graph) fn live_view_state_binding_operation(
    semantic: WorthUiQueryGraphObligationSemantic,
    posture: WorthUiLiveViewStateBindingGraphPosture,
) -> WorthUiQueryGraphOperationDeclaration {
    use ForgeQueryGraphObligationSupportStatus::{Supported, Unsupported};
    let status = match semantic {
        WorthUiQueryGraphObligationSemantic::LiveViewTargetBinding
            if !posture.has_bound_target() =>
        {
            Unsupported
        }
        WorthUiQueryGraphObligationSemantic::LiveViewStateCompatibility
            if !posture.has_compatible_state() =>
        {
            Unsupported
        }
        WorthUiQueryGraphObligationSemantic::LiveViewWritePosture
            if !posture.has_write_posture() =>
        {
            Unsupported
        }
        WorthUiQueryGraphObligationSemantic::LiveViewEffectIntentAdmission
            if !posture.has_effect_intent() =>
        {
            Unsupported
        }
        _ => Supported,
    };
    WorthUiQueryGraphOperationDeclaration::new(semantic, status)
}
