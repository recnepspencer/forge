use std::collections::BTreeMap;

use crate::runtime::WorthUiActiveApplicationGenerationIdentity;

use super::{
    UiDraftByteBudget, UiDraftFieldIdentity, UiDraftMutationReceipt, UiDraftSessionIdentity,
    UiLocalInputStop,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiDraftLifecycleCounters {
    pub(crate) recipients_bound: u64,
    pub(crate) sessions_started: u64,
    pub(crate) sessions_settled: u64,
    pub(crate) mutations: u64,
    pub(crate) stop_outcomes: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiDraftStateSnapshot {
    pub(crate) active_recipients: usize,
    pub(crate) active_sessions: usize,
    pub(crate) retained_utf8_bytes: usize,
    pub(crate) counters: UiDraftLifecycleCounters,
}

pub(crate) enum UiDraftProcessingOutcome {
    Mutation(UiDraftMutationReceipt),
    Semantic(crate::runtime::interaction::UiSemanticInteraction),
    Stopped(UiLocalInputStop),
}

pub(crate) struct UiDraftRuntimeState {
    pub(super) next_identity: Option<u64>,
    pub(super) sessions: BTreeMap<UiDraftSessionIdentity, UiDraftSession>,
    pub(super) active: Option<UiActiveLocalRecipient>,
    pub(super) counters: UiDraftLifecycleCounters,
}

pub(super) struct UiDraftSession {
    pub(super) target: crate::runtime::interaction::UiPresentedInteractionTargetView,
    pub(super) generation: WorthUiActiveApplicationGenerationIdentity,
    pub(super) field: UiDraftFieldIdentity,
    pub(super) budget: UiDraftByteBudget,
    pub(super) committed: String,
    pub(super) preedit: Option<worth_ui_host_contract::UiHostImePreedit>,
    pub(super) last_input_revision: Option<u64>,
    pub(super) draft_revision: u64,
}

pub(super) enum UiActiveLocalRecipient {
    Activation(UiRecipientContext),
    Draft(UiDraftSessionIdentity),
    Submit(UiRecipientContext),
}

#[derive(Clone)]
pub(super) struct UiRecipientContext {
    pub(super) target: crate::runtime::interaction::UiPresentedInteractionTargetView,
    pub(super) generation: WorthUiActiveApplicationGenerationIdentity,
}

pub(super) enum UiValidatedActiveRecipient {
    Activation(crate::runtime::interaction::UiPresentedInteractionTargetView),
    Draft(UiDraftSessionIdentity),
    Submit(crate::runtime::interaction::UiPresentedInteractionTargetView),
}

impl UiDraftSession {
    pub(super) fn retained_utf8_bytes(&self) -> usize {
        self.committed.len()
            + self
                .preedit
                .as_ref()
                .map_or(0, |preedit| preedit.text().len())
    }

    pub(super) fn mutation_receipt(
        &self,
        session: UiDraftSessionIdentity,
        kind: super::UiDraftMutationKind,
        source_sequence: worth_ui_host_contract::UiHostObservationSequence,
        input_revision: Option<u64>,
    ) -> UiDraftMutationReceipt {
        UiDraftMutationReceipt::new(super::transition::UiDraftMutationReceiptInput {
            session,
            kind,
            source_sequence,
            input_revision,
            draft_revision: self.draft_revision,
            committed_utf8_bytes: self.committed.len(),
            preedit_utf8_bytes: self
                .preedit
                .as_ref()
                .map_or(0, |preedit| preedit.text().len()),
        })
    }
}

impl UiValidatedActiveRecipient {
    pub(super) const fn family(&self) -> super::UiLocalInputRecipientFamily {
        match self {
            Self::Activation(_) => super::UiLocalInputRecipientFamily::Activation,
            Self::Draft(_) => super::UiLocalInputRecipientFamily::Draft,
            Self::Submit(_) => super::UiLocalInputRecipientFamily::Submit,
        }
    }
}

pub(super) fn next(value: u64) -> u64 {
    value
        .checked_add(1)
        .expect("bounded interaction lifecycle counter exhausted")
}
