use super::WorthUiNativeApplicationShell;

#[path = "native_managed_rebind/intent_consequence.rs"]
mod intent_consequence;
#[path = "native_managed_rebind/intent_posture.rs"]
mod intent_posture;
#[path = "native_managed_rebind/intent_posture_reconstruction.rs"]
mod intent_posture_reconstruction;
#[path = "native_managed_rebind/model.rs"]
mod model;
#[path = "native_managed_rebind/portal_dismissal.rs"]
mod portal_dismissal;
#[path = "native_managed_rebind/predecessor_reconstruction.rs"]
mod predecessor_reconstruction;
#[path = "native_managed_rebind/shutdown.rs"]
mod shutdown;
pub(super) use intent_consequence::{
    normalize_managed_intent_consequence, ManagedIntentConsequenceNormalization,
};
pub(super) use intent_posture::{
    normalize_managed_intent_posture, ManagedIntentPostureNormalization,
};
pub(super) use model::WorthUiNativePendingManagedRebind;
pub use model::{
    WorthUiNativeManagedRebindDenial, WorthUiNativeManagedRebindProgress,
    WorthUiNativeManagedRebindStop, WorthUiNativePredecessorRecovery,
};
pub(super) use portal_dismissal::UiRetainedPortalDismissalRequest;
pub use portal_dismissal::{
    WorthUiNativeManagedPortalDismissalOutcome, WorthUiNativePortalDismissalStop,
};
use predecessor_reconstruction::{
    detach_required_predecessor_reconstruction, reconstruction_matches_progress,
    reconstruction_settled, RequiredPredecessorReconstruction,
};

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
            WorthUiNativePendingManagedRebind::IntentPosturePredecessorReconstruction {
                retry,
                in_flight,
            } => {
                self.progress_intent_posture_predecessor_reconstruction(retry, in_flight, progress)
            }
            WorthUiNativePendingManagedRebind::IntentPosturePredecessorReconstructionDeferred(
                retry,
            ) => self.progress_deferred_intent_posture_predecessor_reconstruction(retry, progress),
            WorthUiNativePendingManagedRebind::IntentPosturePredecessorIndeterminate {
                retry,
                frame,
            } => self.progress_indeterminate_intent_posture_predecessor_reconstruction(
                retry, frame, progress,
            ),
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
            WorthUiNativePendingManagedRebind::IntentConsequenceIndeterminate(pending) => {
                self.progress_indeterminate_intent_consequence(pending, progress)
            }
            WorthUiNativePendingManagedRebind::IntentConsequenceReconstruction {
                portal,
                resources,
                in_flight,
            } => self
                .progress_intent_consequence_reconstruction(portal, resources, in_flight, progress),
            WorthUiNativePendingManagedRebind::IntentConsequenceReconstructionDeferred {
                portal,
                resources,
            } => self.progress_deferred_intent_consequence_reconstruction(portal, resources),
            WorthUiNativePendingManagedRebind::PortalDismissal(pending) => {
                if pending.session_identity() != self.session.session_identity() {
                    return Err(WorthUiNativeManagedRebindDenial::SessionMismatch);
                }
                if !pending.matches_native_progress(progress) {
                    self.pending_managed_rebind =
                        Some(WorthUiNativePendingManagedRebind::PortalDismissal(pending));
                    return Ok(WorthUiNativeManagedRebindProgress::AwaitingProgress);
                }
                self.managed_rebind_completion_tick =
                    self.managed_rebind_completion_tick.saturating_add(1);
                let outcome =
                    pending.complete(&mut self.session, self.managed_rebind_completion_tick);
                let progress = portal_dismissal::finish(&mut self.pending_managed_rebind, outcome);
                if matches!(
                    progress,
                    WorthUiNativeManagedRebindProgress::PortalDismissed(_)
                ) {
                    self.retained_portal_dismissal = None;
                }
                Ok(progress)
            }
            WorthUiNativePendingManagedRebind::PortalDismissalIndeterminate(pending) => {
                self.progress_indeterminate_portal_dismissal(pending, progress)
            }
            WorthUiNativePendingManagedRebind::PortalDismissalReconstruction {
                proposal,
                in_flight,
            } => self.progress_portal_dismissal_reconstruction(proposal, in_flight, progress),
            WorthUiNativePendingManagedRebind::PortalDismissalReconstructionDeferred {
                proposal,
            } => self.progress_deferred_portal_dismissal_reconstruction(proposal),
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
        UiRebindOutcome::RejectedBeforeEffects(denial) => {
            let host_denials = denial
                .host_rejections()
                .iter()
                .map(|rejection| rejection.denial())
                .collect::<Vec<_>>()
                .into_boxed_slice();
            ManagedRebindNormalization::Stopped(
                WorthUiNativeManagedRebindStop::RejectedBeforeEffects {
                    phase: denial.stopped_phase(),
                    cause: denial.cause(),
                    host_denials,
                },
            )
        }
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
