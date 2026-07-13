mod evidence;
mod trace;

use super::compaction_observation as compaction_interlock_trace;
use evidence as checkpoint_evidence_support;

pub use trace::actor_step_index;
use trace::developer_smoke_production_trace;

use crate::{
    admitted_developer_smoke_driver_contracts, developer_smoke_replay_seed,
    production_backed_physical_fixture_materialization, NativeStoreAspectFixture,
};
use forge_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario,
    reject_same_run_self_comparison_evidence_attempt, shortcut_denial_from_evidence_bundle_denial,
    DetachedSimulationReplayParts, ExecutedPhysicalSimulationObservation, ExecutedTranscriptParts,
    FixtureCapabilityDeclaration, FixtureMutationBoundary, ForbiddenShortcutSet,
    LargeStoreFixtureProfile, PhysicalFixtureBuilder, PhysicalInterleavingSchedule,
    PhysicalIsolationCheckpointPublicationCrashLaneOutput,
    PhysicalIsolationCheckpointPublicationLaneBinding,
    PhysicalIsolationCheckpointPublicationRecoveryOutcomeLaneOutput,
    PhysicalIsolationCheckpointPublicationScheduledLaneOutput,
    PhysicalIsolationCheckpointPublicationShortcutDenialLaneOutput,
    PhysicalIsolationCheckpointPublicationShortcutRejectionOutput, PhysicalScenarioActor,
    PhysicalScenarioActorRole, PhysicalScenarioExpectation, PhysicalScenarioIntent,
    PhysicalScenarioSchedule, PhysicalSimulationCapabilitySet, PhysicalSimulationObserver,
    PhysicalSimulationPlan, PhysicalSimulationProfile, PhysicalSimulationProfileSet,
    PhysicalSimulationScenarioFamily, PhysicalSimulationTranscript,
    ProductionBackedPhysicalFixture, RecoveryOutcomeObservation, SimulationEvidencePolicy,
    SimulationPlanningContext, StateSpaceBudget, SupportedObserverSet, SupportedOracleFamilySet,
};
use forge_store_physical_isolation::{
    CheckpointInterlockEvidenceOrigin, CheckpointInterlockFoundationalEvidence,
};

pub fn checkpoint_trace(
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

pub fn checkpoint_crash_replay_trace(
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

pub fn checkpoint_crash_replay_trace_without_crash_lane(
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

pub fn lower_checkpoint_plan() -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(checkpoint_scenario(), complete_context()).unwrap()
}

pub fn lower_checkpoint_crash_replay_plan() -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(checkpoint_crash_replay_scenario(), complete_context()).unwrap()
}

pub fn lower_checkpoint_shortcut_plan() -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(checkpoint_shortcut_scenario(), complete_context()).unwrap()
}

pub fn lower_recovery_plan() -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(recovery_scenario(), complete_context()).unwrap()
}

pub fn recovery_trace(
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

pub fn checkpoint_shortcut_trace(
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

pub fn schedule(plan: &PhysicalSimulationPlan) -> PhysicalInterleavingSchedule {
    PhysicalInterleavingSchedule::from_lowered_plan(
        plan,
        developer_smoke_replay_seed(),
        StateSpaceBudget::bounded_steps(8).unwrap(),
    )
    .unwrap()
}

pub fn checkpoint_evidence() -> CheckpointInterlockFoundationalEvidence {
    checkpoint_evidence_for_operation("s45-checkpoint-lane", 10, 20, 12)
}

pub fn copied_checkpoint_evidence() -> CheckpointInterlockFoundationalEvidence {
    checkpoint_evidence_for_operation("s45-copied-checkpoint-lane", 30, 40, 32)
}

pub fn same_origin_copied_checkpoint_evidence() -> CheckpointInterlockFoundationalEvidence {
    CheckpointInterlockFoundationalEvidence::copied_report_attempt_from_store_evidence(
        &checkpoint_evidence(),
    )
}

pub fn checkpoint_origin() -> CheckpointInterlockEvidenceOrigin {
    checkpoint_evidence().origin().clone()
}

pub fn production_fixture() -> ProductionBackedPhysicalFixture {
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

pub fn detached_replay_bundle_from_parts(
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
) -> PhysicalIsolationCheckpointPublicationScheduledLaneOutput {
    let schedule = schedule(plan);
    let binding =
        PhysicalIsolationCheckpointPublicationLaneBinding::from_plan_and_schedule(plan, &schedule)
            .unwrap();
    PhysicalIsolationCheckpointPublicationScheduledLaneOutput::from_schedule_step_evidence(
        &binding,
        &schedule,
        binding.checkpoint_actor_step_index(),
        &checkpoint_origin(),
        checkpoint_evidence(),
    )
    .unwrap()
}

pub fn scheduled_checkpoint_crash_lane_output(
    plan: &PhysicalSimulationPlan,
) -> PhysicalIsolationCheckpointPublicationCrashLaneOutput {
    let checkpoint_schedule = schedule(plan);
    let binding = PhysicalIsolationCheckpointPublicationLaneBinding::from_plan_and_schedule(
        plan,
        &checkpoint_schedule,
    )
    .unwrap();
    let recovery_plan = lower_recovery_plan();
    let recovery_schedule = schedule(&recovery_plan);
    let recovery_trace = recovery_trace(&recovery_plan);
    let recovery_output =
        PhysicalIsolationCheckpointPublicationRecoveryOutcomeLaneOutput::from_fresh_runtime_recovery_trace(
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
    PhysicalIsolationCheckpointPublicationCrashLaneOutput::from_schedule_step_recovery(
        &binding,
        &checkpoint_schedule,
        binding.checkpoint_actor_step_index(),
        &checkpoint_origin(),
        checkpoint_evidence(),
        &recovery_output,
    )
    .unwrap()
}

pub fn scheduled_checkpoint_same_run_shortcut_output(
    plan: &PhysicalSimulationPlan,
) -> PhysicalIsolationCheckpointPublicationShortcutRejectionOutput {
    let schedule = schedule(plan);
    let binding =
        PhysicalIsolationCheckpointPublicationLaneBinding::from_plan_and_schedule(plan, &schedule)
            .unwrap();
    let receipt = shortcut_denial_from_evidence_bundle_denial(
        reject_same_run_self_comparison_evidence_attempt().unwrap_err(),
    )
    .unwrap();
    let denial_output =
        PhysicalIsolationCheckpointPublicationShortcutDenialLaneOutput::from_denial_receipt(
            &binding,
            &schedule,
            actor_step_index(&schedule, PhysicalScenarioActorRole::ShortcutRejectionProbe),
            &checkpoint_origin(),
            checkpoint_evidence(),
            receipt,
        )
        .unwrap();
    PhysicalIsolationCheckpointPublicationShortcutRejectionOutput::from_scheduled_same_run_denial(
        &binding,
        &schedule,
        binding.checkpoint_actor_step_index(),
        &checkpoint_origin(),
        checkpoint_evidence(),
        denial_output,
    )
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
        .with_capabilities(
            PhysicalSimulationCapabilitySet::physical_isolation_readiness_shape_probe(),
        )
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::physical_certification_baseline())
}

fn checkpoint_scenario() -> forge_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s45.phase9.checkpoint-publication-lane")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
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
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .unwrap()
}

fn checkpoint_crash_replay_scenario(
) -> forge_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s45.phase9.checkpoint-publication-crash-replay")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
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
            PhysicalScenarioExpectation::non_claiming_physical_isolation_checkpoint_publication_crash_replay(),
        )
        .certify_definition()
        .unwrap()
}

fn checkpoint_shortcut_scenario() -> forge_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s45.phase9.checkpoint-publication-shortcut")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
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
            PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_with_shortcut_rejection(),
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
        .expectation(PhysicalScenarioExpectation::recovery_dogfood())
        .certify_definition()
        .unwrap()
}
