use crate::runtime::{
    WorthUiLiveViewDeclarationReceipt, WorthUiLiveViewPayloadProjectionReceipt,
    WorthUiLiveViewReadinessProjectionReceipt, WorthUiRuntimeFactId, WorthUiRuntimeHost,
};

use super::{
    effect_admission::live_view_effect_intent_graph_posture,
    primitive_binding::append_interaction_primitive_denials,
    primitive_binding::lower_interaction_primitive_binding,
    WorthUiLiveViewInteractionIntentDeclaration, WorthUiLiveViewInteractionIntentDenial,
    WorthUiLiveViewInteractionIntentReceipt,
};

pub(crate) fn interaction_intent_denials(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    readinesses: &[WorthUiLiveViewReadinessProjectionReceipt],
    payloads: &[WorthUiLiveViewPayloadProjectionReceipt],
    declarations: &[WorthUiLiveViewInteractionIntentDeclaration],
) -> Vec<WorthUiLiveViewInteractionIntentDenial> {
    let mut denials = Vec::new();
    for declaration in declarations {
        if invalid_identity(declaration.interaction_id()) {
            denials.push(
                WorthUiLiveViewInteractionIntentDenial::InvalidInteractionId {
                    interaction_id: declaration.interaction_id().to_owned(),
                },
            );
        }
        if !declaration.kind().is_supported() {
            denials.push(WorthUiLiveViewInteractionIntentDenial::UnsupportedKind {
                interaction_id: declaration.interaction_id().to_owned(),
                kind: declaration.kind().token().to_owned(),
            });
        }
        if !live_view_effect_intent_graph_posture(declaration.effect())
            .has_supported_effect_intent()
        {
            denials.push(WorthUiLiveViewInteractionIntentDenial::UnsupportedEffect {
                interaction_id: declaration.interaction_id().to_owned(),
                effect: declaration.effect().to_owned(),
            });
        }
        if !readinesses
            .iter()
            .any(|readiness| readiness.readiness_id() == declaration.readiness_id())
        {
            denials.push(WorthUiLiveViewInteractionIntentDenial::UnknownReadiness {
                interaction_id: declaration.interaction_id().to_owned(),
                readiness_id: declaration.readiness_id().to_owned(),
            });
        }
        if !payloads
            .iter()
            .any(|payload| payload.payload_id() == declaration.payload_id())
        {
            denials.push(WorthUiLiveViewInteractionIntentDenial::UnknownPayload {
                interaction_id: declaration.interaction_id().to_owned(),
                payload_id: declaration.payload_id().to_owned(),
            });
        }
        append_interaction_primitive_denials(
            runtime,
            live_view.live_view_id(),
            declaration,
            &mut denials,
        );
    }
    denials
}

pub(crate) fn lower_live_view_interaction_intents(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    readinesses: &[WorthUiLiveViewReadinessProjectionReceipt],
    payloads: &[WorthUiLiveViewPayloadProjectionReceipt],
    declarations: &[WorthUiLiveViewInteractionIntentDeclaration],
) -> Vec<WorthUiLiveViewInteractionIntentReceipt> {
    declarations
        .iter()
        .map(|declaration| {
            let readiness = readinesses
                .iter()
                .find(|readiness| readiness.readiness_id() == declaration.readiness_id())
                .expect("interaction readiness was admitted before lowering")
                .clone();
            let payload = payloads
                .iter()
                .find(|payload| payload.payload_id() == declaration.payload_id())
                .expect("interaction payload was admitted before lowering")
                .clone();
            let facts = interaction_dependency_facts(live_view, declaration, &readiness, &payload);
            let primitive_binding =
                lower_interaction_primitive_binding(runtime, live_view.live_view_id(), declaration);
            let graph_execution = runtime
                .graph_authority()
                .plan_live_view_interaction_intent_graph_operation(
                    live_view.live_view_id(),
                    declaration.interaction_id(),
                    facts,
                    readiness.posture(),
                    live_view_effect_intent_graph_posture(declaration.effect()),
                )
                .into_execution_receipt();
            WorthUiLiveViewInteractionIntentReceipt::new(
                live_view.live_view_id(),
                declaration,
                readiness,
                payload,
                primitive_binding.flow_layout,
                primitive_binding.appearance,
                primitive_binding.event_geometry,
                graph_execution,
            )
        })
        .collect()
}

fn interaction_dependency_facts(
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewInteractionIntentDeclaration,
    readiness: &WorthUiLiveViewReadinessProjectionReceipt,
    payload: &WorthUiLiveViewPayloadProjectionReceipt,
) -> Vec<WorthUiRuntimeFactId> {
    vec![
        WorthUiRuntimeFactId::live_view_declaration(live_view.live_view_id()),
        WorthUiRuntimeFactId::live_view_interaction_intent(format!(
            "{}:{}",
            live_view.live_view_id(),
            declaration.interaction_id()
        )),
        WorthUiRuntimeFactId::live_view_readiness_projection(format!(
            "{}:{}",
            live_view.live_view_id(),
            readiness.readiness_id()
        )),
        WorthUiRuntimeFactId::live_view_payload_projection(format!(
            "{}:{}",
            live_view.live_view_id(),
            payload.payload_id()
        )),
        WorthUiRuntimeFactId::query_effect_posture(declaration.effect()),
        WorthUiRuntimeFactId::primitive_flow_layout(interaction_primitive_fact_identity(
            live_view,
            declaration,
        )),
        WorthUiRuntimeFactId::primitive_appearance_state(interaction_primitive_fact_identity(
            live_view,
            declaration,
        )),
        WorthUiRuntimeFactId::primitive_event_geometry(interaction_primitive_fact_identity(
            live_view,
            declaration,
        )),
    ]
}

fn interaction_primitive_fact_identity(
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewInteractionIntentDeclaration,
) -> String {
    format!(
        "{}:{}",
        live_view.live_view_id(),
        declaration.interaction_id()
    )
}

fn invalid_identity(value: &str) -> bool {
    value.trim().is_empty() || value.chars().any(char::is_whitespace)
}
