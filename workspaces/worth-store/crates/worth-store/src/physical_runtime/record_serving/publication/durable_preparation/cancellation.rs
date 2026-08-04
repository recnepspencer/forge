use super::PreparedPhysicalMutation;
use crate::physical_runtime::ProvenNoEffectPhysicalMutation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalPreSealCancellationDenial {
    DurabilityAuthorityReleased,
    BindingMismatch,
    GroupAlreadySealed,
    ReopenedUnresolved,
}

pub enum PhysicalPreSealCancellationOutcome {
    ProvenNoEffect(ProvenNoEffectPhysicalMutation),
    NotCancelled {
        prepared: PreparedPhysicalMutation,
        cause: PhysicalPreSealCancellationDenial,
    },
}
