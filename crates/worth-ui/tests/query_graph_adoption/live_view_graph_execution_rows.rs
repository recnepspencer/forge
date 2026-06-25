use worth_ui::facade::{
    WorthUiLiveViewConditionalProjectionGraphPosture, WorthUiLiveViewControlProjectionGraphPosture,
    WorthUiLiveViewEffectIntentGraphPosture, WorthUiLiveViewReadinessPosture,
    WorthUiLiveViewStateBindingGraphPosture, WorthUiQueryGraphObligationSemantic,
    WorthUiRuntimeFactId, WorthUiRuntimeGraphAuthority,
};

use super::support::support_status_for;

#[test]
fn live_view_state_binding_uses_query_graph_execution_rows() {
    let graph_authority = WorthUiRuntimeGraphAuthority::new();
    let receipt = graph_authority
        .plan_live_view_state_binding_graph_operation(
            "validation.live_view.proof",
            99,
            [
                WorthUiRuntimeFactId::live_view_declaration("validation.live_view.proof"),
                WorthUiRuntimeFactId::live_view_state_binding("validation.live_view.proof:name"),
                WorthUiRuntimeFactId::live_view_state_value("validation.state.name"),
            ],
            WorthUiLiveViewStateBindingGraphPosture::ReadOnlyWrite,
        )
        .into_execution_receipt();
    let semantics = receipt
        .rows()
        .iter()
        .map(|row| row.semantic())
        .collect::<Vec<_>>();

    assert_eq!(receipt.selected_obligation_count(), 7);
    for expected in WorthUiQueryGraphObligationSemantic::LIVE_VIEW_STATE_BINDING {
        assert!(
            semantics.contains(&expected),
            "missing live view graph semantic {expected:?}"
        );
    }
    assert_eq!(
        support_status_for(
            &receipt,
            WorthUiQueryGraphObligationSemantic::LiveViewWritePosture
        ),
        "unsupported"
    );
    assert_eq!(
        support_status_for(
            &receipt,
            WorthUiQueryGraphObligationSemantic::LiveViewTargetBinding
        ),
        "supported"
    );
}

#[test]
fn live_view_control_projection_uses_query_graph_execution_rows() {
    let graph_authority = WorthUiRuntimeGraphAuthority::new();
    let receipt = graph_authority
        .plan_live_view_control_projection_graph_operation(
            "validation.live_view.proof",
            "contact_mode_input",
            [
                WorthUiRuntimeFactId::live_view_declaration("validation.live_view.proof"),
                WorthUiRuntimeFactId::live_view_state_binding(
                    "validation.live_view.proof:contact_mode",
                ),
                WorthUiRuntimeFactId::live_view_control_projection(
                    "validation.live_view.proof:contact_mode_input",
                ),
            ],
            WorthUiLiveViewControlProjectionGraphPosture::Admitted,
        )
        .into_execution_receipt();
    let semantics = receipt
        .rows()
        .iter()
        .map(|row| row.semantic())
        .collect::<Vec<_>>();

    assert_eq!(receipt.selected_obligation_count(), 4);
    for expected in WorthUiQueryGraphObligationSemantic::LIVE_VIEW_CONTROL_PROJECTION {
        assert!(
            semantics.contains(&expected),
            "missing live view control projection graph semantic {expected:?}"
        );
    }
    assert_eq!(
        support_status_for(
            &receipt,
            WorthUiQueryGraphObligationSemantic::LiveViewControlProjectionKind
        ),
        "supported"
    );
    assert_eq!(
        support_status_for(
            &receipt,
            WorthUiQueryGraphObligationSemantic::LiveViewControlCompatibility
        ),
        "supported"
    );
}

#[test]
fn live_view_conditional_projection_uses_query_graph_execution_rows() {
    let graph_authority = WorthUiRuntimeGraphAuthority::new();
    let receipt = graph_authority
        .plan_live_view_conditional_projection_graph_operation(
            "validation.live_view.proof",
            "company_name_input",
            [
                WorthUiRuntimeFactId::live_view_declaration("validation.live_view.proof"),
                WorthUiRuntimeFactId::live_view_control_projection(
                    "validation.live_view.proof:company_name_input",
                ),
                WorthUiRuntimeFactId::live_view_state_value("validation.state.contact.mode"),
                WorthUiRuntimeFactId::live_view_participation(
                    "validation.live_view.proof:company_name_input",
                ),
            ],
            WorthUiLiveViewConditionalProjectionGraphPosture::Admitted,
        )
        .into_execution_receipt();
    let semantics = receipt
        .rows()
        .iter()
        .map(|row| row.semantic())
        .collect::<Vec<_>>();

    assert_eq!(receipt.selected_obligation_count(), 4);
    for expected in WorthUiQueryGraphObligationSemantic::LIVE_VIEW_CONDITIONAL_PROJECTION {
        assert!(
            semantics.contains(&expected),
            "missing live view conditional projection graph semantic {expected:?}"
        );
    }
    assert_eq!(
        support_status_for(
            &receipt,
            WorthUiQueryGraphObligationSemantic::LiveViewConditionalParticipation
        ),
        "supported"
    );
    assert_eq!(
        support_status_for(
            &receipt,
            WorthUiQueryGraphObligationSemantic::LiveViewRetainedStatePosture
        ),
        "supported"
    );
}

#[test]
fn live_view_readiness_projection_uses_query_graph_execution_rows() {
    let graph_authority = WorthUiRuntimeGraphAuthority::new();
    let receipt = graph_authority
        .plan_live_view_readiness_projection_graph_operation(
            "validation.live_view.proof",
            "contact_submit_ready",
            [
                WorthUiRuntimeFactId::live_view_declaration("validation.live_view.proof"),
                WorthUiRuntimeFactId::live_view_readiness_projection(
                    "validation.live_view.proof:contact_submit_ready",
                ),
                WorthUiRuntimeFactId::live_view_state_value("validation.state.contact.first_name"),
            ],
        )
        .into_execution_receipt();
    let semantics = receipt
        .rows()
        .iter()
        .map(|row| row.semantic())
        .collect::<Vec<_>>();

    assert_eq!(receipt.selected_obligation_count(), 5);
    for expected in WorthUiQueryGraphObligationSemantic::LIVE_VIEW_READINESS_PROJECTION {
        assert!(
            semantics.contains(&expected),
            "missing live view readiness graph semantic {expected:?}"
        );
    }
}

#[test]
fn live_view_expression_projection_uses_query_graph_execution_rows() {
    let graph_authority = WorthUiRuntimeGraphAuthority::new();
    let receipt = graph_authority
        .plan_live_view_expression_projection_graph_operation(
            "validation.live_view.proof",
            "validation.live_view.proof:contact_submit_ready:requiredness",
            [
                WorthUiRuntimeFactId::live_view_expression_declaration(
                    "validation.live_view.proof:contact_submit_ready:requiredness",
                ),
                WorthUiRuntimeFactId::live_view_state_binding(
                    "validation.live_view.proof:first_name",
                ),
                WorthUiRuntimeFactId::live_view_state_value("validation.state.contact.first_name"),
            ],
        )
        .into_execution_receipt();
    let semantics = receipt
        .rows()
        .iter()
        .map(|row| row.semantic())
        .collect::<Vec<_>>();

    assert_eq!(receipt.selected_obligation_count(), 5);
    for expected in WorthUiQueryGraphObligationSemantic::LIVE_VIEW_EXPRESSION_PROJECTION {
        assert!(
            semantics.contains(&expected),
            "missing live view expression graph semantic {expected:?}"
        );
    }
}

#[test]
fn live_view_interaction_intent_uses_readiness_posture_graph_rows() {
    let graph_authority = WorthUiRuntimeGraphAuthority::new();
    let receipt = graph_authority
        .plan_live_view_interaction_intent_graph_operation(
            "validation.live_view.proof",
            "contact_submit",
            [
                WorthUiRuntimeFactId::live_view_interaction_intent(
                    "validation.live_view.proof:contact_submit",
                ),
                WorthUiRuntimeFactId::live_view_readiness_projection(
                    "validation.live_view.proof:contact_submit_ready",
                ),
                WorthUiRuntimeFactId::live_view_payload_projection(
                    "validation.live_view.proof:contact_submit_payload",
                ),
            ],
            WorthUiLiveViewReadinessPosture::DeniedMissingRequired,
            WorthUiLiveViewEffectIntentGraphPosture::Unsupported,
        )
        .into_execution_receipt();
    let semantics = receipt
        .rows()
        .iter()
        .map(|row| row.semantic())
        .collect::<Vec<_>>();

    assert_eq!(receipt.selected_obligation_count(), 5);
    for expected in WorthUiQueryGraphObligationSemantic::LIVE_VIEW_INTERACTION_INTENT {
        assert!(
            semantics.contains(&expected),
            "missing live view interaction graph semantic {expected:?}"
        );
    }
    assert_eq!(
        support_status_for(
            &receipt,
            WorthUiQueryGraphObligationSemantic::LiveViewReadinessPosture
        ),
        "unsupported"
    );
    assert_eq!(
        support_status_for(
            &receipt,
            WorthUiQueryGraphObligationSemantic::LiveViewInteractionEffect
        ),
        "unsupported"
    );
}

#[test]
fn live_view_payload_projection_uses_query_graph_execution_rows() {
    let graph_authority = WorthUiRuntimeGraphAuthority::new();
    let receipt = graph_authority
        .plan_live_view_payload_projection_graph_operation(
            "validation.live_view.proof",
            "contact_submit_payload",
            [
                WorthUiRuntimeFactId::live_view_payload_projection(
                    "validation.live_view.proof:contact_submit_payload",
                ),
                WorthUiRuntimeFactId::live_view_state_value("validation.state.contact.first_name"),
            ],
        )
        .into_execution_receipt();
    let semantics = receipt
        .rows()
        .iter()
        .map(|row| row.semantic())
        .collect::<Vec<_>>();

    assert_eq!(receipt.selected_obligation_count(), 3);
    for expected in WorthUiQueryGraphObligationSemantic::LIVE_VIEW_PAYLOAD_PROJECTION {
        assert!(
            semantics.contains(&expected),
            "missing live view payload graph semantic {expected:?}"
        );
    }
}
