use super::{PhysicalReadIoPosture, StablePhysicalReadExecutionCounters};
use crate::{PhysicalReadPlanReleaseReceipt, StablePhysicalReadFoundationalEvidence};

#[cfg(any(test, feature = "certification-authority"))]
use crate::epoch::{manifest_epoch_from_entry_seed, root_epoch_from_entry_seed};
#[cfg(any(test, feature = "certification-authority"))]
use crate::{
    admit_seed_stable_read_plan, lower_latch_acquisition_plan, CompactionReadInterlockPlan,
    CurrentPhysicalRoot, CurrentPhysicalRootBasis, GenerationCountedPhysicalReference,
    LatchAcquisitionRequest, LatchAcquisitionStep, PhysicalLatchKey, PhysicalOrderingContract,
    PhysicalReadPlanFootprint, PhysicalReadPlanReleaseSemantics, PhysicalReadPlanRetryPosture,
    PhysicalReadProtectedFootprintBasis, PhysicalReadReachabilityBarrier,
    ProtectedPhysicalReferenceSet, ReadPlanAdmissionScratchArena, ReadPlanCounterSnapshot,
    SeedStableReadPlan, StablePhysicalReadPlan,
};
#[cfg(any(test, feature = "certification-authority"))]
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalSegmentId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StablePhysicalReadReceipt {
    read_plan_release: PhysicalReadPlanReleaseReceipt,
    counters: StablePhysicalReadExecutionCounters,
    io_posture: PhysicalReadIoPosture,
}

impl StablePhysicalReadReceipt {
    pub(crate) const fn new(
        read_plan_release: PhysicalReadPlanReleaseReceipt,
        counters: StablePhysicalReadExecutionCounters,
        io_posture: PhysicalReadIoPosture,
    ) -> Self {
        Self {
            read_plan_release,
            counters,
            io_posture,
        }
    }

    pub const fn read_plan_release(self) -> PhysicalReadPlanReleaseReceipt {
        self.read_plan_release
    }

    pub const fn counters(self) -> StablePhysicalReadExecutionCounters {
        self.counters
    }

    pub const fn io_posture(self) -> PhysicalReadIoPosture {
        self.io_posture
    }

    pub fn lower_to_foundational_evidence(&self) -> StablePhysicalReadFoundationalEvidence {
        StablePhysicalReadFoundationalEvidence::lower(self)
    }
}

#[cfg(any(test, feature = "certification-authority"))]
pub fn stable_physical_read_receipt_for_certification_test(
    guarded_bytes: u64,
) -> StablePhysicalReadReceipt {
    let root = current_root_for_certification_seed(17);
    stable_physical_read_receipt_for_certification_root(root, guarded_bytes)
}

#[cfg(any(test, feature = "certification-authority"))]
pub fn stable_physical_read_receipt_for_mismatched_compaction_test(
    guarded_bytes: u64,
) -> StablePhysicalReadReceipt {
    let root = current_root_for_certification_seed(18);
    stable_physical_read_receipt_for_certification_root(root, guarded_bytes)
}

#[cfg(any(test, feature = "certification-authority"))]
pub fn stable_physical_read_receipt_for_compaction_plan_test(
    plan: &CompactionReadInterlockPlan,
    guarded_bytes: u64,
) -> StablePhysicalReadReceipt {
    StablePhysicalReadReceipt::new(
        PhysicalReadPlanReleaseReceipt::new(
            plan.protected().root(),
            plan.protected().footprint_basis(),
        ),
        StablePhysicalReadExecutionCounters::for_certification_test(guarded_bytes),
        PhysicalReadIoPosture::ordinary(),
    )
}

#[cfg(any(test, feature = "certification-authority"))]
pub fn stable_physical_read_receipt_for_certification_root(
    root: CurrentPhysicalRoot,
    guarded_bytes: u64,
) -> StablePhysicalReadReceipt {
    let footprint = PhysicalReadProtectedFootprintBasis::for_certification_test(1);
    StablePhysicalReadReceipt::new(
        PhysicalReadPlanReleaseReceipt::new(root, footprint),
        StablePhysicalReadExecutionCounters::for_certification_test(guarded_bytes),
        PhysicalReadIoPosture::ordinary(),
    )
}

#[cfg(any(test, feature = "certification-authority"))]
pub fn stable_physical_read_plan_for_certification_test(
    guarded_bytes: u64,
) -> StablePhysicalReadPlan {
    let root = current_root_for_certification_seed(17);
    let reference = current_page_slot_reference_for_certification_test();
    let protected = ProtectedPhysicalReferenceSet::from_current_generation_refs_with_scratch(
        [reference],
        ReadPlanAdmissionScratchArena::for_protected_reference_capacity(1),
    )
    .expect("certification read plan protected set should admit");
    let compact = crate::CompactProtectedReferenceSet::from_reference_set_with_scratch(
        protected,
        ReadPlanAdmissionScratchArena::for_protected_reference_capacity(1),
    )
    .expect("certification read plan compact footprint should admit");
    let footprint = PhysicalReadPlanFootprint::new(compact, guarded_bytes);
    let release = PhysicalReadPlanReleaseSemantics::reader_releases_all();
    let page_epoch = root
        .admit_page_publication_epoch(reference)
        .expect("certification page slot reference should admit")
        .epoch();
    let latch_plan =
        lower_latch_acquisition_plan(LatchAcquisitionRequest::for_declared_footprint(vec![
            LatchAcquisitionStep::shared(PhysicalLatchKey::root(root.epoch())),
            LatchAcquisitionStep::shared(PhysicalLatchKey::manifest(
                root.epoch(),
                root.manifest_epoch(),
            )),
            LatchAcquisitionStep::shared(PhysicalLatchKey::page(root.epoch(), page_epoch)),
        ]))
        .expect("certification latch plan should lower");
    let scratch_usage = footprint.protected().scratch_usage().with_latch_lowering();
    admit_seed_stable_read_plan(SeedStableReadPlan::new(
        root,
        crate::physical_epoch_vector_for_current_root(root)
            .expect("certification root epoch vector should admit"),
        footprint,
        latch_plan,
        PhysicalReadReachabilityBarrier::from_footprint_basis(
            PhysicalReadProtectedFootprintBasis::for_certification_test(1),
            release,
        ),
        release,
        PhysicalReadPlanRetryPosture::Current,
        ReadPlanCounterSnapshot::new(
            1,
            1,
            3,
            2,
            0,
            guarded_bytes,
            1,
            1,
            scratch_usage.protected_reference_capacity() as u64,
            scratch_usage.scratch_allocations(),
            scratch_usage.allocation_events(),
        ),
    ))
    .expect("certification stable read plan should admit")
}

#[cfg(any(test, feature = "certification-authority"))]
fn current_root_for_certification_seed(seed: u64) -> CurrentPhysicalRoot {
    let basis = CurrentPhysicalRootBasis::new(
        root_epoch_from_entry_seed(seed),
        manifest_epoch_from_entry_seed(seed),
    );
    CurrentPhysicalRoot::from_s5_entry(basis, PhysicalOrderingContract::root_swap_acquire_release())
        .expect("certification root ordering should admit")
}

#[cfg(any(test, feature = "certification-authority"))]
fn current_page_slot_reference_for_certification_test() -> crate::CurrentGenerationPhysicalReference
{
    let generation = PhysicalGeneration::from_raw(9).expect("generation");
    let slot_cell = PhysicalGenerationAuthority::s1()
        .slot_cell(
            PhysicalSegmentId::from_raw(7).expect("segment"),
            PhysicalPageId::from_raw(11).expect("page"),
            PhysicalRecordSlot::from_raw(1).expect("slot"),
        )
        .with_slot_generation(generation);
    GenerationCountedPhysicalReference::from_admitted_reference(
        PhysicalReferenceAuthority::s1().admit_page_slot(slot_cell),
    )
    .require_current_generation(generation)
    .expect("certification physical reference should be current")
}
