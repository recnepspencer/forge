use worth_ui::facade::{
    WorthUiLiveViewStateValue, WorthUiMountedInteractionNodeReceipt, WorthUiMountedNodeReceipt,
};
use worth_ui_validation_app::reload::ValidationLiveViewSource;
use worth_ui_validation_app::{ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch};

pub fn prepared_app_with_live_view_source(
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

pub fn apply_text(
    app: &mut worth_ui_validation_app::ValidationWorkbenchApp,
    binding: &str,
    value: &str,
) {
    let intent = app
        .live_view_control_edit_intent(binding, WorthUiLiveViewStateValue::text(value))
        .expect("edit intent derives from admitted binding");
    app.workbench_mut()
        .runtime_mut()
        .apply_live_view_state_edit(intent)
        .expect("edit applies");
}

pub fn mounted_submit_interaction(
    proof: &worth_ui_validation_app::app::ValidationLiveViewProjectionProof,
) -> WorthUiMountedInteractionNodeReceipt {
    fn visit(
        tree: &worth_ui::facade::WorthUiMountedCompositionTreeReceipt,
        parent_id: &str,
    ) -> Option<WorthUiMountedInteractionNodeReceipt> {
        for child in tree.ordered_children(parent_id) {
            if let WorthUiMountedNodeReceipt::Interaction(node) = child.mounted_node() {
                return Some(node.clone());
            }
            if let Some(node) = visit(tree, child.node_id()) {
                return Some(node);
            }
        }
        None
    }
    let tree = proof.mounted_product_view().composition_tree();
    visit(tree, tree.root().root_id().as_str()).expect("mounted submit exists")
}

pub fn contact_source() -> String {
    r##"live_view validation.live_view.primitive_proof {
    target button_proof
    flow_kind column
    flow_gap validation.density.primitive.flow.gap.default
    flow_padding validation.density.primitive.flow.padding.default
    flow_align end
    appearance_rest_background "#ffffff"
    appearance_rest_foreground "#344054"
    appearance_rest_text_color "#344054"
    appearance_rest_border_color "#e4e7ec"
    appearance_rest_border_width validation.density.primitive.border.default
    appearance_rest_radius validation.density.primitive.radius
    state first_name {
        fact validation.state.contact.first_name
        kind text
        access read_write
    }
    state contact_mode {
        fact validation.state.contact.mode
        kind text
        access read_write
    }
    control first_name_input {
        binding first_name
        projection text_input
        label "First name"
        flow_padding validation.density.primitive.flow.padding.compact
        appearance_rest_background "#1f2937"
        appearance_rest_foreground "#f9fafb"
        appearance_rest_text_color "#f9fafb"
        appearance_rest_border_color "#93c5fd"
        appearance_rest_border_width validation.density.primitive.border.default
        appearance_rest_radius validation.density.primitive.radius
        event_cursor text
    }
    control contact_mode_input {
        binding contact_mode
        projection select
        label "Contact mode"
        options yes:Yes,no:No
        flow_padding validation.density.primitive.flow.padding.compact
        appearance_rest_background "#111827"
        appearance_rest_foreground "#f9fafb"
        appearance_rest_text_color "#f9fafb"
        appearance_rest_border_color "#93c5fd"
        appearance_rest_border_width validation.density.primitive.border.default
        appearance_rest_radius validation.density.primitive.radius
        event_cursor pointer
    }
    readiness contact_submit_ready {
        required first_name,contact_mode
    }
    payload contact_submit_payload {
        shape payload_values
    }
    interaction contact_submit {
        kind submit
        effect validation.effect.contact.submit
        readiness contact_submit_ready
        payload contact_submit_payload
        label "Submit"
    }
    composition validation.live_view.primitive_proof {
        root page_content_slot button_proof
        surface live_view.form_card {
            policy local_layout validation.flow.form.card
            container input_stack {
                policy local_layout validation.flow.form.inputs
                child control first_name_input sizing fill(1)
                child control contact_mode_input sizing fill(1)
            }
            container action_row {
                policy local_layout validation.flow.form.actions
                child interaction contact_submit sizing hug
            }
            child diagnostic_panel live_view.evidence
        }
    }
}"##
    .to_owned()
}
