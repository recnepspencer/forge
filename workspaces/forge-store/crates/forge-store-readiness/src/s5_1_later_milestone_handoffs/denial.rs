use forge_store_security::{
    StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope, StoreTenantScope,
};

use crate::{S51LaterMilestoneHandoffCounterSnapshot, S51SecurityScopeReadinessFamily};

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
