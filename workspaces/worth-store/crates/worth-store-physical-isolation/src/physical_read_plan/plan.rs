use crate::{
    CurrentPhysicalRoot, EpochComparisonScope, LatchAcquisitionPlan, PhysicalEpochVector,
    PhysicalEpochVectorDenial, PhysicalOrderingContract,
};

use super::{
    PhysicalReadPlanAdmissionDenial, PhysicalReadPlanFootprint, PhysicalReadPlanReleaseSemantics,
    PhysicalReadPlanRetryPosture, PhysicalReadReachabilityBarrier, ReadPlanCounterSnapshot,
    StablePhysicalReadHandle,
};

#[derive(Debug)]
pub struct SeedStableReadPlan {
    root: CurrentPhysicalRoot,
    epoch_vector: PhysicalEpochVector,
    footprint: PhysicalReadPlanFootprint,
    latch_plan: LatchAcquisitionPlan,
    reachability_barrier: PhysicalReadReachabilityBarrier,
    release: PhysicalReadPlanReleaseSemantics,
    retry_posture: PhysicalReadPlanRetryPosture,
    counters: ReadPlanCounterSnapshot,
}

#[derive(Debug)]
pub struct StablePhysicalReadPlan {
    seed: SeedStableReadPlan,
}

#[derive(Debug)]
pub struct StablePhysicalReadPlanAdmission {
    plan: StablePhysicalReadPlan,
}

#[derive(Debug)]
pub(crate) struct PhysicalReadPlanCompletion {
    reachability_barrier: PhysicalReadReachabilityBarrier,
    release: PhysicalReadPlanReleaseSemantics,
    retry_posture: PhysicalReadPlanRetryPosture,
    counters: ReadPlanCounterSnapshot,
}

impl PhysicalReadPlanCompletion {
    pub(crate) const fn new(
        reachability_barrier: PhysicalReadReachabilityBarrier,
        release: PhysicalReadPlanReleaseSemantics,
        retry_posture: PhysicalReadPlanRetryPosture,
        counters: ReadPlanCounterSnapshot,
    ) -> Self {
        Self {
            reachability_barrier,
            release,
            retry_posture,
            counters,
        }
    }
}

impl SeedStableReadPlan {
    pub(crate) const fn new(
        root: CurrentPhysicalRoot,
        epoch_vector: PhysicalEpochVector,
        footprint: PhysicalReadPlanFootprint,
        latch_plan: LatchAcquisitionPlan,
        completion: PhysicalReadPlanCompletion,
    ) -> Self {
        Self {
            root,
            epoch_vector,
            footprint,
            latch_plan,
            reachability_barrier: completion.reachability_barrier,
            release: completion.release,
            retry_posture: completion.retry_posture,
            counters: completion.counters,
        }
    }

    pub fn admit(self) -> StablePhysicalReadPlanAdmission {
        StablePhysicalReadPlanAdmission {
            plan: StablePhysicalReadPlan { seed: self },
        }
    }
}

impl StablePhysicalReadPlanAdmission {
    pub fn into_plan(self) -> StablePhysicalReadPlan {
        self.plan
    }
}

impl StablePhysicalReadPlan {
    pub const fn root_epoch(&self) -> crate::RootEpoch {
        self.seed.root.epoch()
    }

    pub const fn root(&self) -> CurrentPhysicalRoot {
        self.seed.root
    }

    pub const fn epoch_vector(&self) -> PhysicalEpochVector {
        self.seed.epoch_vector
    }

    pub const fn manifest_epoch(&self) -> crate::ManifestEpoch {
        self.seed.root.manifest_epoch()
    }

    pub const fn ordering(&self) -> PhysicalOrderingContract {
        self.seed.root.ordering()
    }

    pub const fn footprint(&self) -> &PhysicalReadPlanFootprint {
        &self.seed.footprint
    }

    pub const fn latch_plan(&self) -> &LatchAcquisitionPlan {
        &self.seed.latch_plan
    }

    pub const fn reachability_barrier(&self) -> PhysicalReadReachabilityBarrier {
        self.seed.reachability_barrier
    }

    pub const fn release_semantics(&self) -> PhysicalReadPlanReleaseSemantics {
        self.seed.release
    }

    pub const fn retry_posture(&self) -> PhysicalReadPlanRetryPosture {
        self.seed.retry_posture
    }

    pub const fn counters(&self) -> ReadPlanCounterSnapshot {
        self.seed.counters
    }

    pub fn into_execution_ready_handle(self) -> StablePhysicalReadHandle {
        StablePhysicalReadHandle::new(self)
    }
}

pub fn physical_epoch_vector_for_current_root(
    root: CurrentPhysicalRoot,
) -> Result<PhysicalEpochVector, PhysicalEpochVectorDenial> {
    PhysicalEpochVector::for_scope(EpochComparisonScope::read_plan_admission(root.scope()))
        .with_root(root.epoch())
        .with_manifest(root.manifest_epoch())
        .seal()
}

pub fn admit_seed_stable_read_plan(
    plan: SeedStableReadPlan,
) -> Result<StablePhysicalReadPlan, PhysicalReadPlanAdmissionDenial> {
    Ok(plan.admit().into_plan())
}
