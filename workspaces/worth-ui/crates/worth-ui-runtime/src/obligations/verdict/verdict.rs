use crate::obligations::catalog::{UiObligationCheckKind, UiObligationFamily};
use crate::obligations::inspection::UiObligationEvidenceHandle;
use crate::obligations::prerequisites::UiObligationPrerequisiteEvidenceRef;
use crate::obligations::selection::{UiObligationSelectionReason, UiSelectedObligation};

use super::{UiObligationDispatchStopPosture, UiObligationVerdictClass};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiObligationVerdict {
    family: Option<UiObligationFamily>,
    check_kind: Option<UiObligationCheckKind>,
    selected_identity: Option<crate::obligations::selection::UiSelectedObligationIdentity>,
    class: UiObligationVerdictClass,
    stop_posture: UiObligationDispatchStopPosture,
    evidence_handle: UiObligationEvidenceHandle,
    selection_reasons: Box<[UiObligationSelectionReason]>,
    prerequisite_evidence_refs: Box<[UiObligationPrerequisiteEvidenceRef]>,
}

impl UiObligationVerdict {
    pub(crate) fn from_selected(
        selected: &UiSelectedObligation,
        class: UiObligationVerdictClass,
        stop_posture: UiObligationDispatchStopPosture,
    ) -> Self {
        let evidence_handle = UiObligationEvidenceHandle::new(
            crate::obligations::inspection::UiObligationEvidenceHandleKind::Verdict,
            selected.evidence_handle().digest()
                ^ selected.identity().identity_digest().rotate_left(19),
        );
        Self {
            family: Some(selected.family()),
            check_kind: Some(selected.check_kind()),
            selected_identity: Some(selected.identity().clone()),
            class,
            stop_posture,
            evidence_handle,
            selection_reasons: selected.selection_reasons().to_vec().into_boxed_slice(),
            prerequisite_evidence_refs: selected
                .prerequisite_evidence_refs()
                .to_vec()
                .into_boxed_slice(),
        }
    }

    pub(crate) fn global_stop(
        class: UiObligationVerdictClass,
        stop_posture: UiObligationDispatchStopPosture,
    ) -> Self {
        Self {
            family: None,
            check_kind: None,
            selected_identity: None,
            class,
            stop_posture,
            evidence_handle: UiObligationEvidenceHandle::new(
                crate::obligations::inspection::UiObligationEvidenceHandleKind::Verdict,
                0,
            ),
            selection_reasons: Box::new([]),
            prerequisite_evidence_refs: Box::new([]),
        }
    }

    pub fn family(&self) -> Option<UiObligationFamily> {
        self.family
    }

    pub fn check_kind(&self) -> Option<UiObligationCheckKind> {
        self.check_kind
    }

    pub fn selected_identity(
        &self,
    ) -> Option<&crate::obligations::selection::UiSelectedObligationIdentity> {
        self.selected_identity.as_ref()
    }

    pub fn class(&self) -> UiObligationVerdictClass {
        self.class
    }

    pub fn stop_posture(&self) -> UiObligationDispatchStopPosture {
        self.stop_posture
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
