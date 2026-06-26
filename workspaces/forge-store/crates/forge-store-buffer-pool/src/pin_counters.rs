#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PinLifecycleCounterSnapshot {
    pin_attempt_count: u64,
    successful_pin_count: u64,
    explicit_unpin_count: u64,
    defensive_drop_count: u64,
    leaked_pin_count: u64,
    denied_protected_mutation_count: u64,
    active_pinned_pages: u64,
}

impl PinLifecycleCounterSnapshot {
    pub const fn empty() -> Self {
        Self {
            pin_attempt_count: 0,
            successful_pin_count: 0,
            explicit_unpin_count: 0,
            defensive_drop_count: 0,
            leaked_pin_count: 0,
            denied_protected_mutation_count: 0,
            active_pinned_pages: 0,
        }
    }

    pub(crate) const fn with_pin_attempt(self) -> Self {
        Self {
            pin_attempt_count: self.pin_attempt_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_successful_pin(self) -> Self {
        Self {
            successful_pin_count: self.successful_pin_count + 1,
            active_pinned_pages: self.active_pinned_pages + 1,
            ..self
        }
    }

    pub(crate) const fn with_explicit_unpin(self) -> Self {
        Self {
            explicit_unpin_count: self.explicit_unpin_count + 1,
            active_pinned_pages: self.active_pinned_pages - 1,
            ..self
        }
    }

    pub(crate) const fn with_defensive_drop(self) -> Self {
        Self {
            defensive_drop_count: self.defensive_drop_count + 1,
            active_pinned_pages: self.active_pinned_pages - 1,
            ..self
        }
    }

    pub(crate) const fn with_leaked_pins(self, leaked_pins: u64) -> Self {
        Self {
            leaked_pin_count: self.leaked_pin_count + leaked_pins,
            ..self
        }
    }

    pub(crate) const fn with_protected_mutation_denial(self) -> Self {
        Self {
            denied_protected_mutation_count: self.denied_protected_mutation_count + 1,
            ..self
        }
    }

    pub const fn pin_attempt_count(self) -> u64 {
        self.pin_attempt_count
    }

    pub const fn successful_pin_count(self) -> u64 {
        self.successful_pin_count
    }

    pub const fn explicit_unpin_count(self) -> u64 {
        self.explicit_unpin_count
    }

    pub const fn defensive_drop_count(self) -> u64 {
        self.defensive_drop_count
    }

    pub const fn leaked_pin_count(self) -> u64 {
        self.leaked_pin_count
    }

    pub const fn denied_protected_mutation_count(self) -> u64 {
        self.denied_protected_mutation_count
    }

    pub const fn active_pinned_pages(self) -> u64 {
        self.active_pinned_pages
    }
}
