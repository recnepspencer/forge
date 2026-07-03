use forge_store_readiness::S51SecurityScopeReadinessFamily;
use forge_store_security::{
    StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope,
    StoreSecurityScopeAdmissionDeferred, StoreSecurityScopeAdmissionDenial,
    StoreSecurityScopeAdmissionFailure, StoreSecurityScopeAdmissionRebindRequired,
    StoreSecurityScopeAdmissionStale, StoreTenantScope,
};

use crate::{RepairBlastRadiusCounterSnapshot, RepairPhysicalRegion};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepairBlastRadiusDenial {
    SecurityScopeAdmissionDenied {
        source: StoreSecurityScopeAdmissionDenial,
        counters: RepairBlastRadiusCounterSnapshot,
    },
    SecurityScopeAdmissionStale {
        source: StoreSecurityScopeAdmissionStale,
        counters: RepairBlastRadiusCounterSnapshot,
    },
    SecurityScopeAdmissionRebindRequired {
        source: StoreSecurityScopeAdmissionRebindRequired,
        counters: RepairBlastRadiusCounterSnapshot,
    },
    SecurityScopeAdmissionDeferred {
        source: StoreSecurityScopeAdmissionDeferred,
        counters: RepairBlastRadiusCounterSnapshot,
    },
    SecurityScopeAdmissionFailed {
        source: StoreSecurityScopeAdmissionFailure,
        counters: RepairBlastRadiusCounterSnapshot,
    },
    WrongReadinessFamily {
        actual: S51SecurityScopeReadinessFamily,
        counters: RepairBlastRadiusCounterSnapshot,
    },
    WrongKeyScope {
        actual: StoreKeyScope,
        counters: RepairBlastRadiusCounterSnapshot,
    },
    WrongTenantScope {
        actual: StoreTenantScope,
        counters: RepairBlastRadiusCounterSnapshot,
    },
    WrongAuthenticityRequirement {
        actual: StoreAuthenticityRequirement,
        counters: RepairBlastRadiusCounterSnapshot,
    },
    WrongCustodyPosture {
        actual: StoreCustodyPosture,
        counters: RepairBlastRadiusCounterSnapshot,
    },
    CrossScopePhysicalRegion {
        admitted: RepairPhysicalRegion,
        requested: RepairPhysicalRegion,
        counters: RepairBlastRadiusCounterSnapshot,
    },
}
