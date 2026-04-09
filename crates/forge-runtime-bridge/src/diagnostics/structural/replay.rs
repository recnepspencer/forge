use crate::error::{BridgeErrorContext, BridgeReplayError, BridgeReplayErrorKind};
use crate::structural::{
    AdmittedStructuralComparisonContract, PlannedStructuralMatchPacketSet, ReducedStructuralMatchSet,
    StructuralComparisonMode,
};

pub(crate) fn validate_structural_replay_contract(
    original: &AdmittedStructuralComparisonContract,
    reconstructed: &AdmittedStructuralComparisonContract,
) -> Result<(), BridgeReplayError> {
    if reconstructed.digest() != original.digest() {
        return Err(BridgeReplayError::new(
            BridgeReplayErrorKind::PlanningContractMismatch,
            format!(
                "Bridge structural replay reconstructed contract `{}` but original contract was `{}`.",
                reconstructed.contract_identity().as_str(),
                original.contract_identity().as_str()
            ),
        )
        .with_context(BridgeErrorContext::default()));
    }

    Ok(())
}

pub(crate) fn validate_structural_replay_outcome(
    planned: &PlannedStructuralMatchPacketSet,
    reduced: &ReducedStructuralMatchSet,
    expected_mode: StructuralComparisonMode,
) -> Result<(), BridgeReplayError> {
    if planned.comparison_mode() != expected_mode {
        return Err(BridgeReplayError::new(
            BridgeReplayErrorKind::PlanningContractMismatch,
            format!(
                "Bridge structural replay reconstructed comparison mode `{:?}` but expected `{:?}`.",
                planned.comparison_mode(),
                expected_mode
            ),
        )
        .with_context(BridgeErrorContext::default()));
    }

    if reduced.planned_packet_set().digest() != planned.digest() {
        return Err(BridgeReplayError::new(
            BridgeReplayErrorKind::DigestMismatch,
            format!(
                "Bridge structural replay reconstructed reduced packet basis `{}` but planned packet set digest was `{}`.",
                reduced.planned_packet_set().digest(),
                planned.digest()
            ),
        )
        .with_context(BridgeErrorContext::default()));
    }

    Ok(())
}
