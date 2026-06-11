#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetainedCancellationChainCounters {
    checkpoint_count: usize,
    transform_step_count: usize,
    replayed_checkpoint_count: usize,
    trigger_local_replay_count: usize,
    retained_artifact_count: usize,
    projection_consumed_fact_count: usize,
    diagnostic_trigger_count: usize,
    user_outcome_count: usize,
}

impl RetainedCancellationChainCounters {
    pub(crate) fn new(input: RetainedCancellationChainCounterInput) -> Self {
        Self {
            checkpoint_count: input.checkpoint_count,
            transform_step_count: input.transform_step_count,
            replayed_checkpoint_count: input.replayed_checkpoint_count,
            trigger_local_replay_count: input.trigger_local_replay_count,
            retained_artifact_count: input.retained_artifact_count,
            projection_consumed_fact_count: input.projection_consumed_fact_count,
            diagnostic_trigger_count: input.diagnostic_trigger_count,
            user_outcome_count: input.user_outcome_count,
        }
    }

    pub fn checkpoint_count(self) -> usize {
        self.checkpoint_count
    }

    pub fn transform_step_count(self) -> usize {
        self.transform_step_count
    }

    pub fn replayed_checkpoint_count(self) -> usize {
        self.replayed_checkpoint_count
    }

    pub fn trigger_local_replay_count(self) -> usize {
        self.trigger_local_replay_count
    }

    pub fn retained_artifact_count(self) -> usize {
        self.retained_artifact_count
    }

    pub fn projection_consumed_fact_count(self) -> usize {
        self.projection_consumed_fact_count
    }

    pub fn diagnostic_trigger_count(self) -> usize {
        self.diagnostic_trigger_count
    }

    pub fn user_outcome_count(self) -> usize {
        self.user_outcome_count
    }
}

pub(crate) struct RetainedCancellationChainCounterInput {
    pub(crate) checkpoint_count: usize,
    pub(crate) transform_step_count: usize,
    pub(crate) replayed_checkpoint_count: usize,
    pub(crate) trigger_local_replay_count: usize,
    pub(crate) retained_artifact_count: usize,
    pub(crate) projection_consumed_fact_count: usize,
    pub(crate) diagnostic_trigger_count: usize,
    pub(crate) user_outcome_count: usize,
}
