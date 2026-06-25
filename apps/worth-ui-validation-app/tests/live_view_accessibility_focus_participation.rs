#[allow(dead_code)]
mod support;

use support::live_view_accessibility_scenarios::{
    source_with_invalid_accessibility_associations, source_with_moved_title_label,
    source_with_title_label_and_helper,
};
use support::live_view_layout_allocation::{
    allocate, measured_view_with_observations, mounted_product_view,
};
use support::live_view_viewport_fixtures::card_clip_boundary_source;
use worth_ui::facade::{
    WorthUiAccessibilityAssociationKind, WorthUiAccessibilityHostInspectionPosture,
    WorthUiAccessibilityHostInspectionRowFeature, WorthUiAccessibilityParticipationPosture,
    WorthUiCompositionParticipationDenialCode, WorthUiFocusParticipationPosture,
    WorthUiLiveViewProjectionAdmissionDenial,
};
use worth_ui_validation_app::reload::ValidationLiveViewSource;
use worth_ui_validation_app::{ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch};

#[test]
fn authored_label_and_description_mount_as_graph_participation_receipts() {
    let app = prepared_app_with_live_view_source(source_with_title_label_and_helper());
    let proof = app
        .live_view_projection_proof()
        .expect("accessibility participation source admits");
    let participation = proof.mounted_product_view().composition_participation();

    assert_eq!(participation.associations().len(), 3);
    assert!(participation.associations().iter().any(|association| {
        association.kind() == WorthUiAccessibilityAssociationKind::Label
            && association.source_node_id() == "title_label"
            && association.target_node_id() == "live_view.control.title_input"
    }));
    assert!(participation.associations().iter().any(|association| {
        association.kind() == WorthUiAccessibilityAssociationKind::Description
            && association.source_node_id() == "details_helper"
            && association.target_node_id() == "live_view.control.details_input"
    }));
    assert!(participation.associations().iter().any(|association| {
        association.kind() == WorthUiAccessibilityAssociationKind::Error
            && association.source_node_id() == "details_error"
            && association.target_node_id() == "live_view.control.details_input"
    }));

    let title = participation
        .accessibility_nodes()
        .iter()
        .find(|node| node.node_id() == "live_view.control.title_input")
        .expect("title input has accessibility participation");
    assert_eq!(title.name(), Some("Title"));
    assert_eq!(title.role(), "text_input");
    assert_eq!(
        title.posture(),
        WorthUiAccessibilityParticipationPosture::Exposed
    );

    let details = participation
        .accessibility_nodes()
        .iter()
        .find(|node| node.node_id() == "live_view.control.details_input")
        .expect("details input has accessibility participation");
    assert_eq!(
        details.description_node_ids(),
        &["details_helper".to_owned()]
    );
    assert_eq!(details.error_node_ids(), &["details_error".to_owned()]);
    assert!(participation.relationships().iter().any(|relationship| {
        relationship.kind() == WorthUiAccessibilityAssociationKind::Label
            && relationship.source_resolved_text() == Some("Title")
            && relationship.source_role() == "label"
            && relationship.target_role() == "text_input"
    }));
    assert_eq!(participation.counters().relationship_count(), 3);
    assert_eq!(
        participation.traversal().counters().source_reparse_count(),
        0
    );
    assert_eq!(
        participation.traversal().counters().renderer_parse_count(),
        0
    );
    assert_eq!(
        participation
            .traversal()
            .counters()
            .caller_owned_recursive_walk_count(),
        0
    );
}

#[test]
fn host_accessibility_inspection_projects_runtime_rows() {
    let app = prepared_app_with_live_view_source(source_with_title_label_and_helper());
    let participation = app
        .live_view_projection_proof()
        .expect("source admits")
        .mounted_product_view()
        .composition_participation()
        .clone();
    let inspection = app
        .workbench()
        .runtime()
        .inspect_composition_accessibility_host(&participation);

    assert_eq!(
        inspection.posture(),
        WorthUiAccessibilityHostInspectionPosture::ProjectedFromRuntimeReceipt
    );
    assert_eq!(
        inspection.participation_digest(),
        participation.receipt_digest()
    );
    assert_eq!(
        inspection.counters().inspected_node_count(),
        participation.accessibility_nodes().len()
    );
    assert_eq!(inspection.counters().unsupported_host_api_count(), 0);
    assert!(
        inspection.counters().inspected_row_count() > inspection.counters().inspected_node_count()
    );
    assert!(inspection.rows().iter().any(|row| {
        row.node_id() == "live_view.control.title_input"
            && row.feature() == WorthUiAccessibilityHostInspectionRowFeature::Name
            && row.value() == Some("Title")
            && row.posture()
                == WorthUiAccessibilityHostInspectionPosture::ProjectedFromRuntimeReceipt
    }));
    assert!(inspection.rows().iter().any(|row| {
        row.node_id() == "live_view.control.details_input"
            && row.feature() == WorthUiAccessibilityHostInspectionRowFeature::DescribedBy
            && row.value() == Some("Tell us what changed")
    }));
    assert_eq!(inspection.counters().source_reparse_count(), 0);
    assert_eq!(inspection.counters().renderer_parse_count(), 0);
    assert_eq!(inspection.consumed_facts(), participation.consumed_facts());
    assert_ne!(inspection.receipt_digest(), 0);
}

#[test]
fn moving_label_preserves_association_by_graph_identity() {
    let mut app = prepared_app_with_live_view_source(source_with_title_label_and_helper());
    let before = app
        .live_view_projection_proof()
        .expect("initial source admits")
        .mounted_product_view()
        .composition_participation()
        .receipt_digest();

    let after = app
        .hot_reload_live_view_source(source_with_moved_title_label())
        .expect("moving label in composition hot reloads");
    let participation = after.mounted_product_view().composition_participation();
    assert_ne!(before, participation.receipt_digest());
    assert!(participation.associations().iter().any(|association| {
        association.kind() == WorthUiAccessibilityAssociationKind::Label
            && association.source_node_id() == "title_label"
            && association.target_node_id() == "live_view.control.title_input"
    }));
}

#[test]
fn focus_order_is_graph_order_and_disabled_interaction_is_not_focusable() {
    let app = prepared_app_with_live_view_source(source_with_title_label_and_helper());
    let participation = app
        .live_view_projection_proof()
        .expect("source admits")
        .mounted_product_view()
        .composition_participation()
        .clone();

    let focus_rows = participation.focus_nodes();
    let form_card = focus_rows
        .iter()
        .find(|row| row.node_id() == "live_view.form_card")
        .expect("surface participates in focus proof");
    assert_eq!(
        form_card.focus_scope_id(),
        "composition.root.page_content_slot.button_proof"
    );

    let submit = focus_rows
        .iter()
        .find(|row| row.node_id() == "live_view.interaction.proof_submit")
        .expect("submit interaction participates in focus proof");
    assert_eq!(submit.posture(), WorthUiFocusParticipationPosture::Disabled);
    let form_scope = participation
        .focus_scopes()
        .iter()
        .find(|scope| scope.focus_scope_id() == "input_stack")
        .expect("input stack owns a graph focus scope");
    assert_eq!(form_scope.owner_node_id(), "input_stack");
    assert!(form_scope
        .tab_order_node_ids()
        .contains(&"live_view.control.title_input"));

    assert_eq!(participation.counters().source_reparse_count(), 0);
    assert_eq!(participation.counters().renderer_parse_count(), 0);
    assert_eq!(
        participation.counters().caller_owned_recursive_walk_count(),
        0
    );
    assert_eq!(participation.counters().caller_owned_scan_count(), 0);
    assert!(participation.counters().graph_child_row_count() >= focus_rows.len());
    assert!(
        participation
            .query_graph_execution()
            .selected_obligation_count()
            >= 7
    );
}

#[test]
fn invalid_accessibility_association_rejects_before_mounting() {
    let app = prepared_app_with_live_view_source(source_with_invalid_accessibility_associations());
    let report = app
        .live_view_projection_proof_typed()
        .expect_err("invalid accessibility associations deny during projection admission");
    let denials = report
        .denials()
        .iter()
        .filter_map(|denial| match denial {
            WorthUiLiveViewProjectionAdmissionDenial::CompositionParticipation(denial) => {
                Some(denial)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(report.counters().denial_count(), 3);
    assert_eq!(denials.len(), 3);
    assert_eq!(
        denials[0].code(),
        WorthUiCompositionParticipationDenialCode::MissingSourceNode
    );
    assert_eq!(
        denials[1].code(),
        WorthUiCompositionParticipationDenialCode::MissingTargetNode
    );
    assert_eq!(
        denials[2].code(),
        WorthUiCompositionParticipationDenialCode::InvalidSourceKind
    );
    assert_ne!(report.denial_set_digest(), 0);
}

#[test]
fn clipped_viewport_nodes_have_hidden_focus_and_accessibility_posture() {
    let app = prepared_app_with_live_view_source(card_clip_boundary_source());
    let mounted = mounted_product_view(&app);
    let measured = measured_view_with_observations(
        &app,
        &mounted,
        "live_view.form_card",
        420.0,
        70.0,
        |draft| draft,
    );
    let allocation = allocate(&app, &measured, "live_view.form_card");
    let viewport = app
        .workbench()
        .runtime()
        .resolve_viewport_boundaries(&measured, &allocation)
        .expect("card clip viewport admits");
    let effective_viewport = app
        .workbench()
        .runtime()
        .resolve_effective_viewport_participation(&mounted, &viewport);
    let adjusted = app
        .workbench()
        .runtime()
        .resolve_composition_participation_with_effective_viewport(&mounted, &effective_viewport);

    let submit_a11y = adjusted
        .accessibility_nodes()
        .iter()
        .find(|node| node.node_id() == "live_view.interaction.contact_submit")
        .expect("submit has adjusted accessibility participation");
    assert_eq!(
        submit_a11y.posture(),
        WorthUiAccessibilityParticipationPosture::Hidden
    );
    let submit_focus = adjusted
        .focus_nodes()
        .iter()
        .find(|node| node.node_id() == "live_view.interaction.contact_submit")
        .expect("submit has adjusted focus participation");
    assert_eq!(
        submit_focus.posture(),
        WorthUiFocusParticipationPosture::Hidden
    );
    assert!(
        adjusted.consumed_facts().len()
            > mounted.composition_participation().consumed_facts().len()
    );
    assert_eq!(adjusted.counters().source_reparse_count(), 0);
    assert_eq!(adjusted.counters().renderer_parse_count(), 0);
}

fn prepared_app_with_live_view_source(
    source: impl Into<String>,
) -> worth_ui_validation_app::ValidationWorkbenchApp {
    let authored_inputs = ValidationWorkbenchAuthoredInputs::sample()
        .with_live_view_source(ValidationLiveViewSource::new(source.into()));
    let launch = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(authored_inputs)
        .expect("validation app should prepare");
    worth_ui_validation_app::ValidationWorkbenchApp::new(launch)
}
