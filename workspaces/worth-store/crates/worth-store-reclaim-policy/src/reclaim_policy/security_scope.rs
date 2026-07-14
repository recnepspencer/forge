use worth_store_security::{
    StoreAdmittedSecurityScope, StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReclaimPolicySecurityScope {
    identity: StoreSecurityScopeIdentity,
    receipt: StoreSecurityScopeAdmissionReceipt,
}

impl ReclaimPolicySecurityScope {
    pub const fn from_admitted_scope(scope: &StoreAdmittedSecurityScope) -> Self {
        Self {
            identity: scope.identity(),
            receipt: scope.receipt(),
        }
    }

    pub const fn identity(self) -> StoreSecurityScopeIdentity {
        self.identity
    }

    pub const fn receipt(self) -> StoreSecurityScopeAdmissionReceipt {
        self.receipt
    }
}
