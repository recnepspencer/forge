use std::num::NonZeroU64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtocolCheckBounds {
    maximum_states: NonZeroU64,
    maximum_trace_depth: NonZeroU64,
    maximum_runtime_millis: NonZeroU64,
}

impl ProtocolCheckBounds {
    pub const fn new(maximum_states: NonZeroU64, maximum_trace_depth: NonZeroU64) -> Self {
        Self {
            maximum_states,
            maximum_trace_depth,
            maximum_runtime_millis: unsafe_nonzero(60_000),
        }
    }

    pub const fn maximum_states(self) -> NonZeroU64 {
        self.maximum_states
    }

    pub const fn maximum_trace_depth(self) -> NonZeroU64 {
        self.maximum_trace_depth
    }

    pub const fn with_maximum_runtime_millis(mut self, maximum_runtime_millis: NonZeroU64) -> Self {
        self.maximum_runtime_millis = maximum_runtime_millis;
        self
    }

    pub const fn maximum_runtime_millis(self) -> NonZeroU64 {
        self.maximum_runtime_millis
    }
}

const fn unsafe_nonzero(value: u64) -> NonZeroU64 {
    match NonZeroU64::new(value) {
        Some(value) => value,
        None => panic!("constant protocol bound must be nonzero"),
    }
}
