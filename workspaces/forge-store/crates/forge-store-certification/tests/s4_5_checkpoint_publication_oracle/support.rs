#[path = "../s4_checkpoint_cutover/checkpoint_basis_fixture.rs"]
mod checkpoint_basis_fixture;
#[path = "../s4_checkpoint_cutover/checkpoint_durability_fixture.rs"]
mod checkpoint_durability_fixture;
#[path = "../support/recovery/checkpoint_publication_evidence_support/checkpoint_publication_evidence_support.rs"]
mod checkpoint_evidence_support;
#[path = "../s4_5_counter_strength/compaction_interlock_trace.rs"]
mod compaction_interlock_trace;
#[path = "../s5_epoch_scope_and_root_kind/support.rs"]
mod epoch_support;
#[path = "../s5_stable_read_execution/plan_admission.rs"]
mod plan_admission;

use forge_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario,
    reject_same_run_self_comparison_evidence_attempt, shortcut_denial_from_evidence_bundle_denial,
    DetachedSimulationReplayParts, ExecutedPhysicalSimulationObservation, ExecutedTranscriptParts,
    FixtureCapabilityDeclaration, FixtureMutationBoundary, ForbiddenShortcutSet,
    LargeStoreFixtureProfile, PhysicalFixtureBuilder, PhysicalInterleavingSchedule,
    PhysicalScenarioActor, PhysicalScenarioActorRole, PhysicalScenarioExpectation,
    PhysicalScenarioIntent, PhysicalScenarioSchedule, PhysicalSimulationCapabilitySet,
    PhysicalSimulationObserver, PhysicalSimulationPlan, PhysicalSimulationProfile,
    PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily, PhysicalSimulationTranscript,
    ProductionBackedPhysicalFixture, ProductionBoundaryDriverTrace, RecoveryOutcomeObservation,
    S5CheckpointPublicationCrashLaneOutput, S5CheckpointPublicationLaneBinding,
    S5CheckpointPublicationRecoveryOutcomeLaneOutput, S5CheckpointPublicationScheduledLaneOutput,
    S5CheckpointPublicationShortcutDenialLaneOutput,
    S5CheckpointPublicationShortcutRejectionOutput, SimulationEvidencePolicy,
    SimulationPlanningContext, StateSpaceBudget, SupportedObserverSet, SupportedOracleFamilySet,
};
use forge_store_physical_isolation::{
    CheckpointInterlockEvidenceOrigin, CheckpointInterlockFoundationalEvidence,
};
use forge_store_test_support::{
    admitted_developer_smoke_driver_contracts, developer_smoke_replay_seed,
    production_backed_physical_fixture_materialization, NativeStoreAspectFixture,
};

pub(crate) fn checkpoint_trace(
    plan: &PhysicalSimulationPlan,
) -> forge_store_physical_certification::ObservedPhysicalTrace {
    let execution = ExecutedPhysicalSimulationObservation::from_executed_plan(plan).unwrap();
    PhysicalSimulationObserver::independent_physical_trace()
        .observe_executed_plan(plan, &execution)
        .unwrap()
        .with_scheduled_checkpoint_publication_lane(scheduled_checkpoint_lane_output(plan))
        .unwrap()
        .with_compaction_interlock_observation(
            compaction_interlock_trace::store_compaction_observation(),
        )
        .complete()
        .unwrap()
}

pub(crate) fn checkpoint_crash_replay_trace(
    plan: &PhysicalSimulationPlan,
) -> forge_store_physical_certification::ObservedPhysicalTrace {
    let execution = ExecutedPhysicalSimulationObservation::from_executed_plan(plan).unwrap();
    PhysicalSimulationObserver::independent_physical_trace()
        .observe_executed_plan(plan, &execution)
        .unwrap()
        .with_scheduled_checkpoint_publication_lane(scheduled_checkpoint_lane_output(plan))
        .unwrap()
        .with_scheduled_checkpoint_crash_replay_lane(scheduled_checkpoint_crash_lane_output(plan))
        .unwrap()
        .with_compaction_interlock_observation(
            compaction_interlock_trace::store_compaction_observation(),
        )
        .complete()
        .unwrap()
}

pub(crate) fn checkpoint_crash_replay_trace_without_crash_lane(
    plan: &PhysicalSimulationPlan,
) -> forge_store_physical_certification::ObservedPhysicalTrace {
    let execution = ExecutedPhysicalSimulationObservation::from_executed_plan(plan).unwrap();
    PhysicalSimulationObserver::independent_physical_trace()
        .observe_executed_plan(plan, &execution)
        .unwrap()
        .with_scheduled_checkpoint_publication_lane(scheduled_checkpoint_lane_output(plan))
        .unwrap()
        .with_recovery_outcome_observation(RecoveryOutcomeObservation::recovered_new_root())
        .with_compaction_interlock_observation(
            compaction_interlock_trace::store_compaction_observation(),
        )
        .complete()
        .unwrap()
}

pub(crate) fn lower_checkpoint_plan() -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(checkpoint_scenario(), complete_context()).unwrap()
}

pub(crate) fn lower_checkpoint_crash_replay_plan() -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(checkpoint_crash_replay_scenario(), complete_context()).unwrap()
}

pub(crate) fn lower_checkpoint_shortcut_plan() -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(checkpoint_shortcut_scenario(), complete_context()).unwrap()
}

pub(crate) fn lower_recovery_plan() -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(recovery_scenario(), complete_context()).unwrap()
}

pub(crate) fn recovery_trace(
    plan: &PhysicalSimulationPlan,
) -> forge_store_physical_certification::ObservedPhysicalTrace {
    PhysicalSimulationObserver::recovery_outcome()
        .observe_plan(plan)
        .unwrap()
        .with_runtime_trace(developer_smoke_production_trace())
        .with_recovery_outcome_observation(RecoveryOutcomeObservation::recovered_new_root())
        .with_compaction_interlock_observation(
            compaction_interlock_trace::store_compaction_observation(),
        )
        .complete()
        .unwrap()
}

pub(crate) fn checkpoint_shortcut_trace(
    plan: &PhysicalSimulationPlan,
) -> forge_store_physical_certification::ObservedPhysicalTrace {
    let execution = ExecutedPhysicalSimulationObservation::from_executed_plan(plan).unwrap();
    PhysicalSimulationObserver::shortcut_rejection()
        .observe_executed_plan(plan, &execution)
        .unwrap()
        .with_scheduled_checkpoint_shortcut_rejection_lane(
            scheduled_checkpoint_same_run_shortcut_output(plan),
        )
        .unwrap()
        .complete()
        .unwrap()
}

pub(crate) fn schedule(plan: &PhysicalSimulationPlan) -> PhysicalInterleavingSchedule {
    PhysicalInterleavingSchedule::from_lowered_plan(
        plan,
        developer_smoke_replay_seed(),
        StateSpaceBudget::bounded_steps(8).unwrap(),
    )
    .unwrap()
}

pub(crate) fn checkpoint_evidence() -> CheckpointInterlockFoundationalEvidence {
    checkpoint_evidence_for_operation("s45-checkpoint-lane", 10, 20, 12)
}

pub(crate) fn copied_checkpoint_evidence() -> CheckpointInterlockFoundationalEvidence {
    checkpoint_evidence_for_operation("s45-copied-checkpoint-lane", 30, 40, 32)
}

pub(crate) fn same_origin_copied_checkpoint_evidence() -> CheckpointInterlockFoundationalEvidence {
    CheckpointInterlockFoundationalEvidence::copied_report_attempt_from_store_evidence(
        &checkpoint_evidence(),
    )
}

pub(crate) fn checkpoint_origin() -> CheckpointInterlockEvidenceOrigin {
    checkpoint_evidence().origin().clone()
}

pub(crate) fn production_fixture() -> ProductionBackedPhysicalFixture {
    PhysicalFixtureBuilder::production_backed("phase9-checkpoint-publication-replay")
        .materialize_with(
            production_backed_physical_fixture_materialization(
                LargeStoreFixtureProfile::StoreLargerThanMemory,
                9,
            )
            .unwrap(),
        )
        .capability(FixtureCapabilityDeclaration::for_mutation_boundary(
            FixtureMutationBoundary::Manifest,
        ))
        .and_reopen_through_physical_authority()
        .unwrap()
}

pub(crate) fn detached_replay_bundle_from_parts(
    parts: ExecutedTranscriptParts,
) -> forge_store_physical_certification::SimulationReplayBundle {
    let transcript = PhysicalSimulationTranscript::from_executed_parts(
        parts.with_transcript_replay_verdict().unwrap(),
    )
    .unwrap();
    let detached = DetachedSimulationReplayParts::from_transcript(&transcript);
    drop(transcript);
    detached.admit_replay_bundle().unwrap()
}

fn scheduled_checkpoint_lane_output(
    plan: &PhysicalSimulationPlan,
) -> S5CheckpointPublicationScheduledLaneOutput {
    let schedule = schedule(plan);
    let binding =
        S5CheckpointPublicationLaneBinding::from_plan_and_schedule(plan, &schedule).unwrap();
    S5CheckpointPublicationScheduledLaneOutput::from_schedule_step_evidence(
        &binding,
        &schedule,
        binding.checkpoint_actor_step_index(),
        &checkpoint_origin(),
        checkpoint_evidence(),
    )
    .unwrap()
}

pub(crate) fn scheduled_checkpoint_crash_lane_output(
    plan: &PhysicalSimulationPlan,
) -> S5CheckpointPublicationCrashLaneOutput {
    let checkpoint_schedule = schedule(plan);
    let binding =
        S5CheckpointPublicationLaneBinding::from_plan_and_schedule(plan, &checkpoint_schedule)
            .unwrap();
    let recovery_plan = lower_recovery_plan();
    let recovery_schedule = schedule(&recovery_plan);
    let recovery_trace = recovery_trace(&recovery_plan);
    let recovery_output =
        S5CheckpointPublicationRecoveryOutcomeLaneOutput::from_fresh_runtime_recovery_trace(
            &binding,
            &checkpoint_schedule,
            &recovery_plan,
            &recovery_schedule,
            actor_step_index(
                &recovery_schedule,
                PhysicalScenarioActorRole::RecoveryDriver,
            ),
            &recovery_trace,
            &checkpoint_origin(),
            checkpoint_evidence(),
        )
        .unwrap();
    S5CheckpointPublicationCrashLaneOutput::from_schedule_step_recovery(
        &binding,
        &checkpoint_schedule,
        binding.checkpoint_actor_step_index(),
        &checkpoint_origin(),
        checkpoint_evidence(),
        &recovery_output,
    )
    .unwrap()
}

pub(crate) fn scheduled_checkpoint_same_run_shortcut_output(
    plan: &PhysicalSimulationPlan,
) -> S5CheckpointPublicationShortcutRejectionOutput {
    let schedule = schedule(plan);
    let binding =
        S5CheckpointPublicationLaneBinding::from_plan_and_schedule(plan, &schedule).unwrap();
    let receipt = shortcut_denial_from_evidence_bundle_denial(
        reject_same_run_self_comparison_evidence_attempt().unwrap_err(),
    )
    .unwrap();
    let denial_output = S5CheckpointPublicationShortcutDenialLaneOutput::from_denial_receipt(
        &binding,
        &schedule,
        actor_step_index(&schedule, PhysicalScenarioActorRole::ShortcutRejectionProbe),
        &checkpoint_origin(),
        checkpoint_evidence(),
        receipt,
    )
    .unwrap();
    S5CheckpointPublicationShortcutRejectionOutput::from_scheduled_same_run_denial(
        &binding,
        &schedule,
        binding.checkpoint_actor_step_index(),
        &checkpoint_origin(),
        checkpoint_evidence(),
        denial_output,
    )
    .unwrap()
}

pub(crate) fn actor_step_index(
    schedule: &PhysicalInterleavingSchedule,
    role: PhysicalScenarioActorRole,
) -> usize {
    schedule
        .actor_steps()
        .iter()
        .position(|step| step.actor_role() == role)
        .unwrap()
}

fn checkpoint_evidence_for_operation(
    operation_digest: &str,
    covered_start: u64,
    covered_end: u64,
    redo_boundary: u64,
) -> CheckpointInterlockFoundationalEvidence {
    checkpoint_evidence_support::checkpoint_evidence_for_operation(
        operation_digest,
        covered_start,
        covered_end,
        redo_boundary,
    )
}

fn complete_context() -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(PhysicalSimulationProfile::DeveloperSmoke)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(PhysicalSimulationCapabilitySet::s5_readiness_shape_probe())
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::roadmap2_baseline())
}

fn checkpoint_scenario() -> forge_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s45.phase9.checkpoint-publication-lane")
        .family(PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("phase9-checkpoint-publication", 9)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::checkpoint_driver("checkpoint"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_s5_readiness_shape())
        .certify_definition()
        .unwrap()
}

fn checkpoint_crash_replay_scenario(
) -> forge_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s45.phase9.checkpoint-publication-crash-replay")
        .family(PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("phase9-checkpoint-crash-replay", 9)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::checkpoint_driver("checkpoint"))
        .actor(PhysicalScenarioActor::recovery_driver("recovery"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(
            PhysicalScenarioExpectation::non_claiming_s5_checkpoint_publication_crash_replay(),
        )
        .certify_definition()
        .unwrap()
}

fn checkpoint_shortcut_scenario() -> forge_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s45.phase9.checkpoint-publication-shortcut")
        .family(PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("phase9-checkpoint-shortcut", 9)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::checkpoint_driver("checkpoint"))
        .actor(PhysicalScenarioActor::shortcut_rejection_probe("shortcut"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(
            PhysicalScenarioExpectation::non_claiming_s5_readiness_with_shortcut_rejection(),
        )
        .certify_definition()
        .unwrap()
}

fn recovery_scenario() -> forge_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s45.phase9.checkpoint-recovery-replay")
        .family(PhysicalSimulationScenarioFamily::S4RecoveryDogfood)
        .intent(PhysicalScenarioIntent::RecoveryReplayDogfood)
        .fixture(
            NativeStoreAspectFixture::segment_header("phase9-checkpoint-recovery", 9)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::recovery_driver("recovery"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "fresh-runtime-replay-open",
        ))
        .expectation(PhysicalScenarioExpectation::s4_recovery_dogfood())
        .certify_definition()
        .unwrap()
}

fn developer_smoke_production_trace() -> ProductionBoundaryDriverTrace {
    admitted_developer_smoke_driver_contracts()
        .unwrap()
        .iter()
        .find_map(|driver| driver.production_boundary_trace())
        .unwrap()
}
