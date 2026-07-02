use crate::obligations::catalog::{UiObligationCheckKind, UiObligationFamily};
use crate::obligations::inspection::UiObligationEvidenceHandle;
use crate::obligations::prerequisites::UiObligationPrerequisiteEvidenceRef;

use super::{
    UiObligationSelectionReason, UiObligationSupportSelectionPosture, UiSelectedObligationIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiSelectedObligation {
    identity: UiSelectedObligationIdentity,
    family: UiObligationFamily,
    check_kind: UiObligationCheckKind,
    support_posture: UiObligationSupportSelectionPosture,
    evidence_handle: UiObligationEvidenceHandle,
    selection_reasons: Box<[UiObligationSelectionReason]>,
    prerequisite_evidence_refs: Box<[UiObligationPrerequisiteEvidenceRef]>,
}

impl UiSelectedObligation {
    pub(crate) fn new(
        identity: UiSelectedObligationIdentity,
        family: UiObligationFamily,
        check_kind: UiObligationCheckKind,
        support_posture: UiObligationSupportSelectionPosture,
        evidence_handle: UiObligationEvidenceHandle,
        selection_reasons: Box<[UiObligationSelectionReason]>,
        prerequisite_evidence_refs: Box<[UiObligationPrerequisiteEvidenceRef]>,
    ) -> Self {
        Self {
            identity,
            family,
            check_kind,
            support_posture,
            evidence_handle,
            selection_reasons,
            prerequisite_evidence_refs,
        }
    }

    pub fn identity(&self) -> &UiSelectedObligationIdentity {
        &self.identity
    }

    pub fn family(&self) -> UiObligationFamily {
        self.family
    }

    pub fn check_kind(&self) -> UiObligationCheckKind {
        self.check_kind
    }

    pub fn support_posture(&self) -> UiObligationSupportSelectionPosture {
        self.support_posture
    }

    pub fn evidence_handle(&self) -> UiObligationEvidenceHandle {
        self.evidence_handle
    }

    pub fn selection_reasons(&self) -> &[UiObligationSelectionReason] {
        &self.selection_reasons
    }

    pub fn prerequisite_evidence_refs(&self) -> &[UiObligationPrerequisiteEvidenceRef] {
        &self.prerequisite_evidence_refs
    }
}
