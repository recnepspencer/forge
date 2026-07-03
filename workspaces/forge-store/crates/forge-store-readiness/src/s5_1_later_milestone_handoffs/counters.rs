#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct S51LaterMilestoneHandoffCounterSnapshot {
    handoff_attempts: u64,
    admitted: u64,
    denied: u64,
    unsupported: u64,
    unavailable: u64,
}

impl S51LaterMilestoneHandoffCounterSnapshot {
    pub fn start() -> Self {
        Self {
            handoff_attempts: 1,
            ..Self::default()
        }
    }

    pub const fn admitted(mut self) -> Self {
        self.admitted += 1;
        self
    }

    pub const fn denied(mut self) -> Self {
        self.denied += 1;
        self
    }

    pub const fn unsupported(mut self) -> Self {
        self.unsupported += 1;
        self
    }

    pub const fn unavailable(mut self) -> Self {
        self.unavailable += 1;
        self
    }

    pub const fn handoff_attempts(self) -> u64 {
        self.handoff_attempts
    }

    pub const fn admitted_count(self) -> u64 {
        self.admitted
    }

    pub const fn denied_count(self) -> u64 {
        self.denied
    }

    pub const fn unsupported_count(self) -> u64 {
        self.unsupported
    }

    pub const fn unavailable_count(self) -> u64 {
        self.unavailable
    }
}
