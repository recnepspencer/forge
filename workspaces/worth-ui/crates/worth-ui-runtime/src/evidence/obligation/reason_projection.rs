use worth_ui_inspection::{
    UiInspectionEvidenceSource, UiInspectionObligationDecision,
    UiInspectionObligationDenialPosture, UiInspectionObligationDispatchPosture,
    UiInspectionObligationFamily, UiInspectionObligationLegalityReason,
    UiInspectionObligationNonSelectionReason, UiInspectionObligationSelectionReason,
    UiInspectionObligationVerdictClass, UiInspectionObligationVerdictPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiInspectionObligationReasonProjection {
    handle_digest: u64,
    graph_node_digest: u64,
    touch_identity_digest: Option<u64>,
    family: Option<UiInspectionObligationFamily>,
    decision: UiInspectionObligationDecision,
    dispatch_posture: Option<UiInspectionObligationDispatchPosture>,
    verdict_class: Option<UiInspectionObligationVerdictClass>,
    verdict_posture: Option<UiInspectionObligationVerdictPosture>,
    denial_posture: Option<UiInspectionObligationDenialPosture>,
    selection_reasons: Box<[UiInspectionObligationSelectionReason]>,
    prerequisite_sources: Box<[UiInspectionEvidenceSource]>,
    non_selection_reason: Option<UiInspectionObligationNonSelectionReason>,
    legality_reason: Option<UiInspectionObligationLegalityReason>,
}

impl UiInspectionObligationReasonProjection {
    pub(crate) fn new(
        handle_digest: u64,
        graph_node_digest: u64,
        touch_identity_digest: Option<u64>,
        family: Option<UiInspectionObligationFamily>,
        decision: UiInspectionObligationDecision,
        dispatch_posture: Option<UiInspectionObligationDispatchPosture>,
        verdict_class: Option<UiInspectionObligationVerdictClass>,
        verdict_posture: Option<UiInspectionObligationVerdictPosture>,
        denial_posture: Option<UiInspectionObligationDenialPosture>,
        selection_reasons: Box<[UiInspectionObligationSelectionReason]>,
        prerequisite_sources: Box<[UiInspectionEvidenceSource]>,
        non_selection_reason: Option<UiInspectionObligationNonSelectionReason>,
        legality_reason: Option<UiInspectionObligationLegalityReason>,
    ) -> Self {
        Self {
            handle_digest,
            graph_node_digest,
            touch_identity_digest,
            family,
            decision,
            dispatch_posture,
            verdict_class,
            verdict_posture,
            denial_posture,
            selection_reasons,
            prerequisite_sources,
            non_selection_reason,
            legality_reason,
        }
    }

    pub fn handle_digest(&self) -> u64 {
        self.handle_digest
    }

    pub fn graph_node_digest(&self) -> u64 {
        self.graph_node_digest
    }

    pub fn touch_identity_digest(&self) -> Option<u64> {
        self.touch_identity_digest
    }

    pub fn family(&self) -> Option<UiInspectionObligationFamily> {
        self.family
    }

    pub fn decision(&self) -> UiInspectionObligationDecision {
        self.decision
    }

    pub fn dispatch_posture(&self) -> Option<UiInspectionObligationDispatchPosture> {
        self.dispatch_posture
    }

    pub fn verdict_class(&self) -> Option<UiInspectionObligationVerdictClass> {
        self.verdict_class
    }

    pub fn verdict_posture(&self) -> Option<UiInspectionObligationVerdictPosture> {
        self.verdict_posture
    }

    pub fn denial_posture(&self) -> Option<UiInspectionObligationDenialPosture> {
        self.denial_posture
    }

    pub fn selection_reasons(&self) -> &[UiInspectionObligationSelectionReason] {
        &self.selection_reasons
    }

    pub fn prerequisite_sources(&self) -> &[UiInspectionEvidenceSource] {
        &self.prerequisite_sources
    }

    pub fn non_selection_reason(&self) -> Option<UiInspectionObligationNonSelectionReason> {
        self.non_selection_reason
    }

    pub fn legality_reason(&self) -> Option<UiInspectionObligationLegalityReason> {
        self.legality_reason
    }
}
