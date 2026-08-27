use super::WorthUiNativeApplicationShell;

#[path = "native_managed_rebind/intent_consequence.rs"]
mod intent_consequence;
#[path = "native_managed_rebind/intent_posture.rs"]
mod intent_posture;
pub(super) use intent_consequence::{
    normalize_managed_intent_consequence, ManagedIntentConsequenceNormalization,
};
pub(super) use intent_posture::{
    normalize_managed_intent_posture, ManagedIntentPostureNormalization,
};

#[derive(Debug)]
pub enum WorthUiNativeManagedRebindDenial {
    SessionMismatch,
    PredecessorReconstruction,
    Preparation(crate::runtime::rebind::UiRebindPreparationDenial),
}

#[derive(Debug)]
pub enum WorthUiNativeManagedRebindStop {
    Duplicate,
    ObservedNoChange,
    RejectedBeforeEffects {
        phase: crate::runtime::rebind::UiRebindStoppedPhase,
        cause: crate::runtime::rebind::UiRebindDenialCause,
    },
    CancelledBeforeEffects(crate::runtime::rebind::UiRebindStoppedPhase),
    TimedOutBeforeEffects(crate::runtime::rebind::UiRebindStoppedPhase),
    SupersededBeforeEffects(crate::runtime::rebind::UiRebindStoppedPhase),
    Indeterminate,
    PredecessorReconstructionFailed,
    IntentPosture(super::native_intent_posture::WorthUiNativeIntentPosturePublicationStop),
    IntentConsequence(crate::runtime::intent_execution::UiIntentConsequenceStopReason),
    InternalDefect(crate::runtime::rebind::UiRebindInternalDefectKind),
}

pub enum WorthUiNativeManagedRebindProgress {
    Unrelated,
    AwaitingProgress,
    Published(crate::runtime::rebind::UiRebindReceipt),
    Stopped(WorthUiNativeManagedRebindStop),
}

pub(super) enum WorthUiNativePendingManagedRebind {
    Completion(crate::runtime::rebind::UiDetachedRebindCompletion),
    IntentPosture(super::native_intent_posture::DetachedNativeIntentPostureInFlight),
    IntentConsequence(super::intent_consequence_publication::DetachedUiIntentConsequenceInFlight),
    PredecessorReconstruction {
        retry: crate::runtime::rebind::UiDetachedRebindRetry,
        in_flight: crate::mounting::UiMountedPresentationInFlight,
    },
}

pub(super) enum ManagedRebindNormalization {
    Published(crate::runtime::rebind::UiRebindReceipt),
    Pending(crate::runtime::rebind::UiDetachedRebindCompletion),
    Stopped(WorthUiNativeManagedRebindStop),
}

impl WorthUiNativeApplicationShell {
    pub fn progress_managed_rebind(
        &mut self,
        progress: &crate::native_platform::UiNativeApplicationPhysicalProgress,
    ) -> Result<WorthUiNativeManagedRebindProgress, WorthUiNativeManagedRebindDenial> {
        let Some(pending) = self.pending_managed_rebind.take() else {
            return Ok(WorthUiNativeManagedRebindProgress::Unrelated);
        };
        match pending {
            WorthUiNativePendingManagedRebind::Completion(pending) => {
                if pending.session_identity() != self.session.session_identity() {
                    return Err(WorthUiNativeManagedRebindDenial::SessionMismatch);
                }
                if !pending.matches_native_progress(progress) {
                    self.pending_managed_rebind =
                        Some(WorthUiNativePendingManagedRebind::Completion(pending));
                    return Ok(WorthUiNativeManagedRebindProgress::AwaitingProgress);
                }
                self.managed_rebind_completion_tick =
                    self.managed_rebind_completion_tick.saturating_add(1);
                let outcome =
                    pending.complete(&mut self.session, self.managed_rebind_completion_tick);
                let outcome = retry_progressed_text_atlas_deferral(
                    outcome,
                    self.managed_rebind_completion_tick,
                );
                let retry = match detach_required_predecessor_reconstruction(outcome) {
                    RequiredPredecessorReconstruction::NotRequired(outcome) => {
                        return Ok(finish_normalized_managed_rebind(
                            &mut self.pending_managed_rebind,
                            outcome,
                        ));
                    }
                    RequiredPredecessorReconstruction::Required(retry) => retry,
                };
                self.begin_predecessor_reconstruction(retry)
            }
            WorthUiNativePendingManagedRebind::IntentPosture(pending) => {
                if pending.session_identity() != self.session.session_identity() {
                    return Err(WorthUiNativeManagedRebindDenial::SessionMismatch);
                }
                if !pending.matches_native_progress(progress) {
                    self.pending_managed_rebind =
                        Some(WorthUiNativePendingManagedRebind::IntentPosture(pending));
                    return Ok(WorthUiNativeManagedRebindProgress::AwaitingProgress);
                }
                self.managed_rebind_completion_tick =
                    self.managed_rebind_completion_tick.saturating_add(1);
                let outcome =
                    pending.complete(&mut self.session, self.managed_rebind_completion_tick);
                let outcome = intent_posture::retry_progressed_text_atlas_deferral(
                    outcome,
                    self.managed_rebind_completion_tick,
                );
                Ok(intent_posture::finish(
                    &mut self.pending_managed_rebind,
                    outcome,
                ))
            }
            WorthUiNativePendingManagedRebind::IntentConsequence(pending) => {
                if pending.session_identity() != self.session.session_identity() {
                    return Err(WorthUiNativeManagedRebindDenial::SessionMismatch);
                }
                if !pending.matches_native_progress(progress) {
                    self.pending_managed_rebind = Some(
                        WorthUiNativePendingManagedRebind::IntentConsequence(pending),
                    );
                    return Ok(WorthUiNativeManagedRebindProgress::AwaitingProgress);
                }
                self.managed_rebind_completion_tick =
                    self.managed_rebind_completion_tick.saturating_add(1);
                let outcome =
                    pending.complete(&mut self.session, self.managed_rebind_completion_tick);
                Ok(intent_consequence::finish(
                    &mut self.pending_managed_rebind,
                    outcome,
                ))
            }
            WorthUiNativePendingManagedRebind::PredecessorReconstruction { retry, in_flight } => {
                if retry.session_identity() != self.session.session_identity() {
                    return Err(WorthUiNativeManagedRebindDenial::SessionMismatch);
                }
                if !reconstruction_matches_progress(&in_flight, progress) {
                    self.pending_managed_rebind = Some(
                        WorthUiNativePendingManagedRebind::PredecessorReconstruction {
                            retry,
                            in_flight,
                        },
                    );
                    return Ok(WorthUiNativeManagedRebindProgress::AwaitingProgress);
                }
                self.managed_rebind_completion_tick =
                    self.managed_rebind_completion_tick.saturating_add(1);
                let recovery = self
                    .session
                    .complete_mounted_presentation(in_flight, self.managed_rebind_completion_tick);
                match recovery {
                    crate::mounting::UiMountedFrameOutcome::InFlight(in_flight) => {
                        self.pending_managed_rebind = Some(
                            WorthUiNativePendingManagedRebind::PredecessorReconstruction {
                                retry,
                                in_flight,
                            },
                        );
                        Ok(WorthUiNativeManagedRebindProgress::AwaitingProgress)
                    }
                    outcome if reconstruction_settled(&outcome) => {
                        let outcome = retry
                            .rebase_content_and_retry(
                                &mut self.session,
                                self.managed_rebind_completion_tick,
                            )
                            .map_err(WorthUiNativeManagedRebindDenial::Preparation)?;
                        Ok(finish_normalized_managed_rebind(
                            &mut self.pending_managed_rebind,
                            outcome,
                        ))
                    }
                    _ => Ok(WorthUiNativeManagedRebindProgress::Stopped(
                        WorthUiNativeManagedRebindStop::PredecessorReconstructionFailed,
                    )),
                }
            }
        }
    }

    fn begin_predecessor_reconstruction(
        &mut self,
        retry: crate::runtime::rebind::UiDetachedRebindRetry,
    ) -> Result<WorthUiNativeManagedRebindProgress, WorthUiNativeManagedRebindDenial> {
        if retry.session_identity() != self.session.session_identity() {
            return Err(WorthUiNativeManagedRebindDenial::SessionMismatch);
        }
        let recovery = self
            .reconstruct_current_presentation(u64::MAX, self.managed_rebind_completion_tick)
            .map_err(|()| WorthUiNativeManagedRebindDenial::PredecessorReconstruction)?;
        match recovery {
            crate::mounting::UiMountedFrameOutcome::InFlight(in_flight) => {
                self.pending_managed_rebind = Some(
                    WorthUiNativePendingManagedRebind::PredecessorReconstruction {
                        retry,
                        in_flight,
                    },
                );
                Ok(WorthUiNativeManagedRebindProgress::AwaitingProgress)
            }
            outcome if reconstruction_settled(&outcome) => {
                let outcome = retry
                    .rebase_content_and_retry(
                        &mut self.session,
                        self.managed_rebind_completion_tick,
                    )
                    .map_err(WorthUiNativeManagedRebindDenial::Preparation)?;
                Ok(finish_normalized_managed_rebind(
                    &mut self.pending_managed_rebind,
                    outcome,
                ))
            }
            _ => Ok(WorthUiNativeManagedRebindProgress::Stopped(
                WorthUiNativeManagedRebindStop::PredecessorReconstructionFailed,
            )),
        }
    }

    pub(super) fn cancel_managed_rebind_for_shutdown(&mut self) {
        let Some(pending) = self.pending_managed_rebind.take() else {
            return;
        };
        match pending {
            WorthUiNativePendingManagedRebind::Completion(pending) => {
                drop(pending.cancel(&mut self.session));
            }
            WorthUiNativePendingManagedRebind::IntentPosture(pending) => {
                drop(pending.cancel(&mut self.session));
            }
            WorthUiNativePendingManagedRebind::IntentConsequence(pending) => {
                drop(pending.cancel(&mut self.session));
            }
            WorthUiNativePendingManagedRebind::PredecessorReconstruction { retry, in_flight } => {
                drop(self.session.cancel_mounted_presentation(in_flight));
                drop(retry);
            }
        }
    }
}

pub(super) fn normalize_managed_outcome(
    outcome: crate::runtime::rebind::UiRebindOutcome<'_>,
) -> ManagedRebindNormalization {
    use crate::runtime::rebind::UiRebindOutcome;
    match outcome {
        UiRebindOutcome::Published(receipt) => ManagedRebindNormalization::Published(receipt),
        UiRebindOutcome::InFlight(completion) => {
            ManagedRebindNormalization::Pending(completion.detach_for_native())
        }
        UiRebindOutcome::Duplicate(_) => {
            ManagedRebindNormalization::Stopped(WorthUiNativeManagedRebindStop::Duplicate)
        }
        UiRebindOutcome::ObservedNoChange(_) => {
            ManagedRebindNormalization::Stopped(WorthUiNativeManagedRebindStop::ObservedNoChange)
        }
        UiRebindOutcome::RejectedBeforeEffects(denial) => ManagedRebindNormalization::Stopped(
            WorthUiNativeManagedRebindStop::RejectedBeforeEffects {
                phase: denial.stopped_phase(),
                cause: denial.cause(),
            },
        ),
        UiRebindOutcome::CancelledBeforeEffects(receipt) => ManagedRebindNormalization::Stopped(
            WorthUiNativeManagedRebindStop::CancelledBeforeEffects(receipt.stopped_phase()),
        ),
        UiRebindOutcome::TimedOutBeforeEffects(receipt) => ManagedRebindNormalization::Stopped(
            WorthUiNativeManagedRebindStop::TimedOutBeforeEffects(receipt.stopped_phase()),
        ),
        UiRebindOutcome::SupersededBeforeEffects(receipt) => ManagedRebindNormalization::Stopped(
            WorthUiNativeManagedRebindStop::SupersededBeforeEffects(receipt.stopped_phase()),
        ),
        UiRebindOutcome::Indeterminate(_) => {
            ManagedRebindNormalization::Stopped(WorthUiNativeManagedRebindStop::Indeterminate)
        }
        UiRebindOutcome::InternalDefect(defect) => ManagedRebindNormalization::Stopped(
            WorthUiNativeManagedRebindStop::InternalDefect(defect.kind()),
        ),
    }
}

fn finish_normalized_managed_rebind(
    pending: &mut Option<WorthUiNativePendingManagedRebind>,
    outcome: crate::runtime::rebind::UiRebindOutcome<'_>,
) -> WorthUiNativeManagedRebindProgress {
    match normalize_managed_outcome(outcome) {
        ManagedRebindNormalization::Published(receipt) => {
            WorthUiNativeManagedRebindProgress::Published(receipt)
        }
        ManagedRebindNormalization::Pending(completion) => {
            *pending = Some(WorthUiNativePendingManagedRebind::Completion(completion));
            WorthUiNativeManagedRebindProgress::AwaitingProgress
        }
        ManagedRebindNormalization::Stopped(stop) => {
            WorthUiNativeManagedRebindProgress::Stopped(stop)
        }
    }
}

fn retry_progressed_text_atlas_deferral<'session>(
    outcome: crate::runtime::rebind::UiRebindOutcome<'session>,
    now_tick: u64,
) -> crate::runtime::rebind::UiRebindOutcome<'session> {
    let crate::runtime::rebind::UiRebindOutcome::RejectedBeforeEffects(denial) = outcome else {
        return outcome;
    };
    let rejections = denial.host_rejections();
    if !rejections.is_empty()
        && rejections.iter().all(|rejection| {
            rejection.denial()
                == worth_ui_host_contract::UiHostSurfacePresentationDenial::
                    TextAtlasPresentationDeferred
        })
    {
        denial.retry_at(now_tick)
    } else {
        crate::runtime::rebind::UiRebindOutcome::RejectedBeforeEffects(denial)
    }
}

enum RequiredPredecessorReconstruction<'session> {
    NotRequired(crate::runtime::rebind::UiRebindOutcome<'session>),
    Required(crate::runtime::rebind::UiDetachedRebindRetry),
}

fn detach_required_predecessor_reconstruction<'session>(
    outcome: crate::runtime::rebind::UiRebindOutcome<'session>,
) -> RequiredPredecessorReconstruction<'session> {
    let crate::runtime::rebind::UiRebindOutcome::RejectedBeforeEffects(denial) = outcome else {
        return RequiredPredecessorReconstruction::NotRequired(outcome);
    };
    let rejections = denial.host_rejections();
    if !rejections.is_empty()
        && rejections.iter().all(|rejection| {
            rejection.denial()
                == worth_ui_host_contract::UiHostSurfacePresentationDenial::ReconstructionRequired
        })
    {
        match denial.detach_retry_for_native() {
            Ok(retry) => RequiredPredecessorReconstruction::Required(retry),
            Err(denial) => RequiredPredecessorReconstruction::NotRequired(
                crate::runtime::rebind::UiRebindOutcome::RejectedBeforeEffects(denial),
            ),
        }
    } else {
        RequiredPredecessorReconstruction::NotRequired(
            crate::runtime::rebind::UiRebindOutcome::RejectedBeforeEffects(denial),
        )
    }
}

fn reconstruction_matches_progress(
    in_flight: &crate::mounting::UiMountedPresentationInFlight,
    progress: &crate::native_platform::UiNativeApplicationPhysicalProgress,
) -> bool {
    progress.class() == worth_ui_host_native::UiNativePhysicalProgressClass::Presentation
        && in_flight.awaits_progress_class(
            worth_ui_host_contract::UiHostPresentationProgressClass::PhysicalSurface,
        )
        && progress.presentation().is_some_and(|presentation| {
            presentation.attempt() == in_flight.attempt()
                && in_flight
                    .pending_bindings()
                    .any(|binding| binding == presentation.binding())
        })
}

fn reconstruction_settled(outcome: &crate::mounting::UiMountedFrameOutcome) -> bool {
    matches!(
        outcome,
        crate::mounting::UiMountedFrameOutcome::Published(_)
            | crate::mounting::UiMountedFrameOutcome::Unchanged(_)
            | crate::mounting::UiMountedFrameOutcome::Reconciled(_)
    )
}
