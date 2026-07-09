use worth_store_physical_isolation::{
    s5_simulation_harness_readiness_requirement, PhysicalIsolationEntryAdmission,
};
use worth_store_readiness::S5SimulationHarnessReadinessDenial;

use super::{
    accept_store_owned_s5_harness_readiness, AcceptedS5SimulationHarnessReadiness,
    S5HarnessReadinessReceipt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5PhysicalIsolationCertificationLaneRegistration {
    entry_recovered_root: String,
    accepted_harness: AcceptedS5SimulationHarnessReadiness,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S5PhysicalIsolationLaneRegistrationDenial {
    HarnessReadiness(S5SimulationHarnessReadinessDenial),
    CopiedS45ReadinessRows,
    GenericRunner,
    HarnessProjection,
}

pub fn register_s5_physical_isolation_certification_lane(
    entry: &PhysicalIsolationEntryAdmission,
    receipt: S5HarnessReadinessReceipt,
) -> S5PhysicalIsolationCertificationLaneRegistration {
    let accepted = accept_store_owned_s5_harness_readiness(
        receipt,
        s5_simulation_harness_readiness_requirement(),
    );
    S5PhysicalIsolationCertificationLaneRegistration::new(entry, accepted)
}

pub fn reject_copied_s45_readiness_rows_as_s5_lane_registration(
) -> Result<(), S5PhysicalIsolationLaneRegistrationDenial> {
    Err(S5PhysicalIsolationLaneRegistrationDenial::CopiedS45ReadinessRows)
}

pub fn reject_generic_runner_as_s5_lane_registration(
) -> Result<(), S5PhysicalIsolationLaneRegistrationDenial> {
    Err(S5PhysicalIsolationLaneRegistrationDenial::GenericRunner)
}

pub fn reject_harness_projection_as_s5_lane_registration(
) -> Result<(), S5PhysicalIsolationLaneRegistrationDenial> {
    Err(S5PhysicalIsolationLaneRegistrationDenial::HarnessProjection)
}

impl S5PhysicalIsolationCertificationLaneRegistration {
    fn new(
        entry: &PhysicalIsolationEntryAdmission,
        accepted_harness: AcceptedS5SimulationHarnessReadiness,
    ) -> Self {
        Self {
            entry_recovered_root: entry.recovered_root().to_string(),
            accepted_harness,
        }
    }

    pub fn entry_recovered_root(&self) -> &str {
        &self.entry_recovered_root
    }

    pub const fn accepted_harness(&self) -> &AcceptedS5SimulationHarnessReadiness {
        &self.accepted_harness
    }

    pub const fn does_not_claim_s5_correctness(&self) -> bool {
        self.accepted_harness.does_not_claim_s5_correctness()
    }
}

impl From<S5SimulationHarnessReadinessDenial> for S5PhysicalIsolationLaneRegistrationDenial {
    fn from(denial: S5SimulationHarnessReadinessDenial) -> Self {
        Self::HarnessReadiness(denial)
    }
}
