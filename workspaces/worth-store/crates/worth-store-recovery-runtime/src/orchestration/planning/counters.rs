use worth_store::physical_runtime::{
    StoreRecoveryBindingFreshness, StoreRecoveryBindingFreshnessSample,
};
use worth_store_recovery_physics::{
    PhysicalRedoPlanCounters, ReconciledOperationFates, RecoveryOperationFate,
    RecoveryPlanningCounters,
};

pub(super) fn after_sample(
    sample: &StoreRecoveryBindingFreshnessSample,
) -> RecoveryPlanningCounters {
    build(sample, None, PhysicalRedoPlanCounters::default(), 0, 0)
}

pub(super) fn failed_sample(
    freshness_retained: u64,
    freshness_expired: u64,
) -> RecoveryPlanningCounters {
    RecoveryPlanningCounters::new(
        0,
        0,
        PhysicalRedoPlanCounters::default(),
        freshness_retained,
        freshness_expired,
        [0; 4],
    )
}

pub(super) fn after_fates(
    sample: &StoreRecoveryBindingFreshnessSample,
    fates: &ReconciledOperationFates,
    redo: PhysicalRedoPlanCounters,
    reads: u64,
    bytes: u64,
) -> RecoveryPlanningCounters {
    build(sample, Some(fates), redo, reads, bytes)
}

fn build(
    sample: &StoreRecoveryBindingFreshnessSample,
    fates: Option<&ReconciledOperationFates>,
    redo: PhysicalRedoPlanCounters,
    reads: u64,
    bytes: u64,
) -> RecoveryPlanningCounters {
    let (freshness_retained, freshness_expired) =
        sample
            .operations()
            .iter()
            .fold(
                (0_u64, 0_u64),
                |(retained, expired), operation| match operation.freshness() {
                    StoreRecoveryBindingFreshness::Retained => (retained + 1, expired),
                    StoreRecoveryBindingFreshness::ExpiredAtSelectedCheckpoint => {
                        (retained, expired + 1)
                    }
                },
            );
    let mut fate_counts = [0_u64; 4];
    if let Some(fates) = fates {
        for fate in fates.operations() {
            let index = match fate.fate() {
                RecoveryOperationFate::AcknowledgedDurable => 0,
                RecoveryOperationFate::DurableUnacknowledged => 1,
                RecoveryOperationFate::ProvenNoEffect => 2,
                RecoveryOperationFate::Indeterminate => 3,
            };
            fate_counts[index] += 1;
        }
    }
    RecoveryPlanningCounters::new(
        reads,
        bytes,
        redo,
        freshness_retained,
        freshness_expired,
        fate_counts,
    )
}
