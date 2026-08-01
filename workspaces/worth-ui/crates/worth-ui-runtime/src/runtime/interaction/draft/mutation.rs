use worth_ui_host_contract::{
    UiHostObservationPresentationBasis, UiHostObservationSequence, UiHostObservationTimeBasis,
};

use super::model::{next, UiDraftProcessingOutcome, UiDraftRuntimeState};
use super::{
    UiDraftMutationKind, UiDraftSessionIdentity, UiLocalInputStop, UiLocalInputStopReason,
};
use crate::runtime::interaction::{
    UiEditCommitInput, UiEditCommitInteraction, UiSemanticInteraction,
};

pub(super) struct UiCommittedTextMutation<'text> {
    pub(super) sequence: UiHostObservationSequence,
    pub(super) revision: u64,
    pub(super) text: &'text str,
    pub(super) kind: UiDraftMutationKind,
}

impl UiDraftRuntimeState {
    pub(super) fn apply_committed_text(
        &mut self,
        session: UiDraftSessionIdentity,
        input: UiCommittedTextMutation<'_>,
    ) -> UiDraftProcessingOutcome {
        if let Some(stop) = self.require_input_revision(session, input.revision) {
            return UiDraftProcessingOutcome::Stopped(stop);
        }
        let draft = self
            .sessions
            .get_mut(&session)
            .expect("validated active draft remains present");
        let attempted = draft.committed.len().saturating_add(input.text.len());
        if attempted > draft.budget.utf8_bytes() {
            let reason = UiLocalInputStopReason::DraftByteBudgetExceeded {
                limit: draft.budget.utf8_bytes(),
                attempted,
            };
            return UiDraftProcessingOutcome::Stopped(
                self.cancel_session(session, reason)
                    .expect("validated active draft settles"),
            );
        }
        draft.committed.push_str(input.text);
        draft.preedit = None;
        draft.last_input_revision = Some(input.revision);
        draft.draft_revision = next(draft.draft_revision);
        self.counters.mutations = next(self.counters.mutations);
        UiDraftProcessingOutcome::Mutation(draft.mutation_receipt(
            session,
            input.kind,
            input.sequence,
            Some(input.revision),
        ))
    }

    pub(super) fn apply_preedit(
        &mut self,
        session: UiDraftSessionIdentity,
        sequence: UiHostObservationSequence,
        revision: u64,
        preedit: worth_ui_host_contract::UiHostImePreedit,
    ) -> UiDraftProcessingOutcome {
        if let Some(stop) = self.require_input_revision(session, revision) {
            return UiDraftProcessingOutcome::Stopped(stop);
        }
        let draft = self
            .sessions
            .get_mut(&session)
            .expect("validated active draft remains present");
        let attempted = draft.committed.len().saturating_add(preedit.text().len());
        if attempted > draft.budget.utf8_bytes() {
            let reason = UiLocalInputStopReason::DraftByteBudgetExceeded {
                limit: draft.budget.utf8_bytes(),
                attempted,
            };
            return UiDraftProcessingOutcome::Stopped(
                self.cancel_session(session, reason)
                    .expect("validated active draft settles"),
            );
        }
        draft.preedit = Some(preedit);
        draft.last_input_revision = Some(revision);
        draft.draft_revision = next(draft.draft_revision);
        self.counters.mutations = next(self.counters.mutations);
        UiDraftProcessingOutcome::Mutation(draft.mutation_receipt(
            session,
            UiDraftMutationKind::Preedit,
            sequence,
            Some(revision),
        ))
    }

    pub(super) fn cancel_preedit(
        &mut self,
        session: UiDraftSessionIdentity,
        sequence: UiHostObservationSequence,
        revision: u64,
    ) -> UiDraftProcessingOutcome {
        if let Some(stop) = self.require_input_revision(session, revision) {
            return UiDraftProcessingOutcome::Stopped(stop);
        }
        let draft = self
            .sessions
            .get_mut(&session)
            .expect("validated active draft remains present");
        draft.preedit = None;
        draft.last_input_revision = Some(revision);
        draft.draft_revision = next(draft.draft_revision);
        self.counters.mutations = next(self.counters.mutations);
        UiDraftProcessingOutcome::Mutation(draft.mutation_receipt(
            session,
            UiDraftMutationKind::PreeditCancel,
            sequence,
            Some(revision),
        ))
    }

    pub(super) fn backspace(
        &mut self,
        session: UiDraftSessionIdentity,
        sequence: UiHostObservationSequence,
    ) -> Option<UiDraftProcessingOutcome> {
        if self
            .sessions
            .get(&session)
            .is_some_and(|draft| draft.preedit.is_some())
        {
            return Some(UiDraftProcessingOutcome::Stopped(
                self.unsettled_session_stop(session, UiLocalInputStopReason::CompositionActive),
            ));
        }
        let draft = self.sessions.get_mut(&session)?;
        draft.committed.pop()?;
        draft.draft_revision = next(draft.draft_revision);
        self.counters.mutations = next(self.counters.mutations);
        Some(UiDraftProcessingOutcome::Mutation(draft.mutation_receipt(
            session,
            UiDraftMutationKind::Backspace,
            sequence,
            None,
        )))
    }

    pub(super) fn commit(
        &mut self,
        session: UiDraftSessionIdentity,
        presentation: UiHostObservationPresentationBasis,
        sequence: UiHostObservationSequence,
        time_basis: UiHostObservationTimeBasis,
    ) -> UiDraftProcessingOutcome {
        if self
            .sessions
            .get(&session)
            .is_some_and(|draft| draft.preedit.is_some())
        {
            return UiDraftProcessingOutcome::Stopped(
                self.unsettled_session_stop(session, UiLocalInputStopReason::CompositionActive),
            );
        }
        let draft = self
            .sessions
            .remove(&session)
            .expect("validated active draft remains present");
        self.active = None;
        self.counters.sessions_settled = next(self.counters.sessions_settled);
        UiDraftProcessingOutcome::Semantic(UiSemanticInteraction::EditCommit(
            UiEditCommitInteraction::seal(UiEditCommitInput {
                target: draft.target,
                presentation,
                generation: draft.generation,
                session,
                field: draft.field,
                source_sequence: sequence,
                time_basis,
                input_revision: draft.last_input_revision,
                draft_revision: draft.draft_revision,
                committed_text: std::sync::Arc::from(draft.committed.into_boxed_str()),
            }),
        ))
    }

    fn require_input_revision(
        &mut self,
        session: UiDraftSessionIdentity,
        observed: u64,
    ) -> Option<UiLocalInputStop> {
        let previous = self.sessions.get(&session)?.last_input_revision;
        let discontinuity = previous.is_some_and(|prior| prior.checked_add(1) != Some(observed));
        if !discontinuity {
            return None;
        }
        self.cancel_session(
            session,
            UiLocalInputStopReason::InputRevisionDiscontinuity {
                previous: previous.expect("discontinuity requires a predecessor"),
                observed,
            },
        )
    }
}
