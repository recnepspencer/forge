use worth_signal::facade::TemporalDuration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PhysicalCheckpointIdempotencyKey([u8; 32]);

impl PhysicalCheckpointIdempotencyKey {
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub const fn bytes(self) -> [u8; 32] {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalCheckpointDeadline(TemporalDuration);

impl PhysicalCheckpointDeadline {
    pub fn after_milliseconds(milliseconds: u64) -> Option<Self> {
        TemporalDuration::temporal_duration(milliseconds)
            .ok()
            .map(Self)
    }

    pub const fn at(deadline: TemporalDuration) -> Self {
        Self(deadline)
    }

    pub const fn signal_deadline(self) -> TemporalDuration {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::PhysicalCheckpointDeadline;

    #[test]
    fn millisecond_deadline_rejects_zero_and_preserves_positive_duration() {
        assert_eq!(PhysicalCheckpointDeadline::after_milliseconds(0), None);
        assert_eq!(
            PhysicalCheckpointDeadline::after_milliseconds(5_000)
                .unwrap()
                .signal_deadline()
                .get(),
            5_000
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalCheckpointRequest {
    idempotency: PhysicalCheckpointIdempotencyKey,
    deadline: PhysicalCheckpointDeadline,
}

impl PhysicalCheckpointRequest {
    pub const fn fuzzy(
        idempotency: PhysicalCheckpointIdempotencyKey,
        deadline: PhysicalCheckpointDeadline,
    ) -> Self {
        Self {
            idempotency,
            deadline,
        }
    }

    pub const fn idempotency_key(self) -> PhysicalCheckpointIdempotencyKey {
        self.idempotency
    }

    pub const fn deadline(self) -> PhysicalCheckpointDeadline {
        self.deadline
    }
}
