#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueExecutionProgression {
    Lowered,
    ExecutionReady,
    Executed,
}
