use forge_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario, CrashRecoversOldOrNewNeverMixedOracle,
    DetachedSimulationReplayParts, ExecutedSimulationHarnessAcceptanceSuiteEvidence,
    ExecutedSimulationHarnessAcceptanceSuiteEvidenceSet, ExecutedTranscriptParts,
    FaultDeliveryAttempt, ForbiddenShortcutSet, HarnessCoverageStage, LargeStoreFixtureProfile,
    ObservedPhysicalTrace, PhysicalCertificationEvidenceBundle, PhysicalCoverageRegistry,
    PhysicalFixtureBuilder, PhysicalInterleavingSchedule,
    PhysicalIsolationReadinessShapeProbeScenario,
    PhysicalIsolationReadinessShapeProbeSliceEvidence, PhysicalMutationCoverageEvidence,
    PhysicalScenarioActor, PhysicalScenarioExpectation, PhysicalScenarioIntent,
    PhysicalScenarioSchedule, PhysicalSimulationHarnessCloseoutDenial, PhysicalSimulationPlan,
    PhysicalSimulationProfile, PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily,
    PhysicalSimulationTranscript, ProductionBackedPhysicalFixture, ProductionBoundaryDriverTrace,
    RecoveryOutcomeObservation, ReusablePhysicalOracleFamily, S4RecoveryDogfoodScenario,
    S4RecoveryDogfoodSliceEvidence, ShortcutRejectionDogfoodScenario,
    ShortcutRejectionDogfoodSliceEvidence, ShortcutRejectionObservation, SimulationEvidencePolicy,
    SimulationHarnessAcceptanceSuiteExecutionProof, SimulationHarnessAcceptanceSuiteReceipt,
    SimulationHarnessAcceptanceSuiteReceiptSet, SimulationHarnessCloseoutCoverageReport,
    SimulationHarnessDogfoodEvidence, SimulationReplayBundle, SupportedObserverSet,
    SupportedOracleFamilySet, SyntheticHarnessShortcutRejectionReport,
};
use forge_store_test_support::{
    admitted_developer_smoke_driver_contracts, production_backed_physical_fixture_materialization,
    NativeStoreAspectFixture,
};

use crate::{counter_support, coverage_support};

pub(crate) fn recovery_slice_evidence() -> S4RecoveryDogfoodSliceEvidence {
    recovery_slice_evidence_named(
        "store.physical.s45.closeout.s4-recovery-dogfood",
        "closeout-s4-recovery",
    )
}

pub(crate) fn alternate_recovery_slice_evidence() -> S4RecoveryDogfoodSliceEvidence {
    recovery_slice_evidence_named(
        "store.physical.s45.closeout.s4-recovery-dogfood.alternate",
        "closeout-s4-recovery-alternate",
    )
}

pub(crate) fn shortcut_slice_evidence() -> ShortcutRejectionDogfoodSliceEvidence {
    let scenario = ShortcutRejectionDogfoodScenario::from_public_authoring(
        coverage_support::shortcut_scenario(),
    )
    .unwrap();
    let plan = coverage_support::shortcut_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let matrix = complete_registry_for(scenario.scenario(), &plan, &replay)
        .generate_matrix()
        .unwrap();
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();
    ShortcutRejectionDogfoodSliceEvidence::from_replay_evidence(scenario, matrix, evidence).unwrap()
}

pub(crate) fn physical_isolation_readiness_slice_evidence(
) -> PhysicalIsolationReadinessShapeProbeSliceEvidence {
    let scenario = PhysicalIsolationReadinessShapeProbeScenario::from_public_authoring(
        coverage_support::scenario(),
    )
    .unwrap();
    let plan = coverage_support::lowered_ci_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let matrix = complete_registry_for(scenario.scenario(), &plan, &replay)
        .generate_matrix()
        .unwrap();
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();
    PhysicalIsolationReadinessShapeProbeSliceEvidence::from_replay_evidence(
        scenario, matrix, evidence,
    )
    .unwrap()
}

pub(crate) fn complete_acceptance_suite_receipts(
    dogfood: &SimulationHarnessDogfoodEvidence,
    coverage: &SimulationHarnessCloseoutCoverageReport,
) -> SimulationHarnessAcceptanceSuiteReceiptSet {
    forge_store_physical_certification::PhysicalSimulationHarnessCloseoutSuite::simulation_admission()
        .execute_required_acceptance_suites(complete_executed_acceptance_suites(dogfood, coverage))
        .unwrap()
}

pub(crate) fn acceptance_suite_receipts(
    dogfood: &SimulationHarnessDogfoodEvidence,
    coverage: &SimulationHarnessCloseoutCoverageReport,
) -> Vec<SimulationHarnessAcceptanceSuiteReceipt> {
    complete_acceptance_suite_receipts(dogfood, coverage)
        .receipts()
        .to_vec()
}

pub(crate) fn complete_executed_acceptance_suites(
    dogfood: &SimulationHarnessDogfoodEvidence,
    coverage: &SimulationHarnessCloseoutCoverageReport,
) -> ExecutedSimulationHarnessAcceptanceSuiteEvidenceSet {
    ExecutedSimulationHarnessAcceptanceSuiteEvidenceSet::from_executed_suites(
        executed_acceptance_suites(dogfood, coverage),
    )
    .unwrap()
}

pub(crate) fn executed_acceptance_suites(
    dogfood: &SimulationHarnessDogfoodEvidence,
    coverage: &SimulationHarnessCloseoutCoverageReport,
) -> Vec<ExecutedSimulationHarnessAcceptanceSuiteEvidence> {
    vec![
        executed(SimulationHarnessAcceptanceSuiteExecutionProof::entry_boundary_suite_run(
            dogfood, coverage,
        )),
        executed(
            SimulationHarnessAcceptanceSuiteExecutionProof::aspect_native_scenario_definition_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(
            SimulationHarnessAcceptanceSuiteExecutionProof::simulation_plan_lowering_suite_run(dogfood, coverage),
        ),
        executed(
            SimulationHarnessAcceptanceSuiteExecutionProof::golden_path_authoring_suite_run(dogfood, coverage),
        ),
        executed(
            SimulationHarnessAcceptanceSuiteExecutionProof::production_driver_contract_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(SimulationHarnessAcceptanceSuiteExecutionProof::yieldpoint_control_suite_run(dogfood, coverage)),
        executed(
            SimulationHarnessAcceptanceSuiteExecutionProof::deterministic_schedule_replay_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(
            SimulationHarnessAcceptanceSuiteExecutionProof::fault_delivery_boundary_suite_run(dogfood, coverage),
        ),
        executed(
            SimulationHarnessAcceptanceSuiteExecutionProof::observer_oracle_separation_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(SimulationHarnessAcceptanceSuiteExecutionProof::oracle_library_suite_run(
            dogfood, coverage,
        )),
        executed(
            SimulationHarnessAcceptanceSuiteExecutionProof::counter_contract_profile_suite_run(dogfood, coverage),
        ),
        executed(SimulationHarnessAcceptanceSuiteExecutionProof::counter_strength_suite_run(dogfood, coverage)),
        executed(
            SimulationHarnessAcceptanceSuiteExecutionProof::production_backed_fixture_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(
            SimulationHarnessAcceptanceSuiteExecutionProof::transcript_evidence_bundle_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(
            SimulationHarnessAcceptanceSuiteExecutionProof::coverage_maturity_ladder_suite_run(dogfood, coverage),
        ),
        executed(SimulationHarnessAcceptanceSuiteExecutionProof::generated_coverage_suite_run(dogfood, coverage)),
        executed(
            SimulationHarnessAcceptanceSuiteExecutionProof::forbidden_shortcut_rejection_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(
            SimulationHarnessAcceptanceSuiteExecutionProof::harness_dogfood_vertical_slice_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(
            SimulationHarnessAcceptanceSuiteExecutionProof::extension_slot_containment_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(
            SimulationHarnessAcceptanceSuiteExecutionProof::foundational_proof_simulation_evidence_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(
            SimulationHarnessAcceptanceSuiteExecutionProof::physical_isolation_simulation_harness_readiness_suite_run(
                dogfood, coverage,
            ),
        ),
    ]
}

fn executed(
    proof: Result<
        SimulationHarnessAcceptanceSuiteExecutionProof,
        PhysicalSimulationHarnessCloseoutDenial,
    >,
) -> ExecutedSimulationHarnessAcceptanceSuiteEvidence {
    ExecutedSimulationHarnessAcceptanceSuiteEvidence::from_execution_proof(proof.unwrap())
}

pub(crate) fn complete_shortcut_report() -> SyntheticHarnessShortcutRejectionReport {
    SyntheticHarnessShortcutRejectionReport::from_denied_shortcuts(
        complete_shortcut_denial_receipts(),
    )
    .unwrap()
}

fn recovery_slice_evidence_named(
    scenario_name: &str,
    fixture_name: &str,
) -> S4RecoveryDogfoodSliceEvidence {
    let scenario = S4RecoveryDogfoodScenario::from_public_authoring(
        public_recovery_dogfood_scenario(scenario_name, fixture_name),
    )
    .unwrap();
    let plan = lower_physical_simulation_plan(
        scenario.scenario().clone(),
        closeout_context(PhysicalSimulationProfile::DeveloperSmoke),
    )
    .unwrap();
    let replay = recovery_replay_bundle(&plan);
    let matrix = complete_registry_for(scenario.scenario(), &plan, &replay)
        .generate_matrix()
        .unwrap();
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();
    S4RecoveryDogfoodSliceEvidence::from_replay_evidence(scenario, matrix, evidence).unwrap()
}

fn public_recovery_dogfood_scenario(
    scenario_name: &str,
    fixture_name: &str,
) -> forge_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario(scenario_name)
        .family(PhysicalSimulationScenarioFamily::S4RecoveryDogfood)
        .intent(PhysicalScenarioIntent::RecoveryReplayDogfood)
        .fixture(
            NativeStoreAspectFixture::segment_header(fixture_name, 14)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::recovery_driver("recovery"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "fresh-runtime-replay-open",
        ))
        .expectation(PhysicalScenarioExpectation::recovery_dogfood())
        .certify_definition()
        .unwrap()
}

fn complete_registry_for(
    scenario: &forge_store_physical_certification::CertifiedPhysicalScenario,
    plan: &PhysicalSimulationPlan,
    replay: &SimulationReplayBundle,
) -> PhysicalCoverageRegistry {
    PhysicalCoverageRegistry::for_sequence(HarnessCoverageStage::SimulationAdmission)
        .register_scenario(scenario)
        .unwrap()
        .register_plan(plan)
        .unwrap()
        .register_schedule(replay.schedule())
        .unwrap()
        .register_actor_set()
        .unwrap()
        .register_driver_contracts(plan.driver_contracts())
        .unwrap()
        .register_oracle_verdicts(replay.oracle_verdicts())
        .unwrap()
        .register_counter_receipt(replay.counter_receipt())
        .unwrap()
        .register_transcript(replay)
        .unwrap()
        .register_mutation_result(&mutation_evidence(replay))
        .unwrap()
}

fn mutation_evidence(replay: &SimulationReplayBundle) -> PhysicalMutationCoverageEvidence {
    PhysicalMutationCoverageEvidence::from_replay_private_mutation_denial(
        HarnessCoverageStage::SimulationAdmission,
        replay,
        FaultDeliveryAttempt::private_mutation(),
    )
    .unwrap()
}

fn recovery_replay_bundle(plan: &PhysicalSimulationPlan) -> SimulationReplayBundle {
    let trace = recovery_trace(plan);
    let recovery_verdict = ReusablePhysicalOracleFamily::recovery_dogfood()
        .oracle(CrashRecoversOldOrNewNeverMixedOracle)
        .judge(plan, &trace)
        .unwrap();
    let counter_receipt = counter_support::counter_receipt(plan, trace.clone());
    let parts = ExecutedTranscriptParts::new(
        plan,
        PhysicalInterleavingSchedule::from_lowered_plan(
            plan,
            forge_store_test_support::developer_smoke_replay_seed(),
            forge_store_physical_certification::StateSpaceBudget::bounded_steps(8).unwrap(),
        )
        .unwrap(),
        &production_fixture(),
        trace,
        counter_receipt,
    )
    .unwrap()
    .with_oracle_verdict(recovery_verdict)
    .with_transcript_replay_verdict()
    .unwrap();
    let transcript = PhysicalSimulationTranscript::from_executed_parts(parts).unwrap();
    let detached = DetachedSimulationReplayParts::from_transcript(&transcript);
    drop(transcript);
    detached.admit_replay_bundle().unwrap()
}

fn recovery_trace(plan: &PhysicalSimulationPlan) -> ObservedPhysicalTrace {
    forge_store_physical_certification::PhysicalSimulationObserver::recovery_outcome()
        .observe_plan(plan)
        .unwrap()
        .with_runtime_trace(developer_smoke_production_boundary_trace())
        .with_recovery_outcome_observation(RecoveryOutcomeObservation::recovered_old_root())
        .with_shortcut_rejection_observation(ShortcutRejectionObservation::private_mutation_denied())
        .complete()
        .unwrap()
}

fn developer_smoke_production_boundary_trace() -> ProductionBoundaryDriverTrace {
    admitted_developer_smoke_driver_contracts()
        .unwrap()
        .iter()
        .find_map(|driver| driver.production_boundary_trace())
        .unwrap()
}

fn closeout_context(
    profile: PhysicalSimulationProfile,
) -> forge_store_physical_certification::SimulationPlanningContext {
    forge_store_physical_certification::SimulationPlanningContext::for_profile(profile)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(
            forge_store_physical_certification::PhysicalSimulationCapabilitySet::physical_isolation_readiness_shape_probe(),
        )
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::physical_certification_baseline())
}

fn production_fixture() -> ProductionBackedPhysicalFixture {
    PhysicalFixtureBuilder::production_backed("phase14-closeout-fixture")
        .materialize_with(
            production_backed_physical_fixture_materialization(
                LargeStoreFixtureProfile::StoreLargerThanMemory,
                10,
            )
            .unwrap(),
        )
        .capability(
            forge_store_physical_certification::FixtureCapabilityDeclaration::for_mutation_boundary(
                forge_store_physical_certification::FixtureMutationBoundary::Manifest,
            ),
        )
        .and_reopen_through_physical_authority()
        .unwrap()
}

#[path = "support/shortcut_denials.rs"]
mod shortcut_denials;

use shortcut_denials::complete_shortcut_denial_receipts;
