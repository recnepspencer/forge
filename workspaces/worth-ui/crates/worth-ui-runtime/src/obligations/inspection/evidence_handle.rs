use crate::declaration::stable_text_digest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationEvidenceHandleKind {
    Selected,
    NotSelected,
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
            UiObligationEvidenceHandleKind::Verdict => 3_u64,
            UiObligationEvidenceHandleKind::Admission => 4_u64,
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
}
