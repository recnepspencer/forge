use super::{
    intent_consequence_rebind::WorthUiIntentConsequenceRebindTransfer,
    WorthUiActiveApplicationSession,
};

pub enum UiIntentConsequencePublicationOutcome<'session> {
    NoConsequences(crate::runtime::intent_execution::UiIntentConsequenceCompletionReceipt),
    Stopped(crate::runtime::intent_execution::UiIntentConsequenceStop),
    Published(crate::runtime::rebind::UiRebindReceipt),
    InFlight(UiIntentConsequencePublicationCompletion<'session>),
    Indeterminate(UiIntentConsequencePublicationRecovery<'session>),
    InternalDefect(crate::runtime::rebind::UiRebindInternalDefectOutcome),
}

#[must_use = "consequence presentation must be completed or explicitly disposed"]
pub struct UiIntentConsequencePublicationCompletion<'session> {
    state: Option<Box<UiIntentConsequenceInFlight<'session>>>,
}

#[must_use = "indeterminate consequence presentation requires reconciliation or shutdown"]
pub struct UiIntentConsequencePublicationRecovery<'session> {
    state: Option<Box<UiIntentConsequenceIndeterminate<'session>>>,
}

pub(super) struct WorthUiPreparedIntentConsequenceRebind<'session> {
    session: &'session mut WorthUiActiveApplicationSession,
    plan: crate::runtime::rebind::UiRebindPlan,
    reservation: crate::runtime::rebind::UiRebindReservation,
    frame: crate::mounting::UiPreparedMountedFrame,
    transfer: WorthUiIntentConsequenceRebindTransfer,
}

struct UiIntentConsequenceAdmitted<'session> {
    session: &'session mut WorthUiActiveApplicationSession,
    plan: crate::runtime::rebind::UiRebindPlan,
    reservation: crate::runtime::rebind::UiRebindReservation,
    transfer: WorthUiIntentConsequenceRebindTransfer,
    query: Option<worth_ui_query_binding::WorthUiAdmittedCollectionChangePublication>,
}

struct UiIntentConsequenceInFlight<'session> {
    admitted: UiIntentConsequenceAdmitted<'session>,
    mounted: crate::mounting::UiMountedPresentationInFlight,
}

struct UiIntentConsequenceIndeterminate<'session> {
    admitted: UiIntentConsequenceAdmitted<'session>,
    frame: crate::mounting::UiMountedIndeterminateFrame,
}

impl<'session> WorthUiPreparedIntentConsequenceRebind<'session> {
    pub(super) fn new(
        session: &'session mut WorthUiActiveApplicationSession,
        plan: crate::runtime::rebind::UiRebindPlan,
        reservation: crate::runtime::rebind::UiRebindReservation,
        frame: crate::mounting::UiPreparedMountedFrame,
        transfer: WorthUiIntentConsequenceRebindTransfer,
    ) -> Self {
        Self {
            session,
            plan,
            reservation,
            frame,
            transfer,
        }
    }

    pub(super) fn execute(
        mut self,
        now_tick: u64,
    ) -> UiIntentConsequencePublicationOutcome<'session> {
        if let Err(denial) = self.reservation.begin_effecting() {
            return stop_prepared(
                self,
                crate::runtime::intent_execution::UiIntentConsequenceStopReason::RebindAdmission(
                    denial,
                ),
            );
        }
        let query = match self.transfer.query_reference.as_ref() {
            Some(reference) => match self
                .session
                .application
                .prepare_exact_query_change_publication(reference)
            {
                Ok(admission) => Some(admission),
                Err(reason) => return stop_prepared(self, reason),
            },
            None => None,
        };
        let deadline = presentation_deadline(&self.plan);
        let Self {
            session,
            plan,
            reservation,
            frame,
            transfer,
        } = self;
        let outcome = session.present_prepared_mounted_frame_internal(frame, deadline, now_tick);
        finish_first(
            UiIntentConsequenceAdmitted {
                session,
                plan,
                reservation,
                transfer,
                query,
            },
            outcome,
        )
    }
}

impl UiIntentConsequencePublicationCompletion<'_> {
    pub fn attempt(&self) -> worth_ui_host_contract::UiMountedPresentationAttemptIdentity {
        self.state().mounted.attempt()
    }

    pub fn deadline(&self) -> worth_ui_host_contract::UiPresentationDeadline {
        self.state().mounted.deadline()
    }
}

impl<'session> UiIntentConsequencePublicationCompletion<'session> {
    pub fn complete(mut self, now_tick: u64) -> UiIntentConsequencePublicationOutcome<'session> {
        let state = self.take_state();
        let outcome = state
            .admitted
            .session
            .complete_mounted_presentation(state.mounted, now_tick);
        finish_completion(state.admitted, outcome)
    }

    pub fn dispose(mut self) -> UiIntentConsequencePublicationOutcome<'session> {
        let state = self.take_state();
        let outcome = state
            .admitted
            .session
            .cancel_mounted_presentation(state.mounted);
        finish_completion(state.admitted, outcome)
    }

    fn state(&self) -> &UiIntentConsequenceInFlight<'session> {
        self.state
            .as_deref()
            .expect("live consequence completion owns its state")
    }

    fn take_state(&mut self) -> Box<UiIntentConsequenceInFlight<'session>> {
        self.state
            .take()
            .expect("live consequence completion owns its state")
    }
}

impl<'session> UiIntentConsequencePublicationRecovery<'session> {
    pub fn frame(&self) -> &crate::mounting::UiMountedIndeterminateFrame {
        &self
            .state
            .as_deref()
            .expect("live consequence recovery owns its state")
            .frame
    }

    pub fn into_session_for_shutdown(mut self) -> &'session mut WorthUiActiveApplicationSession {
        let mut state = self
            .state
            .take()
            .expect("live consequence recovery owns its state");
        withdraw_query(&mut state.admitted);
        state
            .admitted
            .session
            .intent_execution
            .dispose_consequence_handoff(state.admitted.transfer.consequence);
        drop((state.admitted.plan, state.admitted.reservation, state.frame));
        state.admitted.session
    }
}

impl Drop for UiIntentConsequencePublicationCompletion<'_> {
    fn drop(&mut self) {
        let Some(state) = self.state.take() else {
            return;
        };
        let outcome = state
            .admitted
            .session
            .cancel_mounted_presentation(state.mounted);
        drop(finish_completion(state.admitted, outcome));
    }
}

fn finish_first<'session>(
    mut admitted: UiIntentConsequenceAdmitted<'session>,
    outcome: crate::mounting::UiMountedFrameOutcome,
) -> UiIntentConsequencePublicationOutcome<'session> {
    match outcome {
        crate::mounting::UiMountedFrameOutcome::InFlight(mounted) => {
            admitted
                .reservation
                .retain_completion()
                .expect("effect admission reserved consequence completion capacity");
            UiIntentConsequencePublicationOutcome::InFlight(
                UiIntentConsequencePublicationCompletion {
                    state: Some(Box::new(UiIntentConsequenceInFlight { admitted, mounted })),
                },
            )
        }
        outcome => finish_terminal(admitted, outcome),
    }
}

fn finish_completion<'session>(
    admitted: UiIntentConsequenceAdmitted<'session>,
    outcome: crate::mounting::UiMountedFrameOutcome,
) -> UiIntentConsequencePublicationOutcome<'session> {
    match outcome {
        crate::mounting::UiMountedFrameOutcome::InFlight(mounted) => {
            UiIntentConsequencePublicationOutcome::InFlight(
                UiIntentConsequencePublicationCompletion {
                    state: Some(Box::new(UiIntentConsequenceInFlight { admitted, mounted })),
                },
            )
        }
        outcome => finish_terminal(admitted, outcome),
    }
}

fn finish_terminal<'session>(
    mut admitted: UiIntentConsequenceAdmitted<'session>,
    outcome: crate::mounting::UiMountedFrameOutcome,
) -> UiIntentConsequencePublicationOutcome<'session> {
    match outcome {
        crate::mounting::UiMountedFrameOutcome::Published(mounted) => {
            publish(admitted, mounted)
        }
        crate::mounting::UiMountedFrameOutcome::RejectedBeforeEffects(rejected) => stop_admitted(
            admitted,
            crate::runtime::intent_execution::UiIntentConsequenceStopReason::HostRejectedBeforeEffects {
                rejection_count: rejected.rejections().len(),
            },
        ),
        crate::mounting::UiMountedFrameOutcome::RetentionDenied(rejected) => stop_admitted(
            admitted,
            crate::runtime::intent_execution::UiIntentConsequenceStopReason::MountedRetention(
                rejected.denial(),
            ),
        ),
        crate::mounting::UiMountedFrameOutcome::AdmissionDenied(rejected) => stop_admitted(
            admitted,
            crate::runtime::intent_execution::UiIntentConsequenceStopReason::MountedPresentation(
                rejected.denial(),
            ),
        ),
        crate::mounting::UiMountedFrameOutcome::PresentationIndeterminate(frame) => {
            admitted
                .reservation
                .retain_recovery()
                .expect("effect admission reserved consequence recovery capacity");
            UiIntentConsequencePublicationOutcome::Indeterminate(
                UiIntentConsequencePublicationRecovery {
                    state: Some(Box::new(UiIntentConsequenceIndeterminate { admitted, frame })),
                },
            )
        }
        crate::mounting::UiMountedFrameOutcome::CompletionDenied(_) => {
            panic!("exact consequence completion authority became unknown")
        }
        crate::mounting::UiMountedFrameOutcome::InFlight(_) => {
            unreachable!("in-flight outcomes are retained by the phase-specific mapper")
        }
        crate::mounting::UiMountedFrameOutcome::Unchanged(_)
        | crate::mounting::UiMountedFrameOutcome::Reconciled(_) => {
            unreachable!("explicit consequence content always presents a fresh frame")
        }
    }
}

fn publish<'session>(
    mut admitted: UiIntentConsequenceAdmitted<'session>,
    mounted: crate::mounting::UiMountedFramePublicationReceipt,
) -> UiIntentConsequencePublicationOutcome<'session> {
    if let Some(query) = admitted.query.take() {
        let receipt = admitted
            .session
            .application
            .publish_exact_query_change(query)
            .expect("exclusive exact Query admission must remain publishable");
        assert_eq!(receipt.published_change_count(), 1);
    }
    assert!(matches!(
        admitted.plan.take_semantic_proof(),
        crate::runtime::rebind::UiRebindSemanticProof::NonSource
    ));
    admitted
        .session
        .application
        .commit_prepared_observation_progress(admitted.transfer.observation);
    if let Some(posture) = admitted.transfer.posture {
        admitted.session.intent_postures.commit(posture);
    }
    admitted
        .session
        .intent_execution
        .finish_consequence_handoff(admitted.transfer.consequence);
    let generation = admitted.plan.basis().candidate_generation().clone();
    match crate::runtime::rebind::UiRebindReceipt::content(
        admitted.plan,
        admitted.reservation,
        generation,
        mounted,
    ) {
        Ok(receipt) => UiIntentConsequencePublicationOutcome::Published(receipt),
        Err(defect) => UiIntentConsequencePublicationOutcome::InternalDefect(defect),
    }
}

fn stop_prepared<'session>(
    prepared: WorthUiPreparedIntentConsequenceRebind<'session>,
    reason: crate::runtime::intent_execution::UiIntentConsequenceStopReason,
) -> UiIntentConsequencePublicationOutcome<'session> {
    let WorthUiPreparedIntentConsequenceRebind {
        session,
        plan,
        reservation,
        frame,
        transfer,
    } = prepared;
    drop((reservation, frame));
    retain_stop(session, plan, transfer, reason)
}

fn stop_admitted<'session>(
    mut admitted: UiIntentConsequenceAdmitted<'session>,
    reason: crate::runtime::intent_execution::UiIntentConsequenceStopReason,
) -> UiIntentConsequencePublicationOutcome<'session> {
    withdraw_query(&mut admitted);
    retain_stop(admitted.session, admitted.plan, admitted.transfer, reason)
}

fn withdraw_query(admitted: &mut UiIntentConsequenceAdmitted<'_>) {
    if let Some(query) = admitted.query.take() {
        drop(
            admitted
                .session
                .application
                .withdraw_exact_query_change(query)
                .expect("exclusive exact Query admission must remain withdrawable"),
        );
    }
}

fn retain_stop<'session>(
    session: &'session mut WorthUiActiveApplicationSession,
    plan: crate::runtime::rebind::UiRebindPlan,
    mut transfer: WorthUiIntentConsequenceRebindTransfer,
    reason: crate::runtime::intent_execution::UiIntentConsequenceStopReason,
) -> UiIntentConsequencePublicationOutcome<'session> {
    transfer
        .consequence
        .restore_query_from_facts(plan.into_retained_facts());
    UiIntentConsequencePublicationOutcome::Stopped(
        session
            .intent_execution
            .retain_consequence_handoff(transfer.consequence, reason),
    )
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
