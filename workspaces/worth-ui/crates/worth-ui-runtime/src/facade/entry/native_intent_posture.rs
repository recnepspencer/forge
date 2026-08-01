use super::{
    intent_consequence_observation::prepare_intent_consequence_observation,
    WorthUiActiveApplicationSession, WorthUiNativeApplicationShell, WorthUiNativeIntentPosture,
};

mod execution;

use execution::{
    NativeIntentPostureInFlight, NativeIntentPostureIndeterminate,
    PreparedNativeIntentPostureRebind,
};

pub enum WorthUiNativeIntentPosturePublicationOutcome<'session> {
    Stopped(WorthUiNativeIntentPosturePublicationStop),
    Published(crate::runtime::rebind::UiRebindReceipt),
    InFlight(WorthUiNativeIntentPosturePublicationCompletion<'session>),
    Indeterminate(WorthUiNativeIntentPosturePublicationRecovery<'session>),
    InternalDefect(crate::runtime::rebind::UiRebindInternalDefectOutcome),
}

#[must_use = "posture publication must be completed or explicitly disposed"]
pub struct WorthUiNativeIntentPosturePublicationCompletion<'session> {
    state: Option<Box<NativeIntentPostureInFlight<'session>>>,
}

#[must_use = "indeterminate posture publication requires reconciliation or shutdown"]
pub struct WorthUiNativeIntentPosturePublicationRecovery<'session> {
    state: Option<Box<NativeIntentPostureIndeterminate<'session>>>,
}

pub struct WorthUiNativeIntentPosturePublicationStop {
    reason: crate::runtime::intent_execution::UiIntentConsequenceStopReason,
}

struct NativeIntentPostureTransfer {
    observation: crate::runtime::observation::UiPreparedObservationProgressCommit,
    posture: crate::mounting::UiIntentPostureCommit,
}

impl WorthUiNativeApplicationShell {
    /// Publish a runtime-minted posture through the canonical non-source 3.12
    /// observation, planning, presentation, and commit path.
    pub fn publish_native_intent_posture(
        &mut self,
        posture: WorthUiNativeIntentPosture,
        now_tick: u64,
    ) -> WorthUiNativeIntentPosturePublicationOutcome<'_> {
        self.session.publish_native_intent_posture(
            posture,
            crate::runtime::rebind::UiRebindExecutionPolicy::ordinary(),
            crate::runtime::rebind::UiRebindExecutionRequest::new(now_tick),
        )
    }
}

impl WorthUiActiveApplicationSession {
    fn publish_native_intent_posture(
        &mut self,
        posture: WorthUiNativeIntentPosture,
        policy: crate::runtime::rebind::UiRebindExecutionPolicy,
        execution: crate::runtime::rebind::UiRebindExecutionRequest,
    ) -> WorthUiNativeIntentPosturePublicationOutcome<'_> {
        let batch = crate::runtime::observation::UiIntentConsequenceObservationBatch::new(
            Some((posture.observation, posture.commit)),
            None,
            None,
        );
        let observation = match prepare_intent_consequence_observation(
            &mut self.application,
            self.identity,
            batch,
        ) {
            Ok(observation) => observation,
            Err(stop) => return stopped(stop.reason),
        };
        let change = self
            .application
            .classify_intent_consequence(self.identity, observation.set);
        let scope = match crate::runtime::rebind::UiAffectedScopeResolver::resolve_recoverable(
            change,
            self.identity,
            self.application.prepared_authority(),
        ) {
            Ok(scope) => scope,
            Err(stop) => {
                let (denial, _) = stop.into_parts();
                return stopped(
                    crate::runtime::intent_execution::UiIntentConsequenceStopReason::AffectedScope(
                        Box::new(denial),
                    ),
                );
            }
        };
        let lifecycle =
            match crate::runtime::rebind::UiIdentityLifecycleResolver::resolve_recoverable(scope) {
                Ok(lifecycle) => lifecycle,
                Err(stop) => {
                    let (denial, _) = stop.into_parts();
                    return stopped(
                        crate::runtime::intent_execution::UiIntentConsequenceStopReason::IdentityLifecycle(
                            Box::new(denial),
                        ),
                    );
                }
            };
        let plan = match self.application.compile_non_source_rebind_recoverable(
            self.identity,
            lifecycle,
            policy,
        ) {
            Ok(plan) => plan,
            Err(stop) => {
                let (denial, _) = stop.into_parts();
                return stopped(
                    crate::runtime::intent_execution::UiIntentConsequenceStopReason::Planning(
                        Box::new(denial),
                    ),
                );
            }
        };
        let transfer = NativeIntentPostureTransfer {
            observation: observation.progress,
            posture: observation
                .posture
                .expect("posture-only observation retains one posture commit"),
        };
        match self.prepare_native_intent_posture_rebind(plan, execution, transfer) {
            Ok(prepared) => prepared.execute(),
            Err(reason) => stopped(reason),
        }
    }

    fn prepare_native_intent_posture_rebind(
        &mut self,
        plan: crate::runtime::rebind::UiRebindPlan,
        request: crate::runtime::rebind::UiRebindExecutionRequest,
        transfer: NativeIntentPostureTransfer,
    ) -> Result<
        PreparedNativeIntentPostureRebind<'_>,
        crate::runtime::intent_execution::UiIntentConsequenceStopReason,
    > {
        let now_tick = request.now_tick();
        if !plan.has_non_source_semantic_proof() {
            return Err(
                crate::runtime::intent_execution::UiIntentConsequenceStopReason::Preparation(
                    Box::new(
                        crate::runtime::rebind::UiRebindPreparationDenial::InvalidSemanticProof,
                    ),
                ),
            );
        }
        let reservation = crate::runtime::rebind::admit_plan(
            &self.rebind,
            crate::runtime::rebind::UiRebindFinalAdmissionBasis::new(
                self.identity,
                self.capabilities().digest().as_u64(),
                self.generation_identity(),
            ),
            &plan,
            request,
        )
        .map_err(|denial| {
            crate::runtime::intent_execution::UiIntentConsequenceStopReason::Preparation(Box::new(
                denial,
            ))
        })?;
        let frame = self
            .prepare_intent_consequence_frame(plan.content().clone())
            .map_err(|denial| {
                crate::runtime::intent_execution::UiIntentConsequenceStopReason::Preparation(
                    Box::new(denial),
                )
            })?;
        Ok(PreparedNativeIntentPostureRebind {
            session: self,
            plan,
            reservation,
            frame,
            transfer,
            now_tick,
        })
    }
}

impl WorthUiNativeIntentPosturePublicationStop {
    pub const fn reason(&self) -> &crate::runtime::intent_execution::UiIntentConsequenceStopReason {
        &self.reason
    }
}

pub(super) fn stopped<'session>(
    reason: crate::runtime::intent_execution::UiIntentConsequenceStopReason,
) -> WorthUiNativeIntentPosturePublicationOutcome<'session> {
    WorthUiNativeIntentPosturePublicationOutcome::Stopped(
        WorthUiNativeIntentPosturePublicationStop { reason },
    )
}
