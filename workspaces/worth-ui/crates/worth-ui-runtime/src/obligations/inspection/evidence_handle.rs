use crate::declaration::stable_text_digest;
use crate::evidence::{
    evidence_handle, evidence_identity, UiEvidenceFamily, UiEvidenceHandle, UiEvidenceIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationEvidenceHandleKind {
    Selected,
    NotSelected,
    Dispatch,
    Verdict,
    Admission,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiObligationEvidenceHandle {
    kind: UiObligationEvidenceHandleKind,
    digest: u64,
}

impl UiObligationEvidenceHandle {
    pub(crate) fn new(kind: UiObligationEvidenceHandleKind, seed: u64) -> Self {
        let kind_digest = match kind {
            UiObligationEvidenceHandleKind::Selected => 1_u64,
            UiObligationEvidenceHandleKind::NotSelected => 2_u64,
            UiObligationEvidenceHandleKind::Dispatch => 3_u64,
            UiObligationEvidenceHandleKind::Verdict => 4_u64,
            UiObligationEvidenceHandleKind::Admission => 5_u64,
        };

        Self {
            kind,
            digest: stable_text_digest("obligation-evidence-handle")
                ^ kind_digest.rotate_left(7)
                ^ seed.rotate_left(13),
        }
    }

    pub fn kind(&self) -> UiObligationEvidenceHandleKind {
        self.kind
    }

    pub fn digest(&self) -> u64 {
        self.digest
    }

    pub(crate) fn public_identity(&self) -> UiEvidenceIdentity {
        evidence_identity(UiEvidenceFamily::Obligation, self.digest)
    }

    pub(crate) fn public_handle(&self) -> UiEvidenceHandle {
        evidence_handle(
            UiEvidenceFamily::Obligation,
            self.public_identity(),
            self.digest,
        )
    }
}
