use forge_query::facade::runtime::ForgeQueryGraphObligationSupportStatus;

use crate::runtime::{WorthUiLiveViewEffectIntentGraphPosture, WorthUiLiveViewReadinessPosture};

use super::super::{
    operation_declaration::WorthUiQueryGraphOperationDeclaration,
    WorthUiQueryGraphObligationSemantic,
};

pub(in crate::runtime::query_graph) fn live_view_readiness_projection_operation(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> WorthUiQueryGraphOperationDeclaration {
    WorthUiQueryGraphOperationDeclaration::new(
        semantic,
        ForgeQueryGraphObligationSupportStatus::Supported,
    )
}

pub(in crate::runtime::query_graph) fn live_view_interaction_intent_operation(
    semantic: WorthUiQueryGraphObligationSemantic,
    readiness: WorthUiLiveViewReadinessPosture,
    effect_posture: WorthUiLiveViewEffectIntentGraphPosture,
) -> WorthUiQueryGraphOperationDeclaration {
    use ForgeQueryGraphObligationSupportStatus::{Supported, Unsupported};
    let status = match semantic {
        WorthUiQueryGraphObligationSemantic::LiveViewInteractionEffect
            if !effect_posture.has_supported_effect_intent() =>
        {
            Unsupported
        }
        WorthUiQueryGraphObligationSemantic::LiveViewReadinessPosture
            if !readiness.is_enabled() =>
        {
            Unsupported
        }
        _ => Supported,
    };
    WorthUiQueryGraphOperationDeclaration::new(semantic, status)
}

pub(in crate::runtime::query_graph) fn live_view_expression_projection_operation(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> WorthUiQueryGraphOperationDeclaration {
    WorthUiQueryGraphOperationDeclaration::new(
        semantic,
        ForgeQueryGraphObligationSupportStatus::Supported,
    )
}

pub(in crate::runtime::query_graph) fn live_view_payload_projection_operation(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> WorthUiQueryGraphOperationDeclaration {
    WorthUiQueryGraphOperationDeclaration::new(
        semantic,
        ForgeQueryGraphObligationSupportStatus::Supported,
    )
}

pub(in crate::runtime::query_graph) fn live_view_readiness_projection_operation_catalog(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    vec![live_view_readiness_projection_operation(semantic)]
}

pub(in crate::runtime::query_graph) fn live_view_interaction_intent_operation_catalog(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    use ForgeQueryGraphObligationSupportStatus::{Supported, Unsupported};
    match semantic {
        WorthUiQueryGraphObligationSemantic::LiveViewReadinessPosture
        | WorthUiQueryGraphObligationSemantic::LiveViewInteractionEffect => {
            vec![Supported, Unsupported]
        }
        _ => vec![Supported],
    }
    .into_iter()
    .map(|status| WorthUiQueryGraphOperationDeclaration::new(semantic, status))
    .collect()
}

pub(in crate::runtime::query_graph) fn live_view_expression_projection_operation_catalog(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    vec![live_view_expression_projection_operation(semantic)]
}

pub(in crate::runtime::query_graph) fn live_view_payload_projection_operation_catalog(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    vec![live_view_payload_projection_operation(semantic)]
}
