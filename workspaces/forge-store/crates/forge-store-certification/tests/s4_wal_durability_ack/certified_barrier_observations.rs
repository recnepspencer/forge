use forge_store_physical_backend::{
    BackendDurabilityBarrierAuthority, BackendDurabilityProfile, WalDurabilityBarrier,
    WalDurabilityBarrierReceipt,
};
use forge_store_recovery_physics::{
    WalAppendDurabilityScope, WalAppendPlan, WalAppendProgress, WalDurabilityObservation,
    WalDurabilityObservationSequence,
};

pub fn observation_sequence<P: BackendDurabilityProfile>(
    plan: WalAppendPlan<P>,
) -> WalDurabilityObservationSequence<P> {
    WalDurabilityObservationSequence::new(plan.record_written_bytes(4096))
}

pub fn completed_barrier<P, A>(
    progress: &WalAppendProgress<P>,
    authority: A,
    barrier: WalDurabilityBarrier,
) -> WalDurabilityObservation<P>
where
    P: BackendDurabilityProfile,
    A: BackendDurabilityBarrierAuthority<P>,
{
    WalDurabilityObservation::Completed(certified_barrier(progress, authority, barrier))
}

pub fn certified_barrier<P, A>(
    progress: &WalAppendProgress<P>,
    authority: A,
    barrier: WalDurabilityBarrier,
) -> WalDurabilityBarrierReceipt<P, WalAppendDurabilityScope>
where
    P: BackendDurabilityProfile,
    A: BackendDurabilityBarrierAuthority<P>,
{
    authority
        .certify_completed_barrier(progress.durability_scope(), barrier)
        .unwrap()
}
