use super::WorthUiNativeApplicationShell;

mod posture;
mod transition;

pub use posture::{WorthUiNativeIntentPosture, WorthUiNativeIntentPostureKind};
use transition::{confirmation_stop_posture, stopped, NativePostureTarget};

/// Bounded result of moving adapter-retained observations through the
/// interaction owner and, where applicable, into one typed intent definition.
#[must_use]
pub struct WorthUiNativeIntentIngress {
    transitions: Box<[WorthUiNativeIntentTransition]>,
    duplicate_batches: usize,
    interaction_stops: Box<[WorthUiNativeInteractionIngressStop]>,
}

#[must_use]
pub enum WorthUiNativeIntentTransition {
    AttemptPrepared(WorthUiNativeIntentAttemptPrepared),
    ConfirmationRequired(WorthUiNativeIntentConfirmationRequired),
    Stopped(WorthUiNativeIntentStopped),
}

#[must_use]
pub struct WorthUiNativeIntentAttemptPrepared {
    dispatch: crate::facade::intent::UiIntentExecutionDispatchReceipt,
    posture: WorthUiNativeIntentPosture,
}

#[must_use]
pub struct WorthUiNativeIntentConfirmationRequired {
    pending: crate::facade::intent::UiPendingIntentConfirmation,
    posture: WorthUiNativeIntentPosture,
}

#[must_use]
pub struct WorthUiNativeIntentStopped {
    stop: WorthUiNativeIntentStop,
    posture: Option<WorthUiNativeIntentPosture>,
}

#[must_use]
pub enum WorthUiNativeIntentStop {
    Route(crate::facade::intent::UiIntentRouteResolutionStop),
    Payload(crate::facade::intent::UiIntentPayloadStop),
    Admission(crate::facade::intent::UiIntentAdmissionStop),
    Confirmation(crate::facade::intent::UiIntentConfirmationStop),
    Dispatch(crate::facade::intent::UiIntentExecutionDispatchStop),
    PostureIdentityExhausted,
}

#[must_use]
pub enum WorthUiNativeInteractionIngressStop {
    Quarantined(crate::facade::interaction::UiQuarantinedHostInteractionBatch),
    Denied(crate::facade::interaction::UiInteractionObservationDenial),
}

impl WorthUiNativeApplicationShell {
    /// Move one mechanically bounded adapter drain through semantic interaction,
    /// typed UI admission, confirmation continuation, and managed dispatch.
    pub fn admit_native_intent_observations<I, D>(
        &mut self,
        definition: crate::facade::intent::UiIntentDefinition<I, D>,
        drain: worth_ui_host_contract::UiHostObservationDrain,
        deadline: crate::facade::intent::UiIntentExecutionDeadlineBasis,
    ) -> WorthUiNativeIntentIngress
    where
        I: crate::facade::intent::UiIntent,
        D: crate::facade::intent::UiIntentDefinitionDestination,
    {
        let outcomes = drain
            .into_batches()
            .into_vec()
            .into_iter()
            .map(|batch| self.session.admit_host_interaction_batch(batch))
            .collect::<Vec<_>>();
        self.admit_native_intent_outcomes(definition, outcomes, deadline)
    }

    pub fn admit_native_intent_progress<I, D>(
        &mut self,
        definition: crate::facade::intent::UiIntentDefinition<I, D>,
        progress: crate::native_platform::UiNativeApplicationObservationProgress,
        deadline: crate::facade::intent::UiIntentExecutionDeadlineBasis,
    ) -> WorthUiNativeIntentIngress
    where
        I: crate::facade::intent::UiIntent,
        D: crate::facade::intent::UiIntentDefinitionDestination,
    {
        self.admit_native_intent_outcomes(
            definition,
            progress.into_settlement().into_outcomes().into_vec(),
            deadline,
        )
    }

    fn admit_native_intent_outcomes<I, D>(
        &mut self,
        definition: crate::facade::intent::UiIntentDefinition<I, D>,
        outcomes: Vec<crate::facade::interaction::UiHostInteractionIngressOutcome>,
        deadline: crate::facade::intent::UiIntentExecutionDeadlineBasis,
    ) -> WorthUiNativeIntentIngress
    where
        I: crate::facade::intent::UiIntent,
        D: crate::facade::intent::UiIntentDefinitionDestination,
    {
        let mut transitions = Vec::new();
        let mut duplicate_batches = 0;
        let mut interaction_stops = Vec::new();
        for outcome in outcomes {
            match outcome {
                crate::facade::interaction::UiHostInteractionIngressOutcome::Applied(receipt) => {
                    for transition in receipt.into_transitions() {
                        if let crate::facade::interaction::UiInteractionTransition::Semantic(
                            interaction,
                        ) = transition
                        {
                            transitions.push(self.admit_native_semantic_intent(
                                definition,
                                interaction,
                                deadline,
                            ));
                        }
                    }
                }
                crate::facade::interaction::UiHostInteractionIngressOutcome::Duplicate(_) => {
                    duplicate_batches += 1;
                }
                crate::facade::interaction::UiHostInteractionIngressOutcome::Quarantined(stop) => {
                    interaction_stops.push(WorthUiNativeInteractionIngressStop::Quarantined(stop));
                }
                crate::facade::interaction::UiHostInteractionIngressOutcome::Denied(stop) => {
                    interaction_stops.push(WorthUiNativeInteractionIngressStop::Denied(stop));
                }
            }
        }
        WorthUiNativeIntentIngress {
            transitions: transitions.into_boxed_slice(),
            duplicate_batches,
            interaction_stops: interaction_stops.into_boxed_slice(),
        }
    }

    pub fn host_session_identity(&self) -> crate::facade::WorthUiHostSessionIdentity {
        self.session.host_session_identity()
    }

    fn admit_native_semantic_intent<I, D>(
        &mut self,
        definition: crate::facade::intent::UiIntentDefinition<I, D>,
        interaction: crate::facade::interaction::UiSemanticInteraction,
        deadline: crate::facade::intent::UiIntentExecutionDeadlineBasis,
    ) -> WorthUiNativeIntentTransition
    where
        I: crate::facade::intent::UiIntent,
        D: crate::facade::intent::UiIntentDefinitionDestination,
    {
        let route = match self.session.resolve_intent_route(
            crate::facade::intent::UiIntentRouteSource::mounted_interaction(interaction),
        ) {
            Ok(route) => route,
            Err(stop) => return stopped(WorthUiNativeIntentStop::Route(stop), None),
        };
        let (admission, target) = match route {
            crate::facade::intent::UiIntentRouteResolution::Product(route) => {
                let target = NativePostureTarget::product(&route);
                let payload = match self.session.prepare_intent_payload(route) {
                    Ok(payload) => payload,
                    Err(stop) => {
                        let posture = self.prepare_native_route_posture(
                            target,
                            crate::fact_contract::UiIntentPostureKind::Denied,
                        );
                        return stopped(WorthUiNativeIntentStop::Payload(stop), posture);
                    }
                };
                let operability = self.session.evaluate_intent_operability(payload);
                (self.session.admit_intent(definition, operability), target)
            }
            crate::facade::intent::UiIntentRouteResolution::Confirmation(route) => {
                let target = NativePostureTarget::confirmation(&route);
                match self.session.continue_intent_confirmation(route) {
                    crate::facade::intent::UiIntentConfirmationContinuation::AdmissionReady(
                        candidate,
                    ) => (
                        self.session.admit_confirmed_intent(definition, candidate),
                        target,
                    ),
                    crate::facade::intent::UiIntentConfirmationContinuation::Stopped(stop) => {
                        let kind = confirmation_stop_posture(stop.reason());
                        let posture = self.prepare_native_route_posture(target, kind);
                        return stopped(WorthUiNativeIntentStop::Confirmation(stop), posture);
                    }
                }
            }
        };
        self.finish_native_admission(admission, target, deadline)
    }

    fn finish_native_admission<I: crate::facade::intent::UiIntent>(
        &mut self,
        admission: crate::facade::intent::UiIntentAdmissionDecision<I>,
        target: NativePostureTarget,
        deadline: crate::facade::intent::UiIntentExecutionDeadlineBasis,
    ) -> WorthUiNativeIntentTransition {
        match admission {
            crate::facade::intent::UiIntentAdmissionDecision::Admitted(admitted) => {
                match self.session.dispatch_admitted_intent(admitted, deadline) {
                    crate::facade::intent::UiIntentExecutionDispatchOutcome::AttemptPrepared(
                        dispatch,
                    ) => match self.prepare_native_attempt_posture(
                        target,
                        dispatch,
                        crate::fact_contract::UiIntentPostureKind::Admitted,
                    ) {
                        Some(posture) => WorthUiNativeIntentTransition::AttemptPrepared(
                            WorthUiNativeIntentAttemptPrepared { dispatch, posture },
                        ),
                        None => stopped(WorthUiNativeIntentStop::PostureIdentityExhausted, None),
                    },
                    crate::facade::intent::UiIntentExecutionDispatchOutcome::Stopped(stop) => {
                        let posture = self.prepare_native_route_posture(
                            target,
                            crate::fact_contract::UiIntentPostureKind::Denied,
                        );
                        stopped(WorthUiNativeIntentStop::Dispatch(stop), posture)
                    }
                }
            }
            crate::facade::intent::UiIntentAdmissionDecision::ConfirmationRequired(pending) => {
                match self.prepare_native_confirmation_posture(target, &pending) {
                    Some(posture) => WorthUiNativeIntentTransition::ConfirmationRequired(
                        WorthUiNativeIntentConfirmationRequired { pending, posture },
                    ),
                    None => stopped(WorthUiNativeIntentStop::PostureIdentityExhausted, None),
                }
            }
            crate::facade::intent::UiIntentAdmissionDecision::Stopped(stop) => {
                let posture = self.prepare_native_route_posture(
                    target,
                    crate::fact_contract::UiIntentPostureKind::Denied,
                );
                stopped(WorthUiNativeIntentStop::Admission(stop), posture)
            }
        }
    }

    fn prepare_native_route_posture(
        &self,
        target: NativePostureTarget,
        kind: crate::fact_contract::UiIntentPostureKind,
    ) -> Option<WorthUiNativeIntentPosture> {
        self.prepare_native_posture(
            target,
            crate::fact_contract::UiIntentPostureReference::Route(target.definition),
            kind,
        )
    }

    fn prepare_native_confirmation_posture(
        &self,
        target: NativePostureTarget,
        pending: &crate::facade::intent::UiPendingIntentConfirmation,
    ) -> Option<WorthUiNativeIntentPosture> {
        self.prepare_native_posture(
            target,
            crate::fact_contract::UiIntentPostureReference::Confirmation {
                slot: pending.slot_identity(),
                lineage: pending.lineage(),
            },
            crate::fact_contract::UiIntentPostureKind::ConfirmationRequired,
        )
    }

    fn prepare_native_attempt_posture(
        &self,
        target: NativePostureTarget,
        dispatch: crate::facade::intent::UiIntentExecutionDispatchReceipt,
        kind: crate::fact_contract::UiIntentPostureKind,
    ) -> Option<WorthUiNativeIntentPosture> {
        self.prepare_native_posture(
            target,
            crate::fact_contract::UiIntentPostureReference::Attempt {
                attempt: dispatch.attempt(),
                idempotency: dispatch.idempotency(),
            },
            kind,
        )
    }

    fn prepare_native_posture(
        &self,
        target: NativePostureTarget,
        reference: crate::fact_contract::UiIntentPostureReference,
        kind: crate::fact_contract::UiIntentPostureKind,
    ) -> Option<WorthUiNativeIntentPosture> {
        let (observation, commit) = self.session.intent_postures.prepare(
            target.graph_node,
            target.target,
            reference,
            kind,
        )?;
        Some(WorthUiNativeIntentPosture::new(observation, commit, kind))
    }
}

impl WorthUiNativeIntentIngress {
    pub fn transitions(&self) -> &[WorthUiNativeIntentTransition] {
        &self.transitions
    }

    pub fn into_transitions(self) -> Box<[WorthUiNativeIntentTransition]> {
        self.transitions
    }

    pub const fn duplicate_batches(&self) -> usize {
        self.duplicate_batches
    }

    pub fn interaction_stops(&self) -> &[WorthUiNativeInteractionIngressStop] {
        &self.interaction_stops
    }
}

impl WorthUiNativeIntentAttemptPrepared {
    pub const fn dispatch(&self) -> crate::facade::intent::UiIntentExecutionDispatchReceipt {
        self.dispatch
    }

    pub fn into_posture(self) -> WorthUiNativeIntentPosture {
        self.posture
    }
}

impl WorthUiNativeIntentConfirmationRequired {
    pub const fn pending(&self) -> &crate::facade::intent::UiPendingIntentConfirmation {
        &self.pending
    }

    pub fn into_parts(
        self,
    ) -> (
        crate::facade::intent::UiPendingIntentConfirmation,
        WorthUiNativeIntentPosture,
    ) {
        (self.pending, self.posture)
    }
}

impl WorthUiNativeIntentStopped {
    pub const fn stop(&self) -> &WorthUiNativeIntentStop {
        &self.stop
    }

    pub fn into_parts(self) -> (WorthUiNativeIntentStop, Option<WorthUiNativeIntentPosture>) {
        (self.stop, self.posture)
    }
}
