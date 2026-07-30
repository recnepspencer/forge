use super::{UiDraftSessionIdentity, UiLocalInputRecipientFamily};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiDraftMutationKind {
    CommittedText,
    Backspace,
    Preedit,
    PreeditCommit,
    PreeditCancel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiLocalInputRecipientBindingReceipt {
    session: Option<UiDraftSessionIdentity>,
    family: UiLocalInputRecipientFamily,
    target: crate::runtime::interaction::UiPresentedInteractionTargetView,
    resumed_draft: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiDraftMutationReceipt {
    session: UiDraftSessionIdentity,
    kind: UiDraftMutationKind,
    source_sequence: worth_ui_host_contract::UiHostObservationSequence,
    input_revision: Option<u64>,
    draft_revision: u64,
    committed_utf8_bytes: usize,
    preedit_utf8_bytes: usize,
}

pub(super) struct UiDraftMutationReceiptInput {
    pub(super) session: UiDraftSessionIdentity,
    pub(super) kind: UiDraftMutationKind,
    pub(super) source_sequence: worth_ui_host_contract::UiHostObservationSequence,
    pub(super) input_revision: Option<u64>,
    pub(super) draft_revision: u64,
    pub(super) committed_utf8_bytes: usize,
    pub(super) preedit_utf8_bytes: usize,
}

impl UiLocalInputRecipientBindingReceipt {
    pub(super) const fn new(
        session: Option<UiDraftSessionIdentity>,
        family: UiLocalInputRecipientFamily,
        target: crate::runtime::interaction::UiPresentedInteractionTargetView,
        resumed_draft: bool,
    ) -> Self {
        Self {
            session,
            family,
            target,
            resumed_draft,
        }
    }

    pub const fn session(self) -> Option<UiDraftSessionIdentity> {
        self.session
    }

    pub const fn family(self) -> UiLocalInputRecipientFamily {
        self.family
    }

    pub const fn target(self) -> crate::runtime::interaction::UiPresentedInteractionTargetView {
        self.target
    }

    pub const fn resumed_draft(self) -> bool {
        self.resumed_draft
    }
}

impl UiDraftMutationReceipt {
    pub(super) const fn new(input: UiDraftMutationReceiptInput) -> Self {
        Self {
            session: input.session,
            kind: input.kind,
            source_sequence: input.source_sequence,
            input_revision: input.input_revision,
            draft_revision: input.draft_revision,
            committed_utf8_bytes: input.committed_utf8_bytes,
            preedit_utf8_bytes: input.preedit_utf8_bytes,
        }
    }

    pub const fn session(self) -> UiDraftSessionIdentity {
        self.session
    }

    pub const fn kind(self) -> UiDraftMutationKind {
        self.kind
    }

    pub const fn source_sequence(self) -> worth_ui_host_contract::UiHostObservationSequence {
        self.source_sequence
    }

    pub const fn input_revision(self) -> Option<u64> {
        self.input_revision
    }

    pub const fn draft_revision(self) -> u64 {
        self.draft_revision
    }

    pub const fn committed_utf8_bytes(self) -> usize {
        self.committed_utf8_bytes
    }

    pub const fn preedit_utf8_bytes(self) -> usize {
        self.preedit_utf8_bytes
    }
}
