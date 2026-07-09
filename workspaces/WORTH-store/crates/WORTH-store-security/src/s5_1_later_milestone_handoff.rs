use crate::S51SecurityScopeReadinessFamily;
use crate::{StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope, StoreTenantScope};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S51LaterMilestoneHandoffDenial {
    WrongReadinessFamily {
        expected: S51SecurityScopeReadinessFamily,
        actual: S51SecurityScopeReadinessFamily,
        counters: S51LaterMilestoneHandoffCounterSnapshot,
    },
    WrongKeyScope {
        actual: StoreKeyScope,
        counters: S51LaterMilestoneHandoffCounterSnapshot,
    },
    WrongTenantScope {
        actual: StoreTenantScope,
        counters: S51LaterMilestoneHandoffCounterSnapshot,
    },
    WrongAuthenticityRequirement {
        actual: StoreAuthenticityRequirement,
        counters: S51LaterMilestoneHandoffCounterSnapshot,
    },
    WrongCustodyPosture {
        actual: StoreCustodyPosture,
        counters: S51LaterMilestoneHandoffCounterSnapshot,
    },
    UnsupportedSecurityFoundationClaim {
        counters: S51LaterMilestoneHandoffCounterSnapshot,
    },
}
