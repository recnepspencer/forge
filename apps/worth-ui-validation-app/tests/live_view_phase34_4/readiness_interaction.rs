use worth_ui::facade::{
    WorthUiLiveViewInteractionActivationDenial, WorthUiLiveViewProjectionRenderInteractionPosture,
    WorthUiLiveViewReadinessPosture, WorthUiQueryGraphObligationSemantic, WorthUiRuntimeFactFamily,
};

use super::support::{
    apply_text, contact_submit_source, mounted_interaction_for_id,
    prepared_app_with_live_view_source, prepared_app_with_surface_source,
    source_with_button_component, submit_interaction,
};

#[test]
fn submit_readiness_controls_submission_from_runtime_state_and_participation() {
    let mut app = prepared_app_with_live_view_source(contact_submit_source("payload_values"));
    let proof = app
        .live_view_projection_proof()
        .expect("live view action projection admits");
    let interaction = proof
        .interactions()
        .iter()
        .find(|interaction| interaction.interaction_id() == "contact_submit")
        .expect("contact submit interaction exists");
    assert_eq!(
        interaction.readiness().posture(),
        WorthUiLiveViewReadinessPosture::DeniedMissingRequired
    );
    assert_eq!(
        proof.render_plan().interactions()[0].posture(),
        WorthUiLiveViewProjectionRenderInteractionPosture::ReadinessDenied
    );
    assert!(interaction
        .query_graph_execution()
        .rows()
        .iter()
        .any(|row| row.semantic()
            == WorthUiQueryGraphObligationSemantic::LiveViewReadinessPosture
            && row.support_status() == "unsupported"));

    let denial = app
        .workbench()
        .runtime()
        .activate_mounted_live_view_interaction(&mounted_interaction_for_id(
            &app,
            interaction.interaction_id(),
        ))
        .expect_err("denied readiness does not produce submit eligibility");
    assert!(matches!(
        denial,
        WorthUiLiveViewInteractionActivationDenial::ReadinessDenied { .. }
    ));

    apply_text(&mut app, "first_name", "Esther");
    apply_text(&mut app, "contact_mode", "no");
    let proof = app
        .live_view_projection_proof()
        .expect("live view action projection admits after edits");
    let interaction = proof.interactions().first().expect("submit exists");
    assert_eq!(
        interaction.readiness().posture(),
        WorthUiLiveViewReadinessPosture::Enabled
    );
    assert_eq!(
        proof.render_plan().interactions()[0].posture(),
        WorthUiLiveViewProjectionRenderInteractionPosture::Enabled
    );
    let receipt = submit_interaction(&app, interaction);
    assert_eq!(receipt.emitted_payload().shape_token(), "payload");
    assert!(receipt
        .emitted_payload()
        .fields()
        .iter()
        .any(|field| field.name() == "first_name"));
}

#[test]
fn submit_readiness_and_payload_projection_consume_only_targeted_runtime_facts() {
    let app = prepared_app_with_live_view_source(contact_submit_source("payload_values"));
    let proof = app
        .live_view_projection_proof()
        .expect("live view action projection admits");
    let readiness = proof.readinesses().first().expect("readiness exists");
    assert_eq!(
        readiness.target_binding().binding_digest(),
        proof.declaration().target_binding().binding_digest()
    );
    assert!(
        readiness
            .query_graph_execution()
            .touch_descriptor()
            .descriptor()
            .touched_aspect_count()
            >= 1
    );

    let payload = proof.payloads().first().expect("payload projection exists");
    let payload_state_values = payload
        .consumed_facts()
        .iter()
        .filter(|fact| fact.family() == WorthUiRuntimeFactFamily::LiveViewStateValue)
        .map(|fact| fact.identity().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(
        payload_state_values,
        vec![
            "validation.state.contact.company_name",
            "validation.state.contact.mode",
            "validation.state.contact.first_name",
        ]
    );
}

#[test]
fn stale_target_binding_denies_submission_before_payload_emit() {
    let mut app_with_stale_target =
        prepared_app_with_live_view_source(contact_submit_source("payload_values"));
    apply_text(&mut app_with_stale_target, "first_name", "Esther");
    apply_text(&mut app_with_stale_target, "contact_mode", "no");
    let stale_interaction_id = app_with_stale_target
        .live_view_projection_proof()
        .expect("live view action projection admits")
        .interactions()
        .first()
        .expect("submit exists")
        .interaction_id()
        .to_owned();
    let stale_mounted_interaction =
        mounted_interaction_for_id(&app_with_stale_target, &stale_interaction_id);
    let app_with_current_target = prepared_app_with_surface_source(source_with_button_component());
    let denial = app_with_current_target
        .workbench()
        .runtime()
        .activate_mounted_live_view_interaction(&stale_mounted_interaction)
        .expect_err("stale target binding cannot submit");
    assert!(matches!(
        denial,
        WorthUiLiveViewInteractionActivationDenial::StaleTargetBinding { .. }
    ));
}
