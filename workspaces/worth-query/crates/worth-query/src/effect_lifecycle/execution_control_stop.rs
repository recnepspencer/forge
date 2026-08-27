#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectExecutionControlStopKind {
    Cancelled,
    TimedOut,
}
