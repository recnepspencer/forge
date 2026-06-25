use std::path::{Path, PathBuf};

pub const VALIDATION_SAMPLE_LIVE_VIEW_SOURCE: &str = r##"live_view validation.live_view.primitive_proof {
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
    state title {
        fact validation.state.form.title
        kind text
        access read_write
    }
    state details {
        fact validation.state.form.details
        kind text
        access read_write
    }
    control title_input {
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
    }
    control details_input {
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
    }
    readiness proof_submit_ready {
        required title,details
    }
    payload proof_submit_payload {
        shape data_payload_values
    }
    interaction proof_submit {
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
    }
    composition validation.live_view.primitive_proof {
        root page_content_slot button_proof
        surface live_view.form_card {
            policy local_layout validation.flow.form.card
            container input_stack {
                policy local_layout validation.flow.form.inputs
                child control title_input sizing fill(1)
                child control details_input sizing fill(1)
            }
            container action_row {
                policy local_layout validation.flow.form.actions
                child interaction proof_submit sizing hug
            }
            child diagnostic_panel live_view.evidence
        }
    }
}"##;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationLiveViewSource {
    source_path: PathBuf,
    source_text: String,
    source_digest: u64,
}

impl ValidationLiveViewSource {
    pub fn new(source_text: impl Into<String>) -> Self {
        Self::from_observed_file(
            "apps/worth-ui-validation-app/source/live_view.worth",
            source_text,
        )
    }

    pub fn sample() -> Self {
        Self::new(VALIDATION_SAMPLE_LIVE_VIEW_SOURCE)
    }

    pub fn from_observed_file(
        source_path: impl Into<PathBuf>,
        source_text: impl Into<String>,
    ) -> Self {
        let source_text = source_text.into();
        Self {
            source_path: source_path.into(),
            source_digest: fold_bytes(0x6c69_7665_7669_6577, source_text.as_bytes()),
            source_text,
        }
    }

    pub fn source_path(&self) -> &Path {
        &self.source_path
    }

    pub fn source_text(&self) -> &str {
        &self.source_text
    }

    pub fn source_digest(&self) -> u64 {
        self.source_digest
    }
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
