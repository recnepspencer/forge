use crate::production_backed_physical_fixture_materialization;
use worth_store_physical_certification::{
    FixtureCapabilityDeclaration, FixtureMutationBoundary, LargeStoreFixtureProfile,
    PhysicalFixtureBuilder, PhysicalSimulationPlan, ProductionBackedPhysicalFixture,
};

pub fn buffer_pool_evidence(
    plan: &PhysicalSimulationPlan,
) -> worth_store_buffer_pool::BufferPoolExecutedEvidenceSource {
    let mut allocation = worth_store_buffer_pool::AllocationAdmission::from_declaration(
        plan.resource_envelope().allocation(),
    );
    let grant = allocation
        .admit(
            worth_store_buffer_pool::AllocationRequest::copied_payload(
                worth_store_buffer_pool::AllocationScope::Foreground,
                64,
            )
            .unwrap(),
        )
        .unwrap();
    allocation.record_allocation(grant).unwrap();
    worth_store_buffer_pool::BufferPoolExecutedEvidenceSource::from_allocation_execution(
        &allocation,
    )
    .unwrap()
}

pub fn io_queue_evidence(
    plan: &PhysicalSimulationPlan,
) -> worth_store_io_scheduler::IoQueueExecutedEvidenceSource {
    let mut recorder = worth_store_io_scheduler::IoQueueExecutionRecorder::from_envelope(
        plan.resource_envelope().io_queue(),
    );
    recorder.observe_queue_depth(1).unwrap();
    recorder.executed_evidence().unwrap()
}

pub fn production_fixture() -> ProductionBackedPhysicalFixture {
    PhysicalFixtureBuilder::production_backed("phase12-s5-interleaving")
        .materialize_with(
            production_backed_physical_fixture_materialization(
                LargeStoreFixtureProfile::StoreLargerThanMemory,
                15,
            )
            .unwrap(),
        )
        .capability(FixtureCapabilityDeclaration::for_mutation_boundary(
            FixtureMutationBoundary::Manifest,
        ))
        .and_reopen_through_physical_authority()
        .unwrap()
}
