use crate::declaration::stable_text_digest;
use crate::obligations::catalog::{UiObligationCheckKind, UiObligationFamily};
use crate::obligations::inspection::UiObligationEvidenceHandle;
use crate::obligations::prerequisites::UiObligationPrerequisiteEvidenceRef;
use crate::obligations::selection::{UiObligationSelectionReason, UiSelectedObligation};

use super::{UiObligationDispatchStopPosture, UiObligationVerdictClass};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiObligationVerdict {
    identity_digest: u64,
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
        let identity_digest =
            obligation_verdict_identity_digest(Some(selected), None, class, stop_posture);
        Self {
            identity_digest,
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
        dispatch_shape_digest: u64,
        class: UiObligationVerdictClass,
        stop_posture: UiObligationDispatchStopPosture,
    ) -> Self {
        let identity_digest = obligation_verdict_identity_digest(
            None,
            Some(dispatch_shape_digest),
            class,
            stop_posture,
        );
        Self {
            identity_digest,
            family: None,
            check_kind: None,
            selected_identity: None,
            class,
            stop_posture,
            evidence_handle: UiObligationEvidenceHandle::new(
                crate::obligations::inspection::UiObligationEvidenceHandleKind::Verdict,
                dispatch_shape_digest ^ identity_digest.rotate_left(19),
            ),
            selection_reasons: Box::new([]),
            prerequisite_evidence_refs: Box::new([]),
        }
    }

    pub fn family(&self) -> Option<UiObligationFamily> {
        self.family
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        self.identity_digest
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

fn obligation_verdict_identity_digest(
    selected: Option<&UiSelectedObligation>,
    dispatch_shape_digest: Option<u64>,
    class: UiObligationVerdictClass,
    stop_posture: UiObligationDispatchStopPosture,
) -> u64 {
    let selected_digest = selected
        .map(|entry| entry.identity().identity_digest())
        .unwrap_or(0);
    let dispatch_digest = dispatch_shape_digest.unwrap_or(0);
    stable_text_digest("obligation-verdict")
        ^ selected_digest.rotate_left(7)
        ^ dispatch_digest.rotate_left(13)
        ^ stable_text_digest(&format!("{class:?}")).rotate_left(17)
        ^ stable_text_digest(&format!("{stop_posture:?}")).rotate_left(29)
}

#[cfg(test)]
mod tests {
    use super::{UiObligationDispatchStopPosture, UiObligationVerdict, UiObligationVerdictClass};

    #[test]
    fn global_stop_verdicts_bind_identity_and_handle_to_dispatch_artifact() {
        let left = UiObligationVerdict::global_stop(
            11,
            UiObligationVerdictClass::Violation,
            UiObligationDispatchStopPosture::Unsupported,
        );
        let right = UiObligationVerdict::global_stop(
            29,
            UiObligationVerdictClass::Violation,
            UiObligationDispatchStopPosture::Unsupported,
        );

        assert_ne!(left.identity_digest(), right.identity_digest());
        assert_ne!(left.evidence_handle(), right.evidence_handle());
    }
}
