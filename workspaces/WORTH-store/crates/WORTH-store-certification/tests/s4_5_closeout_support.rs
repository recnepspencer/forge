use worth_proof::{Recipe, Unresolved};
use worth_store_physical_certification::{
    fixture_label_oracle_attempt, lower_physical_simulation_plan, physical_scenario,
    reject_raw_json_scenario_authority_attempt, reject_same_run_self_comparison_evidence_attempt,
    reject_terminal_json_evidence_attempt, reject_unresolved_simulation_plan_recipe,
    shortcut_denial_from_evidence_bundle_denial, shortcut_denial_from_fault_delivery_denial,
    shortcut_denial_from_oracle_denial, shortcut_denial_from_plan_denial,
    shortcut_denial_from_scenario_denial, shortcut_denial_from_terminal_projection_denial,
    shortcut_denial_from_transcript_denial, test_support_oracle_verdict_attempt,
    CrashRecoversOldOrNewNeverMixedOracle, DetachedSimulationReplayParts, ExecutedTranscriptParts,
    FaultDeliveryAttempt, ForbiddenShortcutSet, LargeStoreFixtureProfile, ObservedPhysicalTrace,
    PhysicalCertificationEvidenceBundle, PhysicalFixtureBuilder, PhysicalInterleavingSchedule,
    PhysicalMutationCoverageEvidence, PhysicalScenarioActor, PhysicalScenarioExpectation,
    PhysicalScenarioIntent, PhysicalScenarioSchedule, PhysicalSimulationHarnessCloseoutDenial,
    PhysicalSimulationPlan, PhysicalSimulationProfile, PhysicalSimulationProfileSet,
    PhysicalSimulationScenarioFamily, PhysicalSimulationTranscript,
    ProductionBackedPhysicalFixture, ProductionBoundaryDriverTrace, RecoveryOutcomeObservation,
    ReusablePhysicalOracleFamily, Roadmap2CoverageRegistry, Roadmap2HarnessSequence,
    S45AcceptanceSuiteExecutionProof, S45AcceptanceSuiteReceipt, S45AcceptanceSuiteReceiptSet,
    S45CloseoutCoverageReport, S45ExecutedAcceptanceSuiteEvidence,
    S45ExecutedAcceptanceSuiteEvidenceSet, S45HarnessDogfoodEvidence, S4RecoveryDogfoodScenario,
    S4RecoveryDogfoodSliceEvidence, S5ReadinessShapeProbeScenario,
    S5ReadinessShapeProbeSliceEvidence, ShortcutRejectionDogfoodScenario,
    ShortcutRejectionDogfoodSliceEvidence, ShortcutRejectionObservation, SimulationEvidencePolicy,
    SimulationReplayBundle, SupportedObserverSet, SupportedOracleFamilySet,
    SyntheticHarnessShortcutDenialReceipt, SyntheticHarnessShortcutRejectionReport,
};
use worth_store_test_support::{
    admitted_developer_smoke_driver_contracts, production_backed_physical_fixture_materialization,
    NativeStoreAspectFixture,
};

use crate::{counter_support, coverage_support};

pub(crate) fn s4_recovery_slice_evidence() -> S4RecoveryDogfoodSliceEvidence {
    s4_recovery_slice_evidence_named(
        "store.physical.s45.closeout.s4-recovery-dogfood",
        "closeout-s4-recovery",
    )
}

pub(crate) fn alternate_s4_recovery_slice_evidence() -> S4RecoveryDogfoodSliceEvidence {
    s4_recovery_slice_evidence_named(
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

pub(crate) fn s5_readiness_slice_evidence() -> S5ReadinessShapeProbeSliceEvidence {
    let scenario =
        S5ReadinessShapeProbeScenario::from_public_authoring(coverage_support::scenario()).unwrap();
    let plan = coverage_support::lowered_ci_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let matrix = complete_registry_for(scenario.scenario(), &plan, &replay)
        .generate_matrix()
        .unwrap();
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();
    S5ReadinessShapeProbeSliceEvidence::from_replay_evidence(scenario, matrix, evidence).unwrap()
}

pub(crate) fn complete_acceptance_suite_receipts(
    dogfood: &S45HarnessDogfoodEvidence,
    coverage: &S45CloseoutCoverageReport,
) -> S45AcceptanceSuiteReceiptSet {
    worth_store_physical_certification::PhysicalSimulationHarnessCloseoutSuite::roadmap2_s45()
        .execute_required_acceptance_suites(complete_executed_acceptance_suites(dogfood, coverage))
        .unwrap()
}

pub(crate) fn acceptance_suite_receipts(
    dogfood: &S45HarnessDogfoodEvidence,
    coverage: &S45CloseoutCoverageReport,
) -> Vec<S45AcceptanceSuiteReceipt> {
    complete_acceptance_suite_receipts(dogfood, coverage)
        .receipts()
        .to_vec()
}

pub(crate) fn complete_executed_acceptance_suites(
    dogfood: &S45HarnessDogfoodEvidence,
    coverage: &S45CloseoutCoverageReport,
) -> S45ExecutedAcceptanceSuiteEvidenceSet {
    S45ExecutedAcceptanceSuiteEvidenceSet::from_executed_suites(executed_acceptance_suites(
        dogfood, coverage,
    ))
    .unwrap()
}

pub(crate) fn executed_acceptance_suites(
    dogfood: &S45HarnessDogfoodEvidence,
    coverage: &S45CloseoutCoverageReport,
) -> Vec<S45ExecutedAcceptanceSuiteEvidence> {
    vec![
        executed(S45AcceptanceSuiteExecutionProof::entry_boundary_suite_run(
            dogfood, coverage,
        )),
        executed(
            S45AcceptanceSuiteExecutionProof::aspect_native_scenario_definition_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(
            S45AcceptanceSuiteExecutionProof::simulation_plan_lowering_suite_run(dogfood, coverage),
        ),
        executed(
            S45AcceptanceSuiteExecutionProof::golden_path_authoring_suite_run(dogfood, coverage),
        ),
        executed(
            S45AcceptanceSuiteExecutionProof::production_driver_contract_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(S45AcceptanceSuiteExecutionProof::yieldpoint_control_suite_run(dogfood, coverage)),
        executed(
            S45AcceptanceSuiteExecutionProof::deterministic_schedule_replay_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(
            S45AcceptanceSuiteExecutionProof::fault_delivery_boundary_suite_run(dogfood, coverage),
        ),
        executed(
            S45AcceptanceSuiteExecutionProof::observer_oracle_separation_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(S45AcceptanceSuiteExecutionProof::oracle_library_suite_run(
            dogfood, coverage,
        )),
        executed(
            S45AcceptanceSuiteExecutionProof::counter_contract_profile_suite_run(dogfood, coverage),
        ),
        executed(S45AcceptanceSuiteExecutionProof::counter_strength_suite_run(dogfood, coverage)),
        executed(
            S45AcceptanceSuiteExecutionProof::production_backed_fixture_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(
            S45AcceptanceSuiteExecutionProof::transcript_evidence_bundle_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(
            S45AcceptanceSuiteExecutionProof::coverage_maturity_ladder_suite_run(dogfood, coverage),
        ),
        executed(S45AcceptanceSuiteExecutionProof::generated_coverage_suite_run(dogfood, coverage)),
        executed(
            S45AcceptanceSuiteExecutionProof::forbidden_shortcut_rejection_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(
            S45AcceptanceSuiteExecutionProof::harness_dogfood_vertical_slice_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(
            S45AcceptanceSuiteExecutionProof::extension_slot_containment_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(
            S45AcceptanceSuiteExecutionProof::foundational_proof_simulation_evidence_suite_run(
                dogfood, coverage,
            ),
        ),
        executed(
            S45AcceptanceSuiteExecutionProof::s5_simulation_harness_readiness_suite_run(
                dogfood, coverage,
            ),
        ),
    ]
}

fn executed(
    proof: Result<S45AcceptanceSuiteExecutionProof, PhysicalSimulationHarnessCloseoutDenial>,
) -> S45ExecutedAcceptanceSuiteEvidence {
    S45ExecutedAcceptanceSuiteEvidence::from_execution_proof(proof.unwrap())
}

pub(crate) fn complete_shortcut_report() -> SyntheticHarnessShortcutRejectionReport {
    SyntheticHarnessShortcutRejectionReport::from_denied_shortcuts(
        complete_shortcut_denial_receipts(),
    )
    .unwrap()
}

fn s4_recovery_slice_evidence_named(
    scenario_name: &str,
    fixture_name: &str,
) -> S4RecoveryDogfoodSliceEvidence {
    let scenario = S4RecoveryDogfoodScenario::from_public_authoring(
        public_s4_recovery_dogfood_scenario(scenario_name, fixture_name),
    )
    .unwrap();
    let plan = lower_physical_simulation_plan(
        scenario.scenario().clone(),
        closeout_context(PhysicalSimulationProfile::DeveloperSmoke),
    )
    .unwrap();
    let replay = s4_recovery_replay_bundle(&plan);
    let matrix = complete_registry_for(scenario.scenario(), &plan, &replay)
        .generate_matrix()
        .unwrap();
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();
    S4RecoveryDogfoodSliceEvidence::from_replay_evidence(scenario, matrix, evidence).unwrap()
}

fn public_s4_recovery_dogfood_scenario(
    scenario_name: &str,
    fixture_name: &str,
) -> worth_store_physical_certification::CertifiedPhysicalScenario {
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
        .expectation(PhysicalScenarioExpectation::s4_recovery_dogfood())
        .certify_definition()
        .unwrap()
}

fn complete_registry_for(
    scenario: &worth_store_physical_certification::CertifiedPhysicalScenario,
    plan: &PhysicalSimulationPlan,
    replay: &SimulationReplayBundle,
) -> Roadmap2CoverageRegistry {
    Roadmap2CoverageRegistry::for_sequence(Roadmap2HarnessSequence::S45)
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
        Roadmap2HarnessSequence::S45,
        replay,
        FaultDeliveryAttempt::private_mutation(),
    )
    .unwrap()
}

fn s4_recovery_replay_bundle(plan: &PhysicalSimulationPlan) -> SimulationReplayBundle {
    let trace = s4_recovery_trace(plan);
    let recovery_verdict = ReusablePhysicalOracleFamily::s4_recovery_dogfood()
        .oracle(CrashRecoversOldOrNewNeverMixedOracle)
        .judge(plan, &trace)
        .unwrap();
    let counter_receipt = counter_support::counter_receipt(plan, trace.clone());
    let parts = ExecutedTranscriptParts::new(
        plan,
        PhysicalInterleavingSchedule::from_lowered_plan(
            plan,
            worth_store_test_support::developer_smoke_replay_seed(),
            worth_store_physical_certification::StateSpaceBudget::bounded_steps(8).unwrap(),
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

fn s4_recovery_trace(plan: &PhysicalSimulationPlan) -> ObservedPhysicalTrace {
    worth_store_physical_certification::PhysicalSimulationObserver::recovery_outcome()
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
) -> worth_store_physical_certification::SimulationPlanningContext {
    worth_store_physical_certification::SimulationPlanningContext::for_profile(profile)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(
            worth_store_physical_certification::PhysicalSimulationCapabilitySet::s5_readiness_shape_probe(),
        )
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::roadmap2_baseline())
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
            worth_store_physical_certification::FixtureCapabilityDeclaration::for_mutation_boundary(
                worth_store_physical_certification::FixtureMutationBoundary::Manifest,
            ),
        )
        .and_reopen_through_physical_authority()
        .unwrap()
}

fn complete_shortcut_denial_receipts() -> Vec<SyntheticHarnessShortcutDenialReceipt> {
    vec![
        shortcut_denial_from_evidence_bundle_denial(
            worth_store_physical_certification::reject_loose_log_evidence_attempt().unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_scenario_denial(
            reject_raw_json_scenario_authority_attempt(r#"{"scenario":"fake"}"#).unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_terminal_projection_denial(
            reject_terminal_json_evidence_attempt().unwrap_err(),
        ),
        shortcut_denial_from_evidence_bundle_denial(
            reject_same_run_self_comparison_evidence_attempt().unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_fault_delivery_denial(
            FaultDeliveryAttempt::private_mutation()
                .admit()
                .unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_oracle_denial(fixture_label_oracle_attempt().unwrap_err()).unwrap(),
        shortcut_denial_from_transcript_denial(
            worth_store_physical_certification::reject_copied_transcript_fields().unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_plan_denial(
            reject_unresolved_simulation_plan_recipe(Recipe::<Unresolved, _>::new(
                coverage_support::shortcut_plan(),
            ))
            .unwrap_err(),
        )
        .unwrap(),
        shortcut_denial_from_oracle_denial(test_support_oracle_verdict_attempt().unwrap_err())
            .unwrap(),
    ]
}
