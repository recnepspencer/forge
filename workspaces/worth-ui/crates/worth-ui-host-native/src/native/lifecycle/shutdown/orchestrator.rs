use super::UiNativeShutdownPhase;

pub(in crate::native::lifecycle) trait UiNativeShutdownPort {
    type Census: Copy;

    fn begin_close(&mut self);
    fn settle_external_effects(&mut self) -> bool;
    fn release_derived_state(&mut self);
    fn release_native_resources(&mut self) -> bool;
    fn census(&self) -> Self::Census;
    fn terminal_zero(&self) -> bool;
}

pub(in crate::native::lifecycle) fn progress<Port: UiNativeShutdownPort>(
    phase: &mut UiNativeShutdownPhase,
    port: &mut Port,
) -> Port::Census {
    if *phase == UiNativeShutdownPhase::Closed {
        return port.census();
    }
    if *phase == UiNativeShutdownPhase::Open {
        port.begin_close();
        *phase = UiNativeShutdownPhase::SettlingExternalEffects;
    }
    if *phase == UiNativeShutdownPhase::SettlingExternalEffects {
        if !port.settle_external_effects() {
            return port.census();
        }
        *phase = UiNativeShutdownPhase::ReleasingDerivedState;
    }
    if *phase == UiNativeShutdownPhase::ReleasingDerivedState {
        port.release_derived_state();
        *phase = UiNativeShutdownPhase::ReleasingNativeResources;
    }
    if *phase == UiNativeShutdownPhase::ReleasingNativeResources {
        if !port.release_native_resources() {
            return port.census();
        }
        if port.terminal_zero() {
            *phase = UiNativeShutdownPhase::Closed;
        }
    }
    port.census()
}
