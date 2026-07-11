use forge_store_readiness::PhysicalIsolationHarnessReadinessDenial;

use super::{
    PhysicalIsolationHarnessFutureExtensionReservation, PhysicalIsolationHarnessReadinessReceipt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptedPhysicalIsolationHarnessReadiness {
    readiness: crate::PhysicalIsolationHarnessReadiness,
}

pub fn accept_store_owned_physical_isolation_harness_readiness(
    receipt: PhysicalIsolationHarnessReadinessReceipt,
) -> AcceptedPhysicalIsolationHarnessReadiness {
    AcceptedPhysicalIsolationHarnessReadiness {
        readiness: receipt.into_readiness(),
    }
}

impl AcceptedPhysicalIsolationHarnessReadiness {
    pub const fn readiness(&self) -> &crate::PhysicalIsolationHarnessReadiness {
        &self.readiness
    }

    pub const fn does_not_claim_physical_isolation_correctness(&self) -> bool {
        self.readiness
            .does_not_claim_physical_isolation_correctness()
    }
}

pub fn reject_generic_runner_as_physical_isolation_harness_readiness(
) -> Result<(), PhysicalIsolationHarnessReadinessDenial> {
    Err(PhysicalIsolationHarnessReadinessDenial::GenericRunnerCannotSatisfyReadiness)
}

pub fn reject_future_slot_as_physical_isolation_harness_readiness(
    _slot: PhysicalIsolationHarnessFutureExtensionReservation,
) -> Result<(), PhysicalIsolationHarnessReadinessDenial> {
    Err(PhysicalIsolationHarnessReadinessDenial::FutureBehaviorSlotCannotSatisfyReadiness)
}

pub fn reject_foundational_or_proof_projection_as_physical_isolation_harness_readiness(
) -> Result<(), PhysicalIsolationHarnessReadinessDenial> {
    Err(PhysicalIsolationHarnessReadinessDenial::FoundationalOrProofProjectionCannotSatisfyReadiness)
}

pub const fn require_store_owned_physical_isolation_harness_receipt(
    receipt: &PhysicalIsolationHarnessReadinessReceipt,
) -> &PhysicalIsolationHarnessReadinessReceipt {
    receipt
}
