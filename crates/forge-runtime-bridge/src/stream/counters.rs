#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StreamProtocolCounters {
    stream_member_count: usize,
    stream_window_count: usize,
    stream_window_member_count: usize,
    stream_consumer_contract_count: usize,
    stream_checkpoint_count: usize,
    stream_checkpoint_member_count: usize,
    stream_resume_attempt_count: usize,
    stream_resume_rejection_count: usize,
    stream_replay_count: usize,
    stream_replay_mismatch_count: usize,
    stream_coalesced_member_count: usize,
    stream_coalesced_window_count: usize,
    stream_duplicate_member_observation_count: usize,
    stream_backpressure_signal_count: usize,
    stream_consumer_saturated_count: usize,
    stream_checkpoint_lag_count: usize,
    stream_protocol_mismatch_count: usize,
}

impl StreamProtocolCounters {
    pub(crate) fn for_planned_window(member_count: usize, coalesced: bool) -> Self {
        Self {
            stream_member_count: member_count,
            stream_window_count: 1,
            stream_window_member_count: member_count,
            stream_consumer_contract_count: 1,
            stream_coalesced_member_count: if coalesced { member_count } else { 0 },
            stream_coalesced_window_count: usize::from(coalesced),
            ..Self::default()
        }
    }

    pub(crate) fn with_checkpoint(mut self, checkpoint_member_count: usize) -> Self {
        self.stream_checkpoint_count = 1;
        self.stream_checkpoint_member_count = checkpoint_member_count;
        self
    }

    pub(crate) fn with_resume_attempt(mut self, rejected: bool, lagged: bool) -> Self {
        self.stream_resume_attempt_count = 1;
        self.stream_resume_rejection_count = usize::from(rejected);
        self.stream_checkpoint_lag_count = usize::from(lagged);
        self
    }

    pub(crate) fn with_replay(mut self, mismatched: bool) -> Self {
        self.stream_replay_count = 1;
        self.stream_replay_mismatch_count = usize::from(mismatched);
        self
    }

    pub(crate) fn with_backpressure(mut self, pressured: bool, saturated: bool) -> Self {
        self.stream_backpressure_signal_count = usize::from(pressured);
        self.stream_consumer_saturated_count = usize::from(saturated);
        self
    }

    pub fn stream_member_count(&self) -> usize {
        self.stream_member_count
    }
    pub fn stream_window_count(&self) -> usize {
        self.stream_window_count
    }
    pub fn stream_window_member_count(&self) -> usize {
        self.stream_window_member_count
    }
    pub fn stream_consumer_contract_count(&self) -> usize {
        self.stream_consumer_contract_count
    }
    pub fn stream_checkpoint_count(&self) -> usize {
        self.stream_checkpoint_count
    }
    pub fn stream_checkpoint_member_count(&self) -> usize {
        self.stream_checkpoint_member_count
    }
    pub fn stream_resume_attempt_count(&self) -> usize {
        self.stream_resume_attempt_count
    }
    pub fn stream_resume_rejection_count(&self) -> usize {
        self.stream_resume_rejection_count
    }
    pub fn stream_replay_count(&self) -> usize {
        self.stream_replay_count
    }
    pub fn stream_replay_mismatch_count(&self) -> usize {
        self.stream_replay_mismatch_count
    }
    pub fn stream_coalesced_member_count(&self) -> usize {
        self.stream_coalesced_member_count
    }
    pub fn stream_coalesced_window_count(&self) -> usize {
        self.stream_coalesced_window_count
    }
    pub fn stream_duplicate_member_observation_count(&self) -> usize {
        self.stream_duplicate_member_observation_count
    }
    pub fn stream_backpressure_signal_count(&self) -> usize {
        self.stream_backpressure_signal_count
    }
    pub fn stream_consumer_saturated_count(&self) -> usize {
        self.stream_consumer_saturated_count
    }
    pub fn stream_checkpoint_lag_count(&self) -> usize {
        self.stream_checkpoint_lag_count
    }
    pub fn stream_protocol_mismatch_count(&self) -> usize {
        self.stream_protocol_mismatch_count
    }
}
