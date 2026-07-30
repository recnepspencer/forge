use std::num::NonZeroU64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrubPlanPolicy {
    streaming_window_byte_limit: NonZeroU64,
    protected_read_limit: NonZeroU64,
}

impl ScrubPlanPolicy {
    pub const fn bounded(
        streaming_window_byte_limit: NonZeroU64,
        protected_read_limit: NonZeroU64,
    ) -> Self {
        Self {
            streaming_window_byte_limit,
            protected_read_limit,
        }
    }

    pub const fn constrained_by(
        self,
        streaming_window_byte_limit: NonZeroU64,
        protected_read_limit: NonZeroU64,
    ) -> Self {
        Self {
            streaming_window_byte_limit: minimum(
                self.streaming_window_byte_limit,
                streaming_window_byte_limit,
            ),
            protected_read_limit: minimum(self.protected_read_limit, protected_read_limit),
        }
    }

    pub const fn streaming_window_byte_limit(self) -> u64 {
        self.streaming_window_byte_limit.get()
    }

    pub const fn protected_read_limit(self) -> u64 {
        self.protected_read_limit.get()
    }
}

const fn minimum(left: NonZeroU64, right: NonZeroU64) -> NonZeroU64 {
    if left.get() <= right.get() {
        left
    } else {
        right
    }
}
