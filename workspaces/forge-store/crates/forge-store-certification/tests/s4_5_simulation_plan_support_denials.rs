use forge_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario, CertifiedPhysicalScenario,
    ForbiddenShortcutSet, ObserverKind, OracleFamilyKind, PhysicalDriverKind,
    PhysicalScenarioActor, PhysicalScenarioExpectation, PhysicalScenarioIntent,
    PhysicalScenarioSchedule, PhysicalSimulationCapabilitySet, PhysicalSimulationProfile,
    PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily, SimulationEvidencePolicy,
    SimulationPlanDenial, SimulationPlanningContext, SupportedObserverSet,
    SupportedOracleFamilySet,
};
use forge_store_test_support::{
    admitted_developer_smoke_driver_contracts, NativeStoreAspectFixture,
};

#[test]
fn every_declared_scenario_family_denies_missing_driver_observer_and_oracle_support() {
    for scenario_case in scenario_support_cases() {
        assert_missing_driver_denial(scenario_case);
        assert_missing_observer_denial(scenario_case);
        assert_missing_oracle_denial(scenario_case);
        assert_missing_replay_oracle_denial(scenario_case);
    }
}

#[derive(Debug, Clone, Copy)]
struct ScenarioSupportCase {
    name: &'static str,
    scenario: fn() -> CertifiedPhysicalScenario,
    required_driver: PhysicalDriverKind,
    required_observer: ObserverKind,
    required_oracle: OracleFamilyKind,
}

fn scenario_support_cases() -> [ScenarioSupportCase; 3] {
    [
        ScenarioSupportCase {
            name: "s5 readiness shape",
            scenario: s5_scenario,
            required_driver: PhysicalDriverKind::ProductionBoundaryYieldpoint,
            required_observer: ObserverKind::IndependentPhysicalTrace,
            required_oracle: OracleFamilyKind::PhysicalIsolationReadinessShape,
        },
        ScenarioSupportCase {
            name: "s4 recovery dogfood",
            scenario: s4_scenario,
            required_driver: PhysicalDriverKind::FreshRuntimeRecovery,
            required_observer: ObserverKind::RecoveryOutcomeObserver,
            required_oracle: OracleFamilyKind::S4RecoveryDogfood,
        },
        ScenarioSupportCase {
            name: "shortcut rejection dogfood",
            scenario: shortcut_scenario,
            required_driver: PhysicalDriverKind::ShortcutRejectionBoundary,
            required_observer: ObserverKind::ShortcutRejectionObserver,
            required_oracle: OracleFamilyKind::ForbiddenShortcutRejection,
        },
    ]
}

fn assert_missing_driver_denial(scenario_case: ScenarioSupportCase) {
    let denial = lower_physical_simulation_plan(
        (scenario_case.scenario)(),
        complete_context().with_driver_contracts(
            admitted_developer_smoke_driver_contracts()
                .unwrap()
                .without(scenario_case.required_driver),
        ),
    )
    .expect_err("missing concrete driver must deny before execution");

    assert_eq!(
        denial,
        SimulationPlanDenial::MissingPhysicalDriver(scenario_case.required_driver),
        "{} must deny for its own driver",
        scenario_case.name
    );
}

fn assert_missing_observer_denial(scenario_case: ScenarioSupportCase) {
    let denial = lower_physical_simulation_plan(
        (scenario_case.scenario)(),
        complete_context().with_supported_observers(
            SupportedObserverSet::all_for_developer_smoke()
                .without(scenario_case.required_observer),
        ),
    )
    .expect_err("missing concrete observer must deny before execution");

    assert_eq!(
        denial,
        SimulationPlanDenial::MissingObserver(scenario_case.required_observer),
        "{} must deny for its own observer",
        scenario_case.name
    );
}

fn assert_missing_oracle_denial(scenario_case: ScenarioSupportCase) {
    let denial = lower_physical_simulation_plan(
        (scenario_case.scenario)(),
        complete_context().with_supported_oracle_families(
            SupportedOracleFamilySet::all_for_developer_smoke()
                .without(scenario_case.required_oracle),
        ),
    )
    .expect_err("missing concrete oracle family must deny before execution");

    assert_eq!(
        denial,
        SimulationPlanDenial::MissingOracleFamily(scenario_case.required_oracle),
        "{} must deny for its own oracle family",
        scenario_case.name
    );
}

fn assert_missing_replay_oracle_denial(scenario_case: ScenarioSupportCase) {
    let denial = lower_physical_simulation_plan(
        (scenario_case.scenario)(),
        complete_context().with_supported_oracle_families(
            SupportedOracleFamilySet::all_for_developer_smoke()
                .without(OracleFamilyKind::TranscriptReplayEvidence),
        ),
    )
    .expect_err("missing transcript replay oracle family must deny before execution");

    assert_eq!(
        denial,
        SimulationPlanDenial::MissingOracleFamily(OracleFamilyKind::TranscriptReplayEvidence),
        "{} must deny without generic replay evidence oracle family",
        scenario_case.name
    );
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

fn s5_scenario() -> CertifiedPhysicalScenario {
    physical_scenario("store.physical.s5.readiness.support-denial")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("s5", 5)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .unwrap()
}

fn s4_scenario() -> CertifiedPhysicalScenario {
    physical_scenario("store.physical.s4.recovery.support-denial")
        .family(PhysicalSimulationScenarioFamily::S4RecoveryDogfood)
        .intent(PhysicalScenarioIntent::RecoveryReplayDogfood)
        .fixture(
            NativeStoreAspectFixture::segment_header("s4", 4)
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

fn shortcut_scenario() -> CertifiedPhysicalScenario {
    physical_scenario("store.physical.shortcut.rejection.support-denial")
        .family(PhysicalSimulationScenarioFamily::ShortcutRejectionDogfood)
        .intent(PhysicalScenarioIntent::ForbiddenShortcutRejectionShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("shortcut", 1)
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
