use forge_store_physical_certification::{
    CheckpointInterlockObservation, CompactionInterlockObservation, CoverageGapDenial,
    ExecutedPhysicalSimulationObservation, IndependentVerifierObservation, ObservedPhysicalTrace,
    PhysicalInterleavingSchedule, PhysicalIsolationCompactionMutationObservationSet,
    PhysicalSimulationObserver, PhysicalSimulationPlan, PhysicalSimulationScenarioFamily,
    ShortcutRejectionObservation,
};

pub struct S5PhysicalIsolationTraceFixtures {
    compaction_interlock: Option<CompactionInterlockObservation>,
    compaction_mutations: Option<PhysicalIsolationCompactionMutationObservationSet>,
    checkpoint_interlock: Option<CheckpointInterlockObservation>,
    independent_verifier: Option<IndependentVerifierObservation>,
}

impl S5PhysicalIsolationTraceFixtures {
    pub const fn complete(
        compaction_interlock: CompactionInterlockObservation,
        compaction_mutations: Option<PhysicalIsolationCompactionMutationObservationSet>,
        checkpoint_interlock: CheckpointInterlockObservation,
        independent_verifier: IndependentVerifierObservation,
    ) -> Self {
        Self {
            compaction_interlock: Some(compaction_interlock),
            compaction_mutations,
            checkpoint_interlock: Some(checkpoint_interlock),
            independent_verifier: Some(independent_verifier),
        }
    }

    pub const fn new(
        compaction_interlock: CompactionInterlockObservation,
        compaction_mutations: Option<PhysicalIsolationCompactionMutationObservationSet>,
        checkpoint_interlock: CheckpointInterlockObservation,
        independent_verifier: IndependentVerifierObservation,
    ) -> Self {
        Self::complete(
            compaction_interlock,
            compaction_mutations,
            checkpoint_interlock,
            independent_verifier,
        )
    }

    pub fn without_compaction_interlock(mut self) -> Self {
        self.compaction_interlock = None;
        self
    }

    pub fn without_compaction_mutations(mut self) -> Self {
        self.compaction_mutations = None;
        self
    }

    pub fn without_checkpoint_interlock(mut self) -> Self {
        self.checkpoint_interlock = None;
        self
    }

    pub fn without_independent_verifier(mut self) -> Self {
        self.independent_verifier = None;
        self
    }
}

pub fn observe_physical_isolation_physical_isolation_trace(
    plan: &PhysicalSimulationPlan,
    _schedule: &PhysicalInterleavingSchedule,
    fixtures: S5PhysicalIsolationTraceFixtures,
) -> Result<ObservedPhysicalTrace, CoverageGapDenial> {
    let execution = ExecutedPhysicalSimulationObservation::from_executed_plan(plan).unwrap();
    let builder = PhysicalSimulationObserver::independent_physical_trace()
        .observe_executed_plan(plan, &execution)
        .unwrap()
        .with_shortcut_rejection_observation(
            ShortcutRejectionObservation::private_mutation_denied(),
        );
    let builder = if let Some(observation) = fixtures.compaction_interlock {
        builder.with_compaction_interlock_observation(observation)
    } else {
        builder
    };
    let builder = if requires_compaction_mutation_observations(plan.scenario_family()) {
        builder.with_scheduled_compaction_mutation_lanes(
            fixtures
                .compaction_mutations
                .ok_or(CoverageGapDenial::MissingMutationResult)?,
        )
    } else {
        builder
    };
    let trace = match plan.scenario_family() {
        PhysicalSimulationScenarioFamily::PhysicalIsolationCheckpointPublicationInterlock
        | PhysicalSimulationScenarioFamily::PhysicalIsolationRestartDuringCutover => {
            let builder = if let Some(observation) = fixtures.checkpoint_interlock {
                builder.with_checkpoint_interlock_observation(observation)
            } else {
                builder
            };
            builder.complete().unwrap()
        }
        PhysicalSimulationScenarioFamily::PhysicalIsolationTierMovementStability
        | PhysicalSimulationScenarioFamily::PhysicalIsolationFutureChunkStability => {
            let builder = if let Some(observation) = fixtures.independent_verifier {
                builder.with_independent_verifier_observation(observation)
            } else {
                builder
            };
            builder.complete().unwrap()
        }
        _ => builder.complete().unwrap(),
    };
    Ok(trace)
}

fn requires_compaction_mutation_observations(family: PhysicalSimulationScenarioFamily) -> bool {
    matches!(
        family,
        PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock
            | PhysicalSimulationScenarioFamily::PhysicalIsolationCheckpointPublicationInterlock
            | PhysicalSimulationScenarioFamily::PhysicalIsolationReclaimReachability
            | PhysicalSimulationScenarioFamily::PhysicalIsolationTierMovementStability
            | PhysicalSimulationScenarioFamily::PhysicalIsolationRestartDuringCutover
    )
}
