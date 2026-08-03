use worth_ui_host_contract::{
    UiHostImeCompositionPhase, UiHostKey, UiHostKeyTransition, UiHostObservationCanonicalCore,
    UiHostObservationPayload, UiHostObservationReport, UiHostObservationSequence,
};

use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;

use super::model::{
    UiActiveLocalRecipient, UiDraftProcessingOutcome, UiDraftRuntimeState, UiRecipientContext,
    UiValidatedActiveRecipient,
};
use super::{UiDraftMutationKind, UiLocalInputStopReason};
use crate::runtime::interaction::{
    UiActivateInteraction, UiKeyboardActivationEvidence, UiKeyboardSemanticInput,
    UiSemanticInteraction, UiSubmitInteraction,
};

#[derive(Clone, Copy)]
struct UiDraftReportContext<'world> {
    core: UiHostObservationCanonicalCore,
    sequence: UiHostObservationSequence,
    mounted: &'world crate::mounting::WorthUiMountedSessionState,
    generation: &'world WorthUiPreparedApplicationGenerationIdentity,
}

impl UiDraftRuntimeState {
    pub(crate) fn process_report(
        &mut self,
        core: UiHostObservationCanonicalCore,
        report: &UiHostObservationReport,
        mounted: &crate::mounting::WorthUiMountedSessionState,
        generation: &WorthUiPreparedApplicationGenerationIdentity,
    ) -> Vec<UiDraftProcessingOutcome> {
        let context = UiDraftReportContext {
            core,
            sequence: report.sequence(),
            mounted,
            generation,
        };
        match report.payload() {
            UiHostObservationPayload::Keyboard {
                logical_key,
                modifiers,
                transition,
                ..
            } => self.process_key(context, *logical_key, *modifiers, *transition),
            UiHostObservationPayload::TextInput { revision, text } => self.process_committed_text(
                context,
                *revision,
                text,
                UiDraftMutationKind::CommittedText,
            ),
            UiHostObservationPayload::ImeComposition { revision, phase } => {
                self.process_ime(context, *revision, phase)
            }
            UiHostObservationPayload::Focus { focused: false } => self
                .cancel_all(UiLocalInputStopReason::FocusLost)
                .into_iter()
                .map(UiDraftProcessingOutcome::Stopped)
                .collect(),
            _ => Vec::new(),
        }
    }

    fn process_key(
        &mut self,
        context: UiDraftReportContext<'_>,
        key: UiHostKey,
        modifiers: worth_ui_host_contract::UiHostKeyboardModifiers,
        transition: UiHostKeyTransition,
    ) -> Vec<UiDraftProcessingOutcome> {
        let UiHostKeyTransition::Pressed { repeat: false } = transition else {
            return Vec::new();
        };
        let active = match self.validate_active(context) {
            Some(active) => active,
            None => return self.missing_or_invalid_active(context),
        };
        match active {
            UiValidatedActiveRecipient::Activation(target) if activation_key(key) => {
                vec![UiDraftProcessingOutcome::Semantic(
                    UiSemanticInteraction::Activate(UiActivateInteraction::from_keyboard(
                        UiKeyboardActivationEvidence::new(UiKeyboardSemanticInput {
                            target,
                            presentation: context.core.presentation(),
                            generation: context.generation.clone(),
                            sequence: context.sequence,
                            key,
                            modifiers,
                        }),
                    )),
                )]
            }
            UiValidatedActiveRecipient::Submit(target) if key == UiHostKey::Enter => {
                vec![UiDraftProcessingOutcome::Semantic(
                    UiSemanticInteraction::Submit(UiSubmitInteraction::seal(
                        UiKeyboardSemanticInput {
                            target,
                            presentation: context.core.presentation(),
                            generation: context.generation.clone(),
                            sequence: context.sequence,
                            key,
                            modifiers,
                        },
                    )),
                )]
            }
            UiValidatedActiveRecipient::Draft(session) if key == UiHostKey::Escape => self
                .cancel_session(session, UiLocalInputStopReason::ExplicitCancel)
                .into_iter()
                .map(UiDraftProcessingOutcome::Stopped)
                .collect(),
            UiValidatedActiveRecipient::Draft(session) if key == UiHostKey::Backspace => self
                .backspace(session, context.sequence)
                .into_iter()
                .collect(),
            UiValidatedActiveRecipient::Draft(session) if key == UiHostKey::Enter => {
                vec![self.commit(session, context.core.presentation(), context.sequence)]
            }
            _ => Vec::new(),
        }
    }

    fn process_committed_text(
        &mut self,
        context: UiDraftReportContext<'_>,
        revision: u64,
        text: &str,
        kind: UiDraftMutationKind,
    ) -> Vec<UiDraftProcessingOutcome> {
        let session = match self.validate_active(context) {
            Some(UiValidatedActiveRecipient::Draft(session)) => session,
            Some(active) => {
                return vec![UiDraftProcessingOutcome::Stopped(self.unsettled_stop(
                    context.core,
                    UiLocalInputStopReason::RecipientFamilyMismatch {
                        required: super::UiLocalInputRecipientFamily::Draft,
                        active: active.family(),
                    },
                ))]
            }
            None => return self.missing_or_invalid_active(context),
        };
        vec![self.apply_committed_text(
            session,
            super::mutation::UiCommittedTextMutation {
                sequence: context.sequence,
                revision,
                text,
                kind,
            },
        )]
    }

    fn process_ime(
        &mut self,
        context: UiDraftReportContext<'_>,
        revision: u64,
        phase: &UiHostImeCompositionPhase,
    ) -> Vec<UiDraftProcessingOutcome> {
        let session = match self.validate_active(context) {
            Some(UiValidatedActiveRecipient::Draft(session)) => session,
            Some(active) => {
                return vec![UiDraftProcessingOutcome::Stopped(self.unsettled_stop(
                    context.core,
                    UiLocalInputStopReason::RecipientFamilyMismatch {
                        required: super::UiLocalInputRecipientFamily::Draft,
                        active: active.family(),
                    },
                ))]
            }
            None => return self.missing_or_invalid_active(context),
        };
        vec![match phase {
            UiHostImeCompositionPhase::Preedit(preedit) => {
                self.apply_preedit(session, context.sequence, revision, preedit.clone())
            }
            UiHostImeCompositionPhase::Commit(text) => self.apply_committed_text(
                session,
                super::mutation::UiCommittedTextMutation {
                    sequence: context.sequence,
                    revision,
                    text,
                    kind: UiDraftMutationKind::PreeditCommit,
                },
            ),
            UiHostImeCompositionPhase::Cancel => {
                self.cancel_preedit(session, context.sequence, revision)
            }
        }]
    }

    fn validate_active(
        &self,
        report: UiDraftReportContext<'_>,
    ) -> Option<UiValidatedActiveRecipient> {
        let context = self.active_context()?;
        if context.target.binding() != report.core.binding()
            || &context.generation != report.generation
            || crate::runtime::interaction::targeting::require_current_target(
                report.mounted,
                context.target,
            )
            .is_err()
        {
            return None;
        }
        Some(match self.active.as_ref()? {
            UiActiveLocalRecipient::Activation(_) => {
                UiValidatedActiveRecipient::Activation(context.target)
            }
            UiActiveLocalRecipient::Draft(session) => UiValidatedActiveRecipient::Draft(*session),
            UiActiveLocalRecipient::Submit(_) => UiValidatedActiveRecipient::Submit(context.target),
        })
    }

    fn missing_or_invalid_active(
        &mut self,
        report: UiDraftReportContext<'_>,
    ) -> Vec<UiDraftProcessingOutcome> {
        let Some(context) = self.active_context() else {
            return vec![UiDraftProcessingOutcome::Stopped(self.unsettled_stop(
                report.core,
                UiLocalInputStopReason::NoLocalRecipient,
            ))];
        };
        if context.target.binding() != report.core.binding() {
            return vec![UiDraftProcessingOutcome::Stopped(self.unsettled_stop(
                report.core,
                UiLocalInputStopReason::ForeignBinding {
                    expected: context.target.binding(),
                    observed: report.core.binding(),
                },
            ))];
        }
        let reason = if &context.generation != report.generation {
            UiLocalInputStopReason::ApplicationGenerationChanged
        } else {
            let denial = crate::runtime::interaction::targeting::require_current_target(
                report.mounted,
                context.target,
            )
            .expect_err("the active target was not current");
            UiLocalInputStopReason::TargetNoLongerCurrent(denial)
        };
        self.cancel_active(reason)
            .into_iter()
            .map(UiDraftProcessingOutcome::Stopped)
            .collect()
    }

    pub(super) fn active_context(&self) -> Option<UiRecipientContext> {
        match self.active.as_ref()? {
            UiActiveLocalRecipient::Activation(context)
            | UiActiveLocalRecipient::Submit(context) => Some(context.clone()),
            UiActiveLocalRecipient::Draft(session) => {
                let draft = self.sessions.get(session)?;
                Some(UiRecipientContext {
                    target: draft.target,
                    generation: draft.generation.clone(),
                })
            }
        }
    }
}

fn activation_key(key: UiHostKey) -> bool {
    matches!(key, UiHostKey::Enter | UiHostKey::Space)
}
