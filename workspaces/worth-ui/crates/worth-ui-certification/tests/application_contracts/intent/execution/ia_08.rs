use worth_ui::facade::{
    intent::{
        UiIntentAdmissionDecision, UiIntentAdmissionStopReason, UiIntentConsequenceContract,
        UiIntentConsequencePublicationOutcome, UiIntentConsequenceStopReason, UiIntentDefinition,
        UiIntentExecutionDispatchOutcome, UiIntentExecutionReservationDenial,
        UiIntentExecutionTransition, UiIntentExecutionTransitionPosture,
        UiIntentOperabilityOutcome, UiIntentRecoveryProgressOutcome,
        UiIntentRecoveryProgressPosture, UiIntentRuntimeServiceDestination, UiIntentSupportPosture,
        UiIntentTransitionDestination,
    },
    rebind::{UiRebindExecutionPolicy, UiRebindExecutionRequest},
};
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_host_headless::{UiHeadlessRecorderCapacity, WorthUiHeadlessRecorder};
use worth_ui_runtime::{
    certification_support::{
        UiIntentExecutionCapacityCertificationProfile,
        WorthUiIntentExecutionReservationCertificationExt,
    },
    facade::measurement_exchange::UiViewportExtentObservation,
};

use super::{
    consequence::ConsequenceWorld,
    lifecycle::{
        advance, dispatch, execution_census, AttemptStep, ExecutionScript, RecoveryStep,
        ScriptedProvider,
    },
};
use crate::intent::{
    admission::phase3::world::AdmissionWorld,
    operability::{replacement_input, EmptyOutcome, OperabilityFacts, PrimaryIntent},
};

#[test]
fn framework_execution_matrix_preserves_destination_scope_and_terminal_honesty() {
    prove_framework_execution_matrix_preserves_destination_scope_and_terminal_honesty();
}

pub(in crate::intent) fn prove_framework_execution_matrix_preserves_destination_scope_and_terminal_honesty(
) {
    prove_destination_selection();
    prove_reservation_scope();
    prove_terminal_matrix();
    prove_timeout_retry();
    prove_separate_domain_admission();
}

fn prove_destination_selection() {
    let mut transition = destination_world(BuiltinDestination::Transition);
    let outcome = transition.evaluate(0);
    assert!(matches!(outcome, UiIntentOperabilityOutcome::Operable(_)));
    let admitted = match transition.session.admit_intent(
        UiIntentDefinition::<PrimaryIntent>::ui_transition(
            UiIntentTransitionDestination::NavigatePage,
        ),
        outcome,
    ) {
        UiIntentAdmissionDecision::Admitted(admitted) => admitted,
        _ => panic!("the supported internal transition must admit"),
    };
    assert!(matches!(
        transition
            .session
            .dispatch_admitted_intent(admitted, super::execution_deadline(20)),
        UiIntentExecutionDispatchOutcome::AttemptPrepared(_)
    ));
    assert!(matches!(
        only(advance(&mut transition, 1)).posture(),
        UiIntentExecutionTransitionPosture::Completed { .. }
    ));
    assert_eq!(execution_census(&transition), [0, 0, 0, 1]);
    assert_shutdown_zero(transition);

    let mut unsupported = destination_world(BuiltinDestination::UnsupportedService);
    let outcome = unsupported.evaluate(0);
    let UiIntentOperabilityOutcome::Inoperable(ref decision) = outcome else {
        panic!("an explicitly unsupported runtime service cannot become operable")
    };
    assert_eq!(
        decision.decision().support(),
        UiIntentSupportPosture::Unsupported
    );
    assert!(matches!(
        unsupported.session.admit_intent(
            UiIntentDefinition::<PrimaryIntent>::runtime_service(
                UiIntentRuntimeServiceDestination::InvokeCommand,
            ),
            outcome,
        ),
        UiIntentAdmissionDecision::Stopped(ref stop)
            if matches!(stop.reason(), UiIntentAdmissionStopReason::Inoperable(_))
    ));
    assert_eq!(execution_census(&unsupported), [0, 0, 0, 0]);
    assert_shutdown_zero(unsupported);
}

fn prove_reservation_scope() {
    prove_capacity_denial(
        capacity(1, 16),
        UiIntentExecutionReservationDenial::ApplicationCapacityExceeded { maximum: 1 },
    );
    prove_capacity_denial(
        capacity(16, 1),
        UiIntentExecutionReservationDenial::DestinationCapacityExceeded {
            destination: worth_ui::facade::intent::UiIntentExecutionDestination::ApplicationEffect,
            maximum: 1,
        },
    );
}

fn prove_capacity_denial(
    profile: UiIntentExecutionCapacityCertificationProfile,
    expected: UiIntentExecutionReservationDenial,
) {
    let mut world = AdmissionWorld::launch(2);
    assert!(world
        .session
        .install_intent_execution_capacity_for_certification(profile));
    let occupied = world.admit_exact(0);
    assert!(matches!(
        world.admit(1),
        UiIntentAdmissionDecision::Stopped(ref stop)
            if stop.reason()
                == &UiIntentAdmissionStopReason::ExecutionReservation(expected)
    ));
    assert_eq!(execution_census(&world), [1, 1, 0, 0]);
    let _ = world.session.cancel_admitted_intent(occupied);
    let retry = world.admit_exact(1);
    let _ = world.session.cancel_admitted_intent(retry);
    assert_eq!(execution_census(&world), [0, 0, 0, 0]);
    assert_shutdown_zero(world);
}

fn prove_terminal_matrix() {
    let (provider, observation) = ScriptedProvider::new([
        ExecutionScript::rejected(),
        ExecutionScript::running([AttemptStep::PendingBeforeEffect])
            .with_cancellations([AttemptStep::CancelledBeforeEffect]),
        ExecutionScript::running([AttemptStep::Completed, AttemptStep::FailedBeforeEffect]),
        ExecutionScript::running([
            AttemptStep::PendingBeforeEffect,
            AttemptStep::FailedBeforeEffect,
        ]),
        ExecutionScript::running([AttemptStep::Completed]),
        ExecutionScript::running([AttemptStep::PartialWithoutOutcome])
            .with_recovery([RecoveryStep::Completed]),
        ExecutionScript::running([AttemptStep::Indeterminate])
            .with_recovery([RecoveryStep::Completed]),
    ]);
    let mut world = AdmissionWorld::launch_with_provider(7, provider);
    for target in 0..7 {
        dispatch(&mut world, target, 20);
    }

    let starts = advance(&mut world, 1).into_transitions().into_vec();
    assert_eq!(starts.len(), 7);
    assert!(matches!(
        starts[0].posture(),
        UiIntentExecutionTransitionPosture::RejectedBeforeEffect { .. }
    ));

    let mut settlements = advance(&mut world, 2).into_transitions().into_vec();
    assert_eq!(settlements.len(), 6);
    assert_eq!(
        settlements[0].posture(),
        UiIntentExecutionTransitionPosture::PendingBeforeEffect
    );
    assert!(matches!(
        settlements[1].posture(),
        UiIntentExecutionTransitionPosture::Completed { .. }
    ));
    assert_eq!(
        settlements[2].posture(),
        UiIntentExecutionTransitionPosture::PendingBeforeEffect
    );
    assert!(matches!(
        settlements[3].posture(),
        UiIntentExecutionTransitionPosture::Completed { .. }
    ));
    let indeterminate = settlements
        .pop()
        .unwrap()
        .into_recovery()
        .expect("indeterminate effect retains recovery authority");
    let partial = settlements
        .pop()
        .unwrap()
        .into_recovery()
        .expect("partial effect retains recovery authority");

    world.unmount(1).expect("the pending target unmounts");
    let interrupted = advance(&mut world, 3).into_transitions().into_vec();
    assert_eq!(interrupted.len(), 2);
    assert!(matches!(
        interrupted[0].posture(),
        UiIntentExecutionTransitionPosture::CancelledBeforeEffect { .. }
    ));
    assert!(matches!(
        interrupted[1].posture(),
        UiIntentExecutionTransitionPosture::FailedBeforeEffect { .. }
    ));

    assert_recovery_completed(&mut world, partial, 4);
    assert_recovery_completed(&mut world, indeterminate, 5);
    assert_eq!(execution_census(&world), [0, 0, 0, 4]);
    let shutdown = world.session.shutdown();
    assert_eq!(shutdown.intent_execution().active_after(), 0);
    assert_eq!(shutdown.intent_admission().active_after(), 0);
    assert_eq!(observation.counts(), [7, 7, 1, 2, 6, 2, 1]);
}

fn prove_timeout_retry() {
    let (provider, observation) =
        ScriptedProvider::new([ExecutionScript::running([AttemptStep::Completed])]);
    let mut world = AdmissionWorld::launch_with_provider(1, provider);
    dispatch(&mut world, 0, 0);
    assert!(matches!(
        only(advance(&mut world, 1)).posture(),
        UiIntentExecutionTransitionPosture::TimedOutBeforeEffect { .. }
    ));
    assert_eq!(observation.counts()[0], 0);

    dispatch(&mut world, 0, 10);
    assert_eq!(
        only(advance(&mut world, 2)).posture(),
        UiIntentExecutionTransitionPosture::Started
    );
    assert!(matches!(
        only(advance(&mut world, 3)).posture(),
        UiIntentExecutionTransitionPosture::Completed { .. }
    ));
    assert_eq!(observation.counts()[0..2], [1, 1]);
    assert_shutdown_zero(world);
    assert_eq!(observation.counts()[4..7], [1, 0, 1]);
}

fn prove_separate_domain_admission() {
    let mut world = ConsequenceWorld::launch(UiIntentConsequenceContract::query_collection_change(
        query_identity(),
    ));
    let (mut foreign_owner, consequence, foreign_binding) = foreign_consequence();
    let handle = world.complete_with_consequence(consequence);
    let outcome = world.interaction.session.publish_intent_consequences(
        handle,
        UiRebindExecutionPolicy::ordinary(),
        UiRebindExecutionRequest::new(40),
    );
    assert!(matches!(
        outcome,
        UiIntentConsequencePublicationOutcome::Stopped(ref stop)
            if matches!(
                stop.reason(),
                UiIntentConsequenceStopReason::QueryAdmission(
                    worth_ui_query_binding::WorthUiCollectionChangeAdmissionDenial::ForeignInstalledReference
                )
            )
    ));
    drop(outcome);
    let local = world.query_change_state();
    assert_eq!(local.staged_change_count(), 0);
    assert_eq!(local.admitted_change_count(), 0);
    assert_eq!(world.provider_calls(), [1, 1]);
    world.shutdown();
    assert!(matches!(
        foreign_owner.close_retirement(foreign_binding.into_operation_live_retirement()),
        worth_ui_query_binding::WorthUiOperationLiveRetirementCloseOutcome::Closed(_)
    ));
}

#[derive(Clone, Copy)]
enum BuiltinDestination {
    Transition,
    UnsupportedService,
}

fn destination_world(destination: BuiltinDestination) -> AdmissionWorld {
    let facts = OperabilityFacts::new();
    let input = replacement_input(&facts);
    let host = WorthUiHeadlessRecorder::with_viewport_extent(
        UiHeadlessRecorderCapacity::production_default(),
        UiViewportExtentObservation {
            width: 160.0,
            height: 96.0,
        },
    );
    let builder = FilesystemApplicationLifecycleScenario::new("phase-4-ia-08-destination")
        .visual_identity_application_builder(host)
        .register_intent_boolean_fact(facts.mutability.clone(), true)
        .unwrap()
        .register_intent_boolean_fact(facts.readiness.clone(), true)
        .unwrap()
        .register_intent_boolean_fact(facts.policy.clone(), true)
        .unwrap()
        .register_intent_boolean_fact(facts.confirmation.clone(), false)
        .unwrap();
    let builder = match destination {
        BuiltinDestination::Transition => builder
            .register_intent_transition_definition(
                UiIntentDefinition::<PrimaryIntent>::ui_transition(
                    UiIntentTransitionDestination::NavigatePage,
                ),
            )
            .unwrap(),
        BuiltinDestination::UnsupportedService => builder
            .register_unsupported_intent_definition(
                UiIntentDefinition::<PrimaryIntent>::runtime_service(
                    UiIntentRuntimeServiceDestination::InvokeCommand,
                ),
            )
            .unwrap(),
    };
    let app = builder
        .with_rust_authored_input(input)
        .freeze()
        .expect("the destination world freezes through production preparation");
    AdmissionWorld::launch_application(app, facts, 1)
}

fn assert_recovery_completed(
    world: &mut AdmissionWorld,
    recovery: worth_ui::facade::intent::UiIntentRecoveryHandle,
    tick: u64,
) {
    let UiIntentRecoveryProgressOutcome::Progressed(receipt) = world
        .session
        .retry_intent_recovery(recovery, super::execution_reading(tick))
    else {
        panic!("the exact retained recovery must progress")
    };
    assert_eq!(
        receipt.posture(),
        UiIntentRecoveryProgressPosture::Completed {
            outcome: <EmptyOutcome as worth_ui::facade::intent::UiIntentProductOutcome>::SCHEMA,
        }
    );
    assert!(receipt.into_continuation().is_none());
}

fn only(
    report: worth_ui::facade::intent::UiIntentExecutionAdvanceReport,
) -> UiIntentExecutionTransition {
    let mut transitions = report.into_transitions().into_vec();
    assert_eq!(transitions.len(), 1);
    transitions.pop().unwrap()
}

fn capacity(
    application: usize,
    destination: usize,
) -> UiIntentExecutionCapacityCertificationProfile {
    UiIntentExecutionCapacityCertificationProfile::bounded(application, destination, 16, 16, 4_096)
        .expect("the IA-08 profile only tightens production capacity")
}

fn assert_shutdown_zero(world: AdmissionWorld) {
    let shutdown = world.session.shutdown();
    assert_eq!(shutdown.intent_execution().active_after(), 0);
    assert_eq!(shutdown.intent_admission().active_after(), 0);
}

fn foreign_consequence() -> (
    worth_ui_query_binding::certification::WorthUiOperationLiveTestFixture,
    worth_ui_query_binding::WorthUiCollectionChangeConsequence,
    worth_ui_query_binding::WorthUiRuntimeQueryBinding,
) {
    let mut owner = worth_ui_query_binding::certification::WorthUiOperationLiveTestFixture::new(
        "phase4-ia08-foreign-domain-owner",
    );
    let resource = owner.open_resource();
    let mut binding = owner.binding_plan().prepare_downstream_state();
    binding
        .admit_operation_live(resource)
        .expect("the foreign domain owner retains its own live resource");
    owner.update_measurement();
    let consequence = match binding
        .refresh_operation_live(owner.refresh_request())
        .expect("the foreign domain owner refreshes")
    {
        worth_ui_query_binding::WorthUiOperationLiveRefreshOutcome::Applied(consequence) => {
            consequence
        }
        worth_ui_query_binding::WorthUiOperationLiveRefreshOutcome::NoSemanticDelivery => {
            panic!("the changed foreign domain mints one owner consequence")
        }
    };
    (owner, consequence, binding)
}

fn query_identity() -> worth_ui_query_binding::WorthUiQueryViewIdentity {
    worth_ui_query_binding::WorthUiQueryViewIdentity::new("certification.live.measurements")
        .expect("static consequence Query identity")
}
