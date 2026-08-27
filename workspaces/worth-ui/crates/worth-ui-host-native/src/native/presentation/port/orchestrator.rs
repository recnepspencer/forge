pub(crate) trait UiNativePresentationStagePort {
    type Prepared;
    type Acquired;
    type Encoded;
    type Submitted;
    type PresentHandoff;
    type Observation;
    type Failure;

    fn prepare(&mut self) -> Result<Self::Prepared, Self::Failure>;
    fn acquire(&mut self, prepared: Self::Prepared) -> Result<Self::Acquired, Self::Failure>;
    fn encode(&mut self, acquired: Self::Acquired) -> Result<Self::Encoded, Self::Failure>;
    fn submit(&mut self, encoded: Self::Encoded) -> Result<Self::Submitted, Self::Failure>;
    fn hand_off(
        &mut self,
        submitted: Self::Submitted,
    ) -> Result<Self::PresentHandoff, Self::Failure>;
    fn observe(
        &mut self,
        handoff: Self::PresentHandoff,
    ) -> Result<Self::Observation, Self::Failure>;
}

pub(crate) trait UiNativePresentationStageControl {
    type Stop;

    fn stage_completed(
        &mut self,
        stage: crate::native::UiNativePresentationEffectPhase,
    ) -> Result<(), Self::Stop>;
}

pub(crate) enum UiNativePresentationStageFailure<PortFailure, ControlStop> {
    Port(PortFailure),
    Control(ControlStop),
}

pub(crate) fn run<P: UiNativePresentationStagePort>(
    port: &mut P,
    effect_posture: &mut crate::native::UiNativeEffectPosture,
) -> Result<P::Observation, P::Failure> {
    match run_controlled(port, &mut UiNativeOpenPresentationStages, effect_posture) {
        Ok(observation) => Ok(observation),
        Err(UiNativePresentationStageFailure::Port(failure)) => Err(failure),
        Err(UiNativePresentationStageFailure::Control(never)) => match never {},
    }
}

pub(crate) fn run_controlled<Port, Control>(
    port: &mut Port,
    control: &mut Control,
    effect_posture: &mut crate::native::UiNativeEffectPosture,
) -> Result<Port::Observation, UiNativePresentationStageFailure<Port::Failure, Control::Stop>>
where
    Port: UiNativePresentationStagePort,
    Control: UiNativePresentationStageControl,
{
    let prepared = port
        .prepare()
        .map_err(UiNativePresentationStageFailure::Port)?;
    complete_stage(
        control,
        effect_posture,
        crate::native::UiNativePresentationEffectPhase::Prepared,
    )?;
    let acquired = port
        .acquire(prepared)
        .map_err(UiNativePresentationStageFailure::Port)?;
    complete_stage(
        control,
        effect_posture,
        crate::native::UiNativePresentationEffectPhase::SurfaceAcquired,
    )?;
    let encoded = port
        .encode(acquired)
        .map_err(UiNativePresentationStageFailure::Port)?;
    complete_stage(
        control,
        effect_posture,
        crate::native::UiNativePresentationEffectPhase::Encoded,
    )?;
    let submitted = port
        .submit(encoded)
        .map_err(UiNativePresentationStageFailure::Port)?;
    complete_stage(
        control,
        effect_posture,
        crate::native::UiNativePresentationEffectPhase::Submitted,
    )?;
    let handoff = port
        .hand_off(submitted)
        .map_err(UiNativePresentationStageFailure::Port)?;
    complete_stage(
        control,
        effect_posture,
        crate::native::UiNativePresentationEffectPhase::PresentHandoff,
    )?;
    port.observe(handoff)
        .map_err(UiNativePresentationStageFailure::Port)
}

fn complete_stage<PortFailure, Control: UiNativePresentationStageControl>(
    control: &mut Control,
    effect_posture: &mut crate::native::UiNativeEffectPosture,
    stage: crate::native::UiNativePresentationEffectPhase,
) -> Result<(), UiNativePresentationStageFailure<PortFailure, Control::Stop>> {
    *effect_posture = crate::native::UiNativeEffectPosture::Presentation(stage);
    control
        .stage_completed(stage)
        .map_err(UiNativePresentationStageFailure::Control)
}

struct UiNativeOpenPresentationStages;

impl UiNativePresentationStageControl for UiNativeOpenPresentationStages {
    type Stop = std::convert::Infallible;

    fn stage_completed(
        &mut self,
        _stage: crate::native::UiNativePresentationEffectPhase,
    ) -> Result<(), Self::Stop> {
        Ok(())
    }
}
