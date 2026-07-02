use forge_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario, CrashRecoversOldOrNewNeverMixedOracle,
    ForbiddenShortcutSet, NoJsonAuthorityOracle, NoPrivateMutationOracle, ObservationDenial,
    OracleDenial, PhysicalProofOracleVerdictKind, PhysicalScenarioActor,
    PhysicalScenarioExpectation, PhysicalScenarioIntent, PhysicalScenarioSchedule,
    PhysicalSimulationCapabilitySet, PhysicalSimulationObserver, PhysicalSimulationPlan,
    PhysicalSimulationProfile, PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily,
    RecoveryOutcomeObservation, ReusablePhysicalOracleFamily, ShortcutRejectionObservation,
    ShortcutRejectionObservationKind, SimulationEvidencePolicy, SimulationPlanningContext,
    SupportedObserverSet, SupportedOracleFamilySet,
};
use forge_store_test_support::{
    admitted_developer_smoke_driver_contracts, NativeStoreAspectFixture,
};

#[test]
fn recovery_oracle_requires_recovery_outcome_and_preserves_mixed_root_failure() {
    let plan = lower_recovery_plan();
    let missing = PhysicalSimulationObserver::recovery_outcome()
        .observe_plan(&plan)
        .unwrap()
        .with_runtime_trace(developer_smoke_production_trace())
        .complete()
        .expect_err("recovery observer must carry a recovery outcome fact");

    assert_eq!(
        missing,
        ObservationDenial::MissingRecoveryOutcomeObservation
    );

    let recovered = ReusablePhysicalOracleFamily::s4_recovery_dogfood()
        .oracle(CrashRecoversOldOrNewNeverMixedOracle)
        .judge(
            &plan,
            &recovery_trace(&plan, RecoveryOutcomeObservation::recovered_old_root()),
        )
        .unwrap();
    let mixed = ReusablePhysicalOracleFamily::s4_recovery_dogfood()
        .oracle(CrashRecoversOldOrNewNeverMixedOracle)
        .judge(
            &plan,
            &recovery_trace(&plan, RecoveryOutcomeObservation::mixed_root()),
        )
        .unwrap();

    assert_eq!(recovered.kind(), PhysicalProofOracleVerdictKind::Satisfied);
    assert_eq!(mixed.kind(), PhysicalProofOracleVerdictKind::Failed);
}

#[test]
fn shortcut_oracles_require_matching_shortcut_rejection_facts() {
    let plan = lower_shortcut_plan();
    let missing = PhysicalSimulationObserver::shortcut_rejection()
        .observe_plan(&plan)
        .unwrap()
        .with_runtime_trace(developer_smoke_production_trace())
        .complete()
        .expect_err("shortcut observer must carry shortcut rejection facts");

    assert_eq!(
        missing,
        ObservationDenial::MissingShortcutRejectionObservation
    );

    let private_only_trace =
        PhysicalSimulationObserver::shortcut_rejection()
            .observe_plan(&plan)
            .unwrap()
            .with_runtime_trace(developer_smoke_production_trace())
            .with_shortcut_rejection_observation(
                ShortcutRejectionObservation::private_mutation_denied(),
            )
            .complete()
            .unwrap();
    let private = ReusablePhysicalOracleFamily::forbidden_shortcut_rejection()
        .oracle(NoPrivateMutationOracle)
        .judge(&plan, &private_only_trace)
        .unwrap();
    let missing_json = ReusablePhysicalOracleFamily::forbidden_shortcut_rejection()
        .oracle(NoJsonAuthorityOracle)
        .judge(&plan, &private_only_trace)
        .expect_err("JSON authority oracle requires its own denial fact");

    assert_eq!(private.kind(), PhysicalProofOracleVerdictKind::Satisfied);
    assert_eq!(
        missing_json,
        OracleDenial::MissingRequiredShortcutRejectionObservation {
            required: ShortcutRejectionObservationKind::JsonAuthorityDenied,
        }
    );

    let complete_trace =
        PhysicalSimulationObserver::shortcut_rejection()
            .observe_plan(&plan)
            .unwrap()
            .with_runtime_trace(developer_smoke_production_trace())
            .with_shortcut_rejection_observation(
                ShortcutRejectionObservation::private_mutation_denied(),
            )
            .with_shortcut_rejection_observation(
                ShortcutRejectionObservation::json_authority_denied(),
            )
            .complete()
            .unwrap();
    let json = ReusablePhysicalOracleFamily::forbidden_shortcut_rejection()
        .oracle(NoJsonAuthorityOracle)
        .judge(&plan, &complete_trace)
        .unwrap();

    assert_eq!(json.kind(), PhysicalProofOracleVerdictKind::Satisfied);
}

fn recovery_trace(
    plan: &PhysicalSimulationPlan,
    observation: RecoveryOutcomeObservation,
) -> forge_store_physical_certification::ObservedPhysicalTrace {
    PhysicalSimulationObserver::recovery_outcome()
        .observe_plan(plan)
        .unwrap()
        .with_runtime_trace(developer_smoke_production_trace())
        .with_recovery_outcome_observation(observation)
        .complete()
        .unwrap()
}

fn lower_recovery_plan() -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(recovery_scenario(), complete_context()).unwrap()
}

fn lower_shortcut_plan() -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(shortcut_scenario(), complete_context()).unwrap()
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

fn recovery_scenario() -> forge_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s45.phase7.recovery.oracle")
        .family(PhysicalSimulationScenarioFamily::S4RecoveryDogfood)
        .intent(PhysicalScenarioIntent::RecoveryReplayDogfood)
        .fixture(
            NativeStoreAspectFixture::segment_header("phase7-recovery", 7)
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

fn shortcut_scenario() -> forge_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s45.phase7.shortcut.oracle")
        .family(PhysicalSimulationScenarioFamily::ShortcutRejectionDogfood)
        .intent(PhysicalScenarioIntent::ForbiddenShortcutRejectionShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("phase7-shortcut", 7)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::shortcut_rejection_probe("probe"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "shortcut-rejection-boundary",
        ))
        .expectation(PhysicalScenarioExpectation::shortcut_rejection_dogfood())
        .certify_definition()
        .unwrap()
}

fn developer_smoke_production_trace(
) -> forge_store_physical_certification::ProductionBoundaryDriverTrace {
    admitted_developer_smoke_driver_contracts()
        .unwrap()
        .iter()
        .find_map(|driver| driver.production_boundary_trace())
        .unwrap()
}
