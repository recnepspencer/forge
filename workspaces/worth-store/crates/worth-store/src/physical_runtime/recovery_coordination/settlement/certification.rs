use crate::physical_runtime::work::PhysicalExecutorDispatch;
use crate::physical_runtime::{PhysicalSignalSettlementOutcome, SettledPhysicalWork};

use super::super::{
    PhysicalRecoveryCleanupCommandStage, PhysicalRecoveryCoordination,
    PhysicalRecoveryFreshReopenStage, PhysicalRecoveryPublicationCommandStage,
    PhysicalRecoveryStagingCommandStage,
};

#[derive(Clone, Copy)]
pub(in crate::physical_runtime::recovery_coordination) enum PhysicalRecoverySettlementCertificationStage
{
    Staging(PhysicalRecoveryStagingCommandStage),
    Publication(PhysicalRecoveryPublicationCommandStage),
    FreshReopen(PhysicalRecoveryFreshReopenStage),
    Cleanup(PhysicalRecoveryCleanupCommandStage),
}

pub(in crate::physical_runtime::recovery_coordination) fn settle_with_certification(
    coordination: &PhysicalRecoveryCoordination,
    dispatch: PhysicalExecutorDispatch,
    stage: PhysicalRecoverySettlementCertificationStage,
) -> PhysicalSignalSettlementOutcome {
    super::settle_with(coordination, dispatch, |coordination, settled| {
        if coordination.take_certification_signal_failure(stage) {
            retain_failed_completion(coordination, settled)
        } else {
            coordination.signal.record_settlement(settled)
        }
    })
}

fn retain_failed_completion(
    coordination: &PhysicalRecoveryCoordination,
    settled: &SettledPhysicalWork,
) -> PhysicalSignalSettlementOutcome {
    assert!(
        coordination
            .signal
            .settlement_requires_derived_completion(settled),
        "certification settlement failure requires derived completion, got {:?}",
        settled.evidence().fate(),
    );
    assert!(
        coordination.signal.retain_settlement_obligation(settled),
        "certification settlement failure must retain its physical obligation",
    );
    PhysicalSignalSettlementOutcome::DerivedStateUnavailable
}
