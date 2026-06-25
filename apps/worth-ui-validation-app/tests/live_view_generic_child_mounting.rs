use worth_ui::facade::{
    WorthUiLiveViewCompositionChildSubjectKind,
    WorthUiLiveViewCompositionSubjectReconciliationPosture, WorthUiLiveViewControlHostFrameKind,
    WorthUiRuntimeFactFamily,
};
use worth_ui_validation_app::reload::ValidationLiveViewSource;
use worth_ui_validation_app::{
    ValidationLiveViewCompositionRebindDecision, ValidationWorkbenchAuthoredInputs,
    ValidationWorkbenchLaunch,
};

#[path = "support/live_view_product/mod.rs"]
mod live_view_product;

use live_view_product::scenario::LiveViewProductScenario;
use live_view_product::topology_assertions::{
    assert_container_children, assert_container_excludes_subject, expected_control,
    expected_interaction, mounted_control_child, mounted_interaction_child,
};

#[test]
fn controls_and_interactions_mount_with_graph_child_binding_receipts() {
    let app =
        prepared_app_with_live_view_source(LiveViewProductScenario::contact_form().to_source());
    let proof = app
        .live_view_projection_proof()
        .expect("projection admits through runtime");
    let tree = proof.mounted_product_view().composition_tree();
    assert_container_children(
        tree,
        "input_stack",
        &[
            expected_control("first_name_input", "fill(1)"),
            expected_control("contact_mode_input", "fill(1)"),
        ],
    );
    assert_container_children(
        tree,
        "action_row",
        &[expected_interaction("contact_submit", "hug")],
    );

    let reconciliation = proof.mounted_product_view().child_reconciliation();
    let counters = reconciliation.counters();
    assert_eq!(counters.mounted_subject_count(), 3);
    assert_eq!(counters.declared_unmounted_count(), 0);
    assert_eq!(counters.missing_payload_count(), 0);
    assert_eq!(counters.duplicate_subject_count(), 0);
    assert_eq!(counters.projection_control_scan_count(), 1);
    assert_eq!(counters.projection_interaction_scan_count(), 1);
    assert!(reconciliation.rows().iter().any(|row| {
        row.subject_id() == "first_name_input"
            && row.posture() == WorthUiLiveViewCompositionSubjectReconciliationPosture::Mounted
            && row.parent_id() == Some("input_stack")
    }));

    let first_name_control =
        mounted_control_child(tree, "first_name_input").expect("first name control mounts");
    let first_name_binding = first_name_control.composition_child_binding();

    assert_eq!(
        first_name_binding.subject_kind(),
        WorthUiLiveViewCompositionChildSubjectKind::Control
    );
    assert_eq!(first_name_binding.subject_id(), "first_name_input");
    assert_eq!(
        first_name_binding.composition_node_id(),
        "live_view.control.first_name_input"
    );
    assert_eq!(first_name_binding.authority_identity(), "first_name_input");
    assert_eq!(first_name_binding.parent_id(), "input_stack");
    assert_eq!(first_name_binding.sizing_token(), "fill(1)");
    assert_ne!(first_name_binding.child_access_row_digest(), 0);
    assert!(first_name_binding
        .consumed_facts()
        .iter()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::CompositionNode));
    assert!(first_name_binding
        .consumed_facts()
        .iter()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::CompositionEdge));

    let submit_interaction =
        mounted_interaction_child(tree, "contact_submit").expect("submit interaction mounts");
    let submit_binding = submit_interaction.composition_child_binding();

    assert_eq!(
        submit_binding.subject_kind(),
        WorthUiLiveViewCompositionChildSubjectKind::Interaction
    );
    assert_eq!(submit_binding.subject_id(), "contact_submit");
    assert_eq!(
        submit_binding.composition_node_id(),
        "live_view.interaction.contact_submit"
    );
    assert_eq!(submit_binding.parent_id(), "action_row");
    assert_eq!(submit_binding.sizing_token(), "hug");
    assert_ne!(submit_binding.binding_digest(), 0);
}

#[test]
fn moving_control_between_containers_rebinds_graph_child_not_state_identity() {
    let mut app =
        prepared_app_with_live_view_source(LiveViewProductScenario::contact_form().to_source());
    let before = app
        .live_view_projection_proof()
        .expect("initial projection admits");
    let before_tree = before.mounted_product_view().composition_tree();
    assert_container_children(
        before_tree,
        "input_stack",
        &[
            expected_control("first_name_input", "fill(1)"),
            expected_control("contact_mode_input", "fill(1)"),
        ],
    );
    let before_control =
        mounted_control_child(before_tree, "first_name_input").expect("first name starts mounted");
    let before_binding = before_control.composition_child_binding();
    assert_eq!(before_binding.parent_id(), "input_stack");
    assert_eq!(before_control.host_frame().control_id(), "first_name_input");

    let moved = app
        .hot_reload_live_view_source_with_composition_proof(
            LiveViewProductScenario::contact_form()
                .move_child("first_name_input", "action_row")
                .to_source(),
        )
        .expect("moving a control between containers admits");
    let after_tree = moved.next_product_view().composition_tree();
    assert_container_children(
        after_tree,
        "input_stack",
        &[expected_control("contact_mode_input", "fill(1)")],
    );
    assert_container_children(
        after_tree,
        "action_row",
        &[
            expected_interaction("contact_submit", "hug"),
            expected_control("first_name_input", "fill(1)"),
        ],
    );
    assert_container_excludes_subject(after_tree, "input_stack", "first_name_input");
    let after_control =
        mounted_control_child(after_tree, "first_name_input").expect("first name remains mounted");
    let after_binding = after_control.composition_child_binding();

    assert_eq!(after_control.host_frame().control_id(), "first_name_input");
    assert_eq!(
        after_control.host_frame().subject().control_id(),
        "first_name_input"
    );
    assert_eq!(after_binding.subject_id(), before_binding.subject_id());
    assert_eq!(
        after_binding.composition_node_id(),
        before_binding.composition_node_id()
    );
    assert_ne!(after_binding.parent_id(), before_binding.parent_id());
    assert_eq!(after_binding.parent_id(), "action_row");
    assert_ne!(
        after_binding.binding_digest(),
        before_binding.binding_digest()
    );
    assert!(moved.rows().iter().any(|row| {
        row.semantic_slice() == "MountedCompositionTree"
            && row.decision() == ValidationLiveViewCompositionRebindDecision::Rebind
    }));
    assert!(!moved
        .projection_rebind()
        .control_rebind()
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::LiveViewStateValue));
}

#[test]
fn unmounted_declared_control_does_not_render_or_crash_mounting() {
    let app = prepared_app_with_live_view_source(
        LiveViewProductScenario::contact_form()
            .remove_child("contact_mode_input")
            .to_source(),
    );
    let proof = app
        .live_view_projection_proof()
        .expect("projection admits with an unmounted declared control");
    let tree = proof.mounted_product_view().composition_tree();

    assert_container_children(
        tree,
        "input_stack",
        &[expected_control("first_name_input", "fill(1)")],
    );
    assert!(mounted_control_child(tree, "first_name_input").is_some());
    assert!(mounted_control_child(tree, "contact_mode_input").is_none());
    assert!(mounted_interaction_child(tree, "contact_submit").is_some());
    assert_declared_but_unmounted(
        proof.mounted_product_view().child_reconciliation(),
        WorthUiLiveViewCompositionChildSubjectKind::Control,
        "contact_mode_input",
    );
}

#[test]
fn unmounted_declared_interaction_does_not_render_or_crash_mounting() {
    let app = prepared_app_with_live_view_source(
        LiveViewProductScenario::contact_form()
            .remove_child("contact_submit")
            .to_source(),
    );
    let proof = app
        .live_view_projection_proof()
        .expect("projection admits with an unmounted declared interaction");
    let tree = proof.mounted_product_view().composition_tree();

    assert!(mounted_control_child(tree, "first_name_input").is_some());
    assert!(mounted_control_child(tree, "contact_mode_input").is_some());
    assert!(mounted_interaction_child(tree, "contact_submit").is_none());
    assert_declared_but_unmounted(
        proof.mounted_product_view().child_reconciliation(),
        WorthUiLiveViewCompositionChildSubjectKind::Interaction,
        "contact_submit",
    );
}

#[test]
fn dropdown_control_and_submit_interaction_move_as_graph_children() {
    let mut app = prepared_app_with_live_view_source(
        LiveViewProductScenario::contact_form()
            .with_contact_mode_select()
            .to_source(),
    );
    let before = app
        .live_view_projection_proof()
        .expect("select projection admits");
    let before_tree = before.mounted_product_view().composition_tree();
    let before_dropdown =
        mounted_control_child(before_tree, "contact_mode_input").expect("dropdown mounts");
    let before_submit =
        mounted_interaction_child(before_tree, "contact_submit").expect("submit mounts");

    assert_eq!(
        before_dropdown.kind(),
        WorthUiLiveViewControlHostFrameKind::DropdownInput
    );
    assert_eq!(
        before_dropdown.composition_child_binding().parent_id(),
        "input_stack"
    );
    assert_eq!(
        before_submit.composition_child_binding().parent_id(),
        "action_row"
    );

    let moved = app
        .hot_reload_live_view_source_with_composition_proof(
            LiveViewProductScenario::contact_form()
                .with_contact_mode_select()
                .move_child("contact_mode_input", "action_row")
                .move_child("contact_submit", "input_stack")
                .to_source(),
        )
        .expect("moving dropdown and submit interaction admits");
    let after_tree = moved.next_product_view().composition_tree();
    assert_container_children(
        after_tree,
        "input_stack",
        &[
            expected_control("first_name_input", "fill(1)"),
            expected_interaction("contact_submit", "hug"),
        ],
    );
    assert_container_children(
        after_tree,
        "action_row",
        &[expected_control("contact_mode_input", "fill(1)")],
    );
    let after_dropdown =
        mounted_control_child(after_tree, "contact_mode_input").expect("dropdown remains mounted");
    let after_submit =
        mounted_interaction_child(after_tree, "contact_submit").expect("submit remains mounted");

    assert_eq!(
        after_dropdown
            .composition_child_binding()
            .composition_node_id(),
        before_dropdown
            .composition_child_binding()
            .composition_node_id()
    );
    assert_eq!(
        after_submit
            .composition_child_binding()
            .composition_node_id(),
        before_submit
            .composition_child_binding()
            .composition_node_id()
    );
    assert_eq!(
        after_dropdown.composition_child_binding().parent_id(),
        "action_row"
    );
    assert_eq!(
        after_submit.composition_child_binding().parent_id(),
        "input_stack"
    );
    assert!(!moved
        .projection_rebind()
        .control_rebind()
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::LiveViewStateValue));
}

#[test]
fn text_to_dropdown_hot_reload_preserves_mounted_child_identity() {
    let mut app =
        prepared_app_with_live_view_source(LiveViewProductScenario::contact_form().to_source());
    let before = app
        .live_view_projection_proof()
        .expect("text projection admits");
    let before_control = mounted_control_child(
        before.mounted_product_view().composition_tree(),
        "contact_mode_input",
    )
    .expect("contact mode starts mounted");

    assert_eq!(
        before_control.kind(),
        WorthUiLiveViewControlHostFrameKind::TextInput
    );

    let reloaded = app
        .hot_reload_live_view_source_with_composition_proof(
            LiveViewProductScenario::contact_form()
                .with_contact_mode_select()
                .to_source(),
        )
        .expect("text to dropdown admits");
    let after_control = mounted_control_child(
        reloaded.next_product_view().composition_tree(),
        "contact_mode_input",
    )
    .expect("contact mode remains mounted");

    assert_eq!(
        after_control.kind(),
        WorthUiLiveViewControlHostFrameKind::DropdownInput
    );
    assert_eq!(
        after_control
            .composition_child_binding()
            .composition_node_id(),
        before_control
            .composition_child_binding()
            .composition_node_id()
    );
    assert_eq!(
        after_control.state_binding().binding_id(),
        before_control.state_binding().binding_id()
    );
    assert_ne!(after_control.frame_digest(), before_control.frame_digest());
    assert!(!reloaded
        .projection_rebind()
        .control_rebind()
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::LiveViewStateValue));
}

fn prepared_app_with_live_view_source(
    source_text: String,
) -> worth_ui_validation_app::ValidationWorkbenchApp {
    let launch = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(
            ValidationWorkbenchAuthoredInputs::sample()
                .with_live_view_source(ValidationLiveViewSource::new(source_text)),
        )
        .expect("validation app should prepare");
    worth_ui_validation_app::ValidationWorkbenchApp::new(launch)
}

fn assert_declared_but_unmounted(
    reconciliation: &worth_ui::facade::WorthUiLiveViewCompositionSubjectReconciliationReceipt,
    subject_kind: WorthUiLiveViewCompositionChildSubjectKind,
    subject_id: &str,
) {
    assert_eq!(reconciliation.counters().declared_unmounted_count(), 1);
    assert!(reconciliation.rows().iter().any(|row| {
        row.subject_kind() == subject_kind
            && row.subject_id() == subject_id
            && row.composition_node_id().is_none()
            && row.parent_id().is_none()
            && row.posture()
                == WorthUiLiveViewCompositionSubjectReconciliationPosture::DeclaredButUnmounted
    }));
}
