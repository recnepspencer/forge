use super::*;
use worth_foundational::{BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator};
use worth_store_physical_backend::{ProductionStorageBoundarySeam, ScriptedStorageBoundaryControl};
use worth_store_physical_certification::{
    execute_physical_schedule, ExpectedFaultLocalization, FaultDeliveryDenial,
    PhysicalArtifactFaultLocus, PhysicalFaultEvent, PhysicalFaultEventKind,
    PhysicalScheduleOwnerExecution, PhysicalSimulationBoundaryObservation,
    PhysicalSimulationObservationBasis, PhysicalStorageFaultInjection,
};
use worth_store_physical_isolation::{PhysicalPublicationDenial, PhysicalRootPublicationRuntime};
use worth_store_test_support::deterministic_developer_smoke_schedule;
use worth_store_test_support::harness::physical_isolation::publication::{
    admitted_copy_on_write_plan, publication_inputs,
};

#[test]
fn schedule_dispatches_owner_work_and_observation_requires_the_reached_storage_seam() {
    let plan = owner_execution_plan("store.physical.s9.executed.schedule", "executed-schedule");
    let schedule = deterministic_developer_smoke_schedule(&plan).unwrap();
    let first_inputs = publication_inputs();
    let second_inputs = publication_inputs();
    let mut first_runtime =
        PhysicalRootPublicationRuntime::open_for_testing(first_inputs.old_root).unwrap();
    let mut second_runtime =
        PhysicalRootPublicationRuntime::open_for_testing(second_inputs.old_root).unwrap();
    let fault_locus = root_fault_locus();
    let fault = PhysicalFaultEvent::byte_corruption(
        ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
        fault_locus.clone(),
    )
    .unwrap();
    let injection = PhysicalStorageFaultInjection::for_actor_step(
        &fault,
        schedule.actor_steps().first().unwrap(),
    )
    .unwrap();
    assert_reordered_persistence_is_not_falsely_claimed(&schedule, fault_locus);
    let mut first_publication = Some(admitted_copy_on_write_plan(&first_inputs));
    let mut second_publication = Some(admitted_copy_on_write_plan(&second_inputs));
    let mut dispatched = Vec::new();
    let execution = execute_physical_schedule(
        &schedule,
        ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
        Some(&injection),
        |step, control| {
            dispatched.push(step.actor_id().to_owned());
            let attempt = if step.actor_id() == schedule.actor_steps()[0].actor_id() {
                let attempt = first_runtime
                    .attempt_with_boundary_control(first_publication.take().unwrap(), control);
                assert_eq!(
                    attempt.denial(),
                    Some(PhysicalPublicationDenial::PublicationStoreIo),
                );
                attempt
            } else {
                let attempt = second_runtime
                    .attempt_with_boundary_control(second_publication.take().unwrap(), control);
                assert!(attempt.publication().is_some());
                attempt
            };
            Ok::<_, ()>(
                PhysicalScheduleOwnerExecution::from_root_publication_attempt(&attempt, control)
                    .unwrap(),
            )
        },
    )
    .unwrap();
    assert_eq!(
        dispatched,
        schedule
            .actor_steps()
            .iter()
            .map(|step| step.actor_id().to_owned())
            .collect::<Vec<_>>(),
    );
    let fault_execution = injection.confirm_execution(&execution).unwrap();
    assert_eq!(
        fault_execution.target_actor(),
        schedule.actor_steps()[0].actor_id()
    );
    let observation = PhysicalSimulationBoundaryObservation::from_scheduled_storage_execution(
        &plan, &schedule, &execution,
    )
    .unwrap();
    assert_eq!(observation.plan_identity(), plan.identity());
    assert_eq!(
        observation.basis(),
        PhysicalSimulationObservationBasis::ScheduledStorageOwnerExecution
    );
}

#[test]
fn publication_attempt_from_another_control_cannot_substitute_for_actor_execution() {
    let inputs = publication_inputs();
    let mut runtime = PhysicalRootPublicationRuntime::open_for_testing(inputs.old_root).unwrap();
    let other_control = ScriptedStorageBoundaryControl::observe(
        ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
    );
    let attempt =
        runtime.attempt_with_boundary_control(admitted_copy_on_write_plan(&inputs), &other_control);
    let actor_control = ScriptedStorageBoundaryControl::observe(
        ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
    );

    assert_eq!(
        PhysicalScheduleOwnerExecution::from_root_publication_attempt(
            &attempt,
            &actor_control,
        ),
        Err(
            worth_store_physical_certification::PhysicalScheduleOwnerEvidenceDenial::PublicationWasNotExecutedThroughActorControl,
        )
    );
}

fn owner_execution_plan(
    scenario_name: &'static str,
    fixture: &'static str,
) -> worth_store_physical_certification::PhysicalSimulationPlan {
    lower_physical_simulation_plan(
        physical_isolation_scenario(
            scenario_name,
            fixture,
            PhysicalScenarioActor::checkpoint_driver("checkpoint-a"),
            PhysicalScenarioActor::checkpoint_driver("checkpoint-b"),
        ),
        complete_context(PhysicalSimulationProfile::DeveloperSmoke),
    )
    .unwrap()
}

fn root_fault_locus() -> PhysicalArtifactFaultLocus {
    PhysicalArtifactFaultLocus::root_pointer(
        BoundaryArtifactLocator::new(BoundaryArtifactId::new(7), BoundaryArtifactField::Basis),
        ExpectedFaultLocalization::ProductionDriverBoundary,
    )
}

fn assert_reordered_persistence_is_not_falsely_claimed(
    schedule: &PhysicalInterleavingSchedule,
    fault_locus: PhysicalArtifactFaultLocus,
) {
    let reordered = PhysicalFaultEvent::reordered_persistence(
        ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
        fault_locus,
    )
    .unwrap();
    assert!(matches!(
        PhysicalStorageFaultInjection::for_actor_step(
            &reordered,
            schedule.actor_steps().first().unwrap(),
        ),
        Err(FaultDeliveryDenial::FaultHasNoProductionStorageInjection(
            PhysicalFaultEventKind::ReorderedPersistence
        ))
    ));
}
