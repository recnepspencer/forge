use crate::production_backed_physical_fixture_materialization;
use worth_store_physical_certification::{
    FixtureCapabilityDeclaration, FixtureMutationBoundary, LargeStoreFixtureProfile,
    PhysicalFixtureBuilder, PhysicalSimulationPlan, ProductionBackedPhysicalFixture,
};

pub fn store_residency_observation(
    _plan: &PhysicalSimulationPlan,
) -> worth_store::physical_runtime::PhysicalResidencyObservation {
    crate::harness::physical_residency::observed_store_residency(
        "physical-isolation-interleaving",
        crate::harness::physical_residency::PhysicalResidencyFixtureWorkload::Verification,
        64,
    )
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
