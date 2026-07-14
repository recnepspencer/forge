use crate::PhysicalIsolationHarnessReadinessDenial;
use worth_store_physical_isolation::PhysicalIsolationEntryAdmission;

use super::{
    accept_store_owned_physical_isolation_harness_readiness,
    AcceptedPhysicalIsolationHarnessReadiness, PhysicalIsolationHarnessReadinessReceipt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationCertificationLaneRegistration {
    entry_recovered_root: String,
    accepted_harness: AcceptedPhysicalIsolationHarnessReadiness,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalIsolationLaneRegistrationDenial {
    HarnessReadiness(PhysicalIsolationHarnessReadinessDenial),
    CopiedS45ReadinessRows,
    GenericRunner,
    HarnessProjection,
}

pub fn register_physical_isolation_certification_lane(
    entry: &PhysicalIsolationEntryAdmission,
    receipt: PhysicalIsolationHarnessReadinessReceipt,
) -> PhysicalIsolationCertificationLaneRegistration {
    let accepted = accept_store_owned_physical_isolation_harness_readiness(receipt);
    PhysicalIsolationCertificationLaneRegistration::new(entry, accepted)
}

pub fn reject_copied_simulation_harness_readiness_rows_as_physical_isolation_lane_registration(
) -> Result<(), PhysicalIsolationLaneRegistrationDenial> {
    Err(PhysicalIsolationLaneRegistrationDenial::CopiedS45ReadinessRows)
}

pub fn reject_generic_runner_as_physical_isolation_lane_registration(
) -> Result<(), PhysicalIsolationLaneRegistrationDenial> {
    Err(PhysicalIsolationLaneRegistrationDenial::GenericRunner)
}

pub fn reject_harness_projection_as_physical_isolation_lane_registration(
) -> Result<(), PhysicalIsolationLaneRegistrationDenial> {
    Err(PhysicalIsolationLaneRegistrationDenial::HarnessProjection)
}

impl PhysicalIsolationCertificationLaneRegistration {
    fn new(
        entry: &PhysicalIsolationEntryAdmission,
        accepted_harness: AcceptedPhysicalIsolationHarnessReadiness,
    ) -> Self {
        Self {
            entry_recovered_root: entry.recovered_root().to_string(),
            accepted_harness,
        }
    }

    pub fn entry_recovered_root(&self) -> &str {
        &self.entry_recovered_root
    }

    pub const fn accepted_harness(&self) -> &AcceptedPhysicalIsolationHarnessReadiness {
        &self.accepted_harness
    }

    pub const fn does_not_claim_physical_isolation_correctness(&self) -> bool {
        self.accepted_harness
            .does_not_claim_physical_isolation_correctness()
    }
}

impl From<PhysicalIsolationHarnessReadinessDenial> for PhysicalIsolationLaneRegistrationDenial {
    fn from(denial: PhysicalIsolationHarnessReadinessDenial) -> Self {
        Self::HarnessReadiness(denial)
    }
}
