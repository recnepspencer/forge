use worth_store_formal_models::runner::CanonicalProtocolTrace;
use worth_store_physical_backend::ProductionStorageBoundarySeam;
use worth_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario, AdmittedDriverContractSet,
    CounterMismatchSummary, ForbiddenShortcutSet, OracleVerdictSummary, PhysicalFaultLocus,
    PhysicalInterleavingSchedule, PhysicalScenarioActor, PhysicalScenarioExpectation,
    PhysicalScenarioIntent, PhysicalScenarioSchedule, PhysicalSimulationCapabilitySet,
    PhysicalSimulationProfile, PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily,
    ScheduleFailureClass, ScheduleFailureSignature, ScheduleReplayIdentity, ScheduleShrinkTrace,
    SimulationEvidencePolicy, SimulationPlanningContext, StateSpaceBudget, SupportedObserverSet,
    SupportedOracleFamilySet,
};
use worth_store_test_support::NativeStoreAspectFixture;

use super::mapped_guard::require_mapped_guard;
use super::physical_replay::{ConcreteCounterexampleGuard, CounterexamplePhysicalReplayDenial};
use super::ControlledProtocolMutant;

pub(super) struct ScheduledCounterexampleShrink {
    pub(super) mapped_transcript: CanonicalProtocolTrace,
    pub(super) schedule_identity: ScheduleReplayIdentity,
    pub(super) schedule_shrink: ScheduleShrinkTrace,
}

pub(super) fn shrink_mapped_counterexample_schedule(
    mutant: ControlledProtocolMutant,
    seed: worth_store_physical_certification::ReplaySeed,
    concrete_guard: ConcreteCounterexampleGuard,
    transcript: &CanonicalProtocolTrace,
    mut rerun_guard: impl FnMut() -> ConcreteCounterexampleGuard,
) -> Result<ScheduledCounterexampleShrink, CounterexamplePhysicalReplayDenial> {
    require_mapped_guard(mutant, transcript)?;
    let schedule = counterexample_schedule(mutant, seed)?;
    let steps = schedule.actor_steps().to_vec();
    let owner_step = steps
        .iter()
        .find(|step| step.actor_id() == COUNTEREXAMPLE_OWNER_ACTOR)
        .expect("the certified counterexample scenario declares its owner actor");
    let failure = ScheduleFailureSignature::new(
        ScheduleFailureClass::CounterMismatch,
        PhysicalFaultLocus::from_actor_step(owner_step),
        CounterMismatchSummary::new("controlled-model-edge-disagrees-with-owner-guard"),
        OracleVerdictSummary::satisfied("executed-owner-counterexample-guard"),
    );
    let schedule_shrink = ScheduleShrinkTrace::shrink_reproducing_failure(
        failure.clone(),
        steps.clone(),
        |candidate| {
            (execute_scheduled_owner_guard(candidate, &steps, &mut rerun_guard)
                == Some(concrete_guard))
            .then_some(failure.clone())
        },
    )
    .map_err(CounterexamplePhysicalReplayDenial::ScheduleReplayFailed)?;
    Ok(ScheduledCounterexampleShrink {
        mapped_transcript: transcript.clone(),
        schedule_identity: schedule.identity().clone(),
        schedule_shrink,
    })
}

const SCHEDULE_BOUNDARY_ACTOR: &str = "protocol-schedule-boundary";
const COUNTEREXAMPLE_OWNER_ACTOR: &str = "protocol-counterexample-owner";

fn execute_scheduled_owner_guard(
    candidate: &[worth_store_physical_certification::PhysicalActorStep],
    admitted_steps: &[worth_store_physical_certification::PhysicalActorStep],
    rerun_guard: &mut impl FnMut() -> ConcreteCounterexampleGuard,
) -> Option<ConcreteCounterexampleGuard> {
    if !is_admitted_subsequence(candidate, admitted_steps) {
        return None;
    }
    let mut boundary_steps = 0;
    let mut owner_steps = 0;
    for step in candidate {
        match step.actor_id() {
            SCHEDULE_BOUNDARY_ACTOR => boundary_steps += 1,
            COUNTEREXAMPLE_OWNER_ACTOR => owner_steps += 1,
            _ => return None,
        }
    }
    if boundary_steps != 1 || owner_steps != 1 {
        return None;
    }
    Some(rerun_guard())
}

fn is_admitted_subsequence(
    candidate: &[worth_store_physical_certification::PhysicalActorStep],
    admitted_steps: &[worth_store_physical_certification::PhysicalActorStep],
) -> bool {
    let mut admitted = admitted_steps.iter();
    candidate
        .iter()
        .all(|candidate_step| admitted.by_ref().any(|step| step == candidate_step))
}

fn counterexample_schedule(
    mutant: ControlledProtocolMutant,
    seed: worth_store_physical_certification::ReplaySeed,
) -> Result<PhysicalInterleavingSchedule, CounterexamplePhysicalReplayDenial> {
    let actor = scheduled_actor(mutant);
    let scenario = physical_scenario(format!("store.protocol.counterexample.{mutant:?}"))
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("protocol-counterexample", 1)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::recovery_driver(
            "protocol-schedule-boundary",
        ))
        .actor(actor)
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            counterexample_storage_seam(mutant).token(),
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .map_err(CounterexamplePhysicalReplayDenial::ScheduleDefinitionFailed)?;
    let plan = lower_physical_simulation_plan(scenario, complete_context(mutant)?)
        .map_err(CounterexamplePhysicalReplayDenial::SchedulePlanFailed)?;
    PhysicalInterleavingSchedule::from_lowered_plan(
        &plan,
        seed,
        StateSpaceBudget::bounded_steps(4)
            .map_err(CounterexamplePhysicalReplayDenial::ScheduleReplayFailed)?,
    )
    .map_err(CounterexamplePhysicalReplayDenial::ScheduleReplayFailed)
}

fn complete_context(
    mutant: ControlledProtocolMutant,
) -> Result<SimulationPlanningContext, CounterexamplePhysicalReplayDenial> {
    let drivers =
        AdmittedDriverContractSet::developer_smoke_with_production_storage_yieldpoints(
            crate::courtroom::protocol_models::durability_recovery::scenario::ordinary_durability_profile(),
            [counterexample_storage_seam(mutant)],
        )
        .map_err(|denial| {
            CounterexamplePhysicalReplayDenial::SchedulePlanFailed(
                worth_store_physical_certification::SimulationPlanDenial::DriverAdmissionDenied(
                    denial,
                ),
            )
        })?;
    Ok(
        SimulationPlanningContext::for_profile(PhysicalSimulationProfile::DeveloperSmoke)
            .with_supported_profiles(PhysicalSimulationProfileSet::all())
            .with_capabilities(
                PhysicalSimulationCapabilitySet::physical_isolation_readiness_shape_probe(),
            )
            .with_driver_contracts(drivers)
            .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
            .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
            .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
            .with_forbidden_shortcuts(ForbiddenShortcutSet::physical_certification_baseline()),
    )
}

fn scheduled_actor(mutant: ControlledProtocolMutant) -> PhysicalScenarioActor {
    let id = COUNTEREXAMPLE_OWNER_ACTOR;
    match mutant {
        ControlledProtocolMutant::DurabilityAcknowledgmentBeforeFence => {
            PhysicalScenarioActor::checkpoint_driver(id)
        }
        ControlledProtocolMutant::RecoveryQuarantinedSourceSelected => {
            PhysicalScenarioActor::recovery_driver(id)
        }
        ControlledProtocolMutant::CompactionPublicationBeforeCutover => {
            PhysicalScenarioActor::compaction_driver(id)
        }
        ControlledProtocolMutant::LeaseIdentityReuseWithLiveLease
        | ControlledProtocolMutant::SharedReachableAuthorityReclaimed => {
            PhysicalScenarioActor::maintenance_reclaimer(id)
        }
        ControlledProtocolMutant::QuarantineReleaseWithoutVerification => {
            PhysicalScenarioActor::scrub_driver(id)
        }
        ControlledProtocolMutant::ImportPublicationWithoutDurability
        | ControlledProtocolMutant::ReplicationDivergenceAcceptedAsResume => {
            PhysicalScenarioActor::foreground_writer(id)
        }
    }
}

const fn counterexample_storage_seam(
    mutant: ControlledProtocolMutant,
) -> ProductionStorageBoundarySeam {
    match mutant {
        ControlledProtocolMutant::DurabilityAcknowledgmentBeforeFence => {
            ProductionStorageBoundarySeam::DirectorySync
        }
        ControlledProtocolMutant::RecoveryQuarantinedSourceSelected => {
            ProductionStorageBoundarySeam::RootLoad
        }
        ControlledProtocolMutant::CompactionPublicationBeforeCutover => {
            ProductionStorageBoundarySeam::CompactionCutover
        }
        ControlledProtocolMutant::LeaseIdentityReuseWithLiveLease => {
            ProductionStorageBoundarySeam::ReclaimEligibility
        }
        ControlledProtocolMutant::QuarantineReleaseWithoutVerification => {
            ProductionStorageBoundarySeam::RootSwap
        }
        ControlledProtocolMutant::ImportPublicationWithoutDurability => {
            ProductionStorageBoundarySeam::CheckpointManifestWrite
        }
        ControlledProtocolMutant::ReplicationDivergenceAcceptedAsResume => {
            ProductionStorageBoundarySeam::ReplicationProgressSnapshotDurable
        }
        ControlledProtocolMutant::SharedReachableAuthorityReclaimed => {
            ProductionStorageBoundarySeam::ReclaimEligibility
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MUTANT: ControlledProtocolMutant =
        ControlledProtocolMutant::DurabilityAcknowledgmentBeforeFence;
    const GUARD: ConcreteCounterexampleGuard =
        ConcreteCounterexampleGuard::FailedFencePreventedAcknowledgment;

    #[test]
    fn admitted_schedule_dispatches_the_owner_once() {
        let schedule = counterexample_schedule(
            MUTANT,
            worth_store_physical_certification::ReplaySeed::from_u64(1),
        )
        .expect("counterexample schedule should lower");
        let mut executions = 0;
        let observed = execute_scheduled_owner_guard(
            schedule.actor_steps(),
            schedule.actor_steps(),
            &mut || {
                executions += 1;
                GUARD
            },
        );

        assert_eq!(observed, Some(GUARD));
        assert_eq!(executions, 1);
    }

    #[test]
    fn deleting_either_required_actor_prevents_owner_execution() {
        let schedule = counterexample_schedule(
            MUTANT,
            worth_store_physical_certification::ReplaySeed::from_u64(1),
        )
        .expect("counterexample schedule should lower");
        for removed_actor in [SCHEDULE_BOUNDARY_ACTOR, COUNTEREXAMPLE_OWNER_ACTOR] {
            let candidate = schedule
                .actor_steps()
                .iter()
                .filter(|step| step.actor_id() != removed_actor)
                .cloned()
                .collect::<Vec<_>>();
            let mut executions = 0;
            let observed =
                execute_scheduled_owner_guard(&candidate, schedule.actor_steps(), &mut || {
                    executions += 1;
                    GUARD
                });

            assert_eq!(observed, None, "removed {removed_actor}");
            assert_eq!(executions, 0, "removed {removed_actor}");
        }
    }
}
