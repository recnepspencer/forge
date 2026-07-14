use worth_store_security::{
    StoreAuthenticityRequirement, StoreKeyScope, StoreSecurityScopeIdentity, StoreTenantScope,
};

use super::{QueueDurabilityClass, QueueGroupingDenial, QueueWorkClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueRecoveryOrdering {
    NotRecoveryCritical,
    WalBeforeData,
    RecoveryReadOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueWritebackPolicy {
    None,
    Immediate,
    DeferredWithinFlushEpoch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueGroupingBasis {
    security_scope_identity: StoreSecurityScopeIdentity,
    tenant_scope: StoreTenantScope,
    key_scope: StoreKeyScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    durability_class: QueueDurabilityClass,
    flush_epoch: u64,
    work_class: QueueWorkClass,
    recovery_ordering: QueueRecoveryOrdering,
    writeback_policy: QueueWritebackPolicy,
}

impl QueueGroupingBasis {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        security_scope_identity: StoreSecurityScopeIdentity,
        tenant_scope: StoreTenantScope,
        key_scope: StoreKeyScope,
        authenticity_requirement: StoreAuthenticityRequirement,
        durability_class: QueueDurabilityClass,
        flush_epoch: u64,
        work_class: QueueWorkClass,
        recovery_ordering: QueueRecoveryOrdering,
        writeback_policy: QueueWritebackPolicy,
    ) -> Self {
        Self {
            security_scope_identity,
            tenant_scope,
            key_scope,
            authenticity_requirement,
            durability_class,
            flush_epoch,
            work_class,
            recovery_ordering,
            writeback_policy,
        }
    }

    pub fn compatible_with(self, other: Self) -> Result<(), QueueGroupingDenial> {
        if self.security_scope_identity != other.security_scope_identity {
            return Err(QueueGroupingDenial::SecurityScopeMismatch);
        }
        if self.tenant_scope != other.tenant_scope {
            return Err(QueueGroupingDenial::TenantScopeMismatch);
        }
        if self.key_scope != other.key_scope {
            return Err(QueueGroupingDenial::KeyScopeMismatch);
        }
        if self.authenticity_requirement != other.authenticity_requirement {
            return Err(QueueGroupingDenial::AuthenticityRequirementMismatch);
        }
        if self.durability_class != other.durability_class {
            return Err(QueueGroupingDenial::DurabilityClassMismatch);
        }
        if self.flush_epoch != other.flush_epoch {
            return Err(QueueGroupingDenial::FlushEpochMismatch);
        }
        if self.work_class != other.work_class {
            return Err(QueueGroupingDenial::WorkClassMismatch);
        }
        if self.recovery_ordering != other.recovery_ordering {
            return Err(QueueGroupingDenial::RecoveryOrderingMismatch);
        }
        if self.writeback_policy != other.writeback_policy {
            return Err(QueueGroupingDenial::WritebackPolicyMismatch);
        }
        Ok(())
    }

    pub const fn security_scope_identity(self) -> StoreSecurityScopeIdentity {
        self.security_scope_identity
    }

    pub const fn tenant_scope(self) -> StoreTenantScope {
        self.tenant_scope
    }

    pub const fn key_scope(self) -> StoreKeyScope {
        self.key_scope
    }

    pub const fn authenticity_requirement(self) -> StoreAuthenticityRequirement {
        self.authenticity_requirement
    }

    pub const fn durability_class(self) -> QueueDurabilityClass {
        self.durability_class
    }

    pub const fn flush_epoch(self) -> u64 {
        self.flush_epoch
    }

    pub const fn work_class(self) -> QueueWorkClass {
        self.work_class
    }

    pub const fn recovery_ordering(self) -> QueueRecoveryOrdering {
        self.recovery_ordering
    }

    pub const fn writeback_policy(self) -> QueueWritebackPolicy {
        self.writeback_policy
    }
}
