use forge_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario, CounterContractKind, FixtureClassKind,
    ForbiddenShortcutKind, ForbiddenShortcutSet, ObserverKind, OracleFamilyKind,
    PhysicalDriverKind, PhysicalScenarioActor, PhysicalScenarioActorRole,
    PhysicalScenarioExpectation, PhysicalScenarioFault, PhysicalScenarioIntent,
    PhysicalScenarioSchedule, PhysicalSimulationCapability, PhysicalSimulationCapabilitySet,
    PhysicalSimulationProfile, PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily,
    SimulationEvidencePolicy, SimulationPlanDenial, SimulationPlanningContext,
    SupportedObserverSet, SupportedOracleFamilySet,
};
use forge_store_test_support::{
    admitted_developer_smoke_driver_contracts, NativeStoreAspectFixture,
};

#[test]
fn equivalent_scenarios_lower_into_same_admitted_plan_identity() {
    let first = lower_s5_plan(s5_scenario(false));
    let second = lower_s5_plan(s5_scenario(true));

    assert_eq!(first.identity(), second.identity());
    assert_eq!(
        first.required_capabilities(),
        second.required_capabilities()
    );
    assert_eq!(first.counter_contracts(), second.counter_contracts());
    assert_eq!(first.evidence_policy(), second.evidence_policy());
    assert!(first.identity().canonical_basis_entry_count() > 0);
    assert_ne!(first.identity().digest_bytes(), &[0; 32]);
}

#[test]
fn s5_readiness_shape_plan_names_all_pre_execution_requirements() {
    let plan = lower_s5_plan(s5_scenario(false));

    assert!(plan
        .required_capabilities()
        .contains(PhysicalSimulationCapability::ProductionBoundaryDriver));
    assert!(plan.actors().contains_actor_id("reader"));
    assert!(plan
        .actors()
        .contains_role(PhysicalScenarioActorRole::MaintenanceReclaimer));
    assert!(plan
        .drivers()
        .contains(PhysicalDriverKind::ProductionBoundaryYieldpoint));
    assert!(plan
        .observers()
        .contains(ObserverKind::IndependentPhysicalTrace));
    assert!(plan
        .oracle_families()
        .contains(OracleFamilyKind::S5ReadinessShape));
    assert!(plan
        .counter_contracts()
        .contains(CounterContractKind::ActorStepExact));
    assert!(plan
        .counter_contracts()
        .contains(CounterContractKind::ReplayIdentityExact));
    assert!(plan
        .fixture_classes()
        .contains(FixtureClassKind::AspectNativeBoundaryFact));
    assert_eq!(
        plan.evidence_policy(),
        SimulationEvidencePolicy::MinimalReplayable
    );
    assert!(plan
        .forbidden_shortcuts()
        .contains(ForbiddenShortcutKind::PrivateMutation));
}

#[test]
fn missing_capabilities_deny_before_plan_construction() {
    for missing in [
        PhysicalSimulationCapability::ProductionBoundaryDriver,
        PhysicalSimulationCapability::IndependentObserver,
        PhysicalSimulationCapability::CertificationOracleFamily,
        PhysicalSimulationCapability::CounterContracts,
        PhysicalSimulationCapability::FixtureClassAdmission,
        PhysicalSimulationCapability::EvidencePolicy,
        PhysicalSimulationCapability::ForbiddenShortcutDenial,
    ] {
        let denial = lower_physical_simulation_plan(
            s5_scenario(false),
            complete_context().with_capabilities(
                PhysicalSimulationCapabilitySet::s5_readiness_shape_probe().without(missing),
            ),
        )
        .expect_err("missing capability must deny before execution");

        assert_eq!(denial, SimulationPlanDenial::MissingCapability(missing));
    }
}

#[test]
fn unsupported_profile_denies_before_execution() {
    let denial = lower_physical_simulation_plan(
        s5_scenario(false),
        complete_context()
            .with_supported_profiles(PhysicalSimulationProfileSet::developer_smoke_only())
            .with_capabilities(PhysicalSimulationCapabilitySet::s5_readiness_shape_probe())
            .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
            .with_forbidden_shortcuts(ForbiddenShortcutSet::roadmap2_baseline())
            .for_release_certification_profile(),
    )
    .expect_err("unsupported profile must deny before execution");

    assert_eq!(
        denial,
        SimulationPlanDenial::UnsupportedProfile(PhysicalSimulationProfile::ReleaseCertification)
    );
}

#[test]
fn every_declared_profile_preserves_the_same_planning_proof_model() {
    let baseline = lower_physical_simulation_plan(
        s5_scenario(false),
        complete_context_for_profile(PhysicalSimulationProfile::DeveloperSmoke),
    )
    .unwrap();

    for profile in [
        PhysicalSimulationProfile::CiCertification,
        PhysicalSimulationProfile::LocalSoak,
        PhysicalSimulationProfile::ReleaseCertification,
        PhysicalSimulationProfile::HardwareQualification,
    ] {
        let plan = lower_physical_simulation_plan(
            s5_scenario(false),
            complete_context_for_profile(profile),
        )
        .unwrap();
        assert_eq!(plan.profile(), profile);
        assert_eq!(plan.actors(), baseline.actors());
        assert_eq!(
            plan.required_capabilities(),
            baseline.required_capabilities()
        );
        assert_eq!(plan.drivers(), baseline.drivers());
        assert_eq!(plan.observers(), baseline.observers());
        assert_eq!(plan.oracle_families(), baseline.oracle_families());
        assert_eq!(plan.counter_contracts(), baseline.counter_contracts());
    }
}

#[test]
fn mismatched_scenario_meaning_denies_instead_of_defaulting_to_s5_shape() {
    let denial = lower_physical_simulation_plan(mismatched_scenario(), complete_context())
        .expect_err("mismatched scenario meaning must not default to S5 shape");

    assert_eq!(
        denial,
        SimulationPlanDenial::UnsupportedScenarioShape {
            family: PhysicalSimulationScenarioFamily::S4RecoveryDogfood,
            expectation: forge_store_physical_certification::PhysicalScenarioExpectationKind::S5ReadinessShapeProbe,
        }
    );
}

#[test]
fn absent_or_incomplete_forbidden_shortcuts_deny_before_execution() {
    let absent_denial = lower_physical_simulation_plan(
        s5_scenario(false),
        complete_context_without_forbidden_shortcuts(),
    )
    .expect_err("absent forbidden shortcut set cannot lower");

    assert_eq!(
        absent_denial,
        SimulationPlanDenial::AbsentForbiddenShortcutSet
    );

    let missing_denial = lower_physical_simulation_plan(
        s5_scenario(false),
        complete_context().with_forbidden_shortcuts(
            ForbiddenShortcutSet::roadmap2_baseline()
                .without(ForbiddenShortcutKind::PrivateMutation),
        ),
    )
    .expect_err("incomplete forbidden shortcut set cannot lower");

    assert_eq!(
        missing_denial,
        SimulationPlanDenial::MissingForbiddenShortcut(ForbiddenShortcutKind::PrivateMutation)
    );
}

#[test]
fn ambiguous_future_fault_scope_denies_before_execution() {
    let denial = lower_physical_simulation_plan(
        physical_scenario("store.physical.future.fault")
            .family(PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe)
            .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
            .fixture(
                NativeStoreAspectFixture::segment_header("alpha", 7)
                    .boundary_fact()
                    .clone(),
            )
            .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
            .actor(PhysicalScenarioActor::foreground_reader("reader"))
            .fault(PhysicalScenarioFault::future_extension_slot())
            .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
                "root-publication-before-observe",
            ))
            .expectation(
                PhysicalScenarioExpectation::non_claiming_s5_readiness_shape()
                    .with_future_extension_non_claim(),
            )
            .certify_definition()
            .unwrap(),
        complete_context(),
    )
    .expect_err("future fault scope is not executable in phase 3");

    assert_eq!(denial, SimulationPlanDenial::AmbiguousFaultScope);
}

#[test]
fn scenario_families_lower_to_distinct_plan_requirements() {
    let s4 = lower_physical_simulation_plan(s4_scenario(), complete_context()).unwrap();
    let s5 = lower_s5_plan(s5_scenario(false));
    let shortcut = lower_physical_simulation_plan(shortcut_scenario(), complete_context()).unwrap();

    assert!(s4
        .drivers()
        .contains(PhysicalDriverKind::FreshRuntimeRecovery));
    assert!(s5
        .drivers()
        .contains(PhysicalDriverKind::ProductionBoundaryYieldpoint));
    assert!(shortcut
        .drivers()
        .contains(PhysicalDriverKind::ShortcutRejectionBoundary));
    assert_ne!(s4.identity(), s5.identity());
    assert_ne!(s5.identity(), shortcut.identity());
}

trait ReleaseProfileContext {
    fn for_release_certification_profile(self) -> Self;
}

impl ReleaseProfileContext for SimulationPlanningContext {
    fn for_release_certification_profile(self) -> Self {
        SimulationPlanningContext::for_profile(PhysicalSimulationProfile::ReleaseCertification)
            .with_supported_profiles(self.supported_profiles().clone())
            .with_capabilities(self.capabilities().clone())
            .with_driver_contracts(self.driver_contracts().clone())
            .with_evidence_policy(self.evidence_policy().unwrap())
            .with_forbidden_shortcuts(self.forbidden_shortcuts().unwrap().clone())
    }
}

fn lower_s5_plan(
    scenario: forge_store_physical_certification::CertifiedPhysicalScenario,
) -> forge_store_physical_certification::PhysicalSimulationPlan {
    lower_physical_simulation_plan(scenario, complete_context()).unwrap()
}

fn complete_context() -> SimulationPlanningContext {
    complete_context_for_profile(PhysicalSimulationProfile::DeveloperSmoke)
}

fn complete_context_for_profile(profile: PhysicalSimulationProfile) -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(profile)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(PhysicalSimulationCapabilitySet::s5_readiness_shape_probe())
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::roadmap2_baseline())
}

fn complete_context_without_forbidden_shortcuts() -> SimulationPlanningContext {
    SimulationPlanningContext::developer_smoke()
        .with_capabilities(PhysicalSimulationCapabilitySet::s5_readiness_shape_probe())
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
}

fn s5_scenario(
    reversed_actor_order: bool,
) -> forge_store_physical_certification::CertifiedPhysicalScenario {
    let fixture = NativeStoreAspectFixture::segment_header("alpha", 7);
    let builder = physical_scenario("store.physical.s5.readiness")
        .family(PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(fixture.boundary_fact().clone());
    let builder = if reversed_actor_order {
        builder
            .actor(PhysicalScenarioActor::foreground_reader("reader"))
            .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
    } else {
        builder
            .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
            .actor(PhysicalScenarioActor::foreground_reader("reader"))
    };
    builder
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_s5_readiness_shape())
        .certify_definition()
        .unwrap()
}

fn s4_scenario() -> forge_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s4.recovery")
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

fn mismatched_scenario() -> forge_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.mismatched")
        .family(PhysicalSimulationScenarioFamily::S4RecoveryDogfood)
        .intent(PhysicalScenarioIntent::RecoveryReplayDogfood)
        .fixture(
            NativeStoreAspectFixture::segment_header("mismatch", 4)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::recovery_driver("recovery"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "fresh-runtime-replay-open",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_s5_readiness_shape())
        .certify_definition()
        .unwrap()
}

fn shortcut_scenario() -> forge_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.shortcut.rejection")
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
