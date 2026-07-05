use forge_store_security::{StoreCurrentSecurityScopeWitnessSet, StoreSecurityScopeIdentity};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessPolicySecurityScope {
    identity: StoreSecurityScopeIdentity,
}

impl AccessPolicySecurityScope {
    pub fn from_current_store_scope(witnesses: &StoreCurrentSecurityScopeWitnessSet) -> Self {
        Self {
            identity: witnesses.key_scope().identity(),
        }
    }

    pub const fn identity(self) -> StoreSecurityScopeIdentity {
        self.identity
    }
}
