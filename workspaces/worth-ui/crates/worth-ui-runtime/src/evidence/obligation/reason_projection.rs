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

pub(crate) struct UiInspectionObligationReasonProjectionInput {
    pub(crate) handle_digest: u64,
    pub(crate) graph_node_digest: u64,
    pub(crate) touch_identity_digest: Option<u64>,
    pub(crate) family: Option<UiInspectionObligationFamily>,
    pub(crate) decision: UiInspectionObligationDecision,
    pub(crate) dispatch_posture: Option<UiInspectionObligationDispatchPosture>,
    pub(crate) verdict_class: Option<UiInspectionObligationVerdictClass>,
    pub(crate) verdict_posture: Option<UiInspectionObligationVerdictPosture>,
    pub(crate) denial_posture: Option<UiInspectionObligationDenialPosture>,
    pub(crate) selection_reasons: Box<[UiInspectionObligationSelectionReason]>,
    pub(crate) prerequisite_sources: Box<[UiInspectionEvidenceSource]>,
    pub(crate) non_selection_reason: Option<UiInspectionObligationNonSelectionReason>,
    pub(crate) legality_reason: Option<UiInspectionObligationLegalityReason>,
}

impl UiInspectionObligationReasonProjection {
    pub(crate) fn new(input: UiInspectionObligationReasonProjectionInput) -> Self {
        let UiInspectionObligationReasonProjectionInput {
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
        } = input;
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
