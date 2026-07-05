use forge_store_security::{
    StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope,
    StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeIdentity, StoreTenantScope,
};

use crate::{
    S51AdmittedSecurityScopeReadiness, S51LaterMilestoneHandoffCounterSnapshot,
    S51LaterMilestoneHandoffDenial, S51SecurityScopeReadinessFamily,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S51SecurityFoundationLifecyclePermission {
    identity: StoreSecurityScopeIdentity,
}

impl S51SecurityFoundationLifecyclePermission {
    pub const fn identity(self) -> StoreSecurityScopeIdentity {
        self.identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S51SecurityFoundationNonClaim {
    Encryption,
    KeyRotation,
    Audit,
    OperatorAuthorization,
}

#[derive(Debug, PartialEq, Eq)]
pub struct S51SecurityFoundationHandoff {
    permission: S51SecurityFoundationLifecyclePermission,
    receipt: StoreSecurityScopeAdmissionReceipt,
    counters: S51LaterMilestoneHandoffCounterSnapshot,
}

impl S51SecurityFoundationHandoff {
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
            permission: S51SecurityFoundationLifecyclePermission { identity },
            receipt: readiness.receipt(),
            counters: counters.admitted(),
        })
    }

    pub const fn lifecycle_foundation_permission(
        &self,
    ) -> S51SecurityFoundationLifecyclePermission {
        self.permission
    }

    pub const fn non_claims(&self) -> [S51SecurityFoundationNonClaim; 4] {
        [
            S51SecurityFoundationNonClaim::Encryption,
            S51SecurityFoundationNonClaim::KeyRotation,
            S51SecurityFoundationNonClaim::Audit,
            S51SecurityFoundationNonClaim::OperatorAuthorization,
        ]
    }

    pub const fn receipt(&self) -> StoreSecurityScopeAdmissionReceipt {
        self.receipt
    }

    pub const fn counters(&self) -> S51LaterMilestoneHandoffCounterSnapshot {
        self.counters
    }
}

fn reject_wrong_family(
    readiness: &S51AdmittedSecurityScopeReadiness,
    counters: S51LaterMilestoneHandoffCounterSnapshot,
) -> Result<(), S51LaterMilestoneHandoffDenial> {
    let actual = readiness.reservation().family();
    let expected = S51SecurityScopeReadinessFamily::SecurityFoundation;
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
    if identity.key_scope() == StoreKeyScope::SecurityLifecycleFoundation {
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
    if identity.tenant_scope() == StoreTenantScope::SecurityLifecycleFoundation {
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
