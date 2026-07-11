use forge_store_security::{
    StoreAdmittedSecurityScope, StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope,
    StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeIdentity, StoreTenantScope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchedulerSecurityScopeCapability {
    identity: StoreSecurityScopeIdentity,
}

impl SchedulerSecurityScopeCapability {
    pub const fn identity(self) -> StoreSecurityScopeIdentity {
        self.identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoSchedulerSecurityScopeAdmissionDenial {
    WrongKeyScope {
        actual: StoreKeyScope,
    },
    WrongTenantScope {
        actual: StoreTenantScope,
    },
    WrongAuthenticityRequirement {
        actual: StoreAuthenticityRequirement,
    },
    WrongCustodyPosture {
        actual: StoreCustodyPosture,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct IoSchedulerSecurityScopeAdmission {
    permission: SchedulerSecurityScopeCapability,
    receipt: StoreSecurityScopeAdmissionReceipt,
}

impl IoSchedulerSecurityScopeAdmission {
    pub const fn permission(&self) -> SchedulerSecurityScopeCapability {
        self.permission
    }

    pub const fn receipt(&self) -> StoreSecurityScopeAdmissionReceipt {
        self.receipt
    }
}

pub fn admit_security_scope_for_scheduler(
    security_scope: &StoreAdmittedSecurityScope,
) -> Result<IoSchedulerSecurityScopeAdmission, IoSchedulerSecurityScopeAdmissionDenial> {
    let identity = security_scope.identity();
    reject_wrong_key_scope(identity)?;
    reject_wrong_tenant_scope(identity)?;
    reject_wrong_authenticity(identity)?;
    reject_wrong_custody(identity)?;
    Ok(IoSchedulerSecurityScopeAdmission {
        permission: SchedulerSecurityScopeCapability { identity },
        receipt: security_scope.receipt(),
    })
}

fn reject_wrong_key_scope(
    identity: StoreSecurityScopeIdentity,
) -> Result<(), IoSchedulerSecurityScopeAdmissionDenial> {
    if identity.key_scope() == StoreKeyScope::StoreManagedRoot {
        Ok(())
    } else {
        Err(IoSchedulerSecurityScopeAdmissionDenial::WrongKeyScope {
            actual: identity.key_scope(),
        })
    }
}

fn reject_wrong_tenant_scope(
    identity: StoreSecurityScopeIdentity,
) -> Result<(), IoSchedulerSecurityScopeAdmissionDenial> {
    if identity.tenant_scope() == StoreTenantScope::StoreInternal {
        Ok(())
    } else {
        Err(IoSchedulerSecurityScopeAdmissionDenial::WrongTenantScope {
            actual: identity.tenant_scope(),
        })
    }
}

fn reject_wrong_authenticity(
    identity: StoreSecurityScopeIdentity,
) -> Result<(), IoSchedulerSecurityScopeAdmissionDenial> {
    if identity.authenticity_requirement() == StoreAuthenticityRequirement::not_required() {
        Ok(())
    } else {
        Err(
            IoSchedulerSecurityScopeAdmissionDenial::WrongAuthenticityRequirement {
                actual: identity.authenticity_requirement(),
            },
        )
    }
}

fn reject_wrong_custody(
    identity: StoreSecurityScopeIdentity,
) -> Result<(), IoSchedulerSecurityScopeAdmissionDenial> {
    if identity.custody_posture() == StoreCustodyPosture::InternalStoreCustody {
        Ok(())
    } else {
        Err(
            IoSchedulerSecurityScopeAdmissionDenial::WrongCustodyPosture {
                actual: identity.custody_posture(),
            },
        )
    }
}
