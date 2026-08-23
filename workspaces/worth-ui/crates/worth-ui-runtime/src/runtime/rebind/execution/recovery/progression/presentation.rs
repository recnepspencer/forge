use super::*;

enum RecoveryAttempt {
    Initial(UiRebindReconciliationRequest),
    Completion(UiRebindReconciliationRequest),
}

impl RecoveryAttempt {
    fn into_request(self) -> UiRebindReconciliationRequest {
        match self {
            Self::Initial(request) | Self::Completion(request) => request,
        }
    }
}

impl<'session> UiRebindReconciliation<'session> {
    pub fn present_current(
        mut self,
        request: UiRebindReconciliationRequest,
        now_tick: u64,
    ) -> UiRebindRecoveryOutcome<'session> {
        let outcome =
            self.authority
                .present_current(request.replacements(), request.deadline(), now_tick);
        match outcome {
            Ok(outcome) => {
                map_reconciliation_outcome(self, RecoveryAttempt::Initial(request), outcome)
            }
            Err(denial) => rejected(
                self,
                request,
                UiRebindRecoveryDenialCause::MountedIdentity(denial),
            ),
        }
    }
}

impl<'session> UiRebindRecoveryCompletionHandle<'session> {
    fn new(
        reconciliation: UiRebindReconciliation<'session>,
        in_flight: crate::mounting::UiMountedPresentationInFlight,
        request: UiRebindReconciliationRequest,
    ) -> Self {
        Self {
            state: Some(Box::new(UiRebindRecoveryCompletionState {
                reconciliation,
                in_flight,
                request,
            })),
        }
    }

    pub fn attempt(&self) -> worth_ui_host_contract::UiMountedPresentationAttemptIdentity {
        self.state().in_flight.attempt()
    }

    pub fn deadline(&self) -> worth_ui_host_contract::UiPresentationDeadline {
        self.state().in_flight.deadline()
    }

    pub fn complete(self, now_tick: u64) -> UiRebindRecoveryOutcome<'session> {
        let mut state = self.into_state();
        let outcome = state
            .reconciliation
            .authority
            .complete(state.in_flight, now_tick);
        map_reconciliation_outcome(
            state.reconciliation,
            RecoveryAttempt::Completion(state.request),
            outcome,
        )
    }

    pub fn dispose(self) -> UiRebindRecoveryOutcome<'session> {
        let mut state = self.into_state();
        let outcome = state.reconciliation.authority.cancel(state.in_flight);
        map_reconciliation_outcome(
            state.reconciliation,
            RecoveryAttempt::Completion(state.request),
            outcome,
        )
    }

    fn state(&self) -> &UiRebindRecoveryCompletionState<'session> {
        self.state
            .as_deref()
            .expect("live recovery completion owns its state")
    }

    fn into_state(mut self) -> Box<UiRebindRecoveryCompletionState<'session>> {
        self.state
            .take()
            .expect("live recovery completion owns its state")
    }
}

impl Drop for UiRebindRecoveryCompletionHandle<'_> {
    fn drop(&mut self) {
        let Some(mut state) = self.state.take() else {
            return;
        };
        let outcome = state.reconciliation.authority.cancel(state.in_flight);
        drop(map_reconciliation_outcome(
            state.reconciliation,
            RecoveryAttempt::Completion(state.request),
            outcome,
        ));
    }
}

fn map_reconciliation_outcome<'session>(
    reconciliation: UiRebindReconciliation<'session>,
    attempt: RecoveryAttempt,
    outcome: crate::mounting::UiMountedFrameOutcome,
) -> UiRebindRecoveryOutcome<'session> {
    match outcome {
        crate::mounting::UiMountedFrameOutcome::Reconciled(mounted) => {
            recovered(reconciliation, mounted)
        }
        crate::mounting::UiMountedFrameOutcome::RejectedBeforeEffects(_) => pre_effect_stop(
            reconciliation,
            attempt,
            UiRebindRecoveryDenialCause::HostRejectedBeforeEffects,
        ),
        crate::mounting::UiMountedFrameOutcome::InFlight(in_flight) => {
            UiRebindRecoveryOutcome::InFlight(UiRebindRecoveryCompletionHandle::new(
                reconciliation,
                in_flight,
                attempt.into_request(),
            ))
        }
        crate::mounting::UiMountedFrameOutcome::PresentationIndeterminate(frame) => {
            let UiRebindReconciliation {
                plan,
                registration,
                authority,
                ..
            } = reconciliation;
            UiRebindRecoveryOutcome::Indeterminate(
                super::super::UiRebindRecoveryHandle::after_reconciliation(
                    plan,
                    registration,
                    authority,
                    frame,
                ),
            )
        }
        crate::mounting::UiMountedFrameOutcome::Superseded(_) => {
            unreachable!("rebind recovery cannot overlap a superseding frame")
        }
        crate::mounting::UiMountedFrameOutcome::RetentionDenied(denial) => pre_effect_stop(
            reconciliation,
            attempt,
            UiRebindRecoveryDenialCause::MountedRetention(denial.denial()),
        ),
        crate::mounting::UiMountedFrameOutcome::AdmissionDenied(denial) => pre_effect_stop(
            reconciliation,
            attempt,
            UiRebindRecoveryDenialCause::MountedPresentation(denial.denial()),
        ),
        crate::mounting::UiMountedFrameOutcome::Published(mounted)
        | crate::mounting::UiMountedFrameOutcome::Unchanged(mounted) => internal_defect(
            reconciliation,
            UiRebindRecoveryInternalDefectKind::UnexpectedPublicationPosture,
            Some(mounted),
        ),
        crate::mounting::UiMountedFrameOutcome::CompletionDenied(_) => internal_defect(
            reconciliation,
            UiRebindRecoveryInternalDefectKind::CompletionAuthorityRejected,
            None,
        ),
    }
}

fn pre_effect_stop<'session>(
    reconciliation: UiRebindReconciliation<'session>,
    attempt: RecoveryAttempt,
    cause: UiRebindRecoveryDenialCause,
) -> UiRebindRecoveryOutcome<'session> {
    rejected(reconciliation, attempt.into_request(), cause)
}

fn recovered<'session>(
    reconciliation: UiRebindReconciliation<'session>,
    mounted: crate::mounting::UiMountedFramePublicationReceipt,
) -> UiRebindRecoveryOutcome<'session> {
    let UiRebindReconciliation {
        plan,
        registration,
        authority: _,
        affected_bindings,
    } = reconciliation;
    drop(registration);
    UiRebindRecoveryOutcome::Recovered(UiRebindRecoveryReceipt {
        plan,
        mounted,
        affected_bindings,
    })
}

fn rejected<'session>(
    reconciliation: UiRebindReconciliation<'session>,
    request: UiRebindReconciliationRequest,
    cause: UiRebindRecoveryDenialCause,
) -> UiRebindRecoveryOutcome<'session> {
    UiRebindRecoveryOutcome::RejectedBeforeEffects(Box::new(UiRebindRecoveryDenial {
        cause,
        reconciliation,
        request,
    }))
}

fn internal_defect<'session>(
    reconciliation: UiRebindReconciliation<'session>,
    kind: UiRebindRecoveryInternalDefectKind,
    unexpected_publication: Option<crate::mounting::UiMountedFramePublicationReceipt>,
) -> UiRebindRecoveryOutcome<'session> {
    UiRebindRecoveryOutcome::InternalDefect(UiRebindRecoveryInternalDefect {
        kind,
        reconciliation,
        unexpected_publication,
    })
}
