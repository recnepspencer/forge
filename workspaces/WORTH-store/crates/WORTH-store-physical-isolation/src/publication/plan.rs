use super::{
    AtomicPhysicalRootSwap, PhysicalIdentityReuse, PhysicalPublicationCounterSnapshot,
    PhysicalPublicationDenial, PhysicalPublicationReadiness, PhysicalPublicationReleasePosture,
    RootSwapOrderingContract, ValidatedPhysicalPublicationIntent,
};

#[derive(Debug, Clone)]
pub struct LoweredCopyOnWritePublicationPlan {
    intent: ValidatedPhysicalPublicationIntent,
    ordering: RootSwapOrderingContract,
    counters: PhysicalPublicationCounterSnapshot,
}

#[derive(Debug, Clone)]
pub struct CopyOnWritePublicationPlan {
    intent: ValidatedPhysicalPublicationIntent,
    ordering: RootSwapOrderingContract,
    readiness: PhysicalPublicationReadiness,
    atomic_swap: AtomicPhysicalRootSwap,
    counters: PhysicalPublicationCounterSnapshot,
}

impl ValidatedPhysicalPublicationIntent {
    pub fn lower_with_ordering(
        self,
        ordering: RootSwapOrderingContract,
    ) -> Result<LoweredCopyOnWritePublicationPlan, PhysicalPublicationDenial> {
        Ok(LoweredCopyOnWritePublicationPlan {
            intent: self,
            ordering,
            counters: PhysicalPublicationCounterSnapshot::for_validated_lowering(),
        })
    }
}

impl LoweredCopyOnWritePublicationPlan {
    pub fn join_readiness(
        self,
        readiness: PhysicalPublicationReadiness,
    ) -> Result<CopyOnWritePublicationPlan, PhysicalPublicationDenial> {
        let readiness = readiness.validate_for_intent(&self.intent)?;
        Ok(CopyOnWritePublicationPlan {
            atomic_swap: AtomicPhysicalRootSwap::new(self.ordering),
            intent: self.intent,
            ordering: self.ordering,
            readiness,
            counters: self.counters.with_readiness_join(),
        })
    }

    pub const fn counters(&self) -> PhysicalPublicationCounterSnapshot {
        self.counters
    }
}

impl CopyOnWritePublicationPlan {
    pub(crate) const fn intent(&self) -> &ValidatedPhysicalPublicationIntent {
        &self.intent
    }

    pub(crate) const fn counters(&self) -> PhysicalPublicationCounterSnapshot {
        self.counters
    }

    pub(crate) const fn publish_ordering(&self) -> RootSwapOrderingContract {
        self.ordering
    }

    pub(crate) const fn release_posture(&self) -> PhysicalPublicationReleasePosture {
        match self.intent.identity_reuse() {
            PhysicalIdentityReuse::None => {
                PhysicalPublicationReleasePosture::OldReachabilityRetainedUntilReadRelease
            }
            PhysicalIdentityReuse::Requested => {
                PhysicalPublicationReleasePosture::IdentityReuseProtectedByAllocatorFence
            }
        }
    }

    pub const fn readiness(&self) -> PhysicalPublicationReadiness {
        self.readiness
    }

    pub const fn atomic_swap(&self) -> AtomicPhysicalRootSwap {
        self.atomic_swap
    }
}
