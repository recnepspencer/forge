#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeLifecyclePhase {
    BeforeFirstPresentation,
    Presented,
    SuccessorInFlight,
    ProfileTransition,
    Closing,
    Closed,
}
