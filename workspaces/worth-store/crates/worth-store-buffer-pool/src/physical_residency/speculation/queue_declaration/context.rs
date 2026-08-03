use worth_store_contracts::QueueProducerResourceShape;
use worth_store_security::{
    StoreAuthenticityRequirement, StoreKeyScope, StoreSecurityScopeIdentity, StoreTenantScope,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BufferPoolQueueGroupingScope {
    security_scope_identity: StoreSecurityScopeIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BufferPoolQueueDeclarationContext {
    grouping_scope: BufferPoolQueueGroupingScope,
    flush_epoch: u64,
    resource_shape: QueueProducerResourceShape,
}

impl BufferPoolQueueGroupingScope {
    pub const fn new(security_scope_identity: StoreSecurityScopeIdentity) -> Self {
        Self {
            security_scope_identity,
        }
    }

    pub const fn security_scope_identity(self) -> StoreSecurityScopeIdentity {
        self.security_scope_identity
    }

    pub const fn tenant_scope(self) -> StoreTenantScope {
        self.security_scope_identity.tenant_scope()
    }

    pub const fn key_scope(self) -> StoreKeyScope {
        self.security_scope_identity.key_scope()
    }

    pub const fn authenticity_requirement(self) -> StoreAuthenticityRequirement {
        self.security_scope_identity.authenticity_requirement()
    }
}

impl BufferPoolQueueDeclarationContext {
    pub const fn new(
        grouping_scope: BufferPoolQueueGroupingScope,
        flush_epoch: u64,
        resource_shape: QueueProducerResourceShape,
    ) -> Self {
        Self {
            grouping_scope,
            flush_epoch,
            resource_shape,
        }
    }

    pub(super) const fn grouping_scope(self) -> BufferPoolQueueGroupingScope {
        self.grouping_scope
    }

    pub(super) const fn flush_epoch(self) -> u64 {
        self.flush_epoch
    }

    pub(super) const fn resource_shape(self) -> QueueProducerResourceShape {
        self.resource_shape
    }
}
