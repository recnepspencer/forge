use forge_store_authority::StoreCurrentAuthorityWitness;
use forge_store_offline_verifier::{
    OfflineRepairBlastRadiusObservation, OfflineRepairEvidenceKind,
};
use forge_store_security::{StoreRepairPhysicalRegionWitness, StoreSecurityScopeIdentity};

use crate::{
    repair::blast_radius::repair_region_witness_or_denial, RepairBlastRadiusCounterSnapshot,
    RepairBlastRadiusDenial, RepairBlastRadiusReadiness, RepairPhysicalRegion,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepairQuarantineScopePreservation {
    security_scope: StoreSecurityScopeIdentity,
    physical_region: RepairPhysicalRegion,
    counters: RepairBlastRadiusCounterSnapshot,
}

impl RepairQuarantineScopePreservation {
    pub fn preserve_from_admitted_readiness(
        current_authority: &StoreCurrentAuthorityWitness,
        readiness: RepairBlastRadiusReadiness,
        observation: &OfflineRepairBlastRadiusObservation,
    ) -> Result<Self, RepairBlastRadiusDenial> {
        let counters = readiness.counters();
        if observation.evidence_kind() != OfflineRepairEvidenceKind::QuarantineReport {
            return Err(RepairBlastRadiusDenial::CrossScopePhysicalRegion {
                admitted: readiness.physical_region().clone(),
                requested: observed_region(current_authority, observation, counters)?,
                counters: counters.rejected_cross_scope_region().denied(),
            });
        }

        let observed_region = observed_region(current_authority, observation, counters)?;
        if observed_region.region_id() != readiness.physical_region().region_id()
            || observed_region.identity() != readiness.identity()
        {
            return Err(RepairBlastRadiusDenial::CrossScopePhysicalRegion {
                admitted: readiness.physical_region().clone(),
                requested: observed_region,
                counters: counters.rejected_cross_scope_region().denied(),
            });
        }

        Ok(Self {
            security_scope: readiness.identity(),
            physical_region: observed_region,
            counters: counters.preserved_quarantine_scope(),
        })
    }

    pub const fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.security_scope
    }

    pub fn physical_region(&self) -> &RepairPhysicalRegion {
        &self.physical_region
    }

    pub const fn counters(&self) -> RepairBlastRadiusCounterSnapshot {
        self.counters
    }
}

fn observed_region(
    current_authority: &StoreCurrentAuthorityWitness,
    observation: &OfflineRepairBlastRadiusObservation,
    counters: RepairBlastRadiusCounterSnapshot,
) -> Result<RepairPhysicalRegion, RepairBlastRadiusDenial> {
    let witness = StoreRepairPhysicalRegionWitness::admit_offline_report(
        current_authority,
        observation.physical_region().clone(),
        observation.raw_declaration(),
    );
    let witness = repair_region_witness_or_denial(witness, counters)?;
    Ok(RepairPhysicalRegion::from_admitted_identity(
        witness.region_id(),
        witness.identity(),
    ))
}
