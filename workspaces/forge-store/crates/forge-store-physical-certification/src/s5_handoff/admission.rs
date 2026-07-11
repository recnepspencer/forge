use forge_store_readiness::S5SimulationHarnessReadinessDenial;

use super::{S5HarnessFutureExtensionReservation, S5HarnessReadinessReceipt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptedS5SimulationHarnessReadiness {
    readiness: crate::S5SimulationHarnessReadiness,
}

pub fn accept_store_owned_s5_harness_readiness(
    receipt: S5HarnessReadinessReceipt,
) -> AcceptedS5SimulationHarnessReadiness {
    AcceptedS5SimulationHarnessReadiness {
        readiness: receipt.into_readiness(),
    }
}

impl AcceptedS5SimulationHarnessReadiness {
    pub const fn readiness(&self) -> &crate::S5SimulationHarnessReadiness {
        &self.readiness
    }

    pub const fn does_not_claim_s5_correctness(&self) -> bool {
        self.readiness.does_not_claim_s5_correctness()
    }
}

pub fn reject_generic_runner_as_s5_harness_readiness(
) -> Result<(), S5SimulationHarnessReadinessDenial> {
    Err(S5SimulationHarnessReadinessDenial::GenericRunnerCannotSatisfyReadiness)
}

pub fn reject_future_slot_as_s5_harness_readiness(
    _slot: S5HarnessFutureExtensionReservation,
) -> Result<(), S5SimulationHarnessReadinessDenial> {
    Err(S5SimulationHarnessReadinessDenial::FutureBehaviorSlotCannotSatisfyReadiness)
}

pub fn reject_foundational_or_proof_projection_as_s5_harness_readiness(
) -> Result<(), S5SimulationHarnessReadinessDenial> {
    Err(S5SimulationHarnessReadinessDenial::FoundationalOrProofProjectionCannotSatisfyReadiness)
}

pub const fn require_store_owned_s5_harness_receipt(
    receipt: &S5HarnessReadinessReceipt,
) -> &S5HarnessReadinessReceipt {
    receipt
}
