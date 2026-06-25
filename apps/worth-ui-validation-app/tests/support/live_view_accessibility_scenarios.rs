pub fn source_with_title_label_and_helper() -> String {
    source_with_title_label_in(CardLabelPlacement::BeforeTitleInput, accessibility_block())
}

pub fn source_with_moved_title_label() -> String {
    source_with_title_label_in(CardLabelPlacement::ActionRow, accessibility_block())
}

pub fn source_with_invalid_accessibility_associations() -> String {
    source_with_title_label_in(
        CardLabelPlacement::BeforeTitleInput,
        r#"accessibility {
                label missing_label -> title_input
                description details_helper -> missing_details_input
                error input_stack -> details_input
            }"#,
    )
}

fn source_with_title_label_in(
    title_label_placement: CardLabelPlacement,
    accessibility: &str,
) -> String {
    format!(
        r##"live_view validation.live_view.primitive_proof {{
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
    state title {{
        fact validation.state.form.title
        kind text
        access read_write
    }}
    state details {{
        fact validation.state.form.details
        kind text
        access read_write
    }}
    control title_input {{
        binding title
        projection text_input
        label "Title"
        flow_padding validation.density.primitive.flow.padding.compact
        appearance_rest_background "#ffffff"
        appearance_rest_foreground "#111827"
        appearance_rest_text_color "#111827"
        appearance_rest_border_color "#d0d5dd"
        appearance_rest_border_width validation.density.primitive.border.default
        appearance_rest_radius validation.density.primitive.radius
        event_cursor text
    }}
    control details_input {{
        binding details
        projection text_input
        label "Details"
        flow_padding validation.density.primitive.flow.padding.compact
        appearance_rest_background "#ffffff"
        appearance_rest_foreground "#111827"
        appearance_rest_text_color "#111827"
        appearance_rest_border_color "#d0d5dd"
        appearance_rest_border_width validation.density.primitive.border.default
        appearance_rest_radius validation.density.primitive.radius
        event_cursor text
    }}
    readiness proof_submit_ready {{
        required title,details
    }}
    payload proof_submit_payload {{
        shape data_payload_values
    }}
    interaction proof_submit {{
        kind submit
        effect validation.effect.form.submit
        readiness proof_submit_ready
        payload proof_submit_payload
        label "Submit"
        flow_padding validation.density.primitive.flow.padding.compact
        appearance_rest_background "#2563eb"
        appearance_rest_foreground "#ffffff"
        appearance_rest_text_color "#ffffff"
        appearance_rest_border_color "#2563eb"
        appearance_rest_border_width validation.density.primitive.border.default
        appearance_rest_radius validation.density.primitive.radius
        appearance_disabled_background "#eaecf0"
        appearance_disabled_foreground "#667085"
        appearance_disabled_text_color "#667085"
        appearance_disabled_border_color "#eaecf0"
        event_cursor pointer
    }}
    composition validation.live_view.primitive_proof {{
        root page_content_slot button_proof
        surface live_view.form_card {{
            policy local_layout validation.flow.form.card
            container input_stack {{
                policy local_layout validation.flow.form.inputs
                {title_label_before_input}
                child control title_input sizing fill(1)
                content details_helper {{
                    content_kind plain
                    content_order "text"
                    content_text "Tell us what changed"
                    content_role helper_text
                    content_text_size validation.density.primitive.content.text.default
                }}
                content details_error {{
                    content_kind plain
                    content_order "text"
                    content_text "Details are required"
                    content_role error_text
                    content_text_size validation.density.primitive.content.text.default
                }}
                child control details_input sizing fill(1)
            }}
            container action_row {{
                policy local_layout validation.flow.form.actions
                {title_label_in_action_row}
                child interaction proof_submit sizing hug
            }}
            {accessibility}
            child diagnostic_panel live_view.evidence
        }}
    }}
}}"##,
        title_label_before_input = title_label_placement.before_input_source(),
        title_label_in_action_row = title_label_placement.action_row_source(),
    )
}

fn accessibility_block() -> &'static str {
    r#"accessibility {
                label title_label -> title_input
                description details_helper -> details_input
                error details_error -> details_input
            }"#
}

enum CardLabelPlacement {
    BeforeTitleInput,
    ActionRow,
}

impl CardLabelPlacement {
    fn before_input_source(&self) -> &'static str {
        match self {
            Self::BeforeTitleInput => title_label_source(),
            Self::ActionRow => "",
        }
    }

    fn action_row_source(&self) -> &'static str {
        match self {
            Self::BeforeTitleInput => "",
            Self::ActionRow => title_label_source(),
        }
    }
}

fn title_label_source() -> &'static str {
    r#"content title_label {
                    content_kind plain
                    content_order "text"
                    content_text "Title"
                    content_role label
                    content_text_size validation.density.primitive.content.text.default
                }"#
}
