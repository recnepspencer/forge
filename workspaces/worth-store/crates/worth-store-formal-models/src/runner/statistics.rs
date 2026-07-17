#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtocolCheckStatistics {
    initial_states: u64,
    generated_states: u64,
    distinct_states: u64,
    states_left_on_queue: u64,
    trace_depth: u64,
}

impl ProtocolCheckStatistics {
    pub const fn observed(
        initial_states: u64,
        generated_states: u64,
        distinct_states: u64,
        states_left_on_queue: u64,
        trace_depth: u64,
    ) -> Self {
        Self {
            initial_states,
            generated_states,
            distinct_states,
            states_left_on_queue,
            trace_depth,
        }
    }

    pub const fn initial_states(self) -> u64 {
        self.initial_states
    }

    pub const fn generated_states(self) -> u64 {
        self.generated_states
    }

    pub const fn distinct_states(self) -> u64 {
        self.distinct_states
    }

    pub const fn states_left_on_queue(self) -> u64 {
        self.states_left_on_queue
    }

    pub const fn trace_depth(self) -> u64 {
        self.trace_depth
    }
}
