#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiIntentExecutionAttemptIdentity {
    slot: u8,
    generation: u64,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiIntentExecutionIdempotencyIdentity {
    session: u64,
    lineage: u64,
}

impl UiIntentExecutionAttemptIdentity {
    pub(crate) const fn issued(slot: u8, generation: u64) -> Self {
        Self { slot, generation }
    }

    pub const fn slot(self) -> u8 {
        self.slot
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }
}

impl UiIntentExecutionIdempotencyIdentity {
    pub(crate) const fn issued(session: u64, lineage: u64) -> Self {
        Self { session, lineage }
    }

    pub const fn session(self) -> u64 {
        self.session
    }

    pub const fn lineage(self) -> u64 {
        self.lineage
    }
}
