use forge_store_security::{
    S51AdmittedSecurityScopeReadiness, S51LaterMilestoneHandoffCounterSnapshot,
    S51LaterMilestoneHandoffDenial, S51SecurityScopeReadinessFamily, StoreAuthenticityRequirement,
    StoreCustodyPosture, StoreKeyScope, StoreSecurityScopeAdmissionReceipt,
    StoreSecurityScopeIdentity, StoreTenantScope,
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

#[derive(Debug, PartialEq, Eq)]
pub struct SchedulerSecurityScopeEvidence {
    permission: SchedulerSecurityScopeCapability,
    receipt: StoreSecurityScopeAdmissionReceipt,
    counters: S51LaterMilestoneHandoffCounterSnapshot,
}

impl SchedulerSecurityScopeEvidence {
    pub fn from_s5_1_readiness(
        readiness: S51AdmittedSecurityScopeReadiness,
    ) -> Result<Self, S51LaterMilestoneHandoffDenial> {
        let counters = S51LaterMilestoneHandoffCounterSnapshot::start();
        reject_wrong_family(&readiness, counters)?;
        let identity = readiness.receipt().identity();
        reject_wrong_key_scope(identity, counters)?;
        reject_wrong_tenant_scope(identity, counters)?;
        reject_wrong_authenticity(identity, counters)?;
        reject_wrong_custody(identity, counters)?;

        Ok(Self {
            permission: SchedulerSecurityScopeCapability { identity },
            receipt: readiness.receipt(),
            counters: counters.admitted(),
        })
    }

    pub const fn permission(&self) -> SchedulerSecurityScopeCapability {
        self.permission
    }

    pub const fn receipt(&self) -> StoreSecurityScopeAdmissionReceipt {
        self.receipt
    }

    pub const fn counters(&self) -> S51LaterMilestoneHandoffCounterSnapshot {
        self.counters
    }
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
    handoff: SchedulerSecurityScopeEvidence,
) -> IoSchedulerSecurityScopeAdmission {
    IoSchedulerSecurityScopeAdmission {
        permission: handoff.permission,
        receipt: handoff.receipt,
    }
}

fn reject_wrong_family(
    readiness: &S51AdmittedSecurityScopeReadiness,
    counters: S51LaterMilestoneHandoffCounterSnapshot,
) -> Result<(), S51LaterMilestoneHandoffDenial> {
    let actual = readiness.reservation().family();
    let expected = S51SecurityScopeReadinessFamily::IoQos;
    if actual == expected {
        Ok(())
    } else {
        Err(S51LaterMilestoneHandoffDenial::WrongReadinessFamily {
            expected,
            actual,
            counters: counters.denied(),
        })
    }
}

fn reject_wrong_key_scope(
    identity: StoreSecurityScopeIdentity,
    counters: S51LaterMilestoneHandoffCounterSnapshot,
) -> Result<(), S51LaterMilestoneHandoffDenial> {
    if identity.key_scope() == StoreKeyScope::StoreManagedRoot {
        Ok(())
    } else {
        Err(S51LaterMilestoneHandoffDenial::WrongKeyScope {
            actual: identity.key_scope(),
            counters: counters.denied(),
        })
    }
}

fn reject_wrong_tenant_scope(
    identity: StoreSecurityScopeIdentity,
    counters: S51LaterMilestoneHandoffCounterSnapshot,
) -> Result<(), S51LaterMilestoneHandoffDenial> {
    if identity.tenant_scope() == StoreTenantScope::StoreInternal {
        Ok(())
    } else {
        Err(S51LaterMilestoneHandoffDenial::WrongTenantScope {
            actual: identity.tenant_scope(),
            counters: counters.denied(),
        })
    }
}

fn reject_wrong_authenticity(
    identity: StoreSecurityScopeIdentity,
    counters: S51LaterMilestoneHandoffCounterSnapshot,
) -> Result<(), S51LaterMilestoneHandoffDenial> {
    if identity.authenticity_requirement() == StoreAuthenticityRequirement::not_required() {
        Ok(())
    } else {
        Err(
            S51LaterMilestoneHandoffDenial::WrongAuthenticityRequirement {
                actual: identity.authenticity_requirement(),
                counters: counters.unsupported().denied(),
            },
        )
    }
}

fn reject_wrong_custody(
    identity: StoreSecurityScopeIdentity,
    counters: S51LaterMilestoneHandoffCounterSnapshot,
) -> Result<(), S51LaterMilestoneHandoffDenial> {
    if identity.custody_posture() == StoreCustodyPosture::InternalStoreCustody {
        Ok(())
    } else {
        Err(S51LaterMilestoneHandoffDenial::WrongCustodyPosture {
            actual: identity.custody_posture(),
            counters: counters.unavailable().denied(),
        })
    }
}
