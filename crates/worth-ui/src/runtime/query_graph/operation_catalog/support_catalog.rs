use forge_query::facade::runtime::ForgeQueryGraphObligationSupportStatus;

use super::super::{
    operation_declaration::WorthUiQueryGraphOperationDeclaration,
    WorthUiQueryGraphObligationSemantic,
};

pub(in crate::runtime::query_graph) fn composition_context_operation_catalog(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    supported_only(semantic)
}

pub(in crate::runtime::query_graph) fn composition_participation_operation_catalog(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    supported_only(semantic)
}

pub(in crate::runtime::query_graph) fn composition_topology_operation_catalog(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    supported_only(semantic)
}

pub(in crate::runtime::query_graph) fn mounted_interaction_operation_catalog(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    use ForgeQueryGraphObligationSupportStatus::{NotApplicable, Supported, Unsupported};
    match semantic {
        WorthUiQueryGraphObligationSemantic::ActivationEligibility
        | WorthUiQueryGraphObligationSemantic::CapabilitySupport
        | WorthUiQueryGraphObligationSemantic::CommandSupport
        | WorthUiQueryGraphObligationSemantic::InteractionFocusability => {
            vec![Supported, Unsupported, NotApplicable]
        }
        _ => vec![Supported],
    }
    .into_iter()
    .map(|status| WorthUiQueryGraphOperationDeclaration::new(semantic, status))
    .collect()
}

pub(in crate::runtime::query_graph) fn primitive_event_operation_catalog(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    use ForgeQueryGraphObligationSupportStatus::{DiagnosticOnly, NotApplicable, Supported};
    match semantic {
        WorthUiQueryGraphObligationSemantic::EventDisabledBlock
        | WorthUiQueryGraphObligationSemantic::EventCapturePolicy
        | WorthUiQueryGraphObligationSemantic::EventCursorPosture
        | WorthUiQueryGraphObligationSemantic::EventPropagation => {
            vec![Supported, DiagnosticOnly, NotApplicable]
        }
        _ => vec![Supported],
    }
    .into_iter()
    .map(|status| WorthUiQueryGraphOperationDeclaration::new(semantic, status))
    .collect()
}

pub(in crate::runtime::query_graph) fn primitive_content_operation_catalog(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    use ForgeQueryGraphObligationSupportStatus::{
        DiagnosticOnly, NotApplicable, Supported, Unsupported,
    };
    match semantic {
        WorthUiQueryGraphObligationSemantic::ContentSchemaAdmission
        | WorthUiQueryGraphObligationSemantic::ContentIconCapability => {
            vec![Supported, Unsupported]
        }
        WorthUiQueryGraphObligationSemantic::ContentVectorPosture => {
            vec![Supported, DiagnosticOnly, NotApplicable]
        }
        WorthUiQueryGraphObligationSemantic::ContentSlotParticipation => {
            vec![Supported, NotApplicable]
        }
        _ => vec![Supported],
    }
    .into_iter()
    .map(|status| WorthUiQueryGraphOperationDeclaration::new(semantic, status))
    .collect()
}

pub(in crate::runtime::query_graph) fn user_intent_target_operation_catalog(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    use ForgeQueryGraphObligationSupportStatus::{Supported, Unsupported};
    match semantic {
        WorthUiQueryGraphObligationSemantic::TargetBindingPosture => vec![Supported, Unsupported],
        _ => vec![Supported],
    }
    .into_iter()
    .map(|status| WorthUiQueryGraphOperationDeclaration::new(semantic, status))
    .collect()
}

pub(in crate::runtime::query_graph) fn live_view_state_binding_operation_catalog(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    use ForgeQueryGraphObligationSupportStatus::{Supported, Unsupported};
    match semantic {
        WorthUiQueryGraphObligationSemantic::LiveViewTargetBinding
        | WorthUiQueryGraphObligationSemantic::LiveViewStateCompatibility
        | WorthUiQueryGraphObligationSemantic::LiveViewWritePosture
        | WorthUiQueryGraphObligationSemantic::LiveViewEffectIntentAdmission => {
            vec![Supported, Unsupported]
        }
        _ => vec![Supported],
    }
    .into_iter()
    .map(|status| WorthUiQueryGraphOperationDeclaration::new(semantic, status))
    .collect()
}

pub(in crate::runtime::query_graph) fn live_view_control_projection_operation_catalog(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    use ForgeQueryGraphObligationSupportStatus::{Supported, Unsupported};
    match semantic {
        WorthUiQueryGraphObligationSemantic::LiveViewControlProjectionKind
        | WorthUiQueryGraphObligationSemantic::LiveViewControlOptionSource
        | WorthUiQueryGraphObligationSemantic::LiveViewControlCompatibility => {
            vec![Supported, Unsupported]
        }
        _ => vec![Supported],
    }
    .into_iter()
    .map(|status| WorthUiQueryGraphOperationDeclaration::new(semantic, status))
    .collect()
}

pub(in crate::runtime::query_graph) fn live_view_conditional_projection_operation_catalog(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    use ForgeQueryGraphObligationSupportStatus::{Supported, Unsupported};
    match semantic {
        WorthUiQueryGraphObligationSemantic::LiveViewConditionalExpression
        | WorthUiQueryGraphObligationSemantic::LiveViewConditionalParticipation
        | WorthUiQueryGraphObligationSemantic::LiveViewRetainedStatePosture => {
            vec![Supported, Unsupported]
        }
        _ => vec![Supported],
    }
    .into_iter()
    .map(|status| WorthUiQueryGraphOperationDeclaration::new(semantic, status))
    .collect()
}

fn supported_only(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    vec![WorthUiQueryGraphOperationDeclaration::new(
        semantic,
        ForgeQueryGraphObligationSupportStatus::Supported,
    )]
}
