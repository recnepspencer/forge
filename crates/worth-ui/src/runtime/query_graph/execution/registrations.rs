use forge_query::facade::runtime::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationOperatingWorldSelector,
    ForgeQueryGraphObligationRegistration, ForgeQueryGraphObligationRuleIdentity,
};

use super::super::{
    operation_declaration::{
        composition_context_registration_operations,
        composition_graph_access_registration_operations,
        composition_participation_registration_operations,
        composition_topology_registration_operations,
        live_view_conditional_projection_registration_operations,
        live_view_control_projection_registration_operations,
        live_view_expression_projection_registration_operations,
        live_view_interaction_intent_registration_operations,
        live_view_payload_projection_registration_operations,
        live_view_readiness_projection_registration_operations,
        live_view_state_binding_registration_operations,
        mounted_interaction_registration_operations,
        primitive_construction_registration_operations, primitive_content_registration_operations,
        primitive_event_registration_operations, user_intent_target_registration_operations,
        WorthUiQueryGraphOperationDeclaration,
    },
    WorthUiQueryGraphObligationSemantic, WorthUiQueryGraphTouchDescriptor,
};

pub(in crate::runtime::query_graph) fn primitive_construction_registrations() -> Vec<(
    WorthUiQueryGraphObligationSemantic,
    ForgeQueryGraphObligationRegistration,
)> {
    primitive_construction_registration_operations()
        .into_iter()
        .map(|operation| {
            registration_for_operation(operation, primitive_construction_rule_identity)
        })
        .collect()
}

pub(in crate::runtime::query_graph) fn composition_topology_registrations() -> Vec<(
    WorthUiQueryGraphObligationSemantic,
    ForgeQueryGraphObligationRegistration,
)> {
    composition_topology_registration_operations()
        .into_iter()
        .map(|operation| registration_for_operation(operation, composition_topology_rule_identity))
        .collect()
}

pub(in crate::runtime::query_graph) fn composition_graph_access_registrations() -> Vec<(
    WorthUiQueryGraphObligationSemantic,
    ForgeQueryGraphObligationRegistration,
)> {
    composition_graph_access_registration_operations()
        .into_iter()
        .map(|operation| {
            registration_for_operation(operation, composition_graph_access_rule_identity)
        })
        .collect()
}

pub(in crate::runtime::query_graph) fn composition_context_registrations() -> Vec<(
    WorthUiQueryGraphObligationSemantic,
    ForgeQueryGraphObligationRegistration,
)> {
    composition_context_registration_operations()
        .into_iter()
        .map(|operation| registration_for_operation(operation, composition_context_rule_identity))
        .collect()
}

pub(in crate::runtime::query_graph) fn composition_participation_registrations() -> Vec<(
    WorthUiQueryGraphObligationSemantic,
    ForgeQueryGraphObligationRegistration,
)> {
    composition_participation_registration_operations()
        .into_iter()
        .map(|operation| {
            registration_for_operation(operation, composition_participation_rule_identity)
        })
        .collect()
}

pub(in crate::runtime::query_graph) fn mounted_interaction_registrations() -> Vec<(
    WorthUiQueryGraphObligationSemantic,
    ForgeQueryGraphObligationRegistration,
)> {
    mounted_interaction_registration_operations()
        .into_iter()
        .map(|operation| registration_for_operation(operation, mounted_interaction_rule_identity))
        .collect()
}

pub(in crate::runtime::query_graph) fn primitive_event_dispatch_registrations() -> Vec<(
    WorthUiQueryGraphObligationSemantic,
    ForgeQueryGraphObligationRegistration,
)> {
    primitive_event_registration_operations()
        .into_iter()
        .map(|operation| registration_for_operation(operation, primitive_event_rule_identity))
        .collect()
}

pub(in crate::runtime::query_graph) fn primitive_content_anatomy_registrations() -> Vec<(
    WorthUiQueryGraphObligationSemantic,
    ForgeQueryGraphObligationRegistration,
)> {
    primitive_content_registration_operations()
        .into_iter()
        .map(|operation| registration_for_operation(operation, primitive_content_rule_identity))
        .collect()
}

pub(in crate::runtime::query_graph) fn user_intent_target_binding_registrations() -> Vec<(
    WorthUiQueryGraphObligationSemantic,
    ForgeQueryGraphObligationRegistration,
)> {
    user_intent_target_registration_operations()
        .into_iter()
        .map(|operation| registration_for_operation(operation, user_intent_target_rule_identity))
        .collect()
}

pub(in crate::runtime::query_graph) fn live_view_state_binding_registrations() -> Vec<(
    WorthUiQueryGraphObligationSemantic,
    ForgeQueryGraphObligationRegistration,
)> {
    live_view_state_binding_registration_operations()
        .into_iter()
        .map(|operation| registration_for_operation(operation, live_view_state_rule_identity))
        .collect()
}

pub(in crate::runtime::query_graph) fn live_view_control_projection_registrations() -> Vec<(
    WorthUiQueryGraphObligationSemantic,
    ForgeQueryGraphObligationRegistration,
)> {
    live_view_control_projection_registration_operations()
        .into_iter()
        .map(|operation| registration_for_operation(operation, live_view_projection_rule_identity))
        .collect()
}

pub(in crate::runtime::query_graph) fn live_view_conditional_projection_registrations() -> Vec<(
    WorthUiQueryGraphObligationSemantic,
    ForgeQueryGraphObligationRegistration,
)> {
    live_view_conditional_projection_registration_operations()
        .into_iter()
        .map(|operation| registration_for_operation(operation, live_view_projection_rule_identity))
        .collect()
}

pub(in crate::runtime::query_graph) fn live_view_readiness_projection_registrations() -> Vec<(
    WorthUiQueryGraphObligationSemantic,
    ForgeQueryGraphObligationRegistration,
)> {
    live_view_readiness_projection_registration_operations()
        .into_iter()
        .map(|operation| registration_for_operation(operation, live_view_projection_rule_identity))
        .collect()
}

pub(in crate::runtime::query_graph) fn live_view_expression_projection_registrations() -> Vec<(
    WorthUiQueryGraphObligationSemantic,
    ForgeQueryGraphObligationRegistration,
)> {
    live_view_expression_projection_registration_operations()
        .into_iter()
        .map(|operation| registration_for_operation(operation, live_view_projection_rule_identity))
        .collect()
}

pub(in crate::runtime::query_graph) fn live_view_interaction_intent_registrations() -> Vec<(
    WorthUiQueryGraphObligationSemantic,
    ForgeQueryGraphObligationRegistration,
)> {
    live_view_interaction_intent_registration_operations()
        .into_iter()
        .map(|operation| registration_for_operation(operation, live_view_interaction_rule_identity))
        .collect()
}

pub(in crate::runtime::query_graph) fn live_view_payload_projection_registrations() -> Vec<(
    WorthUiQueryGraphObligationSemantic,
    ForgeQueryGraphObligationRegistration,
)> {
    live_view_payload_projection_registration_operations()
        .into_iter()
        .map(|operation| registration_for_operation(operation, live_view_projection_rule_identity))
        .collect()
}

fn registration_for_operation(
    operation: WorthUiQueryGraphOperationDeclaration,
    rule_identity: fn(WorthUiQueryGraphObligationSemantic) -> ForgeQueryGraphObligationRuleIdentity,
) -> (
    WorthUiQueryGraphObligationSemantic,
    ForgeQueryGraphObligationRegistration,
) {
    let semantic = operation.semantic();
    let registration = ForgeQueryGraphObligationRegistration::new(
        semantic.canonical_kind(),
        rule_identity(semantic),
        WorthUiQueryGraphTouchDescriptor::operation_selector(operation.operation_id()),
        ForgeQueryGraphObligationOperatingWorldSelector::preview(),
    )
    .with_support_posture(operation.support_posture());
    (semantic, registration)
}

pub(in crate::runtime::query_graph) fn mounted_interaction_rule_identity(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> ForgeQueryGraphObligationRuleIdentity {
    ForgeQueryGraphObligationRuleIdentity::new(
        "worth-ui-mounted-interaction",
        semantic.rule_name(),
        "phase29-query-owned",
    )
    .expect("Worth query graph rule identities are non-empty constants")
}

pub(in crate::runtime::query_graph) fn primitive_construction_rule_identity(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> ForgeQueryGraphObligationRuleIdentity {
    ForgeQueryGraphObligationRuleIdentity::new(
        "worth-ui-primitive-construction",
        semantic.rule_name(),
        "phase29-query-owned",
    )
    .expect("Worth primitive construction rule identities are non-empty constants")
}

pub(in crate::runtime::query_graph) fn composition_topology_rule_identity(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> ForgeQueryGraphObligationRuleIdentity {
    let rule_family = match semantic.canonical_kind() {
        ForgeQueryGraphObligationKind::BlockingInvariant => "worth-ui-composition-invariant",
        ForgeQueryGraphObligationKind::OperatingContextGate => "worth-ui-composition-context",
        ForgeQueryGraphObligationKind::SchemaContractValidator => "worth-ui-composition-schema",
        ForgeQueryGraphObligationKind::PreflightSequencingObligation => {
            "worth-ui-composition-sequence"
        }
        _ => "worth-ui-composition",
    };
    ForgeQueryGraphObligationRuleIdentity::new(rule_family, semantic.rule_name(), "milestone4.1")
        .expect("Worth composition topology rule identities are non-empty constants")
}

pub(in crate::runtime::query_graph) fn composition_graph_access_rule_identity(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> ForgeQueryGraphObligationRuleIdentity {
    let rule_family = match semantic.canonical_kind() {
        ForgeQueryGraphObligationKind::BlockingInvariant => "worth-ui-composition-access-invariant",
        ForgeQueryGraphObligationKind::OperatingContextGate => {
            "worth-ui-composition-access-context"
        }
        ForgeQueryGraphObligationKind::PreflightSequencingObligation => {
            "worth-ui-composition-access-sequence"
        }
        _ => "worth-ui-composition-access",
    };
    ForgeQueryGraphObligationRuleIdentity::new(rule_family, semantic.rule_name(), "milestone4.1")
        .expect("Worth composition graph access rule identities are non-empty constants")
}

pub(in crate::runtime::query_graph) fn composition_context_rule_identity(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> ForgeQueryGraphObligationRuleIdentity {
    let rule_family = match semantic.canonical_kind() {
        ForgeQueryGraphObligationKind::BlockingInvariant => {
            "worth-ui-composition-context-invariant"
        }
        ForgeQueryGraphObligationKind::OperatingContextGate => "worth-ui-composition-context-gate",
        ForgeQueryGraphObligationKind::PreflightSequencingObligation => {
            "worth-ui-composition-context-sequence"
        }
        _ => "worth-ui-composition-context",
    };
    ForgeQueryGraphObligationRuleIdentity::new(rule_family, semantic.rule_name(), "milestone4.1")
        .expect("Worth composition context rule identities are non-empty constants")
}

pub(in crate::runtime::query_graph) fn composition_participation_rule_identity(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> ForgeQueryGraphObligationRuleIdentity {
    let rule_family = match semantic.canonical_kind() {
        ForgeQueryGraphObligationKind::BlockingInvariant => {
            "worth-ui-composition-participation-invariant"
        }
        ForgeQueryGraphObligationKind::OperatingContextGate => {
            "worth-ui-composition-participation-context"
        }
        ForgeQueryGraphObligationKind::SchemaContractValidator => {
            "worth-ui-composition-participation-schema"
        }
        _ => "worth-ui-composition-participation",
    };
    ForgeQueryGraphObligationRuleIdentity::new(rule_family, semantic.rule_name(), "milestone4.1")
        .expect("Worth composition participation rule identities are non-empty constants")
}

pub(in crate::runtime::query_graph) fn primitive_event_rule_identity(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> ForgeQueryGraphObligationRuleIdentity {
    let rule_family = match semantic.canonical_kind() {
        ForgeQueryGraphObligationKind::BlockingInvariant => "worth-ui-primitive-event-block",
        ForgeQueryGraphObligationKind::OperatingContextGate => "worth-ui-primitive-event-context",
        ForgeQueryGraphObligationKind::CapabilityGapScreen => "worth-ui-primitive-event-support",
        ForgeQueryGraphObligationKind::PreflightSequencingObligation => {
            "worth-ui-primitive-event-sequence"
        }
        _ => "worth-ui-primitive-event",
    };
    ForgeQueryGraphObligationRuleIdentity::new(rule_family, semantic.rule_name(), "phase33")
        .expect("Worth primitive event rule identities are non-empty constants")
}

pub(in crate::runtime::query_graph) fn primitive_content_rule_identity(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> ForgeQueryGraphObligationRuleIdentity {
    let rule_family = match semantic.canonical_kind() {
        ForgeQueryGraphObligationKind::SchemaContractValidator => {
            "worth-ui-primitive-content-schema"
        }
        ForgeQueryGraphObligationKind::CapabilityGapScreen => {
            "worth-ui-primitive-content-capability"
        }
        ForgeQueryGraphObligationKind::OperatingContextGate => "worth-ui-primitive-content-context",
        ForgeQueryGraphObligationKind::PreflightSequencingObligation => {
            "worth-ui-primitive-content-sequence"
        }
        _ => "worth-ui-primitive-content",
    };
    ForgeQueryGraphObligationRuleIdentity::new(rule_family, semantic.rule_name(), "phase34")
        .expect("Worth primitive content rule identities are non-empty constants")
}

pub(in crate::runtime::query_graph) fn user_intent_target_rule_identity(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> ForgeQueryGraphObligationRuleIdentity {
    let rule_family = match semantic.canonical_kind() {
        ForgeQueryGraphObligationKind::BlockingInvariant => "worth-ui-target-binding-invariant",
        ForgeQueryGraphObligationKind::OperatingContextGate => "worth-ui-target-binding-context",
        ForgeQueryGraphObligationKind::PreflightSequencingObligation => {
            "worth-ui-target-binding-sequence"
        }
        _ => "worth-ui-target-binding",
    };
    ForgeQueryGraphObligationRuleIdentity::new(rule_family, semantic.rule_name(), "phase34.5")
        .expect("Worth user intent target rule identities are non-empty constants")
}

pub(in crate::runtime::query_graph) fn live_view_state_rule_identity(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> ForgeQueryGraphObligationRuleIdentity {
    let rule_family = match semantic.canonical_kind() {
        ForgeQueryGraphObligationKind::BlockingInvariant => "worth-ui-live-view-invariant",
        ForgeQueryGraphObligationKind::OperatingContextGate => "worth-ui-live-view-context",
        ForgeQueryGraphObligationKind::SchemaContractValidator => "worth-ui-live-view-schema",
        ForgeQueryGraphObligationKind::CapabilityGapScreen => "worth-ui-live-view-support",
        ForgeQueryGraphObligationKind::PreflightSequencingObligation => {
            "worth-ui-live-view-sequence"
        }
        _ => "worth-ui-live-view",
    };
    ForgeQueryGraphObligationRuleIdentity::new(rule_family, semantic.rule_name(), "phase34.2")
        .expect("Worth live view rule identities are non-empty constants")
}

pub(in crate::runtime::query_graph) fn live_view_projection_rule_identity(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> ForgeQueryGraphObligationRuleIdentity {
    let rule_family = match semantic.canonical_kind() {
        ForgeQueryGraphObligationKind::SchemaContractValidator => {
            "worth-ui-live-view-projection-schema"
        }
        ForgeQueryGraphObligationKind::CapabilityGapScreen => {
            "worth-ui-live-view-projection-support"
        }
        ForgeQueryGraphObligationKind::OperatingContextGate => {
            "worth-ui-live-view-projection-context"
        }
        ForgeQueryGraphObligationKind::PreflightSequencingObligation => {
            "worth-ui-live-view-projection-sequence"
        }
        _ => "worth-ui-live-view-projection",
    };
    ForgeQueryGraphObligationRuleIdentity::new(rule_family, semantic.rule_name(), "phase34.3")
        .expect("Worth live view projection rule identities are non-empty constants")
}

pub(in crate::runtime::query_graph) fn live_view_interaction_rule_identity(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> ForgeQueryGraphObligationRuleIdentity {
    let rule_family = match semantic.canonical_kind() {
        ForgeQueryGraphObligationKind::SchemaContractValidator => {
            "worth-ui-live-view-interaction-schema"
        }
        ForgeQueryGraphObligationKind::CapabilityGapScreen => {
            "worth-ui-live-view-interaction-support"
        }
        ForgeQueryGraphObligationKind::BlockingInvariant => {
            "worth-ui-live-view-interaction-invariant"
        }
        ForgeQueryGraphObligationKind::PreflightSequencingObligation => {
            "worth-ui-live-view-interaction-sequence"
        }
        _ => "worth-ui-live-view-interaction",
    };
    ForgeQueryGraphObligationRuleIdentity::new(rule_family, semantic.rule_name(), "phase34.4")
        .expect("Worth live view interaction rule identities are non-empty constants")
}
