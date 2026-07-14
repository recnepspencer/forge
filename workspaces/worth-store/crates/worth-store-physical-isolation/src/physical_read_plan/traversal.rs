use crate::{
    lower_latch_acquisition_plan, LatchAcquisitionPlan, LatchAcquisitionRequest,
    LatchAcquisitionStep, PhysicalLatchKey,
};

use super::{
    CompactProtectedReferenceSet, PhysicalReadPlanAdmissionDenial, PhysicalReadPlanFootprint,
    PhysicalReadPlanRetryPosture, ProtectedPhysicalReference, ReadPlanAdmissionScratchArena,
    ReadPlanCounterSnapshot, SeedStableReadPlan, ValidatedRootObservation,
};

#[derive(Debug, Clone)]
pub struct TraversalAdmissionGuard {
    validated: ValidatedRootObservation,
}

#[derive(Debug)]
pub struct TraversalAdmissionReceipt {
    seed: SeedStableReadPlan,
}

#[derive(Debug)]
pub struct StepwiseStableReadCursor {
    receipt: TraversalAdmissionReceipt,
}

impl TraversalAdmissionGuard {
    pub const fn from_validated_root(validated: ValidatedRootObservation) -> Self {
        Self { validated }
    }

    pub fn admit(
        self,
        scratch: ReadPlanAdmissionScratchArena,
    ) -> Result<TraversalAdmissionReceipt, PhysicalReadPlanAdmissionDenial> {
        let intent = self.validated.hazard().intent();
        let compact = CompactProtectedReferenceSet::from_reference_set_with_scratch(
            intent.protected_references().clone(),
            scratch,
        )?;
        let latch_plan =
            lower_latch_acquisition_plan(LatchAcquisitionRequest::for_declared_footprint(
                latch_steps(intent.root(), compact.references())?,
            ))?;
        let footprint = PhysicalReadPlanFootprint::new(compact, intent.resident_bytes());
        let counters = counters(&footprint, &latch_plan, self.validated.retry_posture());
        Ok(TraversalAdmissionReceipt {
            seed: SeedStableReadPlan::new(
                intent.root(),
                self.validated.epoch_vector(),
                footprint,
                latch_plan,
                self.validated.reachability_barrier(),
                intent
                    .release()
                    .ok_or(PhysicalReadPlanAdmissionDenial::MissingReleaseSemantics)?,
                self.validated.retry_posture(),
                counters,
            ),
        })
    }
}

impl TraversalAdmissionReceipt {
    pub fn into_cursor(self) -> StepwiseStableReadCursor {
        StepwiseStableReadCursor { receipt: self }
    }

    pub fn lower_to_seed(self) -> SeedStableReadPlan {
        self.seed
    }
}

impl StepwiseStableReadCursor {
    pub fn finish(self) -> SeedStableReadPlan {
        self.receipt.seed
    }
}

fn latch_steps(
    root: crate::CurrentPhysicalRoot,
    references: &[ProtectedPhysicalReference],
) -> Result<Vec<LatchAcquisitionStep>, PhysicalReadPlanAdmissionDenial> {
    let mut steps = vec![
        LatchAcquisitionStep::shared(PhysicalLatchKey::root(root.epoch())),
        LatchAcquisitionStep::shared(PhysicalLatchKey::manifest(
            root.epoch(),
            root.manifest_epoch(),
        )),
    ];
    for reference in references {
        let key = match super::footprint_ranges::latch_domain(*reference) {
            worth_store_physical_format::PhysicalCellReuseDomain::Segment => {
                PhysicalLatchKey::segment(
                    root.epoch(),
                    root.admit_segment_publication_epoch(reference.current_generation())?
                        .epoch(),
                )
            }
            worth_store_physical_format::PhysicalCellReuseDomain::ExtentAllocation
            | worth_store_physical_format::PhysicalCellReuseDomain::FreeSpaceReuse => {
                PhysicalLatchKey::extent(
                    root.epoch(),
                    root.admit_extent_publication_epoch(reference.current_generation())?
                        .epoch(),
                )
            }
            worth_store_physical_format::PhysicalCellReuseDomain::Page
            | worth_store_physical_format::PhysicalCellReuseDomain::SlotAllocation => {
                PhysicalLatchKey::page(
                    root.epoch(),
                    root.admit_page_publication_epoch(reference.current_generation())?
                        .epoch(),
                )
            }
            worth_store_physical_format::PhysicalCellReuseDomain::RootPublication => continue,
        };
        steps.push(LatchAcquisitionStep::shared(key));
    }
    Ok(steps)
}

fn counters(
    footprint: &PhysicalReadPlanFootprint,
    latch_plan: &LatchAcquisitionPlan,
    retry_posture: PhysicalReadPlanRetryPosture,
) -> ReadPlanCounterSnapshot {
    let scratch_usage = footprint.protected().scratch_usage().with_latch_lowering();
    ReadPlanCounterSnapshot::new(
        footprint.protected().references().len() as u64,
        footprint.protected().ranges().ranges().len() as u64,
        latch_plan.steps().len() as u64,
        1 + footprint.protected().references().len() as u64,
        retry_posture.retry_decisions(),
        footprint.resident_bytes(),
        footprint.protected().references().len() as u64,
        1,
        scratch_usage.protected_reference_capacity() as u64,
        scratch_usage.scratch_allocations(),
        scratch_usage.allocation_events(),
    )
}
