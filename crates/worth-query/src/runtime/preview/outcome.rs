use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPreviewDiff {
    pub(super) session_label: WorthQuerySessionLabel,
    pub(super) write_count: usize,
    pub(super) changed_entity_count: usize,
}

impl WorthQueryPreviewDiff {
    pub fn label(&self) -> &str {
        self.session_label.display()
    }

    pub fn session_label(&self) -> &WorthQuerySessionLabel {
        &self.session_label
    }

    pub fn write_count(&self) -> usize {
        self.write_count
    }

    pub fn changed_entity_count(&self) -> usize {
        self.changed_entity_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPreviewOutcome {
    pub(super) session_label: WorthQuerySessionLabel,
    pub(super) effect_policy: WorthQueryEffectPolicy,
    pub(super) promoted: bool,
    pub(super) discarded: bool,
    pub(super) write_count: usize,
    pub(super) preview_binding_count: usize,
    pub(super) effect_binding_count: usize,
    pub(super) effect_delivery_residue_count: usize,
    pub(super) pending_write_intent_residue_count: usize,
    pub(super) authoritative_residue_count: usize,
    pub(super) closeout_evidence: WorthQueryPreviewCloseoutEvidence,
    pub(super) source_lane: WorthQueryAuthorityLane,
    pub(super) target_lane: WorthQueryAuthorityLane,
}

impl WorthQueryPreviewOutcome {
    pub fn label(&self) -> &str {
        self.session_label.display()
    }

    pub fn session_label(&self) -> &WorthQuerySessionLabel {
        &self.session_label
    }

    pub fn effect_policy(&self) -> WorthQueryEffectPolicy {
        self.effect_policy
    }

    pub fn promoted(&self) -> bool {
        self.promoted
    }

    pub fn discarded(&self) -> bool {
        self.discarded
    }

    pub fn write_count(&self) -> usize {
        self.write_count
    }

    pub fn preview_binding_count(&self) -> usize {
        self.preview_binding_count
    }

    pub fn effect_binding_count(&self) -> usize {
        self.effect_binding_count
    }

    pub fn effect_delivery_residue_count(&self) -> usize {
        self.effect_delivery_residue_count
    }

    pub fn pending_write_intent_residue_count(&self) -> usize {
        self.pending_write_intent_residue_count
    }

    pub fn authoritative_residue_count(&self) -> usize {
        self.authoritative_residue_count
    }

    pub fn closeout_evidence(&self) -> &WorthQueryPreviewCloseoutEvidence {
        &self.closeout_evidence
    }

    pub fn source_lane(&self) -> WorthQueryAuthorityLane {
        self.source_lane
    }

    pub fn target_lane(&self) -> WorthQueryAuthorityLane {
        self.target_lane
    }
}
