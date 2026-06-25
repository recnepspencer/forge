#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveViewProductScenario {
    contact_mode_projection: ControlProjectionScenario,
    input_stack_children: Vec<CompositionChildScenario>,
    action_row_children: Vec<CompositionChildScenario>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ControlProjectionScenario {
    TextInput,
    YesNoSelect,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompositionChildScenario {
    Control {
        control_id: &'static str,
        sizing: &'static str,
    },
    Interaction {
        interaction_id: &'static str,
        sizing: &'static str,
    },
}

impl LiveViewProductScenario {
    pub fn contact_form() -> Self {
        Self {
            contact_mode_projection: ControlProjectionScenario::TextInput,
            input_stack_children: vec![
                CompositionChildScenario::control("first_name_input", "fill(1)"),
                CompositionChildScenario::control("contact_mode_input", "fill(1)"),
            ],
            action_row_children: vec![CompositionChildScenario::interaction(
                "contact_submit",
                "hug",
            )],
        }
    }

    pub fn with_contact_mode_select(mut self) -> Self {
        self.contact_mode_projection = ControlProjectionScenario::YesNoSelect;
        self
    }

    pub fn move_child(mut self, subject_id: &'static str, target_container_id: &str) -> Self {
        let child = self
            .remove_child_from_containers(subject_id)
            .unwrap_or_else(|| panic!("scenario child '{subject_id}' must exist before move"));
        self.children_mut(target_container_id).push(child);
        self
    }

    pub fn remove_child(mut self, subject_id: &'static str) -> Self {
        self.remove_child_from_containers(subject_id)
            .unwrap_or_else(|| panic!("scenario child '{subject_id}' must exist before removal"));
        self
    }

    pub fn to_source(&self) -> String {
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
    state first_name {{
        fact validation.state.contact.first_name
        kind text
        access read_write
    }}
    control first_name_input {{
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
    }}
    state contact_mode {{
        fact validation.state.contact.mode
        kind text
        access read_write
    }}
    control contact_mode_input {{
        binding contact_mode
{}
        label "Contact mode"
        flow_padding validation.density.primitive.flow.padding.compact
        appearance_rest_background "#111827"
        appearance_rest_foreground "#f9fafb"
        appearance_rest_text_color "#f9fafb"
        appearance_rest_border_color "#93c5fd"
        appearance_rest_border_width validation.density.primitive.border.default
        appearance_rest_radius validation.density.primitive.radius
        event_cursor text
    }}
    readiness contact_submit_ready {{
        required first_name,contact_mode
    }}
    payload contact_submit_payload {{
        shape data_payload_values
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
{}
            }}
            container action_row {{
                policy local_layout validation.flow.form.actions
{}
            }}
            child diagnostic_panel live_view.evidence
        }}
    }}
}}"##,
            self.contact_mode_projection.to_source(),
            children_to_source(&self.input_stack_children),
            children_to_source(&self.action_row_children)
        )
    }

    fn children_mut(&mut self, container_id: &str) -> &mut Vec<CompositionChildScenario> {
        match container_id {
            "input_stack" => &mut self.input_stack_children,
            "action_row" => &mut self.action_row_children,
            _ => panic!("unknown scenario container '{container_id}'"),
        }
    }

    fn remove_child_from_containers(
        &mut self,
        subject_id: &str,
    ) -> Option<CompositionChildScenario> {
        remove_child(&mut self.input_stack_children, subject_id)
            .or_else(|| remove_child(&mut self.action_row_children, subject_id))
    }
}

impl CompositionChildScenario {
    pub fn control(control_id: &'static str, sizing: &'static str) -> Self {
        Self::Control { control_id, sizing }
    }

    pub fn interaction(interaction_id: &'static str, sizing: &'static str) -> Self {
        Self::Interaction {
            interaction_id,
            sizing,
        }
    }

    fn subject_id(&self) -> &str {
        match self {
            Self::Control { control_id, .. } => control_id,
            Self::Interaction { interaction_id, .. } => interaction_id,
        }
    }

    fn to_source(&self) -> String {
        match self {
            Self::Control { control_id, sizing } => {
                format!("                child control {control_id} sizing {sizing}")
            }
            Self::Interaction {
                interaction_id,
                sizing,
            } => {
                format!("                child interaction {interaction_id} sizing {sizing}")
            }
        }
    }
}

impl ControlProjectionScenario {
    fn to_source(&self) -> &'static str {
        match self {
            Self::TextInput => "        projection text_input",
            Self::YesNoSelect => "        projection select\n        options yes:Yes,no:No",
        }
    }
}

fn children_to_source(children: &[CompositionChildScenario]) -> String {
    children
        .iter()
        .map(CompositionChildScenario::to_source)
        .collect::<Vec<_>>()
        .join("\n")
}

fn remove_child(
    children: &mut Vec<CompositionChildScenario>,
    subject_id: &str,
) -> Option<CompositionChildScenario> {
    let index = children
        .iter()
        .position(|child| child.subject_id() == subject_id)?;
    Some(children.remove(index))
}
