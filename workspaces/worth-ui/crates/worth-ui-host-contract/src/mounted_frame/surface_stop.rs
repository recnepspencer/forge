/// Why an admitted mounted-surface presentation is being stopped.
///
/// Cancellation abandons the current presentation without a successor. Supersession
/// replaces it with newer mounted work and therefore advances the native physical
/// lifecycle through its distinct recovery posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostSurfaceStopReason {
    Cancelled,
    Superseded,
}
