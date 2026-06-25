use forge_query::facade::runtime::{
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportPosture,
    ForgeQueryGraphObligationSupportStatus,
};

use crate::runtime::{
    WorthUiInteractionKind, WorthUiInteractionOperabilityBasis, WorthUiInteractionReadiness,
    WorthUiInteractionTarget, WorthUiLiveViewConditionalProjectionGraphPosture,
    WorthUiLiveViewControlProjectionGraphPosture, WorthUiLiveViewEffectIntentGraphPosture,
    WorthUiLiveViewReadinessPosture, WorthUiPrimitiveFocusPosture,
    WorthUiUserIntentOperationFamily, WorthUiUserIntentTargetPosture,
};

use super::{
    operation_catalog::{
        composition_access_ops::{
            composition_graph_access_operation, composition_graph_access_operation_catalog,
        },
        composition_context_operation, composition_context_operation_catalog,
        composition_participation_operation, composition_participation_operation_catalog,
        composition_topology_operation, composition_topology_operation_catalog,
        live_view_conditional_projection_operation,
        live_view_conditional_projection_operation_catalog, live_view_control_projection_operation,
        live_view_control_projection_operation_catalog, live_view_interaction_intent_operation,
        live_view_expression_projection_operation,
        live_view_expression_projection_operation_catalog,
        live_view_interaction_intent_operation_catalog, live_view_payload_projection_operation,
        live_view_payload_projection_operation_catalog, live_view_readiness_projection_operation,
        live_view_readiness_projection_operation_catalog, live_view_state_binding_operation,
        live_view_state_binding_operation_catalog, mounted_interaction_operation,
        mounted_interaction_operation_catalog, primitive_content_operation,
        primitive_content_operation_catalog, primitive_event_operation,
        primitive_event_operation_catalog, user_intent_target_operation,
        user_intent_target_operation_catalog,
    },
    WorthUiLiveViewStateBindingGraphPosture, WorthUiPrimitiveContentGraphPosture,
    WorthUiPrimitiveEventGraphDispatchPosture, WorthUiQueryGraphObligationSemantic,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime::query_graph) struct WorthUiQueryGraphOperationDeclaration {
    semantic: WorthUiQueryGraphObligationSemantic,
    status: ForgeQueryGraphObligationSupportStatus,
}

impl WorthUiQueryGraphOperationDeclaration {
    pub(in crate::runtime::query_graph) fn new(
        semantic: WorthUiQueryGraphObligationSemantic,
        status: ForgeQueryGraphObligationSupportStatus,
    ) -> Self {
        Self { semantic, status }
    }

    pub fn semantic(self) -> WorthUiQueryGraphObligationSemantic {
        self.semantic
    }

    pub fn operation_id(self) -> String {
        format!("{}.{}", self.semantic.as_str(), self.status.as_str())
    }

    pub fn support_posture(self) -> ForgeQueryGraphObligationSupportPosture {
        let lane = ForgeQueryGraphObligationSupportLane::PreviewIntent;
        match self.status {
            ForgeQueryGraphObligationSupportStatus::Supported => {
                ForgeQueryGraphObligationSupportPosture::supported(lane)
            }
            ForgeQueryGraphObligationSupportStatus::Unsupported => {
                ForgeQueryGraphObligationSupportPosture::unsupported(lane)
            }
            ForgeQueryGraphObligationSupportStatus::NotApplicable => {
                ForgeQueryGraphObligationSupportPosture::not_applicable(lane)
            }
            ForgeQueryGraphObligationSupportStatus::DiagnosticOnly => {
                ForgeQueryGraphObligationSupportPosture::diagnostic_only(lane)
            }
            ForgeQueryGraphObligationSupportStatus::DeferredToBackstop => {
                ForgeQueryGraphObligationSupportPosture::deferred_to_backstop(lane)
            }
        }
    }
}

pub(in crate::runtime::query_graph) fn primitive_construction_touch_operations() -> Vec<String> {
    WorthUiQueryGraphObligationSemantic::PRIMITIVE_CONSTRUCTION
        .into_iter()
        .map(supported)
        .map(WorthUiQueryGraphOperationDeclaration::operation_id)
        .collect()
}

pub(in crate::runtime::query_graph) fn primitive_construction_registration_operations(
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    WorthUiQueryGraphObligationSemantic::PRIMITIVE_CONSTRUCTION
        .into_iter()
        .map(supported)
        .collect()
}

pub(in crate::runtime::query_graph) fn composition_topology_touch_operations() -> Vec<String> {
    WorthUiQueryGraphObligationSemantic::COMPOSITION_TOPOLOGY
        .into_iter()
        .map(composition_topology_operation)
        .map(WorthUiQueryGraphOperationDeclaration::operation_id)
        .collect()
}

pub(in crate::runtime::query_graph) fn composition_topology_registration_operations(
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    let mut operations = Vec::new();
    for semantic in WorthUiQueryGraphObligationSemantic::COMPOSITION_TOPOLOGY {
        operations.extend(composition_topology_operation_catalog(semantic));
    }
    operations
}

pub(in crate::runtime::query_graph) fn composition_graph_access_touch_operations() -> Vec<String> {
    WorthUiQueryGraphObligationSemantic::COMPOSITION_GRAPH_ACCESS
        .into_iter()
        .map(composition_graph_access_operation)
        .map(WorthUiQueryGraphOperationDeclaration::operation_id)
        .collect()
}

pub(in crate::runtime::query_graph) fn composition_graph_access_registration_operations(
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    let mut operations = Vec::new();
    for semantic in WorthUiQueryGraphObligationSemantic::COMPOSITION_GRAPH_ACCESS {
        operations.extend(composition_graph_access_operation_catalog(semantic));
    }
    operations
}

pub(in crate::runtime::query_graph) fn composition_context_touch_operations() -> Vec<String> {
    WorthUiQueryGraphObligationSemantic::COMPOSITION_CONTEXT
        .into_iter()
        .map(composition_context_operation)
        .map(WorthUiQueryGraphOperationDeclaration::operation_id)
        .collect()
}

pub(in crate::runtime::query_graph) fn composition_context_registration_operations(
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    let mut operations = Vec::new();
    for semantic in WorthUiQueryGraphObligationSemantic::COMPOSITION_CONTEXT {
        operations.extend(composition_context_operation_catalog(semantic));
    }
    operations
}

pub(in crate::runtime::query_graph) fn composition_participation_touch_operations() -> Vec<String> {
    WorthUiQueryGraphObligationSemantic::COMPOSITION_PARTICIPATION
        .into_iter()
        .map(composition_participation_operation)
        .map(WorthUiQueryGraphOperationDeclaration::operation_id)
        .collect()
}

pub(in crate::runtime::query_graph) fn composition_participation_registration_operations(
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    let mut operations = Vec::new();
    for semantic in WorthUiQueryGraphObligationSemantic::COMPOSITION_PARTICIPATION {
        operations.extend(composition_participation_operation_catalog(semantic));
    }
    operations
}

pub(in crate::runtime::query_graph) fn mounted_interaction_touch_operations(
    basis: WorthUiInteractionOperabilityBasis,
    readiness: WorthUiInteractionReadiness,
    kind: WorthUiInteractionKind,
    target: &WorthUiInteractionTarget,
    focus: WorthUiPrimitiveFocusPosture,
) -> Vec<String> {
    WorthUiQueryGraphObligationSemantic::MOUNTED_INTERACTION_ACTIVATION
        .into_iter()
        .map(|semantic| {
            mounted_interaction_operation(semantic, basis, readiness, kind, target, focus)
        })
        .map(WorthUiQueryGraphOperationDeclaration::operation_id)
        .collect()
}

pub(in crate::runtime::query_graph) fn mounted_interaction_registration_operations(
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    let mut operations = Vec::new();
    for semantic in WorthUiQueryGraphObligationSemantic::MOUNTED_INTERACTION_ACTIVATION {
        operations.extend(mounted_interaction_operation_catalog(semantic));
    }
    operations
}

pub(in crate::runtime::query_graph) fn primitive_event_touch_operations(
    posture: WorthUiPrimitiveEventGraphDispatchPosture,
) -> Vec<String> {
    WorthUiQueryGraphObligationSemantic::PRIMITIVE_EVENT_DISPATCH
        .into_iter()
        .map(|semantic| primitive_event_operation(semantic, posture))
        .map(WorthUiQueryGraphOperationDeclaration::operation_id)
        .collect()
}

pub(in crate::runtime::query_graph) fn primitive_event_registration_operations(
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    let mut operations = Vec::new();
    for semantic in WorthUiQueryGraphObligationSemantic::PRIMITIVE_EVENT_DISPATCH {
        operations.extend(primitive_event_operation_catalog(semantic));
    }
    operations
}

pub(in crate::runtime::query_graph) fn primitive_content_touch_operations(
    posture: WorthUiPrimitiveContentGraphPosture,
) -> Vec<String> {
    WorthUiQueryGraphObligationSemantic::PRIMITIVE_CONTENT_ANATOMY
        .into_iter()
        .map(|semantic| primitive_content_operation(semantic, posture))
        .map(WorthUiQueryGraphOperationDeclaration::operation_id)
        .collect()
}

pub(in crate::runtime::query_graph) fn primitive_content_registration_operations(
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    let mut operations = Vec::new();
    for semantic in WorthUiQueryGraphObligationSemantic::PRIMITIVE_CONTENT_ANATOMY {
        operations.extend(primitive_content_operation_catalog(semantic));
    }
    operations
}

pub(in crate::runtime::query_graph) fn user_intent_target_touch_operations(
    operation_family: WorthUiUserIntentOperationFamily,
    posture: WorthUiUserIntentTargetPosture,
) -> Vec<String> {
    WorthUiQueryGraphObligationSemantic::USER_INTENT_TARGET_BINDING
        .into_iter()
        .map(|semantic| user_intent_target_operation(semantic, operation_family, posture))
        .map(WorthUiQueryGraphOperationDeclaration::operation_id)
        .collect()
}

pub(in crate::runtime::query_graph) fn user_intent_target_registration_operations(
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    let mut operations = Vec::new();
    for semantic in WorthUiQueryGraphObligationSemantic::USER_INTENT_TARGET_BINDING {
        operations.extend(user_intent_target_operation_catalog(semantic));
    }
    operations
}

pub(in crate::runtime::query_graph) fn live_view_state_binding_touch_operations(
    posture: WorthUiLiveViewStateBindingGraphPosture,
) -> Vec<String> {
    WorthUiQueryGraphObligationSemantic::LIVE_VIEW_STATE_BINDING
        .into_iter()
        .map(|semantic| live_view_state_binding_operation(semantic, posture))
        .map(WorthUiQueryGraphOperationDeclaration::operation_id)
        .collect()
}

pub(in crate::runtime::query_graph) fn live_view_state_binding_registration_operations(
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    let mut operations = Vec::new();
    for semantic in WorthUiQueryGraphObligationSemantic::LIVE_VIEW_STATE_BINDING {
        operations.extend(live_view_state_binding_operation_catalog(semantic));
    }
    operations
}

pub(in crate::runtime::query_graph) fn live_view_control_projection_touch_operations(
    posture: WorthUiLiveViewControlProjectionGraphPosture,
) -> Vec<String> {
    WorthUiQueryGraphObligationSemantic::LIVE_VIEW_CONTROL_PROJECTION
        .into_iter()
        .map(|semantic| live_view_control_projection_operation(semantic, posture))
        .map(WorthUiQueryGraphOperationDeclaration::operation_id)
        .collect()
}

pub(in crate::runtime::query_graph) fn live_view_control_projection_registration_operations(
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    let mut operations = Vec::new();
    for semantic in WorthUiQueryGraphObligationSemantic::LIVE_VIEW_CONTROL_PROJECTION {
        operations.extend(live_view_control_projection_operation_catalog(semantic));
    }
    operations
}

pub(in crate::runtime::query_graph) fn live_view_conditional_projection_touch_operations(
    posture: WorthUiLiveViewConditionalProjectionGraphPosture,
) -> Vec<String> {
    WorthUiQueryGraphObligationSemantic::LIVE_VIEW_CONDITIONAL_PROJECTION
        .into_iter()
        .map(|semantic| live_view_conditional_projection_operation(semantic, posture))
        .map(WorthUiQueryGraphOperationDeclaration::operation_id)
        .collect()
}

pub(in crate::runtime::query_graph) fn live_view_conditional_projection_registration_operations(
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    let mut operations = Vec::new();
    for semantic in WorthUiQueryGraphObligationSemantic::LIVE_VIEW_CONDITIONAL_PROJECTION {
        operations.extend(live_view_conditional_projection_operation_catalog(semantic));
    }
    operations
}

pub(in crate::runtime::query_graph) fn live_view_readiness_projection_touch_operations(
) -> Vec<String> {
    WorthUiQueryGraphObligationSemantic::LIVE_VIEW_READINESS_PROJECTION
        .into_iter()
        .map(live_view_readiness_projection_operation)
        .map(WorthUiQueryGraphOperationDeclaration::operation_id)
        .collect()
}

pub(in crate::runtime::query_graph) fn live_view_readiness_projection_registration_operations(
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    let mut operations = Vec::new();
    for semantic in WorthUiQueryGraphObligationSemantic::LIVE_VIEW_READINESS_PROJECTION {
        operations.extend(live_view_readiness_projection_operation_catalog(semantic));
    }
    operations
}

pub(in crate::runtime::query_graph) fn live_view_expression_projection_touch_operations(
) -> Vec<String> {
    WorthUiQueryGraphObligationSemantic::LIVE_VIEW_EXPRESSION_PROJECTION
        .into_iter()
        .map(live_view_expression_projection_operation)
        .map(WorthUiQueryGraphOperationDeclaration::operation_id)
        .collect()
}

pub(in crate::runtime::query_graph) fn live_view_expression_projection_registration_operations(
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    let mut operations = Vec::new();
    for semantic in WorthUiQueryGraphObligationSemantic::LIVE_VIEW_EXPRESSION_PROJECTION {
        operations.extend(live_view_expression_projection_operation_catalog(semantic));
    }
    operations
}

pub(in crate::runtime::query_graph) fn live_view_interaction_intent_touch_operations(
    readiness: WorthUiLiveViewReadinessPosture,
    effect_posture: WorthUiLiveViewEffectIntentGraphPosture,
) -> Vec<String> {
    WorthUiQueryGraphObligationSemantic::LIVE_VIEW_INTERACTION_INTENT
        .into_iter()
        .map(|semantic| live_view_interaction_intent_operation(semantic, readiness, effect_posture))
        .map(WorthUiQueryGraphOperationDeclaration::operation_id)
        .collect()
}

pub(in crate::runtime::query_graph) fn live_view_interaction_intent_registration_operations(
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    let mut operations = Vec::new();
    for semantic in WorthUiQueryGraphObligationSemantic::LIVE_VIEW_INTERACTION_INTENT {
        operations.extend(live_view_interaction_intent_operation_catalog(semantic));
    }
    operations
}

pub(in crate::runtime::query_graph) fn live_view_payload_projection_touch_operations() -> Vec<String>
{
    WorthUiQueryGraphObligationSemantic::LIVE_VIEW_PAYLOAD_PROJECTION
        .into_iter()
        .map(live_view_payload_projection_operation)
        .map(WorthUiQueryGraphOperationDeclaration::operation_id)
        .collect()
}

pub(in crate::runtime::query_graph) fn live_view_payload_projection_registration_operations(
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    let mut operations = Vec::new();
    for semantic in WorthUiQueryGraphObligationSemantic::LIVE_VIEW_PAYLOAD_PROJECTION {
        operations.extend(live_view_payload_projection_operation_catalog(semantic));
    }
    operations
}

fn supported(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> WorthUiQueryGraphOperationDeclaration {
    WorthUiQueryGraphOperationDeclaration::new(
        semantic,
        ForgeQueryGraphObligationSupportStatus::Supported,
    )
}
