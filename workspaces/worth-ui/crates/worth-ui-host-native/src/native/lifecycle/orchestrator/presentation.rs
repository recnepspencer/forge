use crate::native::presentation::port::orchestrator::{
    self as presentation_orchestrator, UiNativePresentationStagePort,
};
#[cfg(feature = "certification-support")]
use crate::native::presentation::port::orchestrator::{
    UiNativePresentationStageControl, UiNativePresentationStageFailure,
};
use crate::native::{UiNativeEffectPosture, UiNativePresentationEffectPhase};

impl super::UiNativeLifecycleOrchestrator {
    pub(crate) fn run_presentation<Port: UiNativePresentationStagePort>(
        &mut self,
        port: &mut Port,
    ) -> Result<Port::Observation, Port::Failure> {
        presentation_orchestrator::run(port, &mut self.effect_posture)
    }

    #[cfg(feature = "certification-support")]
    pub(crate) fn run_controlled_presentation<Port, Control>(
        &mut self,
        port: &mut Port,
        control: &mut Control,
    ) -> Result<Port::Observation, UiNativePresentationStageFailure<Port::Failure, Control::Stop>>
    where
        Port: UiNativePresentationStagePort,
        Control: UiNativePresentationStageControl,
    {
        presentation_orchestrator::run_controlled(port, control, &mut self.effect_posture)
    }

    pub(crate) const fn effect_posture(&self) -> UiNativeEffectPosture {
        self.effect_posture
    }

    pub(crate) fn record_presented(&mut self) {
        self.effect_posture = UiNativeEffectPosture::Presented;
    }

    pub(crate) fn record_presentation_indeterminate(&mut self) {
        self.effect_posture = UiNativeEffectPosture::PresentationIndeterminate;
    }

    pub(crate) fn record_presentation_stage(&mut self, stage: UiNativePresentationEffectPhase) {
        self.effect_posture = UiNativeEffectPosture::Presentation(stage);
    }

    #[cfg(feature = "certification-support")]
    pub(crate) fn reset_presentation_effects(&mut self) {
        self.effect_posture = UiNativeEffectPosture::BeforeEffects;
    }
}
