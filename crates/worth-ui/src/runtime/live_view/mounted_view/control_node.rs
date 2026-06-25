use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    WorthUiLiveViewCompositionChildBindingReceipt, WorthUiLiveViewControlEditabilityPosture,
    WorthUiLiveViewControlHostFrameKind, WorthUiLiveViewControlHostFrameReceipt,
    WorthUiLiveViewControlHostFrameStyleReceipt, WorthUiLiveViewControlOptionReceipt,
    WorthUiLiveViewParticipationReceipt, WorthUiLiveViewStateBindingReceipt, WorthUiRuntimeFactId,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedControlNodeReceipt {
    composition_child_binding: WorthUiLiveViewCompositionChildBindingReceipt,
    state_binding: WorthUiLiveViewStateBindingReceipt,
    host_frame: WorthUiLiveViewControlHostFrameReceipt,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    receipt_digest: u64,
}

impl WorthUiMountedControlNodeReceipt {
    pub(super) fn from_parts(
        composition_child_binding: WorthUiLiveViewCompositionChildBindingReceipt,
        state_binding: WorthUiLiveViewStateBindingReceipt,
        host_frame: WorthUiLiveViewControlHostFrameReceipt,
    ) -> Self {
        let mut consumed_facts = host_frame.consumed_facts().to_vec();
        consumed_facts.push(WorthUiRuntimeFactId::live_view_state_binding(format!(
            "{}:{}",
            state_binding.live_view_id(),
            state_binding.binding_id()
        )));
        consumed_facts.extend(composition_child_binding.consumed_facts().iter().cloned());
        consumed_facts.sort();
        consumed_facts.dedup();
        let receipt_digest = digest_parts(
            [
                "mounted_control_node".to_owned(),
                composition_child_binding.binding_digest().to_string(),
                state_binding.binding_digest().to_string(),
                host_frame.frame_digest().to_string(),
            ]
            .into_iter()
            .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned())),
        );
        Self {
            composition_child_binding,
            state_binding,
            host_frame,
            consumed_facts,
            receipt_digest,
        }
    }

    pub fn composition_child_binding(&self) -> &WorthUiLiveViewCompositionChildBindingReceipt {
        &self.composition_child_binding
    }

    pub fn host_frame(&self) -> &WorthUiLiveViewControlHostFrameReceipt {
        &self.host_frame
    }

    pub fn state_binding(&self) -> &WorthUiLiveViewStateBindingReceipt {
        &self.state_binding
    }

    pub fn control_id(&self) -> &str {
        self.host_frame.control_id()
    }

    pub fn label(&self) -> &str {
        self.host_frame.label()
    }

    pub fn kind(&self) -> WorthUiLiveViewControlHostFrameKind {
        self.host_frame.kind()
    }

    pub fn value_text(&self) -> &str {
        self.host_frame.value_text()
    }

    pub fn options(&self) -> &[WorthUiLiveViewControlOptionReceipt] {
        self.host_frame.options()
    }

    pub fn editability(&self) -> WorthUiLiveViewControlEditabilityPosture {
        self.host_frame.editability()
    }

    pub fn participation(&self) -> Option<&WorthUiLiveViewParticipationReceipt> {
        self.host_frame.participation()
    }

    pub fn style(&self) -> &WorthUiLiveViewControlHostFrameStyleReceipt {
        self.host_frame.style()
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn frame_digest(&self) -> u64 {
        self.host_frame.frame_digest()
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}
