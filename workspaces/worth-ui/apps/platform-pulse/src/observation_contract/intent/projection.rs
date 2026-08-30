use super::{
    PlatformPulseIntentCausalTraceObservation, PlatformPulseIntentExecutorStartedObservation,
    PlatformPulseIntentInputObservation, PlatformPulseIntentPostureObservation,
    PlatformPulseIntentPosturePublished, PlatformPulseIntentRoutingStoppedObservation,
    PlatformPulseQueryActionObservation,
};
use crate::observation_contract::{
    PlatformPulseLifecycleObservation, PlatformPulseLifecycleObservationEnvelope,
    PlatformPulseLifecycleObservationProjectionDenial, PlatformPulseLifecycleObservationStream,
};

impl PlatformPulseLifecycleObservationStream {
    pub fn project_intent_routing_stopped(
        &mut self,
        observation: PlatformPulseIntentRoutingStoppedObservation,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        self.published_predecessor()?;
        self.next_envelope(PlatformPulseLifecycleObservation::IntentRoutingStopped(
            observation,
        ))
    }

    pub fn project_portal_dismissed(
        &mut self,
        publication: &worth_ui::facade::app::UiMountedFramePublicationReceipt,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        let validated = self.validate_content_publication(publication)?;
        let next_visual_state = self
            .visual_state
            .after_content_publication(validated.frame().diagnostic_value())?;
        let envelope = self.next_envelope(PlatformPulseLifecycleObservation::PortalDismissed(
            crate::observation_contract::PlatformPulsePortalDismissed::from_publication(
                publication,
            ),
        ))?;
        self.commit_content_publication(validated);
        self.visual_state = next_visual_state;
        Ok(envelope)
    }

    pub fn project_intent_causal_trace(
        &mut self,
        observation: PlatformPulseIntentCausalTraceObservation,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        self.published_predecessor()?;
        self.next_envelope(PlatformPulseLifecycleObservation::IntentCausalTrace(
            observation,
        ))
    }

    pub fn project_intent_input_admitted(
        &mut self,
        observation: PlatformPulseIntentInputObservation,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        self.published_predecessor()?;
        self.next_envelope(PlatformPulseLifecycleObservation::IntentInputAdmitted(
            observation,
        ))
    }

    pub fn project_query_action(
        &mut self,
        observation: PlatformPulseQueryActionObservation,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        self.published_predecessor()?;
        self.next_envelope(PlatformPulseLifecycleObservation::QueryAction(observation))
    }

    pub fn project_intent_executor_started(
        &mut self,
        observation: PlatformPulseIntentExecutorStartedObservation,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        self.published_predecessor()?;
        self.next_envelope(PlatformPulseLifecycleObservation::IntentExecutorStarted(
            observation,
        ))
    }

    pub fn project_intent_posture_published(
        &mut self,
        posture: PlatformPulseIntentPostureObservation,
        publication: &worth_ui::facade::app::UiMountedFramePublicationReceipt,
        latest_command_transition: Option<
            crate::observation_contract::PlatformPulseCommandTransitionInspection,
        >,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        let validated = self.validate_content_publication(publication)?;
        let next_visual_state = self
            .visual_state
            .after_content_publication(validated.frame().diagnostic_value())?;
        let envelope =
            self.next_envelope(PlatformPulseLifecycleObservation::IntentPosturePublished(
                PlatformPulseIntentPosturePublished::new(
                    posture,
                    publication,
                    latest_command_transition,
                ),
            ))?;
        self.commit_content_publication(validated);
        self.visual_state = next_visual_state;
        Ok(envelope)
    }
}
