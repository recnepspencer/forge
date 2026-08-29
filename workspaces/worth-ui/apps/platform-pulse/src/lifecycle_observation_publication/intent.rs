use worth_ui_platform_pulse::observation_contract::{
    PlatformPulseIntentCausalTraceObservation, PlatformPulseIntentExecutorStartedObservation,
    PlatformPulseIntentInputObservation, PlatformPulseIntentPostureObservation,
    PlatformPulseIntentRoutingStoppedObservation, PlatformPulseLifecycleObservationStream,
    PlatformPulseQueryActionObservation,
};

use super::{PlatformPulseObservationPublicationDenial, PlatformPulseObservationPublisher};

impl PlatformPulseObservationPublisher {
    pub(crate) fn intent_routing_stopped(
        &self,
        observation: PlatformPulseIntentRoutingStoppedObservation,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        self.project_observation(|stream| stream.project_intent_routing_stopped(observation))
    }

    pub(crate) fn intent_causal_trace(
        &self,
        observation: PlatformPulseIntentCausalTraceObservation,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        self.project_observation(|stream| stream.project_intent_causal_trace(observation))
    }

    pub(crate) fn intent_input_admitted(
        &self,
        record: &worth_ui_platform_pulse::intent::PlatformPulseIntentInputRecord,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        let observation = PlatformPulseIntentInputObservation::from_record(record);
        self.project_observation(|stream| stream.project_intent_input_admitted(observation))
    }

    pub(crate) fn intent_preparation_failure(
        &self,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        self.project_observation(
            PlatformPulseLifecycleObservationStream::project_intent_preparation_failure,
        )
    }

    pub(crate) fn query_action(
        &self,
        observation: PlatformPulseQueryActionObservation,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        self.project_observation(|stream| stream.project_query_action(observation))
    }

    pub(crate) fn intent_executor_started(
        &self,
        observation: PlatformPulseIntentExecutorStartedObservation,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        self.project_observation(|stream| stream.project_intent_executor_started(observation))
    }

    pub(crate) fn intent_posture_published(
        &self,
        posture: PlatformPulseIntentPostureObservation,
        publication: &worth_ui::facade::app::UiMountedFramePublicationReceipt,
        latest_command_transition: Option<
            worth_ui_platform_pulse::observation_contract::PlatformPulseCommandTransitionInspection,
        >,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        self.project_observation(|stream| {
            stream.project_intent_posture_published(posture, publication, latest_command_transition)
        })
    }
}
