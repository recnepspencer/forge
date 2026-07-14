use worth_store_security::{StoreKeyScope, StoreSecurityScopeIdentity, StoreTenantScope};

use super::{QueueGroupingBasis, QueueWorkDeclaration, QueueWritebackPolicy};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueWriteBackBasis {
    security_scope_identity: StoreSecurityScopeIdentity,
    tenant_scope: StoreTenantScope,
    key_scope: StoreKeyScope,
    writeback_policy: QueueWritebackPolicy,
    admitted_windows: u64,
}

impl QueueWriteBackBasis {
    pub const fn from_grouping(grouping: QueueGroupingBasis, admitted_windows: u64) -> Self {
        Self {
            security_scope_identity: grouping.security_scope_identity(),
            tenant_scope: grouping.tenant_scope(),
            key_scope: grouping.key_scope(),
            writeback_policy: grouping.writeback_policy(),
            admitted_windows,
        }
    }

    pub fn admits(self, work: QueueWorkDeclaration) -> bool {
        let Some(grouping) = work.grouping_basis() else {
            return false;
        };
        self.admits_scope(
            work.requested_budget().write_back_window(),
            grouping.security_scope_identity(),
            grouping.tenant_scope(),
            grouping.key_scope(),
            grouping.writeback_policy(),
        )
    }

    pub fn admits_scope(
        self,
        requested_windows: u64,
        security_scope_identity: StoreSecurityScopeIdentity,
        tenant_scope: StoreTenantScope,
        key_scope: StoreKeyScope,
        writeback_policy: QueueWritebackPolicy,
    ) -> bool {
        security_scope_identity == self.security_scope_identity
            && tenant_scope == self.tenant_scope
            && key_scope == self.key_scope
            && writeback_policy == self.writeback_policy
            && requested_windows > 0
    }

    pub const fn admitted_windows(self) -> u64 {
        self.admitted_windows
    }
}
