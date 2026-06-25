pub fn contact_form_source() -> String {
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
    state contact_mode {
        fact validation.state.contact.mode
        kind text
        access read_write
    }
    control contact_mode_input {
        binding contact_mode
        projection text_input
        label "Contact mode"
        flow_padding validation.density.primitive.flow.padding.compact
        appearance_rest_background "#111827"
        appearance_rest_foreground "#f9fafb"
        appearance_rest_text_color "#f9fafb"
        appearance_rest_border_color "#93c5fd"
        appearance_rest_border_width validation.density.primitive.border.default
        appearance_rest_radius validation.density.primitive.radius
        event_cursor text
    }
    readiness contact_submit_ready {
        required first_name,contact_mode
    }
    payload contact_submit_payload {
        shape data_payload_values
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

#[allow(dead_code)]
pub fn contact_form_source_with_action_before_inputs() -> String {
    contact_form_source()
        .replace(
            "            container input_stack",
            "            container action_row {
                policy local_layout validation.flow.form.actions
                child interaction contact_submit sizing hug
            }
            container input_stack",
        )
        .replace(
            "            container action_row {
                policy local_layout validation.flow.form.actions
                child interaction contact_submit sizing hug
            }
            child diagnostic_panel",
            "            child diagnostic_panel",
        )
}
