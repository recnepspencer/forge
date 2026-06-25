use worth_ui::facade::{
    WorthUiLiveViewInteractionIntentReceipt, WorthUiLiveViewInteractionSubmissionReceipt,
    WorthUiLiveViewStateValue, WorthUiMountedInteractionNodeReceipt, WorthUiMountedNodeReceipt,
};
use worth_ui_validation_app::reload::ValidationLiveViewSource;
use worth_ui_validation_app::reload::ValidationSourcePackage;
use worth_ui_validation_app::sample_source::{
    VALIDATION_SAMPLE_MODULE_PATH, VALIDATION_SAMPLE_SOURCE,
};
use worth_ui_validation_app::{ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch};

pub fn prepared_app_with_live_view_source(
    source_text: String,
) -> worth_ui_validation_app::ValidationWorkbenchApp {
    let inputs = ValidationWorkbenchAuthoredInputs::sample()
        .with_live_view_source(ValidationLiveViewSource::new(source_text));
    let launch = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(inputs)
        .expect("validation app should prepare");
    worth_ui_validation_app::ValidationWorkbenchApp::new(launch)
}

pub fn prepared_app_with_surface_source(
    source_text: String,
) -> worth_ui_validation_app::ValidationWorkbenchApp {
    let inputs = ValidationWorkbenchAuthoredInputs::sample().with_source(
        ValidationSourcePackage::new(VALIDATION_SAMPLE_MODULE_PATH, source_text),
    );
    let launch = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(inputs)
        .expect("validation app should prepare");
    worth_ui_validation_app::ValidationWorkbenchApp::new(launch)
}

pub fn source_with_button_component() -> String {
    VALIDATION_SAMPLE_SOURCE.replace(
        "surface worth.surface.preview.primitive.proof {\n    component worth.component.primitive_proof",
        "surface worth.surface.preview.primitive.proof {\n    component worth.component.button",
    )
}

pub fn contact_submit_source(payload_shape: &str) -> String {
    format!(
        r#"live_view validation.live_view.primitive_proof {{
    target button_proof
    state first_name {{
        fact validation.state.contact.first_name
        kind text
        access read_write
    }}
    state contact_mode {{
        fact validation.state.contact.mode
        kind text
        access read_write
    }}
    state company_name {{
        fact validation.state.contact.company_name
        kind text
        access read_write
    }}
    control first_name_input {{
        binding first_name
        projection text_input
        label "First name"
    }}
    control contact_mode_input {{
        binding contact_mode
        projection select
        label "Contact mode"
        options yes:Yes,no:No
    }}
    control company_name_input {{
        binding company_name
        projection text_input
        label "Company"
    }}
    condition company_name_input {{
        when contact_mode equals "yes"
        true present
        false absent_retaining_state
    }}
    readiness contact_submit_ready {{
        required first_name,contact_mode,company_name
    }}
    payload contact_submit_payload {{
        shape {payload_shape}
    }}
    interaction contact_submit {{
        kind submit
        effect validation.effect.contact.submit
        readiness contact_submit_ready
        payload contact_submit_payload
        label "Submit"
    }}
    composition validation.live_view.primitive_proof {{
        root page_content_slot button_proof
        surface live_view.form_card {{
            policy local_layout validation.flow.form.card
            container input_stack {{
                policy local_layout validation.flow.form.inputs
                child control first_name_input sizing fill(1)
                child control contact_mode_input sizing fill(1)
                child control company_name_input sizing fill(1)
            }}
            container action_row {{
                policy local_layout validation.flow.form.actions
                child interaction contact_submit sizing hug
            }}
            child diagnostic_panel live_view.evidence
        }}
    }}
}}"#
    )
}

pub fn invalid_action_source() -> String {
    r#"live_view validation.live_view.primitive_proof {
    target button_proof
    state first_name {
        fact validation.state.contact.first_name
        kind text
        access read_write
    }
    readiness bad ready {
        required first_name,missing_binding
    }
    payload contact_submit_payload {
        shape array_payload
    }
    interaction contact_submit {
        kind fly
        effect unsupported.effect.submit
        readiness missing_ready
        payload missing_payload
        label "Submit"
    }
}"#
    .to_owned()
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

pub fn submit_interaction(
    app: &worth_ui_validation_app::ValidationWorkbenchApp,
    interaction: &WorthUiLiveViewInteractionIntentReceipt,
) -> WorthUiLiveViewInteractionSubmissionReceipt {
    let mounted_interaction = mounted_interaction_for_id(app, interaction.interaction_id());
    app.workbench()
        .runtime()
        .activate_mounted_live_view_interaction(&mounted_interaction)
        .map(|eligible| {
            app.workbench()
                .runtime()
                .submit_live_view_interaction(eligible)
        })
        .expect("enabled interaction emits submit receipt")
}

pub fn mounted_interaction_for_id(
    app: &worth_ui_validation_app::ValidationWorkbenchApp,
    interaction_id: &str,
) -> WorthUiMountedInteractionNodeReceipt {
    let proof = app
        .live_view_projection_proof()
        .expect("live view projection should admit");
    let tree = proof.mounted_product_view().composition_tree();
    find_mounted_interaction(tree, tree.root().root_id().as_str(), interaction_id)
        .cloned()
        .expect("mounted interaction exists")
}

fn find_mounted_interaction<'a>(
    tree: &'a worth_ui::facade::WorthUiMountedCompositionTreeReceipt,
    parent_id: &str,
    interaction_id: &str,
) -> Option<&'a WorthUiMountedInteractionNodeReceipt> {
    for child in tree.ordered_children(parent_id) {
        if let WorthUiMountedNodeReceipt::Interaction(node) = child.mounted_node() {
            if node.interaction().interaction_id() == interaction_id {
                return Some(node);
            }
        }
        if let Some(node) = find_mounted_interaction(tree, child.node_id(), interaction_id) {
            return Some(node);
        }
    }
    None
}
