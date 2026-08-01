use super::{
    stopped, NativeIntentPostureTransfer, WorthUiNativeIntentPosturePublicationCompletion,
    WorthUiNativeIntentPosturePublicationOutcome, WorthUiNativeIntentPosturePublicationRecovery,
};
use crate::facade::entry::WorthUiActiveApplicationSession;

pub(super) struct PreparedNativeIntentPostureRebind<'session> {
    pub(super) session: &'session mut WorthUiActiveApplicationSession,
    pub(super) plan: crate::runtime::rebind::UiRebindPlan,
    pub(super) reservation: crate::runtime::rebind::UiRebindReservation,
    pub(super) frame: crate::mounting::UiPreparedMountedFrame,
    pub(super) transfer: NativeIntentPostureTransfer,
    pub(super) now_tick: u64,
}

struct NativeIntentPostureAdmitted<'session> {
    session: &'session mut WorthUiActiveApplicationSession,
    plan: crate::runtime::rebind::UiRebindPlan,
    reservation: crate::runtime::rebind::UiRebindReservation,
    transfer: NativeIntentPostureTransfer,
}

pub(super) struct NativeIntentPostureInFlight<'session> {
    admitted: NativeIntentPostureAdmitted<'session>,
    mounted: crate::mounting::UiMountedPresentationInFlight,
}

pub(super) struct NativeIntentPostureIndeterminate<'session> {
    admitted: NativeIntentPostureAdmitted<'session>,
    frame: crate::mounting::UiMountedIndeterminateFrame,
}

impl<'session> PreparedNativeIntentPostureRebind<'session> {
    pub(super) fn execute(mut self) -> WorthUiNativeIntentPosturePublicationOutcome<'session> {
        if let Err(denial) = self.reservation.begin_effecting() {
            return stopped(
                crate::runtime::intent_execution::UiIntentConsequenceStopReason::RebindAdmission(
                    denial,
                ),
            );
        }
        let deadline = presentation_deadline(&self.plan);
        let Self {
            session,
            plan,
            reservation,
            frame,
            transfer,
            now_tick,
        } = self;
        let outcome = session.present_prepared_mounted_frame_internal(frame, deadline, now_tick);
        finish(
            NativeIntentPostureAdmitted {
                session,
                plan,
                reservation,
                transfer,
            },
            outcome,
        )
    }
}

impl WorthUiNativeIntentPosturePublicationCompletion<'_> {
    pub fn attempt(&self) -> worth_ui_host_contract::UiMountedPresentationAttemptIdentity {
        self.state().mounted.attempt()
    }

    pub fn deadline(&self) -> worth_ui_host_contract::UiPresentationDeadline {
        self.state().mounted.deadline()
    }
}

impl<'session> WorthUiNativeIntentPosturePublicationCompletion<'session> {
    pub fn complete(
        mut self,
        now_tick: u64,
    ) -> WorthUiNativeIntentPosturePublicationOutcome<'session> {
        let state = self.take_state();
        let outcome = state
            .admitted
            .session
            .complete_mounted_presentation(state.mounted, now_tick);
        finish(state.admitted, outcome)
    }

    pub fn dispose(mut self) -> WorthUiNativeIntentPosturePublicationOutcome<'session> {
        let state = self.take_state();
        let outcome = state
            .admitted
            .session
            .cancel_mounted_presentation(state.mounted);
        finish(state.admitted, outcome)
    }

    fn state(&self) -> &NativeIntentPostureInFlight<'session> {
        self.state
            .as_deref()
            .expect("live posture completion owns its state")
    }

    fn take_state(&mut self) -> Box<NativeIntentPostureInFlight<'session>> {
        self.state
            .take()
            .expect("live posture completion owns its state")
    }
}

impl WorthUiNativeIntentPosturePublicationRecovery<'_> {
    pub fn frame(&self) -> &crate::mounting::UiMountedIndeterminateFrame {
        &self
            .state
            .as_deref()
            .expect("live posture recovery owns its state")
            .frame
    }
}

impl<'session> WorthUiNativeIntentPosturePublicationRecovery<'session> {
    pub fn into_session_for_shutdown(mut self) -> &'session mut WorthUiActiveApplicationSession {
        let state = self
            .state
            .take()
            .expect("live posture recovery owns its state");
        drop((
            state.admitted.plan,
            state.admitted.reservation,
            state.admitted.transfer,
            state.frame,
        ));
        state.admitted.session
    }
}

impl Drop for WorthUiNativeIntentPosturePublicationCompletion<'_> {
    fn drop(&mut self) {
        let Some(state) = self.state.take() else {
            return;
        };
        let outcome = state
            .admitted
            .session
            .cancel_mounted_presentation(state.mounted);
        drop(finish(state.admitted, outcome));
    }
}

fn finish<'session>(
    admitted: NativeIntentPostureAdmitted<'session>,
    outcome: crate::mounting::UiMountedFrameOutcome,
) -> WorthUiNativeIntentPosturePublicationOutcome<'session> {
    match outcome {
        crate::mounting::UiMountedFrameOutcome::Published(mounted) => publish(admitted, mounted),
        crate::mounting::UiMountedFrameOutcome::InFlight(mounted) => {
            WorthUiNativeIntentPosturePublicationOutcome::InFlight(
                WorthUiNativeIntentPosturePublicationCompletion {
                    state: Some(Box::new(NativeIntentPostureInFlight { admitted, mounted })),
                },
            )
        }
        crate::mounting::UiMountedFrameOutcome::RejectedBeforeEffects(rejected) => stopped(
            crate::runtime::intent_execution::UiIntentConsequenceStopReason::HostRejectedBeforeEffects {
                rejection_count: rejected.rejections().len(),
            },
        ),
        crate::mounting::UiMountedFrameOutcome::RetentionDenied(rejected) => stopped(
            crate::runtime::intent_execution::UiIntentConsequenceStopReason::MountedRetention(
                rejected.denial(),
            ),
        ),
        crate::mounting::UiMountedFrameOutcome::AdmissionDenied(rejected) => stopped(
            crate::runtime::intent_execution::UiIntentConsequenceStopReason::MountedPresentation(
                rejected.denial(),
            ),
        ),
        crate::mounting::UiMountedFrameOutcome::PresentationIndeterminate(frame) => {
            WorthUiNativeIntentPosturePublicationOutcome::Indeterminate(
                WorthUiNativeIntentPosturePublicationRecovery {
                    state: Some(Box::new(NativeIntentPostureIndeterminate { admitted, frame })),
                },
            )
        }
        crate::mounting::UiMountedFrameOutcome::CompletionDenied(_) => {
            panic!("exact posture completion authority became unknown")
        }
        crate::mounting::UiMountedFrameOutcome::Unchanged(_)
        | crate::mounting::UiMountedFrameOutcome::Reconciled(_) => {
            unreachable!("fresh posture content always presents a fresh frame")
        }
    }
}

fn publish<'session>(
    mut admitted: NativeIntentPostureAdmitted<'session>,
    mounted: crate::mounting::UiMountedFramePublicationReceipt,
) -> WorthUiNativeIntentPosturePublicationOutcome<'session> {
    assert!(matches!(
        admitted.plan.take_semantic_proof(),
        crate::runtime::rebind::UiRebindSemanticProof::NonSource
    ));
    admitted
        .session
        .application
        .commit_prepared_observation_progress(admitted.transfer.observation);
    admitted
        .session
        .intent_postures
        .commit(admitted.transfer.posture);
    let generation = admitted.plan.basis().candidate_generation().clone();
    match crate::runtime::rebind::UiRebindReceipt::content(
        admitted.plan,
        admitted.reservation,
        generation,
        mounted,
    ) {
        Ok(receipt) => WorthUiNativeIntentPosturePublicationOutcome::Published(receipt),
        Err(defect) => WorthUiNativeIntentPosturePublicationOutcome::InternalDefect(defect),
    }
}

fn presentation_deadline(
    plan: &crate::runtime::rebind::UiRebindPlan,
) -> worth_ui_host_contract::UiPresentationDeadline {
    let tick = match plan.execution_policy().deadline() {
        crate::runtime::rebind::UiRebindDeadlinePolicy::NoDeadline => u64::MAX,
        crate::runtime::rebind::UiRebindDeadlinePolicy::At(deadline) => deadline.tick(),
    };
    worth_ui_host_contract::UiPresentationDeadline::at_tick(tick)
}
