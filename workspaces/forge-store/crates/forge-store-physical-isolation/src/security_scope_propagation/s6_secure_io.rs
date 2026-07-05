use forge_store_security::StoreSecurityScopeIdentity;

use super::{StableReadObservedSecurityScope, StableReadSecurityScopePropagationCounters};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6SecureIoStableReadDenial {
    KeyScopeMismatch,
    TenantScopeMismatch,
    AuthenticityRequirementMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6SecureIoStableReadPreservation {
    admitted: StoreSecurityScopeIdentity,
    observed: StableReadObservedSecurityScope,
    counters: StableReadSecurityScopePropagationCounters,
}

pub fn preserve_s6_secure_io_stable_read_scope(
    admitted: StoreSecurityScopeIdentity,
    observed: StableReadObservedSecurityScope,
) -> Result<S6SecureIoStableReadPreservation, S6SecureIoStableReadDenial> {
    let metadata = observed.metadata();
    if metadata.key_scope() != admitted.key_scope() {
        return Err(S6SecureIoStableReadDenial::KeyScopeMismatch);
    }
    if metadata.tenant_scope() != admitted.tenant_scope() {
        return Err(S6SecureIoStableReadDenial::TenantScopeMismatch);
    }
    if metadata.authenticity_requirement() != admitted.authenticity_requirement() {
        return Err(S6SecureIoStableReadDenial::AuthenticityRequirementMismatch);
    }
    Ok(S6SecureIoStableReadPreservation {
        admitted,
        observed,
        counters: observed.counters(),
    })
}

impl S6SecureIoStableReadPreservation {
    pub const fn admitted(self) -> StoreSecurityScopeIdentity {
        self.admitted
    }

    pub const fn observed(self) -> StableReadObservedSecurityScope {
        self.observed
    }

    pub const fn counters(self) -> StableReadSecurityScopePropagationCounters {
        self.counters
    }
}
