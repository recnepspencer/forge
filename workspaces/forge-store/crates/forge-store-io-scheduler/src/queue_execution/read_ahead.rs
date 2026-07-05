use forge_store_security::{StoreKeyScope, StoreSecurityScopeIdentity, StoreTenantScope};

use super::{QueueGroupingBasis, QueueWorkDeclaration};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueReadAheadBasis {
    security_scope_identity: StoreSecurityScopeIdentity,
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    admitted_windows: u64,
}

impl QueueReadAheadBasis {
    pub const fn from_grouping(grouping: QueueGroupingBasis, admitted_windows: u64) -> Self {
        Self {
            security_scope_identity: grouping.security_scope_identity(),
            key_scope: grouping.key_scope(),
            tenant_scope: grouping.tenant_scope(),
            admitted_windows,
        }
    }

    pub fn admits(self, work: QueueWorkDeclaration) -> bool {
        let Some(grouping) = work.grouping_basis() else {
            return false;
        };
        self.admits_scope(
            work.requested_budget().read_ahead_window(),
            grouping.security_scope_identity(),
            grouping.tenant_scope(),
            grouping.key_scope(),
        )
    }

    pub fn admits_scope(
        self,
        requested_windows: u64,
        security_scope_identity: StoreSecurityScopeIdentity,
        tenant_scope: StoreTenantScope,
        key_scope: StoreKeyScope,
    ) -> bool {
        security_scope_identity == self.security_scope_identity
            && key_scope == self.key_scope
            && tenant_scope == self.tenant_scope
            && requested_windows > 0
    }

    pub const fn admitted_windows(self) -> u64 {
        self.admitted_windows
    }
}
