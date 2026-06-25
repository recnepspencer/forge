mod style;

pub use style::{
    WorthUiLiveViewControlHostFrameStyleReceipt, WorthUiLiveViewControlHostFrameWidthPolicy,
};

use crate::capability::ComponentId;
use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    WorthUiLiveViewControlOptionReceipt, WorthUiLiveViewControlProjectionKind,
    WorthUiLiveViewControlProjectionReceipt, WorthUiLiveViewParticipationReceipt,
    WorthUiLiveViewStateAccess, WorthUiLiveViewStateValue, WorthUiRuntimeFactId,
    WorthUiRuntimeHost,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewControlPrimitiveSubjectReceipt {
    live_view_id: String,
    control_id: String,
    component_id: ComponentId,
    subject_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewControlHostFrameKind {
    TextInput,
    DropdownInput,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewControlEditabilityPosture {
    Editable,
    ReadOnly,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiLiveViewControlHostFrameReceipt {
    subject: WorthUiLiveViewControlPrimitiveSubjectReceipt,
    kind: WorthUiLiveViewControlHostFrameKind,
    control_id: String,
    label: String,
    value_text: String,
    options: Vec<WorthUiLiveViewControlOptionReceipt>,
    editability: WorthUiLiveViewControlEditabilityPosture,
    participation: Option<WorthUiLiveViewParticipationReceipt>,
    style: WorthUiLiveViewControlHostFrameStyleReceipt,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    frame_digest: u64,
}

impl WorthUiRuntimeHost {
    pub(crate) fn resolve_live_view_control_host_frame_from_parts(
        &self,
        control: WorthUiLiveViewControlProjectionReceipt,
        participation: Option<WorthUiLiveViewParticipationReceipt>,
    ) -> WorthUiLiveViewControlHostFrameReceipt {
        WorthUiLiveViewControlHostFrameReceipt::from_render_control(self, control, participation)
    }
}

impl WorthUiLiveViewControlPrimitiveSubjectReceipt {
    fn new(live_view_id: &str, control_id: &str, component_id: ComponentId) -> Self {
        let subject_digest = digest_parts([
            "live_view_control_primitive_subject",
            live_view_id,
            control_id,
            component_id.as_str(),
        ]);
        Self {
            live_view_id: live_view_id.to_owned(),
            control_id: control_id.to_owned(),
            component_id,
            subject_digest,
        }
    }

    pub fn live_view_id(&self) -> &str {
        &self.live_view_id
    }

    pub fn control_id(&self) -> &str {
        &self.control_id
    }

    pub fn component_id(&self) -> &ComponentId {
        &self.component_id
    }

    pub fn subject_digest(&self) -> u64 {
        self.subject_digest
    }
}

impl WorthUiLiveViewControlHostFrameReceipt {
    fn from_render_control(
        runtime: &WorthUiRuntimeHost,
        control: WorthUiLiveViewControlProjectionReceipt,
        participation: Option<WorthUiLiveViewParticipationReceipt>,
    ) -> Self {
        let binding = control.binding();
        let subject = WorthUiLiveViewControlPrimitiveSubjectReceipt::new(
            control.live_view_id(),
            control.control_id(),
            control.component_id().clone(),
        );
        let kind = match control.kind() {
            WorthUiLiveViewControlProjectionKind::TextInput => {
                WorthUiLiveViewControlHostFrameKind::TextInput
            }
            WorthUiLiveViewControlProjectionKind::Select => {
                WorthUiLiveViewControlHostFrameKind::DropdownInput
            }
            WorthUiLiveViewControlProjectionKind::Unsupported(_) => {
                WorthUiLiveViewControlHostFrameKind::TextInput
            }
        };
        let value_text = runtime
            .live_view_state_value(binding)
            .map(WorthUiLiveViewStateValue::as_display_text)
            .unwrap_or_default();
        let options = control
            .options()
            .map(|options| options.options().to_vec())
            .unwrap_or_default();
        let editability = editability_from_access(binding.access());
        let style = WorthUiLiveViewControlHostFrameStyleReceipt::from_receipts(&control);
        let mut consumed_facts = vec![
            WorthUiRuntimeFactId::live_view_control_projection(format!(
                "{}:{}",
                control.live_view_id(),
                control.control_id()
            )),
            WorthUiRuntimeFactId::live_view_state_binding(format!(
                "{}:{}",
                binding.live_view_id(),
                binding.binding_id()
            )),
            WorthUiRuntimeFactId::live_view_state_value(binding.state_fact().as_str()),
            WorthUiRuntimeFactId::primitive_flow_layout(subject_identity(&subject)),
            WorthUiRuntimeFactId::primitive_appearance_state(subject_identity(&subject)),
            WorthUiRuntimeFactId::primitive_event_geometry(subject_identity(&subject)),
        ];
        if participation.is_some() {
            consumed_facts.push(WorthUiRuntimeFactId::live_view_participation(format!(
                "{}:{}",
                control.live_view_id(),
                control.control_id()
            )));
        }
        let frame_digest = digest_parts(
            [
                subject.subject_digest().to_string(),
                kind.token().to_owned(),
                control.control_id().to_owned(),
                control.label().to_owned(),
                value_text.clone(),
                editability.token().to_owned(),
                participation.as_ref().map_or_else(
                    || "participation:none".to_owned(),
                    |receipt| receipt.participation_digest().to_string(),
                ),
                style.style_digest().to_string(),
            ]
            .into_iter()
            .chain(
                options
                    .iter()
                    .flat_map(|option| [option.value().to_owned(), option.label().to_owned()]),
            )
            .chain(
                consumed_facts
                    .iter()
                    .map(|fact| format!("{:?}:{}", fact.family(), fact.identity())),
            ),
        );
        Self {
            subject,
            kind,
            control_id: control.control_id().to_owned(),
            label: control.label().to_owned(),
            value_text,
            options,
            editability,
            participation,
            style,
            consumed_facts,
            frame_digest,
        }
    }

    pub fn subject(&self) -> &WorthUiLiveViewControlPrimitiveSubjectReceipt {
        &self.subject
    }

    pub fn kind(&self) -> WorthUiLiveViewControlHostFrameKind {
        self.kind
    }

    pub fn control_id(&self) -> &str {
        &self.control_id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn value_text(&self) -> &str {
        &self.value_text
    }

    pub fn options(&self) -> &[WorthUiLiveViewControlOptionReceipt] {
        &self.options
    }

    pub fn editability(&self) -> WorthUiLiveViewControlEditabilityPosture {
        self.editability
    }

    pub fn participation(&self) -> Option<&WorthUiLiveViewParticipationReceipt> {
        self.participation.as_ref()
    }

    pub fn style(&self) -> &WorthUiLiveViewControlHostFrameStyleReceipt {
        &self.style
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn frame_digest(&self) -> u64 {
        self.frame_digest
    }
}

impl WorthUiLiveViewControlHostFrameKind {
    pub fn token(self) -> &'static str {
        match self {
            Self::TextInput => "text_input",
            Self::DropdownInput => "dropdown_input",
        }
    }
}

impl WorthUiLiveViewControlEditabilityPosture {
    pub fn token(self) -> &'static str {
        match self {
            Self::Editable => "editable",
            Self::ReadOnly => "read_only",
        }
    }

    pub fn is_editable(self) -> bool {
        matches!(self, Self::Editable)
    }
}

fn editability_from_access(
    access: WorthUiLiveViewStateAccess,
) -> WorthUiLiveViewControlEditabilityPosture {
    match access {
        WorthUiLiveViewStateAccess::ReadWrite => WorthUiLiveViewControlEditabilityPosture::Editable,
        WorthUiLiveViewStateAccess::ReadOnly => WorthUiLiveViewControlEditabilityPosture::ReadOnly,
    }
}

fn subject_identity(subject: &WorthUiLiveViewControlPrimitiveSubjectReceipt) -> String {
    format!("{}:{}", subject.live_view_id(), subject.control_id())
}
