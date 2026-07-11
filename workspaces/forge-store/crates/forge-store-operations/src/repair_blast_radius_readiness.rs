use forge_store_security::{
    repair_blast_radius_authenticity, StoreAdmittedSecurityScope, StoreCustodyPosture,
    StoreKeyScope, StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeIdentity,
    StoreTenantScope,
};

use crate::{
    RepairBlastRadiusCounterSnapshot, RepairBlastRadiusDenial, RepairPhysicalRegion,
    RepairReadPlan, S10RepairBlastRadiusHandoff,
};

#[derive(Debug, PartialEq, Eq)]
pub struct RepairBlastRadiusReadiness {
    physical_region: RepairPhysicalRegion,
    identity: StoreSecurityScopeIdentity,
    receipt: StoreSecurityScopeAdmissionReceipt,
    counters: RepairBlastRadiusCounterSnapshot,
}

impl RepairBlastRadiusReadiness {
    pub(crate) fn from_admitted_scope(
        physical_region: RepairPhysicalRegion,
        security_scope: StoreAdmittedSecurityScope,
        counters: RepairBlastRadiusCounterSnapshot,
    ) -> Result<Self, RepairBlastRadiusDenial> {
        let identity = security_scope.identity();
        reject_wrong_key_scope(identity, counters)?;
        reject_wrong_tenant_scope(identity, counters)?;
        reject_wrong_authenticity(identity, counters)?;
        reject_wrong_custody(identity, counters)?;

        Ok(Self {
            physical_region,
            identity,
            receipt: security_scope.receipt(),
            counters,
        })
    }

    pub fn prepare_repair_read(
        self,
        requested_region: RepairPhysicalRegion,
    ) -> Result<RepairReadPlan, RepairBlastRadiusDenial> {
        if requested_region == self.physical_region {
            Ok(RepairReadPlan::from_readiness(
                self.identity,
                requested_region,
                self.counters.prepared_repair_read(),
            ))
        } else {
            Err(RepairBlastRadiusDenial::CrossScopePhysicalRegion {
                admitted: self.physical_region,
                requested: requested_region,
                counters: self.counters.rejected_cross_scope_region().denied(),
            })
        }
    }

    pub fn from_s10_handoff(handoff: S10RepairBlastRadiusHandoff) -> Self {
        handoff.into_readiness()
    }

    pub fn physical_region(&self) -> &RepairPhysicalRegion {
        &self.physical_region
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        self.identity
    }

    pub const fn receipt(&self) -> StoreSecurityScopeAdmissionReceipt {
        self.receipt
    }

    pub const fn counters(&self) -> RepairBlastRadiusCounterSnapshot {
        self.counters
    }
}

fn reject_wrong_key_scope(
    identity: StoreSecurityScopeIdentity,
    counters: RepairBlastRadiusCounterSnapshot,
) -> Result<(), RepairBlastRadiusDenial> {
    if identity.key_scope() == StoreKeyScope::RepairScopeEnvelope {
        Ok(())
    } else {
        Err(RepairBlastRadiusDenial::WrongKeyScope {
            actual: identity.key_scope(),
            counters: counters.denied(),
        })
    }
}

fn reject_wrong_tenant_scope(
    identity: StoreSecurityScopeIdentity,
    counters: RepairBlastRadiusCounterSnapshot,
) -> Result<(), RepairBlastRadiusDenial> {
    if identity.tenant_scope() == StoreTenantScope::RepairBlastRadius {
        Ok(())
    } else {
        Err(RepairBlastRadiusDenial::WrongTenantScope {
            actual: identity.tenant_scope(),
            counters: counters.denied(),
        })
    }
}

fn reject_wrong_authenticity(
    identity: StoreSecurityScopeIdentity,
    counters: RepairBlastRadiusCounterSnapshot,
) -> Result<(), RepairBlastRadiusDenial> {
    if identity.authenticity_requirement() == repair_blast_radius_authenticity() {
        Ok(())
    } else {
        Err(RepairBlastRadiusDenial::WrongAuthenticityRequirement {
            actual: identity.authenticity_requirement(),
            counters: counters.denied(),
        })
    }
}

fn reject_wrong_custody(
    identity: StoreSecurityScopeIdentity,
    counters: RepairBlastRadiusCounterSnapshot,
) -> Result<(), RepairBlastRadiusDenial> {
    match identity.custody_posture() {
        StoreCustodyPosture::InternalStoreCustody | StoreCustodyPosture::Readmitted => Ok(()),
        StoreCustodyPosture::CustodyUnavailable => {
            Err(RepairBlastRadiusDenial::WrongCustodyPosture {
                actual: identity.custody_posture(),
                counters: counters.rejected_unavailable_custody().denied(),
            })
        }
        actual => Err(RepairBlastRadiusDenial::WrongCustodyPosture {
            actual,
            counters: counters.denied(),
        }),
    }
}
