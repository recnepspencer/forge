use worth_proof::TransitionOutcome;
use worth_store_authority::StoreCurrentAuthorityWitness;
use worth_store_offline_verifier::OfflineRepairBlastRadiusObservation;
use worth_store_security::{
    StoreAdmittedSecurityScope, StoreCustodyPosture, StoreKeyVersionPosture,
    StoreRepairPhysicalRegionAdmissionOutcome, StoreRepairPhysicalRegionDeclaration,
    StoreRepairPhysicalRegionWitness, StoreSecurityScopeAdmissionDeferred,
    StoreSecurityScopeAdmissionDenial, StoreSecurityScopeAdmissionFailure,
    StoreSecurityScopeAdmissionRebindRequired, StoreSecurityScopeAdmissionStale,
    StoreSecurityScopeIdentity,
};

use crate::{RepairBlastRadiusCounterSnapshot, RepairBlastRadiusDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepairPhysicalRegion {
    region_id: String,
    identity: StoreSecurityScopeIdentity,
}

impl RepairPhysicalRegion {
    fn from_witness(witness: &StoreRepairPhysicalRegionWitness) -> Self {
        Self {
            region_id: witness.region_id().to_string(),
            identity: witness.identity(),
        }
    }

    pub(crate) fn from_admitted_identity(
        region_id: impl Into<String>,
        identity: StoreSecurityScopeIdentity,
    ) -> Self {
        Self {
            region_id: region_id.into(),
            identity,
        }
    }

    pub fn region_id(&self) -> &str {
        &self.region_id
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        self.identity
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RepairBlastRadiusDeclaration {
    physical_region: RepairPhysicalRegion,
    admitted_scope: StoreAdmittedSecurityScope,
    counters: RepairBlastRadiusCounterSnapshot,
}

impl RepairBlastRadiusDeclaration {
    pub fn native(
        current_authority: &StoreCurrentAuthorityWitness,
        physical_region: StoreRepairPhysicalRegionDeclaration,
        key_version_posture: StoreKeyVersionPosture,
        custody_posture: StoreCustodyPosture,
    ) -> Result<Self, RepairBlastRadiusDenial> {
        let counters = RepairBlastRadiusCounterSnapshot::from_declaration().attempted_admission();
        let witness = StoreRepairPhysicalRegionWitness::admit_native(
            current_authority,
            physical_region,
            key_version_posture,
            custody_posture,
        );
        let witness = repair_region_witness_or_denial(witness, counters)?;
        Ok(Self::from_witness(witness, counters.admitted()))
    }

    pub fn from_offline_observation(
        current_authority: &StoreCurrentAuthorityWitness,
        observation: OfflineRepairBlastRadiusObservation,
    ) -> Result<Self, RepairBlastRadiusDenial> {
        let counters = RepairBlastRadiusCounterSnapshot::from_declaration().attempted_admission();
        let witness = StoreRepairPhysicalRegionWitness::admit_offline_report(
            current_authority,
            observation.physical_region().clone(),
            observation.raw_declaration(),
        );
        let witness = repair_region_witness_or_denial(witness, counters)?;
        Ok(Self::from_witness(witness, counters.admitted()))
    }

    fn from_witness(
        witness: StoreRepairPhysicalRegionWitness,
        counters: RepairBlastRadiusCounterSnapshot,
    ) -> Self {
        let physical_region = RepairPhysicalRegion::from_witness(&witness);
        Self {
            physical_region,
            admitted_scope: witness.into_admitted_scope(),
            counters,
        }
    }

    pub fn physical_region(&self) -> &RepairPhysicalRegion {
        &self.physical_region
    }

    pub const fn counters(&self) -> RepairBlastRadiusCounterSnapshot {
        self.counters
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        RepairPhysicalRegion,
        StoreAdmittedSecurityScope,
        RepairBlastRadiusCounterSnapshot,
    ) {
        (self.physical_region, self.admitted_scope, self.counters)
    }
}

pub(crate) fn repair_region_witness_or_denial(
    outcome: StoreRepairPhysicalRegionAdmissionOutcome,
    counters: RepairBlastRadiusCounterSnapshot,
) -> Result<StoreRepairPhysicalRegionWitness, RepairBlastRadiusDenial> {
    match outcome {
        TransitionOutcome::Success(witness) => Ok(witness),
        TransitionOutcome::Denied(source) => Err(admission_denial(source, counters)),
        TransitionOutcome::Stale(source) => Err(admission_stale(source, counters)),
        TransitionOutcome::RebindRequired(source) => Err(admission_rebind(source, counters)),
        TransitionOutcome::Deferred(source) => Err(admission_deferred(source, counters)),
        TransitionOutcome::Failed(source) => Err(admission_failed(source, counters)),
    }
}

pub(crate) fn admission_denial(
    source: StoreSecurityScopeAdmissionDenial,
    counters: RepairBlastRadiusCounterSnapshot,
) -> RepairBlastRadiusDenial {
    let counters = match source {
        StoreSecurityScopeAdmissionDenial::MissingAuthenticityRequirement => {
            counters.rejected_missing_authenticity()
        }
        StoreSecurityScopeAdmissionDenial::UnavailableCustodyPosture
        | StoreSecurityScopeAdmissionDenial::MissingCustodyPosture => {
            counters.rejected_unavailable_custody()
        }
        StoreSecurityScopeAdmissionDenial::DeniedKeyVersionPosture => {
            counters.rejected_stale_key_version()
        }
        _ => counters,
    };
    RepairBlastRadiusDenial::SecurityScopeAdmissionDenied {
        source,
        counters: counters.denied(),
    }
}

fn admission_stale(
    source: StoreSecurityScopeAdmissionStale,
    counters: RepairBlastRadiusCounterSnapshot,
) -> RepairBlastRadiusDenial {
    RepairBlastRadiusDenial::SecurityScopeAdmissionStale {
        source,
        counters: counters.rejected_stale_key_version().denied(),
    }
}

fn admission_rebind(
    source: StoreSecurityScopeAdmissionRebindRequired,
    counters: RepairBlastRadiusCounterSnapshot,
) -> RepairBlastRadiusDenial {
    RepairBlastRadiusDenial::SecurityScopeAdmissionRebindRequired {
        source,
        counters: counters.rejected_key_rebind_required().denied(),
    }
}

fn admission_deferred(
    source: StoreSecurityScopeAdmissionDeferred,
    counters: RepairBlastRadiusCounterSnapshot,
) -> RepairBlastRadiusDenial {
    RepairBlastRadiusDenial::SecurityScopeAdmissionDeferred {
        source,
        counters: counters.denied(),
    }
}

fn admission_failed(
    source: StoreSecurityScopeAdmissionFailure,
    counters: RepairBlastRadiusCounterSnapshot,
) -> RepairBlastRadiusDenial {
    RepairBlastRadiusDenial::SecurityScopeAdmissionFailed {
        source,
        counters: counters.denied(),
    }
}
